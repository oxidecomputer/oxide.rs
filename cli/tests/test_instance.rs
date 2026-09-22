// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

use httpmock::prelude::*;
use oxide::types::{
    ExternalIpCreate, InstanceCreate, InstanceDiskAttachment, InstanceSerialConsoleData, NameOrId,
};
use oxide_httpmock::MockServerExt;
use predicates::prelude::predicate;
use rand::SeedableRng;
use std::str::FromStr;
use test_common::JsonMock;

#[test]
fn test_instance_create() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let body = serde_json::from_str(
        &std::fs::read_to_string("tests/data/test_instance_create.stdin").unwrap(),
    )
    .unwrap();

    let mock = server.instance_create(|when, then| {
        when.body(&body);
        then.created(&oxide::types::Instance {
            description: body.description.clone(),
            hostname: body.hostname.to_string(),
            memory: body.memory.clone(),
            name: body.name.clone(),
            ncpus: body.ncpus.clone(),
            run_state: oxide::types::InstanceState::Creating,
            ..JsonMock::mock_value(&mut src).unwrap()
        });
    });

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("instance")
        .arg("create")
        .arg("--project")
        .arg("projname")
        .arg("--json-body")
        .arg("tests/data/test_instance_create.stdin")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_instance_create.stdout",
        ));

    mock.assert();
}

#[test]
fn test_instance_serial_history() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let results = InstanceSerialConsoleData {
        data: Vec::<u8>::mock_value(&mut src).unwrap(),
        last_byte_offset: u64::mock_value(&mut src).unwrap(),
    };

    let mock = server.instance_serial_console(|when, then| {
        when.instance(&NameOrId::from_str("bran").unwrap())
            .project(&NameOrId::from_str("influences").unwrap())
            .max_bytes(1)
            .most_recent(2);
        then.ok(&results);
    });

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("instance")
        .arg("serial")
        .arg("history")
        .arg("--instance")
        .arg("bran")
        .arg("--project")
        .arg("influences")
        .arg("--json")
        .arg("--max-bytes=1")
        .arg("--byte-offset=-2")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_instance_serial_history.stdout",
        ));

    mock.assert();
}

#[test]
fn test_instance_serial_console() {
    let server = MockServer::start();

    let mock = server.instance_serial_console_stream(|when, then| {
        when.instance(&NameOrId::from_str("miniwheats").unwrap())
            .project(&NameOrId::from_str("influences").unwrap())
            .most_recent(3);
        then.switching_protocols();
    });

    // Since we don't have a real WebSocket here, the connection is dropped
    // immediately, and the CLI quits before attempting to set raw mode in the
    // terminal (which it only does once the first binary frame is received).
    let pred = predicate::str::contains("Connection lost.");

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("instance")
        .arg("serial")
        .arg("console")
        .arg("--instance")
        .arg("miniwheats")
        .arg("--project")
        .arg("influences")
        .arg("--most-recent=3")
        .assert()
        .success()
        .stderr(pred);

    mock.assert();
}

#[test]
fn test_instance_from_image() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let image_response = JsonMock::mock_value(&mut src).unwrap();
    let image_view = server.image_view(|when, then| {
        when.image(&"blank-i-n-g-o".parse().unwrap())
            .project(Some(&"projname".parse().unwrap()));
        then.ok(&image_response);
    });

    let instance_response = oxide::types::Instance {
        name: "bingo".parse().unwrap(),
        description: "there was a farmer who had a dog".to_string(),
        hostname: "bingo".to_string(),
        memory: (100 * 1024 * 1024 * 1024).into(),
        ncpus: 6.into(),
        run_state: oxide::types::InstanceState::Creating,
        ..JsonMock::mock_value(&mut src).unwrap()
    };
    let instance_create = server.instance_create(|when, then| {
        when.project(&"projname".parse().unwrap()).body(
            &InstanceCreate::builder()
                .name(instance_response.name.clone())
                .description(instance_response.description.clone())
                .boot_disk(Some(InstanceDiskAttachment::Create {
                    description: "bingo disk".to_string(),
                    disk_backend: oxide::types::DiskBackend::Distributed(
                        oxide::types::DiskSource::Image {
                            image_id: image_response.id.clone(),
                            read_only: false,
                        },
                    ),
                    name: "bingo-disk".parse().unwrap(),
                    size: (1 * 1024 * 1024 * 1024 * 1024).into(),
                }))
                .external_ips(vec![ExternalIpCreate::Ephemeral {
                    pool_selector: oxide::types::PoolSelector::Auto {
                        ip_version: Some(oxide::types::IpVersion::V6),
                    },
                }])
                .hostname(instance_response.hostname.clone())
                .memory(instance_response.memory.clone())
                .ncpus(instance_response.ncpus.clone())
                .start(false)
                .try_into()
                .unwrap(),
        );
        then.created(&instance_response);
    });

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("instance")
        .arg("from-image")
        .arg("--name")
        .arg("bingo")
        .arg("--project")
        .arg("projname")
        .arg("--description")
        .arg("there was a farmer who had a dog")
        .arg("--hostname")
        .arg("bingo")
        .arg("--memory")
        .arg("100g")
        .arg("--ncpus")
        .arg("6")
        .arg("--image")
        .arg("blank-i-n-g-o")
        .arg("--size")
        .arg("1t")
        .arg("--ip-version")
        .arg("v6")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_instance_from_image.stdout",
        ));

    image_view.assert();
    instance_create.assert();
}
