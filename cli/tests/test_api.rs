// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::prelude::*;
use predicates::prelude::*;
use serde::Serialize;
use serde_json::json;

/// Validate `oxide api` for a simple GET with parameters embedded in URI
/// and via the `--raw-field` option.
#[test]
fn test_simple_api_call() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/simple/test/call")
            .query_param("param1", "value1")
            .query_param("param2", "value2");
        then.status(reqwest::StatusCode::OK.as_u16())
            .json_body(json!(
                {
                    "a": "b"
                }
            ));
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

    Command::cargo_bin("oxide")
        .unwrap()
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("api")
        .arg("-X")
        .arg("GET")
        .arg("/simple/test/call")
        .arg("--raw-field")
        .arg("param1=value1")
        .arg("-f")
        .arg("param2=value2")
        .assert()
        .success()
        .stdout(predicate::str::diff("{\n  \"a\": \"b\"\n}\n"));

    mock.assert_hits(2);
}

#[derive(Serialize)]
struct Item {
    name: String,
}

impl From<&'static str> for Item {
    fn from(value: &'static str) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}

#[derive(Serialize)]
struct ItemResultsPage {
    pub items: Vec<Item>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
}

/// Validate a paginated, multi-page request with no errors.
#[test]
fn test_pagination_success() {
    let page1 = ItemResultsPage {
        items: vec!["aas".into(), "aba".into(), "abb".into()],
        next_page: Some("page-2".to_string()),
    };
    let page2 = ItemResultsPage {
        items: vec!["aby".into(), "ace".into(), "ach".into()],
        next_page: Some("page-3".to_string()),
    };
    let page3 = ItemResultsPage {
        items: vec!["act".into(), "add".into(), "ado".into()],
        next_page: Some("page-4".to_string()),
    };
    let page4 = ItemResultsPage {
        items: vec![],
        next_page: None,
    };

    let server = MockServer::start();
    let mock_p1 = server.mock(|when, then| {
        when.method(GET)
            .path("/paginated")
            .query_param("param1", "value1")
            .query_param("param2", "value2");
        then.status(reqwest::StatusCode::OK.as_u16())
            .json_body_obj(&page1);
    });
    let mock_p2 = server.mock(|when, then| {
        when.method(GET)
            .path("/paginated")
            .query_param("page_token", "page-2");
        then.status(reqwest::StatusCode::OK.as_u16())
            .json_body_obj(&page2);
    });
    let mock_p3 = server.mock(|when, then| {
        when.method(GET)
            .path("/paginated")
            .query_param("page_token", "page-3");
        then.status(reqwest::StatusCode::OK.as_u16())
            .json_body_obj(&page3);
    });
    let mock_p4 = server.mock(|when, then| {
        when.method(GET)
            .path("/paginated")
            .query_param("page_token", "page-4");
        then.status(reqwest::StatusCode::OK.as_u16())
            .json_body_obj(&page4);
    });

    let output: Vec<Item> = vec![
        "aas".into(),
        "aba".into(),
        "abb".into(),
        "aby".into(),
        "ace".into(),
        "ach".into(),
        "act".into(),
        "add".into(),
        "ado".into(),
    ];
    let output_str = format!("{}\n", serde_json::to_string_pretty(&output).unwrap());

    Command::cargo_bin("oxide")
        .unwrap()
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("api")
        .arg("--paginate")
        .arg("/paginated?param1=value1&param2=value2")
        .assert()
        .success()
        .stdout(predicate::str::diff(output_str));

    mock_p1.assert();
    mock_p2.assert();
    mock_p3.assert();
    mock_p4.assert();
}

/// Validate a paginated request where 2 pages return while the 3rd call
/// results in an error. This should print the items we've already retrieved
/// *and* print the error as well.
#[test]
fn test_pagination_midway_failure() {
    let page1 = ItemResultsPage {
        items: vec!["aas".into(), "aba".into(), "abb".into()],
        next_page: Some("page-2".to_string()),
    };
    let page2 = ItemResultsPage {
        items: vec!["aby".into(), "ace".into(), "ach".into()],
        next_page: Some("page-3".to_string()),
    };

    let server = MockServer::start();
    let mock_p1 = server.mock(|when, then| {
        when.method(GET)
            .path("/paginated")
            .query_param("param1", "value1")
            .query_param("param2", "value2");
        then.status(reqwest::StatusCode::OK.as_u16())
            .json_body_obj(&page1);
    });
    let mock_p2 = server.mock(|when, then| {
        when.method(GET)
            .path("/paginated")
            .query_param("page_token", "page-2");
        then.status(reqwest::StatusCode::OK.as_u16())
            .json_body_obj(&page2);
    });
    let mock_p3 = server.mock(|when, then| {
        when.method(GET)
            .path("/paginated")
            .query_param("page_token", "page-3");
        then.status(reqwest::StatusCode::NOT_FOUND.as_u16())
            .json_body(json!({ "oh": "noes!" }));
    });

    let output: Vec<Item> = vec![
        "aas".into(),
        "aba".into(),
        "abb".into(),
        "aby".into(),
        "ace".into(),
        "ach".into(),
    ];
    let output_str = format!(
        "{}\n{}\n{}",
        serde_json::to_string_pretty(&output).unwrap(),
        "An error occurred during a paginated query:",
        "HTTP status client error (404 Not Found)",
    );

    Command::cargo_bin("oxide")
        .unwrap()
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("api")
        .arg("--paginate")
        .arg("/paginated?param1=value1&param2=value2")
        .assert()
        .failure()
        .stdout(predicate::str::starts_with(output_str));

    mock_p1.assert();
    mock_p2.assert();
    mock_p3.assert();
}
