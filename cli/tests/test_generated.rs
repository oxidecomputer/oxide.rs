// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

use std::io::Write;

use httpmock::MockServer;
use oxide::types::{
    Certificate, CertificateCreate, IpPoolRange, IpRange, Ipv4Range, Ipv6Range, Project,
    ProjectCreate, ProjectResultsPage, ServiceUsingCertificate,
};
use oxide_httpmock::MockServerExt;
use predicates::prelude::predicate;
use rand::SeedableRng;
use test_common::JsonMock;

fn oxide_cmd(server: &MockServer) -> assert_cmd::Command {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("oxide");
    cmd.env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token");
    cmd
}

fn pool_range(src: &mut rand::rngs::SmallRng, range: &IpRange) -> IpPoolRange {
    IpPoolRange {
        range: range.clone(),
        ..IpPoolRange::mock_value(src).unwrap()
    }
}

#[test]
fn test_generated_ip_pool_range_add_v4() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let range = IpRange::V4(Ipv4Range {
        first: "10.0.0.1".parse().unwrap(),
        last: "10.0.0.10".parse().unwrap(),
    });
    let response = pool_range(&mut src, &range);
    let mock = server.system_ip_pool_range_add(|when, then| {
        when.pool(&"mypool".parse().unwrap()).body(&range);
        then.created(&response);
    });

    oxide_cmd(&server)
        .args([
            "system",
            "networking",
            "ip-pool",
            "range",
            "add",
            "--pool",
            "mypool",
            "--first",
            "10.0.0.1",
            "--last",
            "10.0.0.10",
        ])
        .assert()
        .success();

    mock.assert();
}

#[test]
fn test_generated_ip_pool_range_add_v6() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let range = IpRange::V6(Ipv6Range {
        first: "fd00::1".parse().unwrap(),
        last: "fd00::ff".parse().unwrap(),
    });
    let response = pool_range(&mut src, &range);
    let mock = server.system_ip_pool_range_add(|when, then| {
        when.pool(&"mypool".parse().unwrap()).body(&range);
        then.created(&response);
    });

    oxide_cmd(&server)
        .args([
            "system",
            "networking",
            "ip-pool",
            "range",
            "add",
            "--pool",
            "mypool",
            "--first",
            "fd00::1",
            "--last",
            "fd00::ff",
        ])
        .assert()
        .success();

    mock.assert();
}

#[test]
fn test_generated_ip_pool_range_remove_v4() {
    let server = MockServer::start();

    let range = IpRange::V4(Ipv4Range {
        first: "10.0.0.1".parse().unwrap(),
        last: "10.0.0.10".parse().unwrap(),
    });
    let mock = server.system_ip_pool_range_remove(|when, then| {
        when.pool(&"mypool".parse().unwrap()).body(&range);
        then.no_content();
    });

    oxide_cmd(&server)
        .args([
            "system",
            "networking",
            "ip-pool",
            "range",
            "remove",
            "--pool",
            "mypool",
            "--first",
            "10.0.0.1",
            "--last",
            "10.0.0.10",
        ])
        .assert()
        .success();

    mock.assert();
}

#[test]
fn test_generated_ip_pool_range_add_mixed() {
    let server = MockServer::start();
    let any = server.mock(|when, then| {
        when.any_request();
        then.status(500);
    });

    oxide_cmd(&server)
        .args([
            "system",
            "networking",
            "ip-pool",
            "range",
            "add",
            "--pool",
            "mypool",
            "--first",
            "10.0.0.1",
            "--last",
            "fd00::ff",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "first and last must either both be ipv4 or ipv6 addresses",
        ));

    any.assert_calls(0);
}

#[test]
fn test_generated_certificate_create() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let cert_pem = "-----BEGIN CERTIFICATE-----\nMIIBcert\n-----END CERTIFICATE-----\n";
    let key_pem = "-----BEGIN PRIVATE KEY-----\nMIIBkey\n-----END PRIVATE KEY-----\n";

    let mut cert_file = tempfile::NamedTempFile::new().unwrap();
    cert_file.write_all(cert_pem.as_bytes()).unwrap();
    let mut key_file = tempfile::NamedTempFile::new().unwrap();
    key_file.write_all(key_pem.as_bytes()).unwrap();

    let body = CertificateCreate {
        cert: cert_pem.to_string(),
        description: "a cert".to_string(),
        key: key_pem.to_string(),
        name: "mycert".parse().unwrap(),
        service: ServiceUsingCertificate::ExternalApi,
    };
    let response = Certificate::mock_value(&mut src).unwrap();
    let mock = server.certificate_create(|when, then| {
        when.body(&body);
        then.created(&response);
    });

    oxide_cmd(&server)
        .args(["certificate", "create", "--name", "mycert"])
        .args(["--description", "a cert", "--service", "external_api"])
        .arg("--cert")
        .arg(cert_file.path())
        .arg("--key")
        .arg(key_file.path())
        .assert()
        .success();

    mock.assert();
}

#[test]
fn test_generated_json_body() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let body = ProjectCreate {
        defaults: None,
        description: "from the file".to_string(),
        name: "file-name".parse().unwrap(),
    };
    let mut body_file = tempfile::NamedTempFile::new().unwrap();
    serde_json::to_writer(&mut body_file, &body).unwrap();

    let response = Project {
        description: body.description.clone(),
        name: body.name.clone(),
        ..Project::mock_value(&mut src).unwrap()
    };
    let mock = server.project_create(|when, then| {
        when.body(&body);
        then.created(&response);
    });

    oxide_cmd(&server)
        .args(["project", "create", "--json-body"])
        .arg(body_file.path())
        .assert()
        .success()
        .stdout(format!(
            "{}\n",
            serde_json::to_string_pretty(&response).unwrap()
        ));

    mock.assert();
}

#[test]
fn test_generated_json_body_schema() {
    let server = MockServer::start();
    let any = server.mock(|when, then| {
        when.any_request();
        then.status(500);
    });

    oxide_cmd(&server)
        .args(["project", "create", "--json-body-schema"])
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_generated_json_body_schema.stdout",
        ));

    any.assert_calls(0);
}

#[test]
fn test_generated_empty_list() {
    let server = MockServer::start();

    let mock = server.project_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&ProjectResultsPage {
            items: Vec::new(),
            next_page: None,
        });
    });

    oxide_cmd(&server)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout("[]\n");

    mock.assert();
}

#[test]
fn test_generated_api_error() {
    let server = MockServer::start();

    let message = "not found: project with name nope";
    let mock = server.project_view(|when, then| {
        when.project(&"nope".parse().unwrap());
        then.client_error(
            404,
            &oxide::types::Error {
                error_code: Some("ObjectNotFound".to_string()),
                message: message.to_string(),
                request_id: "req-1234".to_string(),
            },
        );
    });

    oxide_cmd(&server)
        .args(["project", "view", "--project", "nope"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(message));

    mock.assert();
}
