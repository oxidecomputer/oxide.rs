// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use anyhow::Result;
use clap::{Command, Parser};
use serde::Serialize;

/// Generate CLI docs in JSON format
#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
#[clap(name = "docs")]
pub struct CmdDocs;

/// Arg to CLI command for the JSON doc
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct JsonArg {
    #[serde(skip_serializing_if = "Option::is_none")]
    short: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help: Option<String>,
}

/// CLI docs in JSON format
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct JsonDoc {
    title: String,
    excerpt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    about: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<JsonArg>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    subcommands: Vec<JsonDoc>,
}

impl CmdDocs {
    pub async fn run(&self, app: &Command) -> Result<()> {
        println!("Writing docs.json");

        // let title = app.get_name().to_string();
        // let filename = format!("{}.json", title);

        let json = self.generate_json(app)?;
        let pretty_json = serde_json::to_string_pretty(&json)?;
        println!("{}", pretty_json);

        Ok(())
    }

    fn generate_json(&self, cmd: &Command) -> Result<JsonDoc> {
        let title = cmd.get_name().to_string().replace('_', " ");
        let excerpt = cmd.get_about().unwrap_or_default().to_string();

        Ok(JsonDoc {
            title,
            excerpt,
            about: cmd.get_long_about().map(|s| s.to_string()),
            args: cmd
                .get_arguments()
                .filter(|arg| arg.get_short().is_some() || arg.get_long().is_some())
                .map(|arg| JsonArg {
                    short: arg.get_short().map(|char| char.to_string()),
                    long: arg.get_long().map(String::from),
                    help: arg.get_help().map(|s| s.to_string()),
                })
                .collect(),
            subcommands: cmd
                .get_subcommands()
                .filter_map(|subcmd| self.generate_json(subcmd).ok())
                .collect(),
        })
    }
}

#[test]
fn docs_success() {
    use assert_cmd::Command;

    let mut cmd = Command::cargo_bin("oxide").unwrap();

    cmd.arg("docs");
    cmd.assert().success().stdout(format!("Writing docs.json",));
}
