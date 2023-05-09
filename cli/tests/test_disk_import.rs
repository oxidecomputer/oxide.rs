// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::MockServer;
use oxide_api::types::Disk;
use oxide_api::types::Image;
use oxide_api::types::Snapshot;
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use test_common::JsonMock;
use uuid::Uuid;

// A disk import where everything succeeds
#[test]
fn test_disk_import() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let disk_view_mock = server.disk_view(|when, then| {
        when.into_inner().any_request();
        then.client_error(
            404,
            &oxide_api::types::Error {
                error_code: None,
                message: "disk not found".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let disk_create_mock = server.disk_create(|when, then| {
        when.into_inner().any_request();
        then.created(&Disk {
            name: "test-import".parse().unwrap(),
            ..Disk::mock_value(&mut src).unwrap()
        });
    });

    let start_bulk_write_mock = server.disk_bulk_write_import_start(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let disk_bulk_write_mock = server.disk_bulk_write_import(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let stop_bulk_write_mock = server.disk_bulk_write_import_stop(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let finalize_mock = server.disk_finalize_import(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--description")
        .arg("disk description")
        .arg("--path")
        .arg("tests/data/testpost.iso")
        .arg("--disk-name")
        .arg("test-import")
        .assert()
        .success();

    disk_view_mock.assert();

    disk_create_mock.assert();
    start_bulk_write_mock.assert();
    disk_bulk_write_mock.assert_hits(2);
    stop_bulk_write_mock.assert();
    finalize_mock.assert();
}

// A disk import where everything succeeds and an image is created
// XXX this does not work because httpmock does not have a way for one endpoint
// to return different values for the same request.
/*
#[test]
fn test_disk_import_with_image() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let disk_view_mock = server.disk_view(|when, then| {
        when.into_inner().any_request();
        then.client_error(
            404,
            &oxide_api::types::Error {
                error_code: None,
                message: "disk not found".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let snapshot_view_mock = server.snapshot_view(|when, then| {
        when.into_inner().any_request();
        // XXX this doesn't work!
        // first time is the check for an existing object
        then.client_error(
            404,
            &oxide_api::types::Error {
                error_code: None,
                message: "snapshot not found".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        )
        // second time is when the snapshot was created by the finalize
        .ok(&Snapshot {
            name: "test-import-snap".parse().unwrap(),
            ..Snapshot::mock_value(&mut src).unwrap()
        });
    });

    let image_view_mock = server.image_view(|when, then| {
        when.into_inner().any_request();
        then.client_error(
            404,
            &oxide_api::types::Error {
                error_code: None,
                message: "image not found".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let disk_create_mock = server.disk_create(|when, then| {
        when.into_inner().any_request();
        then.created(&Disk {
            name: "test-import".parse().unwrap(),
            ..Disk::mock_value(&mut src).unwrap()
        });
    });

    let start_bulk_write_mock = server.disk_bulk_write_import_start(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let disk_bulk_write_mock = server.disk_bulk_write_import(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let stop_bulk_write_mock = server.disk_bulk_write_import_stop(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let finalize_mock = server.disk_finalize_import(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let image_create_mock = server.image_create(|when, then| {
        when.into_inner().any_request();
        then.created(&Image {
            name: "test-import".parse().unwrap(),
            ..Image::mock_value(&mut src).unwrap()
        });
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--description")
        .arg("disk description")
        .arg("--path")
        .arg("tests/data/testpost.iso")
        .arg("--disk-name")
        .arg("test-import")
        .arg("--snapshot-name")
        .arg("test-import-snap")
        .arg("--image-name")
        .arg("test-import")
        .arg("--image-description")
        .arg("image description")
        .arg("--image-os")
        .arg("dos")
        .arg("--image-version")
        .arg("6.1")
        .assert()
        .success();

    disk_view_mock.assert();
    snapshot_view_mock.assert();
    image_view_mock.assert();

    disk_create_mock.assert();
    start_bulk_write_mock.assert();
    disk_bulk_write_mock.assert_hits(2);
    stop_bulk_write_mock.assert();
    finalize_mock.assert();
    image_create_mock.assert();
}
*/

// A disk import where the disk exists already
#[test]
fn test_disk_import_disk_exists_already() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let disk_view_mock = server.disk_view(|when, then| {
        when.into_inner().any_request();
        then.ok(&Disk {
            name: "test-import".parse().unwrap(),
            ..Disk::mock_value(&mut src).unwrap()
        });
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--description")
        .arg("disk description")
        .arg("--path")
        .arg("tests/data/testpost.iso")
        .arg("--disk-name")
        .arg("test-import")
        .arg("--snapshot-name")
        .arg("test-import-snap")
        .arg("--image-name")
        .arg("test-import")
        .arg("--image-description")
        .arg("image description")
        .arg("--image-os")
        .arg("dos")
        .arg("--image-version")
        .arg("6.1")
        .assert()
        .failure();

    disk_view_mock.assert();
}

// A disk import where the snapshot exists already
#[test]
fn test_disk_import_snapshot_exists_already() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let disk_view_mock = server.disk_view(|when, then| {
        when.into_inner().any_request();
        then.client_error(
            404,
            &oxide_api::types::Error {
                error_code: None,
                message: "disk not found".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let snapshot_view_mock = server.snapshot_view(|when, then| {
        when.into_inner().any_request();
        then.ok(&Snapshot {
            name: "test-import-snap".parse().unwrap(),
            ..Snapshot::mock_value(&mut src).unwrap()
        });
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--description")
        .arg("disk description")
        .arg("--path")
        .arg("tests/data/testpost.iso")
        .arg("--disk-name")
        .arg("test-import")
        .arg("--snapshot-name")
        .arg("test-import-snap")
        .arg("--image-name")
        .arg("test-import")
        .arg("--image-description")
        .arg("image description")
        .arg("--image-os")
        .arg("dos")
        .arg("--image-version")
        .arg("6.1")
        .assert()
        .failure();

    disk_view_mock.assert();
    snapshot_view_mock.assert();
}

// A disk import where the image exists already
#[test]
fn test_disk_import_image_exists_already() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let disk_view_mock = server.disk_view(|when, then| {
        when.into_inner().any_request();
        then.client_error(
            404,
            &oxide_api::types::Error {
                error_code: None,
                message: "disk not found".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let snapshot_view_mock = server.snapshot_view(|when, then| {
        when.into_inner().any_request();
        then.client_error(
            404,
            &oxide_api::types::Error {
                error_code: None,
                message: "snapshot not found".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let image_view_mock = server.image_view(|when, then| {
        when.into_inner().any_request();
        then.ok(&Image {
            name: "test-import".parse().unwrap(),
            ..Image::mock_value(&mut src).unwrap()
        });
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--description")
        .arg("disk description")
        .arg("--path")
        .arg("tests/data/testpost.iso")
        .arg("--disk-name")
        .arg("test-import")
        .arg("--snapshot-name")
        .arg("test-import-snap")
        .arg("--image-name")
        .arg("test-import")
        .arg("--image-description")
        .arg("image description")
        .arg("--image-os")
        .arg("dos")
        .arg("--image-version")
        .arg("6.1")
        .assert()
        .failure();

    disk_view_mock.assert();
    snapshot_view_mock.assert();
    image_view_mock.assert();
}

// A disk import where the bulk_import_start fails
#[test]
fn test_disk_import_bulk_import_start_fail() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let disk_view_mock = server.disk_view(|when, then| {
        when.into_inner().any_request();
        then.client_error(
            404,
            &oxide_api::types::Error {
                error_code: None,
                message: "disk not found".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let disk_create_mock = server.disk_create(|when, then| {
        when.into_inner().any_request();
        then.created(&Disk {
            name: "test-import".parse().unwrap(),
            ..Disk::mock_value(&mut src).unwrap()
        });
    });

    let start_bulk_write_mock = server.disk_bulk_write_import_start(|when, then| {
        when.into_inner().any_request();
        then.server_error(
            503,
            &oxide_api::types::Error {
                error_code: None,
                message: "I can't do that Dave".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let unwind_finalize_mock = server.disk_finalize_import(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let unwind_disk_delete_mock = server.disk_delete(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--description")
        .arg("disk description")
        .arg("--path")
        .arg("tests/data/testpost.iso")
        .arg("--disk-name")
        .arg("test-import")
        .assert()
        .failure();

    disk_view_mock.assert();
    disk_create_mock.assert();
    start_bulk_write_mock.assert();
    unwind_finalize_mock.assert();
    unwind_disk_delete_mock.assert();
}

// A disk import where the bulk_write_import fails
#[test]
fn test_disk_import_bulk_write_import_fail() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let disk_view_mock = server.disk_view(|when, then| {
        when.into_inner().any_request();
        then.client_error(
            404,
            &oxide_api::types::Error {
                error_code: None,
                message: "disk not found".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let disk_create_mock = server.disk_create(|when, then| {
        when.into_inner().any_request();
        then.created(&Disk {
            name: "test-import".parse().unwrap(),
            ..Disk::mock_value(&mut src).unwrap()
        });
    });

    let start_bulk_write_mock = server.disk_bulk_write_import_start(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let disk_bulk_write_mock = server.disk_bulk_write_import(|when, then| {
        when.into_inner().any_request();
        then.server_error(
            503,
            &oxide_api::types::Error {
                error_code: None,
                message: "I can't do that Dave".into(),
                request_id: Uuid::mock_value(&mut src).unwrap().to_string(),
            },
        );
    });

    let unwind_stop_bulk_write_mock = server.disk_bulk_write_import_stop(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let unwind_finalize_mock = server.disk_finalize_import(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    let unwind_disk_delete_mock = server.disk_delete(|when, then| {
        when.into_inner().any_request();
        then.no_content();
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--description")
        .arg("disk description")
        .arg("--path")
        .arg("tests/data/testpost.iso")
        .arg("--disk-name")
        .arg("test-import")
        .assert()
        .failure();

    disk_view_mock.assert();
    disk_create_mock.assert();
    start_bulk_write_mock.assert();
    // there are 8 upload tasks, and both will receive a 500 on their POSTs
    disk_bulk_write_mock.assert_hits(2);
    unwind_stop_bulk_write_mock.assert();
    unwind_finalize_mock.assert();
    unwind_disk_delete_mock.assert();
}

// A disk import where the requested block size is invalid
#[test]
fn test_disk_import_bad_block_size() {
    let server = MockServer::start();

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--disk-block-size")
        .arg("123")
        .arg("--project")
        .arg("myproj")
        .arg("--description")
        .arg("disk description")
        .arg("--path")
        .arg("tests/data/testpost.iso")
        .arg("--disk-name")
        .arg("test-import")
        .assert()
        .failure();
}
