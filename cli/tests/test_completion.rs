// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

use predicates::prelude::*;

/// Subcommands that every completion script must mention.
const SUBCOMMANDS: &[&str] = &[
    "project",
    "instance",
    "disk",
    "bundle",
    "completion",
    "version",
];

/// Checks that the script for `shell` contains `marker` and the subcommands.
fn check_completion(shell: &str, marker: &str) {
    let mut pred = predicate::str::contains(marker).boxed();
    for subcommand in SUBCOMMANDS {
        pred = pred.and(predicate::str::contains(*subcommand)).boxed();
    }

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .args(["completion", "--shell", shell])
        .assert()
        .success()
        .stdout(pred);
}

#[test]
fn test_completion_bash() {
    check_completion("bash", "complete -F _oxide");
}

#[test]
fn test_completion_zsh() {
    check_completion("zsh", "#compdef oxide");
}

#[test]
fn test_completion_fish() {
    check_completion("fish", "complete -c oxide");
}

#[test]
fn test_completion_powershell() {
    check_completion("powershell", "Register-ArgumentCompleter");
}

#[test]
fn test_completion_elvish() {
    check_completion("elvish", "edit:completion:arg-completer[oxide]");
}
