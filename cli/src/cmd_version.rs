// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use clap::Parser;

/// Prints version information about the CLI, associated API and source code location.
#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
#[clap(name = "version")]
pub struct CmdVersion;

impl CmdVersion {
    pub async fn run(&self) -> Result<()> {
        println!("oxide CLI {}", env!("CARGO_PKG_VERSION"));

        let path = env!("CARGO_MANIFEST_DIR");
        let meta = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .current_dir(&path)
            .no_deps()
            .exec()
            .unwrap();

        let root = meta.workspace_packages();
        let api = root.into_iter().find(|&x| x.name == "oxide-api").unwrap();
        println!("{} {}", api.name, api.version);

        println!("source {}", env!("CARGO_PKG_REPOSITORY"));

        Ok(())
    }
}

#[test]
fn version_success() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("oxide").unwrap();

    cmd.arg("version");
    cmd.assert().success().stdout(
        "oxide CLI 0.1.0\noxide-api 0.1.0\nsource https://github.com/oxidecomputer/oxide-sdk-and-cli\n",
    );
}
