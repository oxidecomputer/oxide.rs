// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

use crate::cli_builder::OperationId;
use crate::context::Context;
use crate::{println_nopipe, RunnableCmd};

use super::cmd_version::built_info;
use anyhow::Result;
use async_trait::async_trait;
use clap::{Command, Parser, ValueEnum};
use serde::Serialize;

/// List all CLI commands
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "docs")]
pub struct CmdDocs {
    /// Output format
    #[clap(long, default_value = "flat")]
    format: DocsFormat,
}

#[derive(ValueEnum, Debug, Clone)]
enum DocsFormat {
    /// One line per command: full path and description
    Flat,
    /// Full JSON tree with args and metadata
    Json,
}

/// Arg to CLI command for the JSON doc
#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JsonArg {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    values: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    global: bool,
}

fn is_false(value: &bool) -> bool {
    !value
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
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_id: Option<String>,
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
        .filter(|arg| !arg.is_hide_set())
        .map(|arg| JsonArg {
            name: (arg.get_long().is_none() && arg.get_short().is_none())
                .then_some(arg.get_id().to_string()),
            short: arg.get_short().map(|char| char.to_string()),
            long: arg.get_long().map(ToString::to_string),
            values: arg
                .get_possible_values()
                .into_iter()
                .map(|value| value.get_name().to_string())
                .collect(),
            help: arg.get_help().map(ToString::to_string),
            global: arg.is_global_set(),
        })
        .collect::<Vec<_>>();
    args.sort_unstable();

    let name = cmd.get_name().to_string();
    let version: Option<String> = if name == "oxide" {
        Some(built_info::PKG_VERSION.to_string())
    } else {
        None
    };
    let operation_id = cmd.get::<OperationId>().map(|id| id.0.clone());

    JsonDoc {
        name,
        version,
        about: cmd.get_about().map(ToString::to_string),
        long_about: cmd.get_long_about().map(ToString::to_string),
        operation_id,
        args,
        subcommands,
    }
}

fn print_flat(cmd: &Command, prefix: &str) {
    let name = if prefix.is_empty() {
        cmd.get_name().to_string()
    } else {
        format!("{} {}", prefix, cmd.get_name())
    };

    let mut subs: Vec<_> = cmd
        .get_subcommands()
        .filter(|c| c.get_name() != "help")
        .collect();

    // Print this command if it's a leaf or if it's runnable on its own
    // (i.e., subcommands exist but aren't required, like `oxide disk import`)
    if subs.is_empty() || !cmd.is_subcommand_required_set() {
        let about = cmd
            .get_about()
            .map(|a| format!(" — {a}"))
            .unwrap_or_default();
        println_nopipe!("{name}{about}");
    }
    subs.sort_by_key(|c| c.get_name().to_owned());
    for sub in subs {
        print_flat(sub, &name);
    }
}

#[async_trait]
impl RunnableCmd for CmdDocs {
    async fn run(&self, _ctx: &Context) -> Result<()> {
        let cli = crate::make_cli();
        let mut app = cli.command_take();
        app.build();

        match self.format {
            DocsFormat::Json => {
                let json_doc = to_json(&app);
                let pretty_json = serde_json::to_string_pretty(&json_doc)?;
                println_nopipe!("{}", pretty_json);
            }
            DocsFormat::Flat => {
                print_flat(&app, "");
            }
        }

        Ok(())
    }
}
