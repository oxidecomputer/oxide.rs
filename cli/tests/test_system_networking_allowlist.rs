// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use assert_cmd::Command;
use httpmock::MockServer;
use oxide::types::{AllowedSourceIps2, AllowedSourceIpsUpdate};
use oxide_httpmock::MockServerExt;

// Check that we have at least one of --any or --ip
#[test]
fn test_allowlist_neither() {
    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", "")
        .env("OXIDE_TOKEN", "fake-token")
        .arg("system")
        .arg("networking")
        .arg("allow-list")
        .arg("update")
        .assert()
        .failure()
        .stderr(expectorate::eq_file_or_panic(
            "tests/data/test_system_networking_allowlist_neither.stderr",
        ));
}

// Check that we don't have both --any *and* --ip
#[test]
fn test_allowlist_both() {
    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", "")
        .env("OXIDE_TOKEN", "fake-token")
        .arg("system")
        .arg("networking")
        .arg("allow-list")
        .arg("update")
        .arg("--any")
        .arg("--ip")
        .arg("1.2.3.4/5")
        .assert()
        .failure()
        .stderr(expectorate::eq_file_or_panic(
            "tests/data/test_system_networking_allowlist_both.stderr",
        ));
}

// Just --any
#[test]
fn test_allowlist_any() {
    let server = MockServer::start();

    let mock = server.networking_allowed_source_ips_update(|when, then| {
        when.body(&AllowedSourceIpsUpdate {
            allowed_ips: AllowedSourceIps2::Any,
        });
        then.no_content();
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("system")
        .arg("networking")
        .arg("allow-list")
        .arg("update")
        .arg("--any")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_system_networking_allowlist_any.stdout",
        ));

    mock.assert();
}

// One ip
#[test]
fn test_allowlist_one_ip() {
    let server = MockServer::start();

    let mock = server.networking_allowed_source_ips_update(|when, then| {
        when.body(&AllowedSourceIpsUpdate {
            allowed_ips: AllowedSourceIps2::List(vec!["1.2.3.4/5".try_into().unwrap()]),
        });
        then.no_content();
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("system")
        .arg("networking")
        .arg("allow-list")
        .arg("update")
        .arg("--ip")
        .arg("1.2.3.4/5")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_system_networking_allowlist_one_ip.stdout",
        ));

    mock.assert();
}

#[test]
fn test_allowlist_many_ips() {
    let server = MockServer::start();

    let mock = server.networking_allowed_source_ips_update(|when, then| {
        when.body(&AllowedSourceIpsUpdate {
            allowed_ips: AllowedSourceIps2::List(vec![
                "1.2.3.4/5".try_into().unwrap(),
                "5.6.7.8/9".try_into().unwrap(),
            ]),
        });
        then.no_content();
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("system")
        .arg("networking")
        .arg("allow-list")
        .arg("update")
        .arg("--ip")
        .arg("1.2.3.4/5")
        .arg("--ip")
        .arg("5.6.7.8/9")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_system_networking_allowlist_many_ips.stdout",
        ));

    mock.assert();
}
