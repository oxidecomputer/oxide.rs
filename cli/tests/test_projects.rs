// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::MockServer;
use oxide_api::types::{Project, ProjectResultsPage};
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use test_common::JsonMock;

#[test]
fn test_projects_simple_list() {
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
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_projects_simple_list.stdout",
        ));

    mock.assert();
}

#[test]
fn test_projects_list_paginated() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let results = (0..10)
        .map(|_| Project::mock_value(&mut src).unwrap())
        .collect::<Vec<_>>();

    let mock_p3 = server.project_list(|when, then| {
        when.page_token("page-3");
        then.ok(&ProjectResultsPage {
            items: Vec::new(),
            next_page: None,
        });
    });
    let mock_p2 = server.project_list(|when, then| {
        when.page_token("page-2");
        then.ok(&ProjectResultsPage {
            items: results[5..].into(),
            next_page: Some("page-3".to_string()),
        });
    });
    let mock_p1 = server.project_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&ProjectResultsPage {
            items: results[0..5].into(),
            next_page: Some("page-2".to_string()),
        });
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("project")
        .arg("list")
        .arg("--limit")
        .arg("10")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_projects_list_paginated.stdout",
        ));

    mock_p1.assert();
    mock_p2.assert();
    // Because we hit the limit, we should not fetch the final page.
    mock_p3.assert_hits(0);
}

#[test]
fn test_projects_list_paginated_fail() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let results = (0..10)
        .map(|_| Project::mock_value(&mut src).unwrap())
        .collect::<Vec<_>>();

    let mock_p3 = server.project_list(|when, then| {
        when.page_token("page-3");
        then.client_error(
            400,
            &oxide_api::types::Error {
                error_code: None,
                message: "".to_string(),
                request_id: "".to_string(),
            },
        );
    });
    let mock_p2 = server.project_list(|when, then| {
        when.page_token("page-2");
        then.ok(&ProjectResultsPage {
            items: results[5..].into(),
            next_page: Some("page-3".to_string()),
        });
    });
    let mock_p1 = server.project_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&ProjectResultsPage {
            items: results[0..5].into(),
            next_page: Some("page-2".to_string()),
        });
    });

    let output = format!(
        "{}error\nError Response: status: 400 Bad Request",
        results
            .iter()
            .map(|item| format!("{:#?}\n", item))
            .collect::<Vec<_>>()
            .join(""),
    );

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("project")
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::starts_with(output));

    mock_p1.assert();
    mock_p2.assert();
    mock_p3.assert();
}
