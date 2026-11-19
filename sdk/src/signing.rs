// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use oxide_http_signatures::{
    content_digest, sign, Components, SigningKey, CONTENT_DIGEST_HEADER, SIGNATURE_HEADER,
    SIGNATURE_INPUT_HEADER,
};
use reqwest::header::HeaderValue;

use crate::Client;

pub(crate) struct RequestSigner {
    pub(crate) key_id: String,
    pub(crate) key: SigningKey,
}

#[derive(Clone, Default)]
pub struct ClientState {
    signer: Option<Arc<RequestSigner>>,
}

impl ClientState {
    pub(crate) fn with_signer(signer: RequestSigner) -> Self {
        Self {
            signer: Some(Arc::new(signer)),
        }
    }
}

impl std::fmt::Debug for ClientState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientState")
            .field(
                "signer_key_id",
                &self.signer.as_ref().map(|s| s.key_id.as_str()),
            )
            .finish()
    }
}

impl Client {
    pub fn signs_requests(&self) -> bool {
        progenitor_client::ClientInfo::inner(self).signer.is_some()
    }
}

pub(crate) fn sign_request(state: &ClientState, request: &mut reqwest::Request) -> Result<(), String> {
    let Some(signer) = state.signer.as_ref() else {
        return Ok(());
    };

    let digest = match request.body() {
        None => content_digest(b""),
        Some(body) => match body.as_bytes() {
            Some(bytes) => content_digest(bytes),
            None => request
                .headers()
                .get(CONTENT_DIGEST_HEADER)
                .ok_or_else(|| {
                    format!(
                        "request signing requires a {CONTENT_DIGEST_HEADER} header \
                         on requests with a streaming body"
                    )
                })?
                .to_str()
                .map_err(|e| format!("invalid {CONTENT_DIGEST_HEADER} header: {e}"))?
                .to_string(),
        },
    };

    let url = request.url();
    let host = url
        .host_str()
        .ok_or_else(|| format!("cannot sign request to {url}: no host"))?;
    let authority = match url.port() {
        Some(port) => format!("{host}:{port}"),
        None => host.to_string(),
    };
    let created = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as i64;

    let headers = sign(
        &signer.key,
        &signer.key_id,
        created,
        &Components {
            method: request.method().as_str(),
            authority: &authority,
            path: url.path(),
            query: url.query(),
            content_digest: &digest,
        },
    );

    let header = |value: &str| HeaderValue::from_str(value).map_err(|e| e.to_string());
    let content_digest = header(&digest)?;
    let signature_input = header(&headers.signature_input)?;
    let signature = header(&headers.signature)?;
    let request_headers = request.headers_mut();
    request_headers.insert(CONTENT_DIGEST_HEADER, content_digest);
    request_headers.insert(SIGNATURE_INPUT_HEADER, signature_input);
    request_headers.insert(SIGNATURE_HEADER, signature);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_http_signatures::parse;
    use progenitor_client::{ClientHooks, ClientInfo, OperationInfo};

    const KEY_ID: &str = "0b9c1f4e-6a8f-4d55-9a8e-2f7f4c1d6e3a";
    const BASEURL: &str = "https://rack.example:12220";

    fn signing_client() -> (Client, SigningKey) {
        let key = SigningKey::from_bytes(&[7; 32]);
        let state = ClientState::with_signer(RequestSigner {
            key_id: KEY_ID.to_string(),
            key: key.clone(),
        });
        (Client::new(BASEURL, state).unwrap(), key)
    }

    async fn run_pre(client: &Client, request: &mut reqwest::Request) -> Result<(), String> {
        let info = OperationInfo {
            operation_id: "test",
        };
        client
            .pre::<()>(request, &info)
            .await
            .map_err(|e| e.to_string())
    }

    fn assert_verifies(
        request: &reqwest::Request,
        key: &SigningKey,
        authority: &str,
        path: &str,
        query: Option<&str>,
        digest: &str,
    ) {
        let headers = request.headers();
        assert_eq!(headers[CONTENT_DIGEST_HEADER], digest);
        assert!(headers.get(reqwest::header::AUTHORIZATION).is_none());
        let parsed = parse(
            headers[SIGNATURE_INPUT_HEADER].to_str().unwrap(),
            headers[SIGNATURE_HEADER].to_str().unwrap(),
        )
        .unwrap();
        assert_eq!(parsed.key_id, KEY_ID);
        parsed
            .verify(
                &key.verifying_key(),
                &Components {
                    method: request.method().as_str(),
                    authority,
                    path,
                    query,
                    content_digest: digest,
                },
            )
            .unwrap();
    }

    #[tokio::test]
    async fn signs_json_body() {
        let (client, key) = signing_client();
        let mut request = client
            .client()
            .post(format!("{BASEURL}/v1/projects"))
            .json(&serde_json::json!({ "name": "p", "description": "d" }))
            .build()
            .unwrap();
        let body = request.body().unwrap().as_bytes().unwrap().to_vec();
        run_pre(&client, &mut request).await.unwrap();
        assert_verifies(
            &request,
            &key,
            "rack.example:12220",
            "/v1/projects",
            None,
            &content_digest(&body),
        );
    }

    #[tokio::test]
    async fn signs_empty_get_with_query() {
        let (client, key) = signing_client();
        let mut request = client
            .client()
            .get("https://rack.example:443/v1/instances?project=p&limit=5")
            .build()
            .unwrap();
        run_pre(&client, &mut request).await.unwrap();
        assert_verifies(
            &request,
            &key,
            "rack.example",
            "/v1/instances",
            Some("project=p&limit=5"),
            &content_digest(b""),
        );
    }

    #[tokio::test]
    async fn signs_streaming_body_with_preset_digest() {
        let (client, key) = signing_client();
        let chunks = || {
            reqwest::Body::wrap_stream(futures::stream::iter([
                Ok::<_, std::io::Error>(b"tuf".to_vec()),
                Ok(b"repo".to_vec()),
            ]))
        };
        let digest = content_digest(b"tufrepo");

        let mut request = client
            .client()
            .put(format!(
                "{BASEURL}/v1/system/update/repositories?file_name=r.zip"
            ))
            .header(CONTENT_DIGEST_HEADER, &digest)
            .body(chunks())
            .build()
            .unwrap();
        run_pre(&client, &mut request).await.unwrap();
        assert_verifies(
            &request,
            &key,
            "rack.example:12220",
            "/v1/system/update/repositories",
            Some("file_name=r.zip"),
            &digest,
        );

        let mut request = client
            .client()
            .put(format!("{BASEURL}/v1/system/update/repositories"))
            .body(chunks())
            .build()
            .unwrap();
        let err = run_pre(&client, &mut request).await.unwrap_err();
        assert!(err.contains(CONTENT_DIGEST_HEADER), "{err}");
    }
}
