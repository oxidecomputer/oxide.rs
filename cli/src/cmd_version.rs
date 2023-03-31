// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use anyhow::Result;
use clap::Parser;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Prints version information about the CLI.
#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
#[clap(name = "version")]
pub struct CmdVersion;

impl CmdVersion {
    pub async fn run(&self) -> Result<()> {
        println!("Oxide CLI {}", clap::crate_version!());

        println!(
            "Built from commit: {} {}",
            built_info::GIT_COMMIT_HASH.unwrap(),
            if matches!(built_info::GIT_DIRTY, Some(true)) {
                "(dirty)"
            } else {
                ""
            }
        );

        Ok(())
    }
}

#[test]
fn version_success() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("oxide").unwrap();

    cmd.arg("version");
    cmd.assert().success().stdout(format!(
        "Oxide CLI 0.1.0\nBuilt from commit: {} {}\n",
        built_info::GIT_COMMIT_HASH.unwrap(),
        if matches!(built_info::GIT_DIRTY, Some(true)) {
            "(dirty)"
        } else {
            ""
        }
    ));
}
