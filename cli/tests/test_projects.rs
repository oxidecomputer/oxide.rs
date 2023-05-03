// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::MockServer;
use oxide_api::types::{Project, ProjectResultsPage};
use oxide_httpmock::MockServerExt;
use predicates::prelude::*;
use rand::SeedableRng;
use test_common::JsonMock;

#[test]
fn test_simple_list() {
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

    let output = results
        .items
        .iter()
        .map(|item| format!("{:#?}\n", item))
        .collect::<Vec<_>>()
        .join("");

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("project")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::diff(output));

    mock.assert();
}
