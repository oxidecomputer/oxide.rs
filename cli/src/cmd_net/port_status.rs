// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::RunnableCmd;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use colored::*;
use oxide::context::Context;
use oxide::types::SwitchPort;
use oxide::{Client, ClientSystemHardwareExt};
use serde::{Deserialize, Serialize};
use std::io::Write;
use tabwriter::TabWriter;

/// Get the status of switch ports.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "net port status")]
pub struct CmdPortStatus {}

#[async_trait]
impl RunnableCmd for CmdPortStatus {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let c = ctx.client()?;

        let ports = c
            .networking_switch_port_list()
            .limit(u32::MAX)
            .send()
            .await?
            .into_inner()
            .items;

        let (mut sw0, mut sw1): (Vec<&SwitchPort>, Vec<&SwitchPort>) = ports
            .iter()
            .partition(|x| x.switch_location.as_str() == "switch0");

        sw0.sort_by_key(|x| x.port_name.as_str());
        sw1.sort_by_key(|x| x.port_name.as_str());

        println!("{}", "switch0".dimmed());
        println!("{}", "=======".dimmed());
        self.show_switch(&c, "switch0", &sw0).await?;

        println!("{}", "switch1".dimmed());
        println!("{}", "=======".dimmed());
        self.show_switch(&c, "switch1", &sw1).await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MacAddr {
    a: [u8; 6],
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct LinkStatus {
    address: MacAddr,
    enabled: bool,
    autoneg: bool,
    fec: String,
    link_state: String,
    fsm_state: String,
    media: String,
    speed: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ReceiverPower {
    /// The measurement is represents average optical power, in mW.
    Average(f32),

    /// The measurement represents a peak-to-peak, in mW.
    PeakToPeak(f32),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Monitors {
    receiver_power: Vec<ReceiverPower>,
    transmitter_bias_current: Vec<f32>,
    transmitter_power: Vec<f32>,
}

impl CmdPortStatus {
    async fn show_switch(&self, c: &Client, sw: &str, ports: &Vec<&SwitchPort>) -> Result<()> {
        let mut ltw = TabWriter::new(std::io::stdout()).ansi(true);
        let mut mtw = TabWriter::new(std::io::stdout()).ansi(true);

        writeln!(
            &mut ltw,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            "Port".dimmed(),
            "Configured".dimmed(),
            "Enabled".dimmed(),
            "MAC".dimmed(),
            "Autoneg".dimmed(),
            "FEC".dimmed(),
            "Link/FSM State".dimmed(),
            "Media".dimmed(),
            "Speed".dimmed(),
        )?;

        writeln!(
            &mut mtw,
            "{}\t{}\t{}",
            "Receiver Power".dimmed(),
            "Transmitter Bias Current".dimmed(),
            "Transmitter Power".dimmed(),
        )?;

        for p in ports {
            let status = c
                .networking_switch_port_status()
                .port(&p.port_name)
                .rack_id(p.rack_id)
                .switch_location(sw)
                .send()
                .await
                .ok()
                .map(|x| x.into_inner().0);

            //println!("{:?}", status);

            let link = status.as_ref().map(|x| {
                let ls: LinkStatus =
                    serde_json::from_value(x.get("link").unwrap().clone()).unwrap();
                ls
            });

            writeln!(
                &mut ltw,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                p.port_name,
                p.port_settings_id.is_some(),
                link.as_ref()
                    .map(|x| x.enabled.to_string())
                    .unwrap_or("-".to_string()),
                link.as_ref()
                    .map(|x| format!(
                        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                        x.address.a[0],
                        x.address.a[1],
                        x.address.a[2],
                        x.address.a[3],
                        x.address.a[4],
                        x.address.a[5]
                    ))
                    .unwrap_or("-".to_string()),
                link.as_ref()
                    .map(|x| x.autoneg.to_string())
                    .unwrap_or("-".to_string()),
                link.as_ref()
                    .map(|x| x.fec.clone())
                    .unwrap_or("-".to_string()),
                link.as_ref()
                    .map(|x| format!("{}/{}", x.link_state, x.fsm_state))
                    .unwrap_or("-".to_string()),
                link.as_ref()
                    .map(|x| x.media.clone())
                    .unwrap_or("-".to_string()),
                link.as_ref()
                    .map(|x| x.speed.clone())
                    .unwrap_or("-".to_string()),
            )?;

            let monitors = status.as_ref().map(|x| {
                let ls: Monitors =
                    serde_json::from_value(x.get("monitors").unwrap().clone()).unwrap();
                ls
            });

            writeln!(
                &mut mtw,
                "{}\t{}\t{}",
                monitors
                    .as_ref()
                    .map(|x| format!("{:?}", x.receiver_power))
                    .unwrap_or("-".to_string()),
                monitors
                    .as_ref()
                    .map(|x| format!("{:?}", x.transmitter_bias_current))
                    .unwrap_or("-".to_string()),
                monitors
                    .as_ref()
                    .map(|x| format!("{:?}", x.transmitter_power))
                    .unwrap_or("-".to_string()),
            )?;
        }

        ltw.flush()?;
        println!();
        mtw.flush()?;
        println!();

        Ok(())
    }
}
