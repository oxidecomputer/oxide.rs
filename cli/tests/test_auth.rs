// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::MockServer;
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use serde_json::json;

#[test]
fn test_auth_login_browser() {
    let mut _src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let device_auth_request_mock = server.device_auth_request(|when, then| {
        when.into_inner().any_request();
        then.default_response(
            200,
            json!({
                "device_code": "foodevicecode",
                "user_code": "foousercode",
                "verification_uri": server.url("/verify"),
                "expires_in": 10000
            }),
        );
    });

    let device_access_token_mock = server.device_access_token(|when, then| {
        when.into_inner().any_request();
        then.default_response(
            200,
            json!({
                "access_token": "footoken",
                "token_type": "bearer",
                "expires_in": 10000
            }),
        );
    });

    // TODO: Use server.current_user_view(config_fn) instead. It was not working last it was tried.
    let current_user_view_mock = server.mock(|when, then| {
        when.path_contains("v1/me");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "display_name": "foo",
                "id": "bf8f5c05-2aa6-47f2-9f8a-92896e1fe175",
                "silo_id": "aa09e636-e452-466d-a3b3-45e24c49da61",
                "silo_name": "foo-silo"
            }));
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .arg("auth")
        .arg("login")
        .arg("--host")
        .arg(server.url(""))
        .write_stdin("y")
        .assert()
        .success()
        .stdout(format!(
            r#"Copy your one-time code:
  foousercode
Press ENTER to open {} in your browser...CurrentUser {{
    display_name: "foo",
    id: bf8f5c05-2aa6-47f2-9f8a-92896e1fe175,
    silo_id: aa09e636-e452-466d-a3b3-45e24c49da61,
    silo_name: Name(
        "foo-silo",
    ),
}}
Logged in as bf8f5c05-2aa6-47f2-9f8a-92896e1fe175
"#,
            server.url("/verify")
        ));

    device_auth_request_mock.assert();
    device_access_token_mock.assert();
    current_user_view_mock.assert();
}

#[test]
fn test_auth_login_no_browser() {
    let mut _src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let device_auth_request_mock = server.device_auth_request(|when, then| {
        when.into_inner().any_request();
        then.default_response(
            200,
            json!({
                "device_code": "foodevicecode",
                "user_code": "foousercode",
                "verification_uri": server.url("/verify"),
                "expires_in": 10000
            }),
        );
    });

    let device_access_token_mock = server.device_access_token(|when, then| {
        when.into_inner().any_request();
        then.default_response(
            200,
            json!({
                "access_token": "footoken",
                "token_type": "bearer",
                "expires_in": 10000
            }),
        );
    });

    // TODO: Use server.current_user_view(config_fn) instead. It was not working last it was tried.
    let current_user_view_mock = server.mock(|when, then| {
        when.path_contains("v1/me");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "display_name": "foo",
                "id": "bf8f5c05-2aa6-47f2-9f8a-92896e1fe175",
                "silo_id": "aa09e636-e452-466d-a3b3-45e24c49da61",
                "silo_name": "foo-silo"
            }));
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .arg("auth")
        .arg("login")
        .arg("--no-browser")
        .arg("--host")
        .arg(server.url(""))
        .assert()
        .success()
        .stdout(format!(
            r#"Copy your one-time code:
  foousercode
Open this URL in your browser:
  {}
CurrentUser {{
    display_name: "foo",
    id: bf8f5c05-2aa6-47f2-9f8a-92896e1fe175,
    silo_id: aa09e636-e452-466d-a3b3-45e24c49da61,
    silo_name: Name(
        "foo-silo",
    ),
}}
Logged in as bf8f5c05-2aa6-47f2-9f8a-92896e1fe175
"#,
            server.url("/verify")
        ));

    device_auth_request_mock.assert();
    device_access_token_mock.assert();
    current_user_view_mock.assert();
}
