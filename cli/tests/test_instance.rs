// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use std::path::PathBuf;

use assert_cmd::Command;
use httpmock::prelude::*;
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use test_common::JsonMock;

#[test]
fn test_instance_create() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let body = oxide_api::types::InstanceCreate {
        description: "description is required for some reason".to_string(),
        disks: vec![oxide_api::types::InstanceDiskAttachment::Create {
            description: "boot disk".to_string(),
            disk_source: oxide_api::types::DiskSource::Image {
                image_id: JsonMock::mock_value(&mut src).unwrap(),
            },
            name: "boot".try_into().unwrap(),
            size: (1024 * 1024 * 1024 * 1024).try_into().unwrap(),
        }],
        external_ips: vec![oxide_api::types::ExternalIpCreate::Ephemeral { pool_name: None }],
        hostname: "hostname".to_string(),
        memory: (4 * 1024 * 1024 * 1024).try_into().unwrap(),
        name: "name".try_into().unwrap(),
        ncpus: 4.try_into().unwrap(),
        network_interfaces: oxide_api::types::InstanceNetworkInterfaceAttachment::Default,
        start: true,
        user_data: String::new(),
    };
    let body_as_str = serde_json::to_string_pretty(&body).unwrap();

    let mock = server.instance_create(|when, then| {
        when.body(&body);
        then.created(&oxide_api::types::Instance {
            description: body.description.clone(),
            hostname: body.hostname.clone(),
            memory: body.memory.clone(),
            name: body.name.clone(),
            ncpus: body.ncpus.clone(),
            run_state: oxide_api::types::InstanceState::Creating,
            ..JsonMock::mock_value(&mut src).unwrap()
        });
    });

    let path = PathBuf::from_iter(["tests", "output", "test_instance_create.stdout"].iter());

    Command::cargo_bin("oxide")
        .unwrap()
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("instance")
        .arg("create")
        .arg("--project")
        .arg("projname")
        .arg("--json-body")
        .arg("/dev/stdin")
        .write_stdin(body_as_str)
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(path));

    mock.assert();
}
