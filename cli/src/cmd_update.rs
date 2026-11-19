// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use std::path::{Path, PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use futures::TryStreamExt;
use oxide::{
    oxide_http_signatures::{DigestHasher, CONTENT_DIGEST_HEADER},
    types, Client, ClientInfo, ClientSystemUpdateExt, ResponseValue,
};
use progenitor_client::{ClientHooks, OperationInfo};
use tokio::{fs::File, io::AsyncReadExt, sync::watch};
use tokio_util::io::ReaderStream;

use crate::{generated_cli::CliConfig, util::start_progress_bar, AuthenticatedCmd, OxideOverride};

/// Upload a system release repository
///
/// The repository is a signed archive of the artifacts that make up a system
/// release. After uploading it, use
/// `oxide system update target-release update` to make that release the
/// rack's target.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
pub struct CmdUpload {
    /// Path to the TUF repository
    #[clap(short, long)]
    path: PathBuf,
}

#[async_trait]
impl AuthenticatedCmd for CmdUpload {
    async fn run(&self, client: &Client) -> Result<()> {
        let file_name = self.path.file_name().unwrap().to_str().unwrap();

        let content_digest = if client.signs_requests() {
            Some(file_content_digest(&self.path).await?)
        } else {
            None
        };

        let file = File::open(&self.path).await?;

        let len = file.metadata().await?.len();

        let (progress_tx, progress_rx) = watch::channel(0);

        let mut position = 0;

        let file_stream = ReaderStream::new(file).inspect_ok(move |buf| {
            position += buf.len();
            let _ = progress_tx.send(position as u64);
        });

        let _pb = start_progress_bar(
            progress_rx,
            len,
            &format!("Uploading {}...", self.path.to_string_lossy()),
        );

        let body = reqwest::Body::wrap_stream(file_stream);

        let response = match content_digest {
            Some(digest) => upload_with_content_digest(client, file_name, body, digest).await,
            None => {
                client
                    .system_update_repository_upload()
                    .file_name(file_name)
                    .body(body)
                    .send()
                    .await
            }
        };

        // TODO probably we want to figure out the output modality for all
        // commands, but let's figure that out once we have multiple output
        // modes.
        let out = OxideOverride::default();

        match response {
            Ok(r) => {
                out.success_item(&r);
                Ok(())
            }
            Err(err) => {
                out.error(&err);
                Err(anyhow::Error::new(err))
            }
        }
    }

    fn maybe_long() -> bool {
        true
    }
}

async fn file_content_digest(path: &Path) -> Result<String> {
    let mut file = File::open(path).await?;
    let mut hasher = DigestHasher::new();
    let mut buf = vec![0; 1 << 20];
    loop {
        let n = file.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finish())
}

async fn upload_with_content_digest(
    client: &Client,
    file_name: &str,
    body: reqwest::Body,
    content_digest: String,
) -> Result<ResponseValue<types::TufRepoUpload>, oxide::Error<types::Error>> {
    let info = OperationInfo {
        operation_id: "system_update_repository_upload",
    };
    let mut request = client
        .client()
        .put(format!(
            "{}/v1/system/update/repositories",
            client.baseurl()
        ))
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::CONTENT_TYPE, "application/octet-stream")
        .header("api-version", Client::api_version())
        .header(CONTENT_DIGEST_HEADER, content_digest)
        .query(&[("file_name", file_name)])
        .body(body)
        .build()?;
    client.pre::<types::Error>(&mut request, &info).await?;
    let response = client.exec(request, &info).await?;
    match response.status().as_u16() {
        200 => ResponseValue::from_response(response).await,
        400..=599 => Err(oxide::Error::ErrorResponse(
            ResponseValue::from_response::<types::Error>(response).await?,
        )),
        _ => Err(oxide::Error::UnexpectedResponse(response)),
    }
}
