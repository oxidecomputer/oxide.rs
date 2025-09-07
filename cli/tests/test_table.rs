// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use assert_cmd::Command;
use httpmock::MockServer;
use oxide::types::{Project, ProjectResultsPage};
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

    mock.assert_hits(3);
}
