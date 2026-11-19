// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

use std::io::Read;
use std::process::Stdio;

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

// A reader that closes the pipe early, as `oxide completion -s bash | head`
// does, is not an error. The script is larger than the pipe's buffer,
// so the CLI is still writing when the reader closes its end.
#[test]
fn test_completion_closed_pipe() {
    let config_dir = tempfile::tempdir().unwrap();
    let mut child = std::process::Command::new(assert_cmd::cargo::cargo_bin!("oxide"))
        .arg("--config-dir")
        .arg(config_dir.path())
        .args(["completion", "--shell", "bash"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout = child.stdout.take().unwrap();
    let mut buf = [0u8; 64];
    stdout.read_exact(&mut buf).unwrap();
    drop(stdout);

    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr:\n{stderr}");
    assert!(stderr.is_empty(), "stderr:\n{stderr}");
}
