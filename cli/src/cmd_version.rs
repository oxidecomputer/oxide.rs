// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use oxide::Client;

use crate::println_nopipe;
use crate::{context::Context, RunnableCmd};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Prints version information about the CLI.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "version")]
pub struct CmdVersion;

#[async_trait]
impl RunnableCmd for CmdVersion {
    async fn run(&self, _ctx: &Context) -> Result<()> {
        let cli_version = built_info::PKG_VERSION;
        let api_version = Client::new("").api_version();

        println_nopipe!("Oxide CLI {}", cli_version);

        println_nopipe!(
            "Built from commit: {} {}",
            built_info::GIT_COMMIT_HASH.unwrap(),
            if matches!(built_info::GIT_DIRTY, Some(true)) {
                "(dirty)"
            } else {
                ""
            }
        );

        println_nopipe!("Oxide API: {}", api_version);

        Ok(())
    }
}

#[test]
fn version_success() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("oxide").unwrap();
    let cli_version = built_info::PKG_VERSION;
    let api_version = Client::new("").api_version();

    cmd.arg("version");
    cmd.assert().success().stdout(format!(
        "Oxide CLI {}\nBuilt from commit: {} {}\nOxide API: {}\n",
        cli_version,
        built_info::GIT_COMMIT_HASH.unwrap(),
        if matches!(built_info::GIT_DIRTY, Some(true)) {
            "(dirty)"
        } else {
            ""
        },
        api_version,
    ));
}
