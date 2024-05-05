// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io::Write;

use crate::RunnableCmd;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use colored::*;
use oxide::context::Context;
use oxide::{ClientSystemHardwareExt, ClientSystemNetworkingExt};
use tabwriter::TabWriter;

/// Get the configuration of switch ports.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "net port config")]
pub struct CmdPortConfig {}

#[async_trait]
impl RunnableCmd for CmdPortConfig {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let c = ctx.client()?;

        let ports = c
            .networking_switch_port_list()
            .limit(u32::MAX)
            .send()
            .await?
            .into_inner()
            .items;

        let mut tw = TabWriter::new(std::io::stdout()).ansi(true);

        // TODO bad API, having to pull all address lots to work backwards from
        // address reference to address lot block is terribad
        let addr_lots = c
            .networking_address_lot_list()
            .limit(u32::MAX)
            .send()
            .await?
            .into_inner()
            .items;

        let mut addr_lot_blocks = Vec::new();
        for a in addr_lots.iter() {
            let blocks = c
                .networking_address_lot_block_list()
                .address_lot(a.id)
                .limit(u32::MAX)
                .send()
                .await?
                .into_inner()
                .items;

            for b in blocks.iter() {
                addr_lot_blocks.push((a.clone(), b.clone()));
            }
        }

        for p in &ports {
            if let Some(id) = p.port_settings_id {
                let config = c
                    .networking_switch_port_settings_view()
                    .port(id)
                    .send()
                    .await?
                    .into_inner();

                println!("{}", p.port_name.blue());
                println!("{}", "=".repeat(p.port_name.len()).dimmed());
                println!("");

                writeln!(&mut tw, "{}\t{}", "Address".dimmed(), "Lot".dimmed())?;
                for a in &config.addresses {
                    let addr = match &a.address {
                        oxide::types::IpNet::V4(a) => a.to_string(),
                        oxide::types::IpNet::V6(a) => a.to_string(),
                    };

                    let alb = addr_lot_blocks
                        .iter()
                        .find(|x| x.1.id == a.address_lot_block_id)
                        .unwrap();

                    writeln!(&mut tw, "{}\t{}", addr, alb.0.name.to_string())?;
                }
                tw.flush()?;
                println!("");

                writeln!(&mut tw, "{}", "BGP Peer".dimmed())?;
                for p in &config.bgp_peers {
                    writeln!(&mut tw, "{}", p.addr)?;
                }

                println!("");
                println!("{:#?}", config);
                println!("");
            }
        }

        Ok(())
    }
}
