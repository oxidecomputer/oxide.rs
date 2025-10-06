// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use assert_cmd::Command;
use chrono::{DateTime, Utc};
use httpmock::MockServer;
use oxide::types::{TufRepo, TufRepoUpload, TufRepoUploadStatus};
use oxide_httpmock::MockServerExt;

#[test]
fn test_update_upload() {
    const CONTENTS: &str = "nuggets";
    let server = MockServer::start();

    let mock = server.system_update_repository_upload(|when, then| {
        when.into_inner().body(CONTENTS);
        then.ok(&TufRepoUpload {
            repo: TufRepo {
                file_name: String::from("repo"),
                hash: String::from("browns"),
                system_version: "0.0.0".parse().unwrap(),
                time_created: DateTime::<Utc>::from_timestamp(0x8000_0000, 0).unwrap(),
            },
            status: TufRepoUploadStatus::Inserted,
        });
    });

    let tempdir = tempfile::tempdir().unwrap();
    let tempdir_path = tempdir.path();

    let repo_path = tempdir_path.join("repo");
    std::fs::write(&repo_path, CONTENTS).unwrap();

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("--config-dir")
        .arg(tempdir_path.as_os_str())
        .arg("system")
        .arg("update")
        .arg("repo")
        .arg("upload")
        .arg("--path")
        .arg(repo_path.as_os_str())
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_update_upload.stdout",
        ));

    mock.assert();
}
