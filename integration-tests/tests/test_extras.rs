// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use httpmock::MockServer;
use oxide::extras::disk::types::DiskInfo;
use oxide::extras::ClientExtraDiskExt;
use oxide::types::Disk;
use oxide::{Client, ClientConfig};
use oxide_httpmock::MockServerExt;
use rand::{rng, Rng, SeedableRng};
use std::fs;
use tempfile::TempDir;
use test_common::JsonMock;
use uuid::Uuid;

// A disk import where everything succeeds
#[tokio::test]
async fn test_disk_import() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let disk_view_mock = server.disk_view(|when, then| {
        when.into_inner().any_request();
        then.client_error(
            404,
            &oxide::types::Error {
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

    let dir = TempDir::new().unwrap();
    let image_path = dir.path().join("image.raw");

    let mut rng = rng();
    let mut contents = vec![0u8; 8192];
    rng.fill(&mut contents[..]);
    fs::write(&image_path, contents).unwrap();

    let disk_info = DiskInfo::calculate(image_path.clone(), None, None).unwrap();

    let cfg = ClientConfig::default().with_host_and_token(server.url(""), "test_extra");
    let client = Client::new_authenticated_config(&cfg).unwrap();

    // Execute without control.
    client
        .disk_import()
        .project("hi")
        .description("test extra")
        .upload_task_ct(1)
        .disk("test-import")
        .disk_info(disk_info.clone())
        .execute()
        .unwrap()
        .await
        .unwrap();

    // Execute with control.
    let (fut, handle) = client
        .disk_import()
        .project("hi")
        .description("test extra")
        .upload_task_ct(8)
        .disk("test-import")
        .disk_info(disk_info.clone())
        .execute_with_control()
        .unwrap();

    // Confirm progress channel is updated.
    let mut progress = handle.progress();
    let progress_updated = tokio::spawn(async move { progress.changed().await.is_ok() });
    fut.await.unwrap();
    assert!(progress_updated.await.unwrap());

    // The initial call to `disk_view` races with the cancellation. In general,
    // the call is sent, but this is likely to vary with available parallelism
    // and system load. In theory we could see multiple endpoints hit before
    // the cancel is received, but this seems less likely.
    //
    // Assert this endpoint before the next request to avoid defining an
    // expectation for whether this call should be made upon immediate
    // cancellation.
    disk_view_mock.assert_calls(2);

    let (fut_to_cancel, handle) = client
        .disk_import()
        .project("hi")
        .description("test extra")
        .upload_task_ct(8)
        .disk("test-import")
        .disk_info(disk_info.clone())
        .execute_with_control()
        .unwrap();

    // Cancel request immediately.
    handle.cancel();
    assert_eq!(
        fut_to_cancel.await.unwrap_err().to_string(),
        "Disk import canceled",
    );

    // No calls received for the cancelled request.
    disk_create_mock.assert_calls(2);
    start_bulk_write_mock.assert_calls(2);
    disk_bulk_write_mock.assert_calls(2);
    stop_bulk_write_mock.assert_calls(2);
    finalize_mock.assert_calls(2);
}
