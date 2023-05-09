// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::prelude::*;
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
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

    Command::cargo_bin("oxide")
        .unwrap()
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
