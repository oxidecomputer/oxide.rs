// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use std::{
    fs::{read_to_string, write, File},
    path::Path,
};

use chrono::{DateTime, Utc};
use expectorate::assert_contents;
use httpmock::{Method::POST, Mock, MockServer};
use oxide::types::CurrentUser;
use oxide_httpmock::MockServerExt;
use predicates::prelude::*;
use predicates::str;
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
                "token_id": "xyz",
                "time_expires": DateTime::<Utc>::from_timestamp(0x8000_0000, 0).unwrap().to_string(),
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
                fleet_viewer: false,
                silo_admin: false,
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

/// Assert the mode of a file on Unix. Does nothing on Windows.
#[track_caller]
fn assert_mode(path: &Path, expected_mode: u32) {
    #[cfg(not(unix))]
    {
        // Avoid unused parameter warnings on Windows.
        let _ = path;
        let _ = expected_mode;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let file = std::fs::File::open(path).unwrap();
        let stat = file.metadata().unwrap();
        let mode = stat.permissions().mode();

        let umask = get_umask();
        let derived_mode = expected_mode & !umask;

        // Validate only the bottom nine permission bits.
        if mode & 0o777 != derived_mode & 0o777 {
            panic!("assertion failed: modes do not match for {}\n  expected: 0o{:o}\n    actual: 0o{:o}\n     umask: 0o{umask:03o}", path.display(), derived_mode & 0o777, mode & 0o777);
        }
    }
}

#[cfg(unix)]
fn get_umask() -> u32 {
    // Taken from https://github.com/rust-lang/cargo/blob/0473ee8b87dc7dbee53d13065c204ae63a0a2a9e/src/cargo/util/mod.rs#L143-L159
    use std::sync::OnceLock;

    static UMASK: OnceLock<u32> = OnceLock::new();

    // We cannot retrieve umask without modifying it. Set it to zero, then
    // immediately revert it to the original value. Store this so that we have a
    // consistent value for the life of the program.
    *UMASK.get_or_init(|| unsafe {
        let umask = libc::umask(0);
        libc::umask(umask);
        umask as u32
    })
}

// Test the first login where no config files exist yet.
#[test]
fn test_auth_login_first() {
    let server = MockServer::start();
    let mock = MockOAuth::new(&server);

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    // Make sure we know how to make non-existent directories.
    let config_dir = temp_dir_path.join(".config").join("oxide");

    let cmd = assert_cmd::cargo::cargo_bin_cmd!("oxide")
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
            read_to_string(config_dir.join("credentials.toml")).unwrap(),
            server.url(""),
        ),
    );
    assert_mode(&config_dir.join("credentials.toml"), 0o600);

    assert_contents(
        "tests/data/test_auth_login_first_config.toml",
        &read_to_string(config_dir.join("config.toml")).unwrap(),
    );
    assert_mode(&config_dir.join("config.toml"), 0o644);
}

fn write_creds(dir: &Path, creds: &str) {
    let cred_path = dir.join("credentials.toml");
    write(&cred_path, creds).unwrap();

    // On Unix set permissions to 0600 to avoid triggering permissions warning.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let file = File::open(&cred_path).unwrap();
        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o600);
        file.set_permissions(perms).unwrap();
    }
}

fn write_first_creds(dir: &Path) {
    write_creds(
        dir,
        "\
        [profile.first]\n\
        host = \"https://oxide.internal\"\n\
        token = \"***-***-***\"\n\
        user = \"00000000-0000-0000-0000-000000000000\"\n\
    ",
    );
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

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    write_first_creds(temp_dir_path);
    let creds_path = temp_dir_path.join("credentials.toml");
    assert_mode(&creds_path, 0o600);

    write_first_config(temp_dir_path);
    assert_mode(&temp_dir_path.join("config.toml"), 0o644);

    let cmd = assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .arg("--profile")
        .arg("crystal-palace")
        .arg("--config-dir")
        .arg(temp_dir_path.as_os_str())
        .arg("auth")
        .arg("login")
        .arg("--no-browser")
        .arg("--host")
        .arg(server.url(""))
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&cmd.get_output().stdout);

    let creds_path = temp_dir_path.join("credentials.toml");
    assert_contents(
        "tests/data/test_auth_existing_default.stdout",
        &scrub_server(stdout.to_string(), server.url("")),
    );

    mock.assert();

    assert_contents(
        "tests/data/test_auth_existing_default_credentials.toml",
        &scrub_server(read_to_string(&creds_path).unwrap(), server.url("")),
    );
    assert_mode(&creds_path, 0o600);

    assert_contents(
        "tests/data/test_auth_existing_default_config.toml",
        &read_to_string(temp_dir_path.join("config.toml")).unwrap(),
    );
    assert_mode(&temp_dir_path.join("config.toml"), 0o644);
}

#[test]
fn test_auth_login_existing_no_default() {
    let server = MockServer::start();
    let mock = MockOAuth::new(&server);

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    write_first_creds(temp_dir_path);
    assert_mode(&temp_dir_path.join("credentials.toml"), 0o600);

    let cmd = assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .arg("--config-dir")
        .arg(temp_dir_path.as_os_str())
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
            read_to_string(temp_dir_path.join("credentials.toml")).unwrap(),
            server.url(""),
        ),
    );
    assert_mode(&temp_dir_path.join("credentials.toml"), 0o600);

    assert_contents(
        "tests/data/test_auth_existing_no_default_config.toml",
        &read_to_string(temp_dir_path.join("config.toml")).unwrap(),
    );
    assert_mode(&temp_dir_path.join("config.toml"), 0o644);
}

#[test]
#[cfg(unix)]
fn test_auth_credentials_permissions() {
    let server = MockServer::start();

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();
    let cred_path = temp_dir_path.join("credentials.toml");
    let creds = format!(
        "\
        [profile.lightman]\n\
        host = \"{}\"\n\
        token = \"***-***-*ok\"\n\
        user = \"00000000-0000-0000-0000-000000000000\"\n\
        \n\
        ",
        server.url(""),
    );
    write(&cred_path, creds).unwrap();
    assert_mode(&cred_path, 0o644);

    // Validate authenticated credentials
    let cmd = assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .arg("--config-dir")
        .arg(temp_dir_path.as_os_str())
        .arg("auth")
        .arg("status")
        .assert()
        .success();
    let stderr = String::from_utf8_lossy(&cmd.get_output().stderr);

    fn scrub_creds(raw: String, path: &Path) -> String {
        let path = path.to_string_lossy().to_string();
        raw.replace(&path, "<CREDENTIALS-PATH>")
    }

    assert_contents(
        "tests/data/test_auth_credentials_permissions.stderr",
        &scrub_creds(stderr.to_string(), &cred_path),
    );
    assert_mode(&cred_path, 0o644);
}

#[test]
fn test_auth_login_double() {
    let server = MockServer::start();
    let mock = MockOAuth::new(&server);

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .arg("--config-dir")
        .arg(temp_dir_path.as_os_str())
        .arg("auth")
        .arg("login")
        .arg("--no-browser")
        .arg("--host")
        .arg(server.url(""))
        .assert()
        .success();
    let cmd = assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .arg("--config-dir")
        .arg(temp_dir_path.as_os_str())
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
            read_to_string(temp_dir_path.join("credentials.toml")).unwrap(),
            server.url(""),
        ),
    );
    assert_mode(&temp_dir_path.join("credentials.toml"), 0o600);

    assert_contents(
        "tests/data/test_auth_double_config.toml",
        &read_to_string(temp_dir_path.join("config.toml")).unwrap(),
    );
    assert_mode(&temp_dir_path.join("config.toml"), 0o644);
}

#[test]
fn test_cmd_auth_status() {
    let server = MockServer::start();

    let temp_dir = tempfile::tempdir().unwrap();
    let cred_path = temp_dir.path().join("credentials.toml");
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
    write(&cred_path, creds).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let file = File::open(&cred_path).unwrap();
        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o600);
        file.set_permissions(perms).unwrap()
    }

    let empty_creds_dir = tempfile::tempdir().unwrap();
    File::create(empty_creds_dir.path().join("credentials.toml")).unwrap();

    let ok = server.current_user_view(|when, then| {
        when.into_inner()
            .header("authorization", "Bearer ***-***-*ok");

        then.ok(&oxide::types::CurrentUser {
            display_name: "privileged".to_string(),
            id: "001de000-05e4-4000-8000-000000004007".parse().unwrap(),
            silo_id: "d1bb398f-872c-438c-a4c6-2211e2042526".parse().unwrap(),
            silo_name: "funky-town".parse().unwrap(),
            fleet_viewer: false,
            silo_admin: false,
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
    let cmd = assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .arg("--config-dir")
        .arg(temp_dir.path().as_os_str())
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

    // Validate empty `credentials.toml` does not error.
    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .arg("--config-dir")
        .arg(empty_creds_dir.path().as_os_str())
        .arg("auth")
        .arg("status")
        .assert()
        .success()
        .stdout(str::is_empty());

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
            fleet_viewer: false,
            silo_admin: false,
        });
    });

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    write_creds(
        temp_dir_path,
        &format!(
            "\
        [profile.funky-town]\n\
        host = \"{}\"\n\
        token = \"oxide-token-good\"\n\
        user = \"00000000-0000-0000-0000-000000000000\"\n\
    ",
            server.url("")
        ),
    );

    // Validate authenticated credentials
    assert_cmd::cargo::cargo_bin_cmd!("oxide")
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

    // OXIDE_PROFILE also works, uses creds file
    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("OXIDE_PROFILE", "funky-town")
        .arg("--config-dir")
        .arg(temp_dir.path().as_os_str())
        .arg("auth")
        .arg("status")
        .assert()
        .success()
        .stdout(format!(
            "Profile \"funky-town\" ({}) status: Authenticated\n",
            server.url(""),
        ));

    // OXIDE_HOST conflicts with OXIDE_PROFILE
    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "oxide-token-good")
        .env("OXIDE_PROFILE", "ignored")
        .arg("--config-dir")
        .arg(temp_dir.path().as_os_str())
        .arg("auth")
        .arg("status")
        .assert()
        .failure()
        .stderr(format!("{}\n", oxide::OxideAuthError::HostProfileConflict));

    oxide_mock.assert_hits(2);

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
    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .arg("auth")
        .arg("status")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "oxide-token-bad")
        .assert()
        .success()
        .stdout(format!(
            "{}: Server responded with an error message: oops\n",
            server.url("")
        ));
    oxide_mock.assert();
}

#[test]
fn test_cmd_auth_debug_logging() {
    let server = MockServer::start();

    let oxide_mock = server.current_user_view(|when, then| {
        when.into_inner()
            .header("authorization", "Bearer oxide-token-good");

        then.ok(&oxide::types::CurrentUser {
            display_name: "privileged".to_string(),
            id: "001de000-05e4-4000-8000-000000004007".parse().unwrap(),
            silo_id: "d1bb398f-872c-438c-a4c6-2211e2042526".parse().unwrap(),
            silo_name: "funky-town".parse().unwrap(),
            fleet_viewer: false,
            silo_admin: false,
        });
    });

    let cmd = assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .arg("--debug")
        .arg("auth")
        .arg("status")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "oxide-token-good")
        .assert()
        .success();

    let stderr_str = std::str::from_utf8(&cmd.get_output().stderr).unwrap();
    assert!(str::is_match(r#""level":"DEBUG""#)
        .unwrap()
        .eval(stderr_str));
    assert!(str::is_match(r#""message":"request succeeded""#)
        .unwrap()
        .eval(stderr_str));
    assert!(str::is_match(r#""url":"http://127.0.0.1:\d+/v1/me""#)
        .unwrap()
        .eval(stderr_str));
    assert!(str::is_match(r#""path":"/v1/me""#)
        .unwrap()
        .eval(stderr_str));
    assert!(str::is_match(r#""remote_addr":"127.0.0.1:\d+""#)
        .unwrap()
        .eval(stderr_str));
    assert!(str::is_match(r#""http.request.method":"GET""#)
        .unwrap()
        .eval(stderr_str));
    assert!(str::is_match(r#""http.response.content_length":\d+"#)
        .unwrap()
        .eval(stderr_str));
    assert!(str::is_match(r#""http.response.status_code":200"#)
        .unwrap()
        .eval(stderr_str));
    assert!(str::is_match(r#""duration_ms":\d+"#)
        .unwrap()
        .eval(stderr_str));

    oxide_mock.assert();
}

#[test]
fn test_cmd_auth_login() {
    use predicates::str;

    let temp_dir = tempfile::tempdir().unwrap();

    let bad_url = "sys.oxide.invalid";

    // Validate connection error details are printed
    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .arg("--config-dir")
        .arg(temp_dir.path().as_os_str())
        .arg("auth")
        .arg("login")
        .arg("--host")
        .arg(bad_url)
        .assert()
        .failure()
        .stderr(str::starts_with(
            "Request failed: client error: error sending request",
        ));
}
