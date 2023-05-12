// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use oxide_api::Client;

use crate::{context::Context, RunnableCmd};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Prints version information about the CLI.
#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
#[clap(name = "version")]
pub struct CmdVersion;

#[async_trait]
impl RunnableCmd for CmdVersion {
    async fn run(&self, _ctx: &Context) -> Result<()> {
        println!("Oxide CLI {}", built_info::PKG_VERSION);

        println!(
            "Built from commit: {} {}",
            built_info::GIT_COMMIT_HASH.unwrap(),
            if matches!(built_info::GIT_DIRTY, Some(true)) {
                "(dirty)"
            } else {
                ""
            }
        );

        let client = Client::new("");
        println!("Oxide API: {}", client.api_version());

        Ok(())
    }
}

#[test]
fn version_success() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("oxide").unwrap();

    cmd.arg("version");
    cmd.assert().success().stdout(format!(
        "Oxide CLI 0.1.0\nBuilt from commit: {} {}\nOxide API: 0.0.1\n",
        built_info::GIT_COMMIT_HASH.unwrap(),
        if matches!(built_info::GIT_DIRTY, Some(true)) {
            "(dirty)"
        } else {
            ""
        }
    ));
}
