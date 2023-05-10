// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use anyhow::Result;
use assert_cmd::Command;
use httpmock::MockServer;
use oxide_api::types::Disk;
use oxide_api::types::Image;
use oxide_api::types::Snapshot;
use oxide_httpmock::MockServerExt;
use predicates::prelude::*;
use rand::SeedableRng;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;
use test_common::JsonMock;
use uuid::Uuid;

struct Testfile {
    pub _tempdir: TempDir,
    pub file_path: PathBuf,
}

const CHUNK_SIZE: usize = 512 * 1024;

impl Testfile {
    pub fn new_random(sz: usize) -> Result<Testfile> {
        Self::new(sz, true)
    }

    pub fn new_blank(sz: usize) -> Result<Testfile> {
        Self::new(sz, false)
    }

    fn new(sz: usize, populate_with_random: bool) -> Result<Testfile> {
        let tempdir = tempfile::tempdir()?;
        let file_path = tempdir.path().join("dos622.iso");

        let mut file = File::create(&file_path)?;

        if populate_with_random {
            let mut rng = thread_rng();

            let mut data: Vec<u8> = vec![0u8; sz];
            rng.fill(&mut data[..]);
            file.write_all(&data)?;
        } else {
            file.set_len(sz as u64)?;
        }

        file.flush()?;
        drop(file);

        Ok(Testfile {
            _tempdir: tempdir,
            file_path,
        })
    }

    pub fn random_data_in_chunk(&self, i: usize) -> Result<()> {
        let file_size = std::fs::metadata(&self.file_path)?.len();
        assert!((i * CHUNK_SIZE) < file_size as usize);

        let mut file = std::fs::OpenOptions::new()
            .create(false)
            .write(true)
            .open(&self.file_path)?;
        file.seek(SeekFrom::Start((i * CHUNK_SIZE) as u64))?;

        let mut data: Vec<u8> = vec![0u8; CHUNK_SIZE];
        let mut rng = thread_rng();
        rng.fill(&mut data[..]);
        file.write_all(&data)?;

        drop(file);

        Ok(())
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }
}

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

    let test_file = Testfile::new_random(CHUNK_SIZE * 2).unwrap();

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
        .arg(test_file.path())
        .arg("--disk")
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

// A disk import of a sparse file where everything succeeds
#[test]
fn test_disk_import_sparse() {
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

    // 10 chunks, but only one is non-sparse
    let test_file = Testfile::new_blank(CHUNK_SIZE * 10).unwrap();
    test_file.random_data_in_chunk(4).unwrap();

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
        .arg(test_file.path())
        .arg("--disk")
        .arg("test-import")
        .assert()
        .success();

    disk_view_mock.assert();

    disk_create_mock.assert();
    start_bulk_write_mock.assert();
    disk_bulk_write_mock.assert_hits(1);
    stop_bulk_write_mock.assert();
    finalize_mock.assert();
}

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

    let test_file = Testfile::new_random(CHUNK_SIZE * 2).unwrap();

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
        .arg(test_file.path())
        .arg("--disk")
        .arg("test-import")
        .arg("--snapshot")
        .arg("test-import-snap")
        .arg("--image")
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

    let test_file = Testfile::new_random(CHUNK_SIZE * 2).unwrap();

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
        .arg(test_file.path())
        .arg("--disk")
        .arg("test-import")
        .arg("--snapshot")
        .arg("test-import-snap")
        .arg("--image")
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

    let test_file = Testfile::new_random(CHUNK_SIZE * 2).unwrap();

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
        .arg(test_file.path())
        .arg("--disk")
        .arg("test-import")
        .arg("--snapshot")
        .arg("test-import-snap")
        .arg("--image")
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

    let test_file = Testfile::new_random(CHUNK_SIZE * 2).unwrap();

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
        .arg(test_file.path())
        .arg("--disk")
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

    let test_file = Testfile::new_random(CHUNK_SIZE * 2).unwrap();

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
        .arg(test_file.path())
        .arg("--disk")
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
    let test_file = Testfile::new_random(CHUNK_SIZE * 2).unwrap();

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
        .arg(test_file.path())
        .arg("--disk")
        .arg("test-import")
        .assert()
        .failure();
}

// A disk import where the file size doesn't divide the block size
#[test]
fn test_disk_import_bad_file_size() {
    let server = MockServer::start();
    let test_file = Testfile::new_random(512 + 1).unwrap();

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
        .arg(test_file.path())
        .arg("--disk")
        .arg("test-import")
        .assert()
        .failure();
}

// Test for required parameters being supplied
#[test]
fn test_disk_import_required_parameters() {
    let server = MockServer::start();
    let test_file = Testfile::new_random(512 + 1).unwrap();

    // only supplying --image-description won't work
    let output: &str = r#"error: the following required arguments were not provided:
  --snapshot <SNAPSHOT>
  --image-os <IMAGE_OS>
  --image-version <IMAGE_VERSION>
  --image <IMAGE>"#;

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--path")
        .arg(test_file.path())
        .arg("--disk")
        .arg("test-import")
        .arg("--description")
        .arg("disk description")
        .arg("--image-description")
        .arg("value")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(output));

    // only supplying --image won't work
    let output: &str = r#"error: the following required arguments were not provided:
  --snapshot <SNAPSHOT>
  --image-os <IMAGE_OS>
  --image-version <IMAGE_VERSION>
  --image-description <IMAGE_DESCRIPTION>"#;

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--path")
        .arg(test_file.path())
        .arg("--disk")
        .arg("test-import")
        .arg("--description")
        .arg("disk description")
        .arg("--image")
        .arg("value")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(output));

    // supplying all of the image group but no snapshot won't work
    let output: &str = r#"error: the following required arguments were not provided:
  --snapshot <SNAPSHOT>"#;

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--path")
        .arg(test_file.path())
        .arg("--disk")
        .arg("test-import")
        .arg("--description")
        .arg("disk description")
        .arg("--image")
        .arg("value")
        .arg("--image-description")
        .arg("value")
        .arg("--image-os")
        .arg("value")
        .arg("--image-version")
        .arg("value")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(output));

    // supplying snapshot and all of the image group will work (and will error
    // at the bad path)
    let mut bad_path = test_file.path().clone();
    bad_path.push("does.not.exist");

    let output = format!(
        "error: path {} does not exist",
        bad_path.clone().into_os_string().to_str().unwrap()
    );

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("disk")
        .arg("import")
        .arg("--project")
        .arg("myproj")
        .arg("--path")
        .arg(bad_path)
        .arg("--disk")
        .arg("test-import")
        .arg("--description")
        .arg("disk description")
        .arg("--snapshot")
        .arg("value")
        .arg("--image")
        .arg("value")
        .arg("--image-description")
        .arg("value")
        .arg("--image-os")
        .arg("value")
        .arg("--image-version")
        .arg("value")
        .assert()
        .failure()
        .stdout(predicate::str::starts_with(output));
}
