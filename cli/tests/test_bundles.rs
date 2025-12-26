// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use httpmock::MockServer;
use oxide_httpmock::MockServerExt;

#[test]
fn test_bundle_download_ranges() {
    let server = MockServer::start();

    let id = uuid::Uuid::try_from("0a27f7c9-a18c-4c6b-9ba9-7e461e60f556").unwrap();
    let output_path = tempfile::NamedTempFile::new().expect("Failed to make tempfile");
    let chunk_size: u64 = 1024;

    let mock_head = server.support_bundle_head(|when, then| {
        when.bundle_id(&id).into_inner().any_request();
        then.into_inner()
            .header("accept-ranges", "bytes")
            .header("content-length", "2048")
            .header("content-type", "application/octet-stream")
            .status(200);
    });

    let mock_download1 = server.support_bundle_download(|when, then| {
        when.bundle_id(&id)
            .into_inner()
            .header("range", "bytes=0-1023")
            .any_request();

        then.into_inner()
            .header("accept-ranges", "bytes")
            .header("content-length", "1024")
            .header("content-range", "bytes 0-1023/2048")
            .header("content-type", "application/octet-stream")
            .body([0; 1024]);
    });
    let mock_download2 = server.support_bundle_download(|when, then| {
        when.bundle_id(&id)
            .into_inner()
            .header("range", "bytes=1024-2047")
            .any_request();

        then.into_inner()
            .header("accept-ranges", "bytes")
            .header("content-length", "1024")
            .header("content-range", "bytes 1024-2047/2048")
            .header("content-type", "application/octet-stream")
            .body([1; 1024]);
    });

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("bundle")
        .arg("download")
        .arg("--id")
        .arg(id.to_string())
        .arg("--output")
        .arg(output_path.path())
        .arg("--chunk-size")
        .arg(chunk_size.to_string())
        .assert()
        .success();

    assert_eq!(
        std::fs::read(output_path.path()).expect("Failed to read output"),
        [[0; 1024], [1; 1024]].concat()
    );

    mock_head.assert();
    mock_download1.assert();
    mock_download2.assert();
}

// This server claims to support range requests, but it returns the entire file
// anyway.
//
// Our CLI should cope by streaming the file as it is returned.
#[test]
fn test_bundle_download_range_too_large_response() {
    let server = MockServer::start();

    let id = uuid::Uuid::try_from("0a27f7c9-a18c-4c6b-9ba9-7e461e60f556").unwrap();
    let output_path = tempfile::NamedTempFile::new().expect("Failed to make tempfile");
    let chunk_size: u64 = 1024;

    let mock_head = server.support_bundle_head(|when, then| {
        when.bundle_id(&id).into_inner().any_request();
        then.into_inner()
            .header("accept-ranges", "bytes")
            .header("content-length", "4096")
            .header("content-type", "application/octet-stream")
            .status(200);
    });

    // Even though the server advertised that range requests were supported,
    // it'll return the whole file at once. Confirm that we download it
    // in a single call anyway.
    let mock_download = server.support_bundle_download(|when, then| {
        when.bundle_id(&id)
            .into_inner()
            .header("range", "bytes=0-1023")
            .any_request();

        then.into_inner()
            .header("accept-ranges", "bytes")
            .header("content-length", "4096")
            .header("content-range", "bytes 0-4095/4096")
            .header("content-type", "application/octet-stream")
            .body([0; 4096]);
    });

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("bundle")
        .arg("download")
        .arg("--id")
        .arg(id.to_string())
        .arg("--output")
        .arg(output_path.path())
        .arg("--chunk-size")
        .arg(chunk_size.to_string())
        .assert()
        .success();

    assert_eq!(
        std::fs::read(output_path.path()).expect("Failed to read output"),
        [0; 4096]
    );

    mock_head.assert();
    mock_download.assert();
}
