// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::{prelude::*, Then, When};
use oxide_api::types::{Project, ProjectResultsPage};
use predicates::prelude::*;
use serde::Serialize;
use serde_json::json;

/// Validate `oxide api` for a simple GET with parameters.
#[test]
fn text_xxx() {
    let server = MockServer::start();

    let mock = server.project_list(|when, then| {
        when.limit(10.try_into().unwrap());
        then.success_ok(ProjectResultsPage {
            items: vec![Project::builder().try_into().unwrap()],
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

struct ProjectListWhen(When);

impl ProjectListWhen {
    fn new(inner: When) -> Self {
        Self(inner.path("/v1/projects").method(GET))
    }

    fn limit(self, value: std::num::NonZeroU32) -> Self {
        Self(self.0.query_param("limit", value.to_string()))
    }
}

impl From<ProjectListWhen> for When {
    fn from(value: ProjectListWhen) -> Self {
        value.0
    }
}

struct ProjectListThen(Then);
impl From<ProjectListThen> for Then {
    fn from(value: ProjectListThen) -> Self {
        value.0
    }
}

impl ProjectListThen {
    fn new(inner: Then) -> Self {
        Self(inner)
    }

    fn success_ok(self, body: oxide_api::types::ProjectResultsPage) -> Self {
        Self(
            self.0
                .status(http::StatusCode::OK.as_u16())
                .json_body(serde_json::to_value(body).unwrap()),
        )
    }

    fn error(self, status: u16, body: oxide_api::types::Error) -> Self {
        Self(
            self.0
                .status(status)
                .json_body(serde_json::to_value(body).unwrap()),
        )
    }
}

trait MockServerExt {
    fn project_list<F>(&self, f: F) -> httpmock::Mock
    where
        F: FnOnce(ProjectListWhen, ProjectListThen);
}

impl MockServerExt for MockServer {
    fn project_list<F>(&self, f: F) -> httpmock::Mock
    where
        F: FnOnce(ProjectListWhen, ProjectListThen),
    {
        self.mock(|when, then| {
            f(ProjectListWhen::new(when), ProjectListThen::new(then));
        })
    }
}
