// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::MockServer;
use oxide_api::types::{Project, ProjectResultsPage};
use oxide_httpmock::MockServerExt;
use predicates::prelude::*;

/// Validate `oxide api` for a simple GET with parameters.
#[test]
fn text_xxx() {
    let server = MockServer::start();

    let mock = server.project_list(|when, then| {
        when.limit(10.try_into().unwrap());
        then.ok(&ProjectResultsPage {
            items: vec![Project::builder()
                .name("proj")
                .description("dummy description")
                .id(uuid::Uuid::new_v4())
                .try_into()
                .unwrap()],
            next_page: None,
        });
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("api")
        .arg("/simple/test/call?param1=value1&param2=value2")
        .assert()
        .success()
        .stdout(predicate::str::diff("{\n  \"a\": \"b\"\n}\n"));

    mock.assert();
}
