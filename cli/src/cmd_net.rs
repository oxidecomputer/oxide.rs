use std::{collections::HashMap, net::IpAddr};

use crate::RunnableCmd;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use colored::*;
use oxide::{
    context::Context,
    types::{
        Address, AddressConfig, BgpPeer, BgpPeerConfig, BgpPeerStatus, ImportExportPolicy, IpNet,
        LinkConfigCreate, LinkFec, LinkSpeed, LldpServiceConfigCreate, Name, NameOrId, Route,
        RouteConfig, SwitchInterfaceConfigCreate, SwitchInterfaceKind, SwitchInterfaceKind2,
        SwitchLocation, SwitchPort, SwitchPortConfigCreate, SwitchPortGeometry,
        SwitchPortGeometry2, SwitchPortSettingsCreate,
    },
    Client, ClientSystemHardwareExt, ClientSystemNetworkingExt,
};
use serde::{Deserialize, Serialize};
use std::io::Write;
use tabwriter::TabWriter;
use uuid::Uuid;

// We do not yet support port breakouts, but the API is phrased in terms of
// ports that can be broken out. The constant phy0 represents the first port
// in a breakout.
const PHY0: &str = "phy0";

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "net")]
pub struct CmdNet {
    #[clap(subcommand)]
    subcmd: NetSubCommand,
}

#[derive(Parser, Debug, Clone)]
enum NetSubCommand {
    /// Address management.
    Addr(CmdAddr),

    /// Port management.
    Port(CmdPort),

    /// Link management.
    Link(CmdLink),

    /// BGP management.
    Bgp(CmdBgp),
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "port")]
pub struct CmdPort {
    #[clap(subcommand)]
    subcmd: PortSubCommand,
}

#[async_trait]
impl RunnableCmd for CmdPort {
    async fn run(&self, ctx: &Context) -> Result<()> {
        match &self.subcmd {
            PortSubCommand::Config(cmd) => cmd.run(ctx).await,
            PortSubCommand::Status(cmd) => cmd.run(ctx).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum PortSubCommand {
    /// Manage port configuration.
    Config(CmdPortConfig),

    /// Observe port status.
    Status(CmdPortStatus),
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "link")]
pub struct CmdLink {
    #[clap(subcommand)]
    subcmd: LinkSubCommand,
}

#[async_trait]
impl RunnableCmd for CmdLink {
    async fn run(&self, ctx: &Context) -> Result<()> {
        match &self.subcmd {
            LinkSubCommand::Add(cmd) => cmd.run(ctx).await,
            LinkSubCommand::Del(cmd) => cmd.run(ctx).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum LinkSubCommand {
    /// Add a link to a port.
    Add(CmdLinkAdd),

    /// Remove a link from a port.
    Del(CmdLinkDel),
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "add")]
pub struct CmdLinkAdd {
    /// Id of the rack to add the link to.
    rack: Uuid,

    /// Switch to add the link to.
    #[arg(value_enum)]
    switch: Switch,

    /// Port to add the link to.
    #[arg(value_enum)]
    port: Port,

    /// Whether or not to set auto-negotiation
    #[arg(long)]
    pub autoneg: bool,

    /// The forward error correction mode of the link.
    #[arg(value_enum)]
    pub fec: LinkFec,

    /// Maximum transmission unit for the link.
    #[arg(long, default_value_t = 1500u16)]
    pub mtu: u16,

    /// The speed of the link.
    #[arg(value_enum)]
    pub speed: LinkSpeed,
}

#[async_trait]
impl RunnableCmd for CmdLinkAdd {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let mut settings = current_port_settings(ctx, &self.rack, &self.switch, &self.port).await?;
        let link = LinkConfigCreate {
            autoneg: self.autoneg,
            fec: self.fec,
            mtu: self.mtu,
            speed: self.speed,
            //TODO not fully plumbed on the back end yet.
            lldp: LldpServiceConfigCreate {
                enabled: false,
                lldp_config: None,
            },
        };
        match settings.links.get(PHY0) {
            Some(_) => {
                return Err(anyhow::anyhow!("only one link per port supported"));
            }
            None => {
                settings.links.insert(String::from(PHY0), link);
            }
        }
        ctx.client()?
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "del")]
pub struct CmdLinkDel {
    /// Id of the rack to remove the link from.
    rack: Uuid,

    /// Switch to remove the link from.
    #[arg(value_enum)]
    switch: Switch,

    /// Port to remove the link from.
    #[arg(value_enum)]
    port: Port,
}

#[async_trait]
impl RunnableCmd for CmdLinkDel {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let mut settings = current_port_settings(ctx, &self.rack, &self.switch, &self.port).await?;
        settings.links.clear();
        ctx.client()?
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "bgp")]
pub struct CmdBgp {
    #[clap(subcommand)]
    subcmd: BgpSubCommand,
}

#[async_trait]
impl RunnableCmd for CmdBgp {
    async fn run(&self, ctx: &Context) -> Result<()> {
        match &self.subcmd {
            BgpSubCommand::Status(cmd) => cmd.run(ctx).await,
            BgpSubCommand::Config(cmd) => cmd.run(ctx).await,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Parser, Debug, Clone)]
enum BgpSubCommand {
    /// Observe BGP status.
    Status(CmdBgpStatus),

    /// Manage BGP configuration.
    Config(CmdBgpConfig),
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "addr")]
pub struct CmdAddr {
    #[clap(subcommand)]
    subcmd: AddrSubCommand,
}

#[async_trait]
impl RunnableCmd for CmdAddr {
    async fn run(&self, ctx: &Context) -> Result<()> {
        match &self.subcmd {
            AddrSubCommand::Add(cmd) => cmd.run(ctx).await,
            AddrSubCommand::Del(cmd) => cmd.run(ctx).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum AddrSubCommand {
    /// Add an address to a port configuration.
    Add(CmdAddrAdd),

    /// Remove an address from a port configuration.
    Del(CmdAddrDel),
}

#[async_trait]
impl RunnableCmd for CmdNet {
    async fn run(&self, ctx: &Context) -> Result<()> {
        match &self.subcmd {
            NetSubCommand::Addr(cmd) => cmd.run(ctx).await,
            NetSubCommand::Port(cmd) => cmd.run(ctx).await,
            NetSubCommand::Link(cmd) => cmd.run(ctx).await,
            NetSubCommand::Bgp(cmd) => cmd.run(ctx).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "add")]
pub struct CmdAddrAdd {
    /// Id of the rack to add the address to.
    rack: Uuid,

    /// Switch to add the address to.
    #[arg(value_enum)]
    switch: Switch,

    /// Port to add the port to.
    #[arg(value_enum)]
    port: Port,

    /// Address to add.
    addr: oxnet::Ipv4Net,

    /// Address lot to allocate from.
    lot: NameOrId,

    /// Optional VLAN to assign to the address.
    vlan: Option<u16>,
}

#[async_trait]
impl RunnableCmd for CmdAddrAdd {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let mut settings = current_port_settings(ctx, &self.rack, &self.switch, &self.port).await?;
        let addr = Address {
            address: IpNet::V4(self.addr.to_string().parse().unwrap()),
            address_lot: self.lot.clone(),
            vlan_id: self.vlan,
        };
        match settings.addresses.get_mut(PHY0) {
            Some(ac) => {
                ac.addresses.push(addr);
            }
            None => {
                settings.addresses.insert(
                    String::from(PHY0),
                    AddressConfig {
                        addresses: vec![addr],
                    },
                );
            }
        }
        ctx.client()?
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "del")]
pub struct CmdAddrDel {
    /// Id of the rack to remove the address from.
    rack: Uuid,

    /// Switch to remove the address from.
    #[arg(value_enum)]
    switch: Switch,

    /// Port to remove the address from.
    #[arg(value_enum)]
    port: Port,

    /// Address to remove.
    addr: oxnet::Ipv4Net,
}

#[async_trait]
impl RunnableCmd for CmdAddrDel {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let mut settings = current_port_settings(ctx, &self.rack, &self.switch, &self.port).await?;
        if let Some(addrs) = settings.addresses.get_mut(PHY0) {
            addrs
                .addresses
                .retain(|x| x.address.to_string() != self.addr.to_string());
        }
        ctx.client()?
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "config")]
pub struct CmdBgpConfig {
    #[clap(subcommand)]
    subcmd: BgpConfigSubCommand,
}

#[async_trait]
impl RunnableCmd for CmdBgpConfig {
    async fn run(&self, ctx: &Context) -> Result<()> {
        match &self.subcmd {
            BgpConfigSubCommand::Peer(cmd) => cmd.run(ctx).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum BgpConfigSubCommand {
    /// Manage BGP peer configuration.
    Peer(CmdBgpPeer),
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "peer")]
pub struct CmdBgpPeer {
    #[clap(subcommand)]
    subcmd: BgpConfigPeerSubCommand,
}

#[async_trait]
impl RunnableCmd for CmdBgpPeer {
    async fn run(&self, ctx: &Context) -> Result<()> {
        match &self.subcmd {
            BgpConfigPeerSubCommand::Add(cmd) => cmd.run(ctx).await,
            BgpConfigPeerSubCommand::Del(cmd) => cmd.run(ctx).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum BgpConfigPeerSubCommand {
    /// Add a BGP peer to a port configuration.
    Add(CmdBgpPeerAdd),

    /// Remove a BGP from a port configuration.
    Del(CmdBgpPeerDel),
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "add")]
pub struct CmdBgpPeerAdd {
    /// Id of the rack to add the peer to.
    rack: Uuid,

    /// Switch to add the peer to.
    #[arg(value_enum)]
    switch: Switch,

    /// Port to add the peer to.
    #[arg(value_enum)]
    port: Port,

    /// Address of the peer to add.
    addr: IpAddr,

    /// BGP configuration this peer is associated with.
    bgp_config: NameOrId,

    /// Prefixes that may be imported form the peer. Empty list means all prefixes
    /// allowed.
    #[clap(long)]
    allowed_imports: Vec<oxnet::IpNet>,

    /// Prefixes that may be exported to the peer. Empty list means all prefixes
    /// allowed.
    #[clap(long)]
    allowed_exports: Vec<oxnet::IpNet>,

    /// Include the provided communities in updates sent to the peer.
    #[clap(long)]
    communities: Vec<u32>,

    /// How long to to wait between TCP connection retries (seconds).
    #[clap(long, default_value_t = 0u32)]
    pub connect_retry: u32,

    /// How long to delay sending an open request after establishing a TCP
    /// session (seconds).
    #[clap(long, default_value_t = 0u32)]
    pub delay_open: u32,

    /// Enforce that the first AS in paths received from this peer is the
    /// peer's AS.
    #[clap(long, default_value_t = false)]
    pub enforce_first_as: bool,

    /// How long to hold peer connections between keepalives (seconds).
    #[clap(long, default_value_t = 6u32)]
    pub hold_time: u32,

    /// How often to send keepalive requests (seconds).
    #[clap(long, default_value_t = 2u32)]
    pub keepalive: u32,

    /// How long to hold a peer in idle before attempting a new session
    /// (seconds).
    #[clap(long, default_value_t = 0u32)]
    pub idle_hold_time: u32,

    /// Apply a local preference to routes received from this peer.
    #[clap(long)]
    pub local_pref: Option<u32>,

    /// Use the given key for TCP-MD5 authentication with the peer.
    #[clap(long)]
    pub md5_auth_key: Option<String>,

    /// Require messages from a peer have a minimum IP time to live field.
    #[clap(long)]
    pub min_ttl: Option<u8>,

    /// Apply the provided multi-exit discriminator (MED) updates sent to
    /// the peer.
    #[clap(long)]
    pub multi_exit_discriminator: Option<u32>,

    /// Require that a peer has a specified ASN.
    #[clap(long)]
    pub remote_asn: Option<u32>,

    /// Associate a VLAN ID with a peer.
    #[clap(long)]
    pub vlan_id: Option<u16>,
}

#[async_trait]
impl RunnableCmd for CmdBgpPeerAdd {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let mut settings = current_port_settings(ctx, &self.rack, &self.switch, &self.port).await?;
        let peer = BgpPeer {
            addr: self.addr,
            allowed_import: if self.allowed_imports.is_empty() {
                ImportExportPolicy::NoFiltering
            } else {
                ImportExportPolicy::Allow(
                    self.allowed_imports
                        .clone()
                        .into_iter()
                        .map(|x| x.to_string().parse().unwrap())
                        .collect(),
                )
            },
            allowed_export: if self.allowed_exports.is_empty() {
                ImportExportPolicy::NoFiltering
            } else {
                ImportExportPolicy::Allow(
                    self.allowed_exports
                        .clone()
                        .into_iter()
                        .map(|x| x.to_string().parse().unwrap())
                        .collect(),
                )
            },
            bgp_config: self.bgp_config.clone(),
            communities: self.communities.clone(),
            connect_retry: self.connect_retry,
            delay_open: self.delay_open,
            enforce_first_as: self.enforce_first_as,
            hold_time: self.hold_time,
            idle_hold_time: self.idle_hold_time,
            interface_name: PHY0.to_owned(),
            keepalive: self.keepalive,
            local_pref: self.local_pref,
            md5_auth_key: self.md5_auth_key.clone(),
            min_ttl: self.min_ttl,
            multi_exit_discriminator: self.multi_exit_discriminator,
            remote_asn: self.remote_asn,
            vlan_id: self.vlan_id,
        };
        match settings.bgp_peers.get_mut(PHY0) {
            Some(conf) => {
                conf.peers.push(peer);
            }
            None => {
                settings
                    .bgp_peers
                    .insert(String::from(PHY0), BgpPeerConfig { peers: vec![peer] });
            }
        }
        ctx.client()?
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "del")]
pub struct CmdBgpPeerDel {
    /// Id of the rack to remove the peer from.
    rack: Uuid,

    /// Switch to remove the peer from.
    #[arg(value_enum)]
    switch: Switch,

    /// Port to remove the peer from.
    #[arg(value_enum)]
    port: Port,

    /// Address of the peer to remove.
    addr: IpAddr,
}

#[async_trait]
impl RunnableCmd for CmdBgpPeerDel {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let mut settings = current_port_settings(ctx, &self.rack, &self.switch, &self.port).await?;
        if let Some(config) = settings.bgp_peers.get_mut(PHY0) {
            config.peers.retain(|x| x.addr != self.addr);
        }
        ctx.client()?
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

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

                let bgp_configs: HashMap<Uuid, Name> = c
                    .networking_bgp_config_list()
                    .limit(u32::MAX)
                    .send()
                    .await?
                    .into_inner()
                    .items
                    .into_iter()
                    .map(|x| (x.id, x.name))
                    .collect();

                println!(
                    "{}{}{}",
                    p.switch_location.to_string().blue(),
                    "/".dimmed(),
                    p.port_name.blue(),
                );

                println!(
                    "{}",
                    "=".repeat(p.port_name.len() + p.switch_location.to_string().len() + 1)
                        .dimmed()
                );

                writeln!(
                    &mut tw,
                    "{}\t{}\t{}",
                    "Autoneg".dimmed(),
                    "Fec".dimmed(),
                    "Speed".dimmed(),
                )?;

                for l in &config.links {
                    writeln!(&mut tw, "{:?}\t{:?}\t{:?}", l.autoneg, l.fec, l.speed,)?;
                }
                tw.flush()?;
                println!();

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

                    writeln!(&mut tw, "{}\t{}", addr, *alb.0.name)?;
                }
                tw.flush()?;
                println!();

                writeln!(
                    &mut tw,
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    "BGP Peer".dimmed(),
                    "Config".dimmed(),
                    "Export".dimmed(),
                    "Import".dimmed(),
                    "Communities".dimmed(),
                    "Connect Retry".dimmed(),
                    "Delay Open".dimmed(),
                    "Enforce First AS".dimmed(),
                    "Hold Time".dimmed(),
                    "Idle Hold Time".dimmed(),
                    "Keepalive".dimmed(),
                    "Local Pref".dimmed(),
                    "Md5 Auth".dimmed(),
                    "Min TTL".dimmed(),
                    "MED".dimmed(),
                    "Remote ASN".dimmed(),
                    "VLAN".dimmed(),
                )?;
                for p in &config.bgp_peers {
                    writeln!(
                        &mut tw,
                        "{}\t{}\t{:?}\t{:?}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
                        p.addr,
                        match &p.bgp_config {
                            NameOrId::Id(id) =>  bgp_configs[id].to_string(),
                            NameOrId::Name(name) => name.to_string(),
                        },
                        p.allowed_export,
                        p.allowed_import,
                        p.communities,
                        p.connect_retry,
                        p.delay_open,
                        p.enforce_first_as,
                        p.hold_time,
                        p.idle_hold_time,
                        p.keepalive,
                        p.local_pref,
                        p.md5_auth_key,
                        p.min_ttl,
                        p.multi_exit_discriminator,
                        p.remote_asn,
                        p.vlan_id,
                    )?;
                }
                tw.flush()?;
                println!();

                // Uncomment to see full payload
                //println!("");
                //println!("{:#?}", config);
                //println!("");
            }
        }

        Ok(())
    }
}

/// Get the status of BGP.
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
        self.show_switch(c, "switch0", &sw0).await?;

        println!("{}", "switch1".dimmed());
        println!("{}", "=======".dimmed());
        self.show_switch(c, "switch1", &sw1).await?;

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

            let monitors: Option<Monitors> = match status.as_ref() {
                Some(x) => match x.get("monitors") {
                    Some(x) => match serde_json::from_value(x.clone()) {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    },
                    None => None,
                },
                None => None,
            };

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

// NOTE: This bonanza of befuckerry is needed to translate the current
//       switch port settings view into a corresponding switch port
//       settings create request. It's the preliminary step for a read-
//       modify-write operation.
async fn create_current(settings_id: Uuid, ctx: &Context) -> Result<SwitchPortSettingsCreate> {
    let list = ctx
        .client()?
        .networking_switch_port_settings_list()
        .limit(u32::MAX)
        .send()
        .await
        .unwrap()
        .into_inner()
        .items;

    let name = list
        .iter()
        .find(|x| x.id == settings_id)
        .ok_or(anyhow::anyhow!("settings not found for {}", settings_id))?
        .name
        .clone();

    let current = ctx
        .client()?
        .networking_switch_port_settings_view()
        .port(settings_id)
        .send()
        .await
        .unwrap()
        .into_inner();

    let mut block_to_lot = HashMap::new();
    let lots = ctx
        .client()?
        .networking_address_lot_list()
        .limit(u32::MAX)
        .send()
        .await
        .unwrap()
        .into_inner()
        .items;
    for lot in lots.iter() {
        let lot_blocks = ctx
            .client()?
            .networking_address_lot_block_list()
            .address_lot(lot.id)
            .limit(u32::MAX)
            .send()
            .await
            .unwrap()
            .into_inner()
            .items;
        for block in lot_blocks.iter() {
            block_to_lot.insert(block.id, lot.id);
        }
    }

    let addrs: Vec<Address> = current
        .addresses
        .clone()
        .into_iter()
        .map(|x| Address {
            address: x.address,
            address_lot: NameOrId::Id(block_to_lot[&x.address_lot_block_id]),
            vlan_id: x.vlan_id,
        })
        .collect();

    let mut addresses = HashMap::new();
    addresses.insert(String::from(PHY0), AddressConfig { addresses: addrs });

    let mut bgp_peers = HashMap::new();
    bgp_peers.insert(
        String::from(PHY0),
        BgpPeerConfig {
            peers: current.bgp_peers,
        },
    );

    let groups: Vec<NameOrId> = current
        .groups
        .iter()
        .map(|x| NameOrId::Id(x.port_settings_group_id))
        .collect();

    let mut interfaces: HashMap<String, SwitchInterfaceConfigCreate> = current
        .interfaces
        .iter()
        .map(|x| {
            (
                x.interface_name.clone(),
                SwitchInterfaceConfigCreate {
                    kind: match x.kind {
                        SwitchInterfaceKind2::Primary => SwitchInterfaceKind::Primary,
                        SwitchInterfaceKind2::Loopback => SwitchInterfaceKind::Loopback,
                        SwitchInterfaceKind2::Vlan => {
                            todo!("vlan interface outside vlan interfaces?")
                        }
                    },
                    v6_enabled: x.v6_enabled,
                },
            )
        })
        .collect();

    for v in current.vlan_interfaces.iter() {
        interfaces.insert(
            format!("vlan-{}", v.vlan_id),
            SwitchInterfaceConfigCreate {
                kind: SwitchInterfaceKind::Vlan(v.vlan_id),
                v6_enabled: false,
            },
        );
    }

    let links: HashMap<String, LinkConfigCreate> = current
        .links
        .iter()
        .enumerate()
        .map(|(i, x)| {
            (
                format!("phy{}", i),
                LinkConfigCreate {
                    autoneg: x.autoneg,
                    fec: x.fec,
                    lldp: LldpServiceConfigCreate {
                        //TODO
                        enabled: false,
                        lldp_config: None,
                    },
                    mtu: x.mtu,
                    speed: x.speed,
                },
            )
        })
        .collect();

    let port_config = SwitchPortConfigCreate {
        geometry: match current.port.geometry {
            SwitchPortGeometry2::Qsfp28x1 => SwitchPortGeometry::Qsfp28x1,
            SwitchPortGeometry2::Qsfp28x2 => SwitchPortGeometry::Qsfp28x2,
            SwitchPortGeometry2::Sfp28x4 => SwitchPortGeometry::Sfp28x4,
        },
    };

    let route_config = RouteConfig {
        routes: current
            .routes
            .iter()
            .map(|x| Route {
                dst: x.dst.clone(),
                gw: x.gw.to_string().parse().unwrap(),
                vid: x.vlan_id,
            })
            .collect(),
    };

    let mut routes = HashMap::new();
    routes.insert(String::from(PHY0), route_config);

    let create = SwitchPortSettingsCreate {
        addresses,
        bgp_peers,
        description: String::from("switch port settings"),
        groups,
        interfaces,
        links,
        name: name.parse().unwrap(),
        port_config,
        routes,
    };

    Ok(create)
}
#[derive(clap::ValueEnum, Clone, Debug)]
enum Switch {
    Switch0,
    Switch1,
}

impl std::fmt::Display for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Port {
    Qsfp0,
    Qsfp1,
    Qsfp2,
    Qsfp3,
    Qsfp4,
    Qsfp5,
    Qsfp6,
    Qsfp7,
    Qsfp8,
    Qsfp9,
    Qsfp10,
    Qsfp11,
    Qsfp12,
    Qsfp13,
    Qsfp14,
    Qsfp15,
    Qsfp16,
    Qsfp17,
    Qsfp18,
    Qsfp19,
    Qsfp20,
    Qsfp21,
    Qsfp22,
    Qsfp23,
    Qsfp24,
    Qsfp25,
    Qsfp26,
    Qsfp27,
    Qsfp28,
    Qsfp29,
    Qsfp30,
    Qsfp31,
}

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

async fn get_port(
    ctx: &Context,
    rack_id: &Uuid,
    switch: &Switch,
    port: &Port,
) -> Result<SwitchPort> {
    let ports = ctx
        .client()?
        .networking_switch_port_list()
        .limit(u32::MAX)
        .send()
        .await
        .unwrap()
        .into_inner()
        .items;

    let port = ports
        .into_iter()
        .find(|x| {
            x.rack_id == *rack_id
                && x.switch_location == switch.to_string()
                && x.port_name == port.to_string()
        })
        .ok_or(anyhow::anyhow!(
            "port {} not found for rack {} switch {}",
            port,
            switch,
            rack_id
        ))?;

    Ok(port)
}

async fn get_port_settings_id(
    ctx: &Context,
    rack_id: &Uuid,
    switch: &Switch,
    port: &Port,
) -> Result<Uuid> {
    let port = get_port(ctx, rack_id, switch, port).await?;
    let id = port.port_settings_id.ok_or(anyhow::anyhow!(
        "Port settings uninitialized. Initialize by creating a link."
    ))?;
    Ok(id)
}

async fn current_port_settings(
    ctx: &Context,
    rack_id: &Uuid,
    switch: &Switch,
    port: &Port,
) -> Result<SwitchPortSettingsCreate> {
    let id = get_port_settings_id(ctx, rack_id, switch, port).await?;
    let settings = create_current(id, ctx).await?;
    Ok(settings)
}