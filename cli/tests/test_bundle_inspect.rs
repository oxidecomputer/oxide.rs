// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

// A successful `bundle inspect` runs an interactive viewer, so these tests
// cover only the errors that precede it.

use std::io::Write;

use httpmock::MockServer;
use oxide_httpmock::MockServerExt;
use predicates::prelude::*;

#[test]
fn test_bundle_inspect_no_source() {
    let server = MockServer::start();

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .args(["bundle", "inspect"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("--id").and(predicate::str::contains("--path")));
}

#[test]
fn test_bundle_inspect_both_sources() {
    let server = MockServer::start();

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .args(["bundle", "inspect"])
        .args(["--id", "0a27f7c9-a18c-4c6b-9ba9-7e461e60f556"])
        .args(["--path", "bundle.zip"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("--id").and(predicate::str::contains("--path")));
}

#[test]
fn test_bundle_inspect_path_missing() {
    let server = MockServer::start();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing.zip");

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .args(["bundle", "inspect", "--path"])
        .arg(&path)
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("(os error 2)"));
}

#[test]
fn test_bundle_inspect_path_not_zip() {
    let server = MockServer::start();
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(b"this is not a zip file").unwrap();

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .args(["bundle", "inspect", "--path"])
        .arg(file.path())
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("panicked").not());
}

#[test]
fn test_bundle_inspect_id_not_found() {
    let server = MockServer::start();
    let id = uuid::Uuid::try_from("0a27f7c9-a18c-4c6b-9ba9-7e461e60f556").unwrap();

    let mock = server.support_bundle_index(|when, then| {
        when.bundle_id(&id);
        then.default_response(
            404,
            serde_json::json!({
                "error_code": "ObjectNotFound",
                "message": "not found: support-bundle with id \"0a27f7c9\"",
                "request_id": "f3c8a0b2-5d0e-4a3f-9b1c-2e6d7f8a9b0c",
            }),
        );
    });

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .args(["bundle", "inspect", "--id"])
        .arg(id.to_string())
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::contains(id.to_string()).and(predicate::str::contains("404 Not Found")),
        );

    mock.assert();
}

// Inspecting a downloaded bundle reads only the local file, so it requires
// neither a server nor credentials.
#[test]
fn test_bundle_inspect_path_unauthenticated() {
    let config_dir = tempfile::tempdir().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing.zip");

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env_remove("OXIDE_HOST")
        .env_remove("OXIDE_TOKEN")
        .arg("--config-dir")
        .arg(config_dir.path())
        .args(["bundle", "inspect", "--path"])
        .arg(&path)
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("(os error 2)"));
}
