// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::{
    fs::{read_to_string, write},
    path::Path,
};

use assert_cmd::Command;
use expectorate::assert_contents;
use httpmock::{Method::POST, Mock, MockServer};
use oxide::types::CurrentUser;
use oxide_httpmock::MockServerExt;
use serde_json::json;

fn scrub_server(raw: String, server: String) -> String {
    raw.replace(&server, "<TEST-SERVER>")
}
struct MockOAuth<'a> {
    device_auth: Mock<'a>,
    device_token: Mock<'a>,
    me: Mock<'a>,
}

impl<'a> MockOAuth<'a> {
    fn new(server: &'a MockServer) -> Self {
        let device_auth = server.mock(|when, then| {
            let body = json!({
                "device_code": "DEV-CODE",
                "user_code": "0X1-D3C",
                "verification_uri": "http://go.here.to/verify",
                "expires_in": 10,
            });
            when.method(POST).path("/device/auth");
            then.status(200)
                .json_body(body)
                .header("content-type", "application/json");
        });

        // This is where we'd poll, but let's just wave them through.
        let device_token = server.mock(|when, then| {
            let body = json!({
                "access_token": "123-456-789",
                "token_type": "Bearer",
            });
            when.method(POST).path("/device/token");
            then.delay(std::time::Duration::from_secs(1))
                .status(200)
                .json_body(body)
                .header("content-type", "application/json");
        });

        // User and silo identity now that we're "authenticated".
        let me = server.current_user_view(|when, then| {
            when.into_inner().any_request();
            then.ok(&CurrentUser {
                display_name: "falken".to_string(),
                id: "831dedf4-0a66-4b04-a232-b610f9f8924c".parse().unwrap(),
                silo_id: "12e8c7a4-399f-41e2-985e-7b120ecbcc1a".parse().unwrap(),
                silo_name: "crystal-palace".try_into().unwrap(),
            });
        });
        Self {
            device_auth,
            device_token,
            me,
        }
    }

    fn assert(&self) {
        self.device_auth.assert();
        self.device_token.assert();
        self.me.assert();
    }

    fn assert_hits(&self, hits: usize) {
        self.device_auth.assert_hits(hits);
        self.device_token.assert_hits(hits);
        self.me.assert_hits(hits);
    }
}

// Test the first login where no config files exist yet.
#[test]
fn test_auth_login_first() {
    let server = MockServer::start();
    let mock = MockOAuth::new(&server);

    let temp_dir = tempfile::tempdir().unwrap().into_path();
    let config_dir = temp_dir.join(".config").join("oxide");

    let cmd = Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .arg("--config-dir")
        .arg(config_dir.as_os_str())
        .arg("auth")
        .arg("login")
        .arg("--no-browser")
        .arg("--host")
        .arg(server.url(""))
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&cmd.get_output().stdout);

    assert_contents(
        "tests/data/test_auth_login_first.stdout",
        &scrub_server(stdout.to_string(), server.url("")),
    );

    mock.assert();

    assert_contents(
        "tests/data/test_auth_login_first_credentials.toml",
        &scrub_server(
            read_to_string(temp_dir.join("credentials.toml")).unwrap(),
            server.url(""),
        ),
    );

    assert_contents(
        "tests/data/test_auth_login_first_config.toml",
        &read_to_string(temp_dir.join("config.toml")).unwrap(),
    );
}

fn write_first_creds(dir: &Path) {
    let cred_path = dir.join("credentials.toml");
    let creds = "\
        [profile.first]\n\
        host = \"https://oxide.internal\"\n\
        token = \"***-***-***\"\n\
        user = \"00000000-0000-0000-0000-000000000000\"\n\
    ";
    write(cred_path, creds).unwrap();
}
fn write_first_config(dir: &Path) {
    let config_path = dir.join("config.toml");
    let config = "\
        default-profile = \"first\"
    ";
    write(config_path, config).unwrap();
}

#[test]
fn test_auth_login_existing_default() {
    let server = MockServer::start();
    let mock = MockOAuth::new(&server);

    let temp_dir = tempfile::tempdir().unwrap().into_path();
    write_first_creds(&temp_dir);
    write_first_config(&temp_dir);

    let cmd = Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .arg("--profile")
        .arg("crystal-palace")
        .arg("--config-dir")
        .arg(temp_dir.as_os_str())
        .arg("auth")
        .arg("login")
        .arg("--no-browser")
        .arg("--host")
        .arg(server.url(""))
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&cmd.get_output().stdout);

    assert_contents(
        "tests/data/test_auth_existing_default.stdout",
        &scrub_server(stdout.to_string(), server.url("")),
    );

    mock.assert();

    assert_contents(
        "tests/data/test_auth_existing_default_credentials.toml",
        &scrub_server(
            read_to_string(temp_dir.join("credentials.toml")).unwrap(),
            server.url(""),
        ),
    );

    assert_contents(
        "tests/data/test_auth_existing_default_config.toml",
        &read_to_string(temp_dir.join("config.toml")).unwrap(),
    );
}

#[test]
fn test_auth_login_existing_no_default() {
    let server = MockServer::start();
    let mock = MockOAuth::new(&server);

    let temp_dir = tempfile::tempdir().unwrap().into_path();
    write_first_creds(&temp_dir);

    let cmd = Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .arg("--config-dir")
        .arg(temp_dir.as_os_str())
        .arg("auth")
        .arg("login")
        .arg("--no-browser")
        .arg("--host")
        .arg(server.url(""))
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&cmd.get_output().stdout);

    assert_contents(
        "tests/data/test_auth_existing_no_default.stdout",
        &scrub_server(stdout.to_string(), server.url("")),
    );

    mock.assert();

    assert_contents(
        "tests/data/test_auth_existing_no_default_credentials.toml",
        &scrub_server(
            read_to_string(temp_dir.join("credentials.toml")).unwrap(),
            server.url(""),
        ),
    );

    assert_contents(
        "tests/data/test_auth_existing_no_default_config.toml",
        &read_to_string(temp_dir.join("config.toml")).unwrap(),
    );
}

#[test]
fn test_auth_login_double() {
    let server = MockServer::start();
    let mock = MockOAuth::new(&server);

    let temp_dir = tempfile::tempdir().unwrap().into_path();

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .arg("--config-dir")
        .arg(temp_dir.as_os_str())
        .arg("auth")
        .arg("login")
        .arg("--no-browser")
        .arg("--host")
        .arg(server.url(""))
        .assert()
        .success();
    let cmd = Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .arg("--config-dir")
        .arg(temp_dir.as_os_str())
        .arg("auth")
        .arg("login")
        .arg("--no-browser")
        .arg("--host")
        .arg(server.url(""))
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&cmd.get_output().stdout);

    assert_contents(
        "tests/data/test_auth_double.stdout",
        &scrub_server(stdout.to_string(), server.url("")),
    );

    mock.assert_hits(2);

    assert_contents(
        "tests/data/test_auth_double_credentials.toml",
        &scrub_server(
            read_to_string(temp_dir.join("credentials.toml")).unwrap(),
            server.url(""),
        ),
    );

    assert_contents(
        "tests/data/test_auth_double_config.toml",
        &read_to_string(temp_dir.join("config.toml")).unwrap(),
    );
}

#[test]
fn test_cmd_auth_status() {
    let server = MockServer::start();

    let temp_dir = tempfile::tempdir().unwrap().into_path();
    let cred_path = temp_dir.join("credentials.toml");
    let creds = format!(
        "\
        [profile.lightman]\n\
        host = \"{}\"\n\
        token = \"***-***-*ok\"\n\
        user = \"00000000-0000-0000-0000-000000000000\"\n\
        \n\
        [profile.jennifer]\n\
        host = \"{}\"\n\
        token = \"***-***-*ok\"\n\
        user = \"00000000-0000-0000-0000-000000000001\"\n\
        \n\
        [profile.malvin]\n\
        host = \"{}\"\n\
        token = \"***-***-bad\"\n\
        user = \"00000000-0000-0000-0000-000000000002\"\n\
        \n\
        [profile.sting]\n\
        host = \"https://unresolvabledomainnameihope\"\n\
        token = \"***-***-***\"\n\
        user = \"00000000-0000-0000-0000-000000000002\"\n\
        \n\
        ",
        server.url(""),
        server.url(""),
        server.url(""),
    );
    write(cred_path, creds).unwrap();

    let ok = server.current_user_view(|when, then| {
        when.into_inner()
            .header("authorization", "Bearer ***-***-*ok");

        then.ok(&oxide::types::CurrentUser {
            display_name: "privileged".to_string(),
            id: "001de000-05e4-4000-8000-000000004007".parse().unwrap(),
            silo_id: "d1bb398f-872c-438c-a4c6-2211e2042526".parse().unwrap(),
            silo_name: "funky-town".parse().unwrap(),
        });
    });
    let bad = server.current_user_view(|when, then| {
        when.into_inner()
            .header("authorization", "Bearer ***-***-bad");

        then.client_error(
            401,
            &oxide::types::Error {
                error_code: None,
                message: "** IMPROPER REQUEST **".to_string(),
                request_id: "42".to_string(),
            },
        );
    });

    // Validate authenticated credentials
    let cmd = Command::cargo_bin("oxide")
        .unwrap()
        .arg("--config-dir")
        .arg(temp_dir.as_os_str())
        .arg("auth")
        .arg("status")
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&cmd.get_output().stdout);

    // DNS failure output can vary by platform
    let stdout = match stdout.find("dns error:") {
        Some(ii) => &stdout[..ii],
        None => &stdout,
    };

    assert_contents(
        "tests/data/test_auth_status.stdout",
        &scrub_server(stdout.to_string(), server.url("")),
    );

    ok.assert_hits(2);
    bad.assert();
}

#[test]
fn test_cmd_auth_status_env() {
    let server = MockServer::start();

    let oxide_mock = server.current_user_view(|when, then| {
        when.into_inner()
            .header("authorization", "Bearer oxide-token-good");

        then.ok(&oxide::types::CurrentUser {
            display_name: "privileged".to_string(),
            id: "001de000-05e4-4000-8000-000000004007".parse().unwrap(),
            silo_id: "d1bb398f-872c-438c-a4c6-2211e2042526".parse().unwrap(),
            silo_name: "funky-town".parse().unwrap(),
        });
    });

    // Validate authenticated credentials
    Command::cargo_bin("oxide")
        .unwrap()
        .arg("auth")
        .arg("status")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "oxide-token-good")
        .assert()
        .success()
        .stdout(format!(
            "Logged in to {} as 001de000-05e4-4000-8000-000000004007\n",
            server.url("")
        ));

    oxide_mock.assert();

    let oxide_mock = server.current_user_view(|when, then| {
        when.into_inner()
            .header("authorization", "Bearer oxide-token-bad");
        then.server_error(
            500,
            &oxide::types::Error {
                error_code: None,
                message: "oops".to_string(),
                request_id: "42".to_string(),
            },
        );
    });

    // Try invalid credentials.
    Command::cargo_bin("oxide")
        .unwrap()
        .arg("auth")
        .arg("status")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "oxide-token-bad")
        .assert()
        .success()
        .stdout(format!("{}: Error Response: oops\n", server.url("")));
    oxide_mock.assert();
}
