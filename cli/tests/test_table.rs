// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use assert_cmd::Command;
use httpmock::MockServer;
use oxide::types::{
    AntiAffinityGroupMember, AntiAffinityGroupMemberResultsPage, BgpPeerState, BgpPeerStatus,
    Project, ProjectResultsPage, SwitchLocation,
};
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use test_common::JsonMock;

#[test]
fn test_table_arg_parse() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let results = ProjectResultsPage {
        items: Vec::<Project>::mock_value(&mut src).unwrap(),
        next_page: None,
    };

    let mock = server.project_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&results);
    });

    let format_args = [
        // JSON
        ["--format", "json"],
        // Simple table
        ["--format", "table"],
        // Table with column names specified
        ["--format", "table:name,id"],
        // Table with spaces between column names
        ["--format", "table: name, id "],
    ];

    for args in format_args {
        Command::cargo_bin("oxide")
            .unwrap()
            .env("RUST_BACKTRACE", "1")
            .env("OXIDE_HOST", server.url(""))
            .env("OXIDE_TOKEN", "fake-token")
            .arg("project")
            .arg("list")
            .arg("--sort-by")
            .arg("name_ascending")
            .args(args)
            .assert()
            .success();
    }

    let bad_args = [
        // Unknown format
        ["--format", "foo"],
        // JSON with field args
        ["--format", "json:name,id"],
    ];

    for args in bad_args {
        Command::cargo_bin("oxide")
            .unwrap()
            .env("RUST_BACKTRACE", "1")
            .env("OXIDE_HOST", server.url(""))
            .env("OXIDE_TOKEN", "fake-token")
            .arg("project")
            .arg("list")
            .arg("--sort-by")
            .arg("name_ascending")
            .args(args)
            .assert()
            .failure();
    }

    mock.assert_hits(format_args.len());
}

#[test]
fn test_table_project_list() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let results = ProjectResultsPage {
        items: Vec::<Project>::mock_value(&mut src).unwrap(),
        next_page: None,
    };

    let mock = server.project_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&results);
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("project")
        .arg("list")
        .arg("--sort-by")
        .arg("name_ascending")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_table_project_list_json.stdout",
        ));

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("project")
        .arg("list")
        .arg("--sort-by")
        .arg("name_ascending")
        .arg("--format")
        .arg("table")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_table_project_list_basic_table.stdout",
        ));

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("project")
        .arg("list")
        .arg("--sort-by")
        .arg("name_ascending")
        .arg("--format")
        .arg("table:name,id")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_table_project_list_table_with_fields.stdout",
        ));

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("project")
        .arg("list")
        .arg("--sort-by")
        .arg("name_ascending")
        .arg("--format")
        .arg("table:not_a_field")
        .assert()
        .failure()
        .stderr(expectorate::eq_file_or_panic(
            "tests/data/test_table_project_list_table_with_no_requested_fields.stderr",
        ));

    mock.assert_hits(3);
}

/// Validate an endpoint returning `Vec<T>`.
#[test]
fn test_table_bgp_routes() {
    let server = MockServer::start();

    // Manually construct the response as `Vec<BgpPeerStatus>` is not compatible with
    // `serde_json::from_value()`.
    let results = vec![
        BgpPeerStatus {
            addr: "10.0.0.1".parse().unwrap(),
            local_asn: 65001,
            remote_asn: 65002,
            state: BgpPeerState::OpenConfirm,
            state_duration_millis: 1_000_000,
            switch: SwitchLocation::Switch0,
        },
        BgpPeerStatus {
            addr: "10.0.0.2".parse().unwrap(),
            local_asn: 65003,
            remote_asn: 65004,
            state: BgpPeerState::Established,
            state_duration_millis: 1_000_000,
            switch: SwitchLocation::Switch1,
        },
    ];

    let mock = server.networking_bgp_status(|when, then| {
        when.into_inner().any_request();
        then.ok(&results);
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("system")
        .arg("networking")
        .arg("bgp")
        .arg("status")
        .arg("--format")
        .arg("table")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_table_bgp_routes.stdout",
        ));

    mock.assert();
}

/// Validate an endpoint returning an enum.
#[test]
fn test_table_anti_affinity_group_members() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let results = AntiAffinityGroupMemberResultsPage {
        items: Vec::<AntiAffinityGroupMember>::mock_value(&mut src).unwrap(),
        next_page: None,
    };

    let mock = server.anti_affinity_group_member_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&results);
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("instance")
        .arg("anti-affinity")
        .arg("member")
        .arg("list")
        .arg("--anti-affinity-group")
        .arg("42e56270-889a-e74d-fa7c-849b22449cd6")
        .arg("--format")
        .arg("table")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_table_anti_affinity_group_members.stdout",
        ));

    mock.assert();
}
