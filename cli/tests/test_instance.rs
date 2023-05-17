// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::prelude::*;
use oxide_api::types::{InstanceSerialConsoleData, NameOrId};
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

    Command::cargo_bin("oxide")
        .unwrap()
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

    // should fail to call tcgetattr on the non-tty stdout, with differing
    // specifics on what the syscall number is / how the IOErrors format
    // depending on the OS.
    #[cfg(target_family = "unix")]
    let pred = predicate::str::starts_with(
        "error: Failed to set raw mode: tcgetattr(stdout, termios) call failed:",
    );

    // non-unix should unconditionally fail
    #[cfg(not(target_family = "unix"))]
    let pred =
        expectorate::eq_file_or_panic("tests/data/test_instance_serial_console_windows.stdout");

    Command::cargo_bin("oxide")
        .unwrap()
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("instance")
        .arg("serial")
        .arg("console")
        .arg("--instance")
        .arg("miniwheats")
        .arg("--project")
        .arg("influences")
        .arg("--byte-offset=-3")
        .assert()
        .failure()
        .stdout(pred);

    if cfg!(target_family = "unix") {
        mock.assert();
    } else {
        mock.assert_hits(0);
    }
}
