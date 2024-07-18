// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use crate::context::Context;
use crate::{println_nopipe, RunnableCmd};

use super::cmd_version::built_info;
use anyhow::Result;
use async_trait::async_trait;
use clap::{Command, Parser};
use serde::Serialize;

/// Generate CLI docs in JSON format
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "docs")]
pub struct CmdDocs;

/// Arg to CLI command for the JSON doc
#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JsonArg {
    #[serde(skip_serializing_if = "Option::is_none")]
    long: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    values: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help: Option<String>,
}

/// CLI docs in JSON format
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct JsonDoc {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
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
    let mut subcommands = cmd
        .get_subcommands()
        .filter(|cmd| cmd.get_name() != "help")
        .map(to_json)
        .collect::<Vec<_>>();
    subcommands.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    let mut args = cmd
        .get_arguments()
        .filter(|arg| arg.get_long() != Some("help"))
        .filter(|arg| arg.get_short().is_some() || arg.get_long().is_some())
        .map(|arg| JsonArg {
            short: arg.get_short().map(|char| char.to_string()),
            long: arg.get_long().map(ToString::to_string),
            values: arg
                .get_possible_values()
                .into_iter()
                .map(|value| value.get_name().to_string())
                .collect(),
            help: arg.get_help().map(ToString::to_string),
        })
        .collect::<Vec<_>>();
    args.sort_unstable();

    let name = cmd.get_name().to_string();
    let version: Option<String> = if name == "oxide" {
        Some(built_info::PKG_VERSION.to_string())
    } else {
        None
    };

    JsonDoc {
        name,
        version,
        about: cmd.get_about().map(ToString::to_string),
        long_about: cmd.get_long_about().map(ToString::to_string),
        args,
        subcommands,
    }
}

#[async_trait]
impl RunnableCmd for CmdDocs {
    async fn run(&self, _ctx: &Context) -> Result<()> {
        let cli = crate::make_cli();
        let mut app = cli.command_take();
        app.build();
        let json_doc = to_json(&app);
        let pretty_json = serde_json::to_string_pretty(&json_doc)?;
        println_nopipe!("{}", pretty_json);
        Ok(())
    }
}
