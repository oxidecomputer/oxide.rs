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
#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JsonArg {
    #[serde(skip_serializing_if = "Option::is_none")]
    long: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help: Option<String>,
}

/// CLI docs in JSON format
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct JsonDoc {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_about: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<JsonArg>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    subcommands: Vec<JsonDoc>,
}

fn to_json(cmd: &Command) -> JsonDoc {
    let mut subcommands = cmd.get_subcommands().map(to_json).collect::<Vec<_>>();
    subcommands.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    let mut args = cmd
        .get_arguments()
        .filter(|arg| arg.get_short().is_some() || arg.get_long().is_some())
        .map(|arg| JsonArg {
            short: arg.get_short().map(|char| char.to_string()),
            long: arg.get_long().map(ToString::to_string),
            help: arg.get_help().map(ToString::to_string),
        })
        .collect::<Vec<_>>();
    args.sort_unstable();
    JsonDoc {
        name: cmd.get_name().to_string(),
        about: cmd.get_about().map(ToString::to_string),
        long_about: cmd.get_long_about().map(ToString::to_string),
        args,
        subcommands,
    }
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct OutputJson {
    version: String,
    commands: JsonDoc,
}

impl CmdDocs {
    pub async fn run(&self, app: &Command) -> Result<()> {
        const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");
        let json_doc = to_json(app);
        let output = OutputJson {
            version: CLI_VERSION.to_string(),
            commands: json_doc,
        };
        let pretty_json = serde_json::to_string_pretty(&output)?;
        println!("{}", pretty_json);
        Ok(())
    }
}
