// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

//! This test involves starting a dropshot server in order to validate
//! TLS certificates. This mechanism should not be copied! Instead use httpmock
//! or--better--generated mocks from the local sdk-httpmock crate.

use assert_cmd::Command;
use dropshot::{
    endpoint, ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, ConfigTls,
    HttpError, HttpResponseUpdatedNoContent, HttpServerStarter, RequestContext,
};

#[endpoint {
    method = GET,
    path = "/aok"
}]
async fn aok(_: RequestContext<()>) -> Result<HttpResponseUpdatedNoContent, HttpError> {
    Ok(HttpResponseUpdatedNoContent())
}

#[tokio::test]
async fn test_resolve_and_cacert() {
    const HOSTNAME: &str = "fake.cloud.oxide.computer";
    let cert = rcgen::generate_simple_self_signed(vec![HOSTNAME.to_string()]).unwrap();

    let mut api = ApiDescription::new();
    api.register(aok).unwrap();

    let tempdir = tempfile::tempdir().unwrap();

    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Critical,
    }
    .to_logger("test")
    .unwrap();

    let cert_path = tempdir.path().join("cert.pem");
    std::fs::write(cert_path.clone(), cert.serialize_pem().unwrap()).unwrap();

    let key_path = tempdir.path().join("key.pem");
    std::fs::write(key_path.clone(), cert.serialize_private_key_pem()).unwrap();

    let server = HttpServerStarter::new_with_tls(
        &ConfigDropshot {
            bind_address: "127.0.0.1:0".parse().unwrap(),
            request_body_max_bytes: 1024,
            default_handler_task_mode: dropshot::HandlerTaskMode::CancelOnDisconnect,
        },
        api,
        (),
        &log,
        Some(ConfigTls::AsFile {
            cert_file: cert_path.clone(),
            key_file: key_path,
        }),
    )
    .map_err(|error| format!("failed to start server: {}", error))
    .unwrap()
    .start();

    let addr = server.local_addr();

    // This is non-async, blocking code so we need to shove it into its own
    // task. We're just looking for the command to succeed.
    tokio::task::spawn_blocking(move || {
        Command::cargo_bin("oxide")
            .unwrap()
            .env(
                "OXIDE_HOST",
                format!("https://{}:{}", HOSTNAME, addr.port()),
            )
            .env("OXIDE_TOKEN", "fake-token")
            .arg("--resolve")
            .arg(format!("{}:{}:{}", HOSTNAME, addr.port(), addr.ip()))
            .arg("--cacert")
            .arg(cert_path)
            .arg("api")
            .arg("/aok")
            .assert()
            .success();
    })
    .await
    .unwrap();

    server.close().await.unwrap();
}
