// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;

#[test]
fn test_docs_json() {
    Command::cargo_bin("oxide")
        .unwrap()
        .arg("docs")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic("docs/cli.json"));
}
