// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::RunnableCmd;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use colored::*;
use oxide::context::Context;
use oxide::types::{BgpPeerStatus, SwitchLocation};
use oxide::ClientSystemNetworkingExt;
use std::io::Write;
use tabwriter::TabWriter;

/// Get the status of switch ports.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "net bgp status")]
pub struct CmdBgpStatus {}

#[async_trait]
impl RunnableCmd for CmdBgpStatus {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let c = ctx.client()?;

        let status = c.networking_bgp_status().send().await?.into_inner();

        let (sw0, sw1) = status
            .iter()
            .partition(|x| x.switch == SwitchLocation::Switch0);

        println!("{}", "switch0".dimmed());
        println!("{}", "=======".dimmed());
        show_status(&sw0)?;
        println!();

        println!("{}", "switch1".dimmed());
        println!("{}", "=======".dimmed());
        show_status(&sw1)?;

        Ok(())
    }
}

fn show_status(st: &Vec<&BgpPeerStatus>) -> Result<()> {
    let mut tw = TabWriter::new(std::io::stdout()).ansi(true);
    writeln!(
        &mut tw,
        "{}\t{}\t{}\t{}\t{}",
        "Peer Address".dimmed(),
        "Local ASN".dimmed(),
        "Remote ASN".dimmed(),
        "Session State".dimmed(),
        "State Duration".dimmed(),
    )?;
    for s in st {
        writeln!(
            tw,
            "{}\t{}\t{}\t{:?}\t{}",
            s.addr,
            s.local_asn,
            s.remote_asn,
            s.state,
            humantime::Duration::from(std::time::Duration::from_millis(s.state_duration_millis)),
        )?;
    }
    tw.flush()?;
    Ok(())
}
