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
fn text_simple_list() {
    let mut src = rand::rngs::StdRng::from_seed([42; 32]);
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
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("project")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::diff(format!("success\n{:#?}\n", results)));

    mock.assert();
}
