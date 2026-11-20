// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

/// The API version from `info.version` in oxide.json.
fn api_version() -> String {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../oxide.json");
    let spec: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    spec["info"]["version"].as_str().unwrap().to_string()
}

#[test]
fn test_version() {
    let output = assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .arg("version")
        .assert()
        .success()
        .get_output()
        .clone();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines = stdout.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 3, "unexpected output:\n{stdout}");

    assert_eq!(lines[0], format!("Oxide CLI {}", env!("CARGO_PKG_VERSION")));

    let commit = lines[1]
        .strip_prefix("Built from commit: ")
        .unwrap_or_else(|| panic!("unexpected commit line: {:?}", lines[1]));
    let hash = commit.strip_suffix(" (dirty)").unwrap_or(commit);
    assert!(
        hash.len() == 40 && hash.chars().all(|c| c.is_ascii_hexdigit()),
        "unexpected commit line: {:?}",
        lines[1],
    );

    assert_eq!(lines[2], format!("Oxide API: {}", api_version()));
}
