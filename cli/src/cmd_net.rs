// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::{collections::HashMap, net::IpAddr};

use crate::{eprintln_nopipe, AuthenticatedCmd};
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use colored::*;
use futures::TryStreamExt;
use oxide::{
    types::{
        Address, AddressConfig, BgpAnnounceSetCreate, BgpAnnouncementCreate, BgpPeer,
        BgpPeerConfig, BgpPeerStatus, ImportExportPolicy, IpNet, LinkConfigCreate, LinkFec,
        LinkSpeed, LldpLinkConfigCreate, Name, NameOrId, Route, RouteConfig,
        SwitchInterfaceConfigCreate, SwitchInterfaceKind, SwitchInterfaceKind2, SwitchLocation,
        SwitchPort, SwitchPortConfigCreate, SwitchPortGeometry, SwitchPortGeometry2,
        SwitchPortSettingsCreate,
    },
    Client, ClientSystemHardwareExt, ClientSystemNetworkingExt,
};
use serde::{Deserialize, Serialize};
use std::io::Write;
use tabwriter::TabWriter;
use uuid::Uuid;

use crate::println_nopipe;

// We do not yet support port breakouts, but the API is phrased in terms of
// ports that can be broken out. The constant phy0 represents the first port
// in a breakout.
const PHY0: &str = "phy0";

/// Manage switch port links.
///
/// Links carry layer-2 Ethernet properties for a lane or set of lanes on a
/// switch port. Lane geometry is defined in physical port settings. At the
/// present time only single lane configurations are supported, and thus only
/// a single link per physical port is supported.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "link")]
pub struct CmdLink {
    #[clap(subcommand)]
    subcmd: LinkSubCommand,
}

#[async_trait]
impl AuthenticatedCmd for CmdLink {
    async fn run(&self, client: &Client) -> Result<()> {
        match &self.subcmd {
            LinkSubCommand::Add(cmd) => cmd.run(client).await,
            LinkSubCommand::Delete(cmd) => cmd.run(client).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum LinkSubCommand {
    /// Add a link to a port.
    Add(CmdLinkAdd),

    /// Remove a link from a port.
    Delete(CmdLinkDel),
}

/// Add a link to a switch port.
///
/// This operation performs a read-modify write on the port settings object
/// identified by the rack/switch/port parameters, adding a link to the
/// corresponding port settings.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "add")]
pub struct CmdLinkAdd {
    /// Id of the rack to add the link to.
    #[arg(long)]
    rack: Uuid,

    /// Switch to add the link to.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to add the link to.
    #[arg(long, value_enum)]
    port: Port,

    /// Whether or not to set auto-negotiation
    #[arg(long)]
    pub autoneg: bool,

    /// The forward error correction mode of the link.
    #[arg(long, value_enum)]
    pub fec: CliLinkFec,

    /// Maximum transmission unit for the link.
    #[arg(long, default_value_t = 1500u16)]
    pub mtu: u16,

    /// The speed of the link.
    #[arg(long, value_enum)]
    pub speed: CliLinkSpeed,
}

#[async_trait]
impl AuthenticatedCmd for CmdLinkAdd {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings = current_port_settings(client, &self.rack, &self.switch, &self.port)
            .await
            .unwrap_or(SwitchPortSettingsCreate {
                addresses: HashMap::default(),
                bgp_peers: HashMap::default(),
                description: String::default(),
                groups: Vec::default(),
                interfaces: HashMap::default(),
                links: HashMap::default(),
                name: format!("{}-{}", self.switch, self.port).parse().unwrap(),
                port_config: SwitchPortConfigCreate {
                    geometry: SwitchPortGeometry::Qsfp28x1,
                },
                routes: HashMap::default(),
            });
        let link = LinkConfigCreate {
            autoneg: self.autoneg,
            fec: self.fec.into(),
            mtu: self.mtu,
            speed: self.speed.into(),
            lldp: LldpLinkConfigCreate {
                enabled: false,
                link_name: None,
                link_description: None,
                chassis_id: None,
                system_name: None,
                system_description: None,
                management_ip: None,
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
        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Remove a link from a switch port.
///
/// This operation performs a read-modify write on the port settings object
/// identified by the rack/switch/port parameters, removing a link from the
/// corresponding port settings.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "delete")]
pub struct CmdLinkDel {
    /// Id of the rack to remove the link from.
    #[arg(long)]
    rack: Uuid,

    /// Switch to remove the link from.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to remove the link from.
    #[arg(long, value_enum)]
    port: Port,
}

#[async_trait]
impl AuthenticatedCmd for CmdLinkDel {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;
        settings.links.clear();
        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Manage and query rack Border Gateway Protocol (BGP) configuration and
/// status.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "bgp")]
pub struct CmdBgp {
    #[clap(subcommand)]
    subcmd: BgpSubCommand,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgp {
    async fn run(&self, client: &Client) -> Result<()> {
        match &self.subcmd {
            BgpSubCommand::Status(cmd) => cmd.run(client).await,
            BgpSubCommand::Config(cmd) => cmd.run(client).await,
            BgpSubCommand::Announce(cmd) => cmd.run(client).await,
            BgpSubCommand::Withdraw(cmd) => cmd.run(client).await,
            BgpSubCommand::Filter(cmd) => cmd.run(client).await,
            BgpSubCommand::Auth(cmd) => cmd.run(client).await,
            BgpSubCommand::LocalPref(cmd) => cmd.run(client).await,
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

    /// Make a BGP announcement.
    Announce(CmdBgpAnnounce),

    /// Make a BGP announcement.
    Withdraw(CmdBgpWithdraw),

    /// Set a filtering specification for a peer.
    Filter(CmdBgpFilter),

    /// Set an authentication string.
    Auth(CmdBgpAuth),

    /// Set a local preference for a peer.
    LocalPref(CmdBgpLocalPref),
}

/// Announce a prefix over BGP.
///
/// This command adds the provided prefix to the specified announce set. It is
/// required that the prefix be available in the given address lot. The add is
/// performed as a read-modify-write on the announce set.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "announce")]
pub struct CmdBgpAnnounce {
    /// The announce set to announce from.
    #[arg(long)]
    announce_set: Name,

    /// The address lot to draw from.
    #[arg(long)]
    address_lot: Name,

    /// The prefix to announce.
    #[arg(long)]
    prefix: oxnet::IpNet,

    #[arg(long, default_value = "")]
    description: String,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgpAnnounce {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut current: Vec<BgpAnnouncementCreate> = client
            .networking_bgp_announcement_list()
            .announce_set(NameOrId::Name(self.announce_set.clone()))
            .send()
            .await?
            .into_inner()
            .into_iter()
            .map(|x| BgpAnnouncementCreate {
                address_lot_block: NameOrId::Id(x.address_lot_block_id),
                network: x.network,
            })
            .collect();

        current.push(BgpAnnouncementCreate {
            address_lot_block: NameOrId::Name(self.address_lot.clone()),
            network: self.prefix.to_string().parse().unwrap(),
        });

        client
            .networking_bgp_announce_set_update()
            .body(BgpAnnounceSetCreate {
                announcement: current,
                name: self.announce_set.clone(),
                description: self.description.clone(),
            })
            .send()
            .await?;

        Ok(())
    }
}

/// Withdraw a prefix over BGP.
///
/// This command removes the provided prefix from the specified announce set.
/// The remove is performed as a read-modify-write on the announce set.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "withdraw")]
pub struct CmdBgpWithdraw {
    /// The announce set to withdraw from.
    #[arg(long)]
    announce_set: Name,

    /// The prefix to withdraw.
    #[arg(long)]
    prefix: oxnet::IpNet,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgpWithdraw {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut current: Vec<BgpAnnouncementCreate> = client
            .networking_bgp_announcement_list()
            .announce_set(NameOrId::Name(self.announce_set.clone()))
            .send()
            .await?
            .into_inner()
            .into_iter()
            .map(|x| BgpAnnouncementCreate {
                address_lot_block: NameOrId::Id(x.address_lot_block_id),
                network: x.network,
            })
            .collect();

        let before = current.len();
        current.retain(|x| x.network.to_string() != self.prefix.to_string());
        let after = current.len();
        if before == after {
            eprintln_nopipe!("no originated prefixes match the provided args");
            return Ok(());
        }

        client
            .networking_bgp_announce_set_update()
            .body(BgpAnnounceSetCreate {
                announcement: current,
                name: self.announce_set.clone(),
                description: self.announce_set.to_string(), //TODO?
            })
            .send()
            .await?;

        Ok(())
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum FilterDirection {
    Import,
    Export,
}

/// Add a filtering requirement to a BGP session.
///
/// The Oxide BGP implementation can filter prefixes received from peers
/// on import and filter prefixes sent to peers on export. This command
/// provides a way to specify import/export filtering. Filtering is a
/// property of the BGP peering settings found in the port settings configuration.
/// This command works by performing a read-modify-write on the port settings
/// configuration identified by the specified rack/switch/port.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "filter")]
pub struct CmdBgpFilter {
    /// Id of the rack to add the filter to.
    #[arg(long)]
    rack: Uuid,

    /// Switch to add the filter to.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to add the filter to.
    #[arg(long, value_enum)]
    port: Port,

    /// Peer to apply allow list to.
    #[arg(long)]
    peer: IpAddr,

    /// Whether to apply the filter to imported or exported prefixes.
    #[arg(long, value_enum)]
    direction: FilterDirection,

    /// Prefixes to allow for the peer.
    #[clap(long, conflicts_with = "no_filtering")]
    allowed: Vec<oxnet::IpNet>,

    /// Do not filter.
    #[clap(long, conflicts_with = "allowed")]
    no_filtering: bool,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgpFilter {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;
        match settings.bgp_peers.get_mut(PHY0) {
            None => return Err(anyhow::anyhow!("no BGP peers configured")),
            Some(config) => {
                let peer = config
                    .peers
                    .iter_mut()
                    .find(|x| x.addr == self.peer)
                    .ok_or(anyhow::anyhow!("specified peer does not exist"))?;

                let list: Vec<IpNet> = self
                    .allowed
                    .iter()
                    .map(|x| x.to_string().parse().unwrap())
                    .collect();
                match self.direction {
                    FilterDirection::Import => {
                        if self.no_filtering {
                            peer.allowed_import = ImportExportPolicy::NoFiltering;
                        } else {
                            peer.allowed_import = ImportExportPolicy::Allow(list);
                        }
                    }
                    FilterDirection::Export => {
                        if self.no_filtering {
                            peer.allowed_export = ImportExportPolicy::NoFiltering;
                        } else {
                            peer.allowed_export = ImportExportPolicy::Allow(list);
                        }
                    }
                }
            }
        }
        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Set an authentication string.
///
/// This command sets the authentication string that the specified BGP session
/// will use for establishing a TCP-MD5 authenticated connection with its peer.
/// This command works by performing a read-modify-write on the switch port
/// settings configuration identified by the specified rack/switch/port.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "auth")]
pub struct CmdBgpAuth {
    /// Id of the rack to add the auth config to.
    #[arg(long)]
    rack: Uuid,

    /// Switch to add the auth config to.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to add the auth config to.
    #[arg(long, value_enum)]
    port: Port,

    /// Peer to add the auth config to.
    #[arg(long)]
    peer: IpAddr,

    /// Use the given authorization string for TCP-MD5 authentication with the peer.
    #[clap(long)]
    authstring: Option<String>,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgpAuth {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;
        match settings.bgp_peers.get_mut(PHY0) {
            None => return Err(anyhow::anyhow!("no BGP peers configured")),
            Some(config) => {
                let peer = config
                    .peers
                    .iter_mut()
                    .find(|x| x.addr == self.peer)
                    .ok_or(anyhow::anyhow!("specified peer does not exist"))?;

                peer.md5_auth_key = self.authstring.clone();
            }
        }
        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Set a local preference for a peer.
///
/// This command associates a local preference for the specified peer. When
/// routes are imported by this peer, they will be installed into the routing
/// information base (RIB) with the specified preference. This command works
/// by performing a read-modify-write on the switch port settings configuration
/// identified by the specified rack/switch/port.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "pref")]
pub struct CmdBgpLocalPref {
    /// Id of the rack to set the route preference on.
    #[arg(long)]
    rack: Uuid,

    /// Switch to set the route preference on.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to set the route preference on.
    #[arg(long, value_enum)]
    port: Port,

    /// Peer to set the route preference on.
    #[arg(long)]
    peer: IpAddr,

    /// Apply this local preference to routes received from the peer.
    #[clap(long)]
    local_pref: Option<u32>,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgpLocalPref {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;
        match settings.bgp_peers.get_mut(PHY0) {
            None => return Err(anyhow::anyhow!("no BGP peers configured")),
            Some(config) => {
                let peer = config
                    .peers
                    .iter_mut()
                    .find(|x| x.addr == self.peer)
                    .ok_or(anyhow::anyhow!("specified peer does not exist"))?;

                peer.local_pref = self.local_pref
            }
        }
        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Manage static switch routes.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "route")]
pub struct CmdStaticRoute {
    #[clap(subcommand)]
    subcommand: RouteSubCommand,
}

#[async_trait]
impl AuthenticatedCmd for CmdStaticRoute {
    async fn run(&self, client: &Client) -> Result<()> {
        match &self.subcommand {
            RouteSubCommand::Set(cmd) => cmd.run(client).await,
            RouteSubCommand::Delete(cmd) => cmd.run(client).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum RouteSubCommand {
    /// Set a static route.
    Set(CmdStaticRouteSet),
    /// Delete a static route.
    Delete(CmdStaticRouteDelete),
}

/// Set a static route.
///
/// This command sets a static route. If a route with the specified destination,
/// nexthop, and vlan_id already exists the route will be updated. Otherwise
/// a new route will be created. This command works by performing a
/// read-modify-write on the switch port settings configuration identified by
/// the specified rack/switch/port.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "route set")]
pub struct CmdStaticRouteSet {
    /// Id of the rack to configure the route on.
    #[arg(long)]
    rack: Uuid,

    /// Switch to configure the route on.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to configure the route on.
    #[arg(long, value_enum)]
    port: Port,

    /// The route destination.
    #[clap(long)]
    destination: IpNet,

    /// The route nexthop
    #[clap(long)]
    nexthop: IpAddr,

    /// The route vlan id
    #[clap(long)]
    vlan_id: Option<u16>,

    /// RIB Priority indicating priority within and across protocols.
    #[clap(long)]
    rib_priority: Option<u8>,
}

#[async_trait]
impl AuthenticatedCmd for CmdStaticRouteSet {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;

        match settings.routes.get_mut(PHY0) {
            None => {
                settings.routes.insert(
                    PHY0.to_owned(),
                    RouteConfig {
                        routes: vec![Route {
                            dst: self.destination.clone(),
                            gw: self.nexthop,
                            rib_priority: self.rib_priority,
                            vid: self.vlan_id,
                        }],
                    },
                );
            }
            Some(config) => {
                let exists = config.routes.iter_mut().find(|x| {
                    x.dst.to_string() == self.destination.to_string()
                        && x.gw == self.nexthop
                        && x.vid == self.vlan_id
                });
                match exists {
                    Some(route) => {
                        *route = Route {
                            dst: self.destination.clone(),
                            gw: self.nexthop,
                            rib_priority: self.rib_priority,
                            vid: self.vlan_id,
                        };
                    }
                    None => {
                        config.routes.push(Route {
                            dst: self.destination.clone(),
                            gw: self.nexthop,
                            rib_priority: self.rib_priority,
                            vid: self.vlan_id,
                        });
                    }
                }
            }
        }

        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Delete a static route.
///
/// This command removes a static route from a port configuration. This
/// command works by performing a read-modify-write on the switch port settings
/// configuration identified by the specified rack/switch/port.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "route delete")]
pub struct CmdStaticRouteDelete {
    /// Id of the rack to remove the route from.
    #[arg(long)]
    rack: Uuid,

    /// Switch to remove the route from.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to remove the route from.
    #[arg(long, value_enum)]
    port: Port,

    /// The route destination.
    #[clap(long)]
    destination: IpNet,

    /// The route nexthop
    #[clap(long)]
    nexthop: IpAddr,

    /// The route vlan id
    #[clap(long)]
    vlan_id: Option<u16>,
}

#[async_trait]
impl AuthenticatedCmd for CmdStaticRouteDelete {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;

        match settings.routes.get_mut(PHY0) {
            None => {}
            Some(config) => {
                let before = config.routes.len();
                config.routes.retain(|x| {
                    !(x.dst.to_string() == self.destination.to_string()
                        && x.gw == self.nexthop
                        && x.vid == self.vlan_id)
                });
                let after = config.routes.len();
                if before == after {
                    eprintln_nopipe!("no routes match the provided args");
                    return Ok(());
                }
            }
        }

        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Manage switch port addresses.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "addr")]
pub struct CmdAddr {
    #[clap(subcommand)]
    subcmd: AddrSubCommand,
}

#[async_trait]
impl AuthenticatedCmd for CmdAddr {
    async fn run(&self, client: &Client) -> Result<()> {
        match &self.subcmd {
            AddrSubCommand::Add(cmd) => cmd.run(client).await,
            AddrSubCommand::Delete(cmd) => cmd.run(client).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum AddrSubCommand {
    /// Add an address to a port configuration.
    Add(CmdAddrAdd),

    /// Remove an address from a port configuration.
    Delete(CmdAddrDel),
}

/// Add an address to a switch port.
///
/// Addresses are a part of switch port settings configuration. This command
/// works by performing a read-modify-write on the switch port settings
/// configuration identified by the specified rack/switch/port.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "add")]
pub struct CmdAddrAdd {
    /// Id of the rack to add the address to.
    #[arg(long)]
    rack: Uuid,

    /// Switch to add the address to.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to add the address to.
    #[arg(long, value_enum)]
    port: Port,

    /// Address to add.
    #[arg(long)]
    addr: oxnet::Ipv4Net,

    /// Address lot to allocate from.
    #[arg(long)]
    lot: NameOrId,

    /// Optional VLAN to assign to the address.
    #[arg(long)]
    vlan: Option<u16>,
}

#[async_trait]
impl AuthenticatedCmd for CmdAddrAdd {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;
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
        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Remove an address from a switch port.
///
/// Addresses are a part of switch port settings configuration. This command
/// works by performing a read-modify-write on the switch port settings
/// configuration identified by the specified rack/switch/port.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "delete")]
pub struct CmdAddrDel {
    /// Id of the rack to remove the address from.
    #[arg(long)]
    rack: Uuid,

    /// Switch to remove the address from.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to remove the address from.
    #[arg(long, value_enum)]
    port: Port,

    /// Address to remove.
    #[arg(long)]
    addr: oxnet::Ipv4Net,
}

#[async_trait]
impl AuthenticatedCmd for CmdAddrDel {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;
        if let Some(addrs) = settings.addresses.get_mut(PHY0) {
            let before = addrs.addresses.len();
            addrs
                .addresses
                .retain(|x| x.address.to_string() != self.addr.to_string());
            let after = addrs.addresses.len();
            if before == after {
                eprintln_nopipe!("no addresses match the provided args");
                return Ok(());
            }
        }
        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Manage a rack's BGP configuration.
///
/// Rack BGP configuration is centered around peers. The subcommands available
/// within this command allow for managing BGP peer sessions.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "config")]
pub struct CmdBgpConfig {
    #[clap(subcommand)]
    subcmd: BgpConfigSubCommand,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgpConfig {
    async fn run(&self, client: &Client) -> Result<()> {
        match &self.subcmd {
            BgpConfigSubCommand::Peer(cmd) => cmd.run(client).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum BgpConfigSubCommand {
    /// Manage BGP peer configuration.
    Peer(CmdBgpPeer),
}

/// Manage BGP peers.
///
/// This command provides set and delete subcommands for managing BGP peers.
/// BGP peer configuration is a part of a switch port settings configuration.
/// The peer set and delete subcommands perform read-modify-write operations
/// on switch port settings objects to manage BGP peer configurations.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "peer")]
pub struct CmdBgpPeer {
    #[clap(subcommand)]
    subcmd: BgpConfigPeerSubCommand,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgpPeer {
    async fn run(&self, client: &Client) -> Result<()> {
        match &self.subcmd {
            BgpConfigPeerSubCommand::Set(cmd) => cmd.run(client).await,
            BgpConfigPeerSubCommand::Delete(cmd) => cmd.run(client).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum BgpConfigPeerSubCommand {
    /// Set a BGP peer on a port configuration.
    Set(CmdBgpPeerSet),

    /// Remove a BGP from a port configuration.
    Delete(CmdBgpPeerDel),
}

/// Set a BGP peer configuration.
///
/// This performs a read-modify-write on the specified switch port
/// configuration.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "set")]
pub struct CmdBgpPeerSet {
    /// Id of the rack to set the peer config on.
    #[arg(long)]
    rack: Uuid,

    /// Switch to set the peer config on.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to set the peer config on.
    #[arg(long, value_enum)]
    port: Port,

    /// Address of the peer to set the config for.
    #[arg(long)]
    addr: IpAddr,

    /// BGP configuration this peer is associated with.
    #[arg(long)]
    bgp_config: NameOrId,

    /// Prefixes that may be imported from the peer. Empty list means all prefixes
    /// allowed.
    #[arg(long)]
    allowed_imports: Vec<oxnet::IpNet>,

    /// Prefixes that may be exported to the peer. Empty list means all prefixes
    /// allowed.
    #[arg(long)]
    allowed_exports: Vec<oxnet::IpNet>,

    /// Include the provided communities in updates sent to the peer.
    #[arg(long)]
    communities: Vec<u32>,

    /// How long to wait between TCP connection retries (seconds).
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

    /// Apply this local preference to routes received from the peer.
    #[arg(long)]
    pub local_pref: Option<u32>,

    /// Use the given authorization string for TCP-MD5 authentication with the peer.
    #[arg(long)]
    pub authstring: Option<String>,

    /// Require messages from a peer have a minimum IP time to live field.
    #[arg(long)]
    pub min_ttl: Option<u8>,

    /// Apply the provided multi-exit discriminator (MED) updates sent to
    /// the peer.
    #[arg(long)]
    pub multi_exit_discriminator: Option<u32>,

    /// Require that a peer has a specified ASN.
    #[arg(long)]
    pub remote_asn: Option<u32>,

    /// Associate a VLAN ID with a peer.
    #[arg(long)]
    pub vlan_id: Option<u16>,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgpPeerSet {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;
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
            md5_auth_key: self.authstring.clone(),
            min_ttl: self.min_ttl,
            multi_exit_discriminator: self.multi_exit_discriminator,
            remote_asn: self.remote_asn,
            vlan_id: self.vlan_id,
        };
        match settings.bgp_peers.get_mut(PHY0) {
            Some(conf) => match conf.peers.iter_mut().find(|x| x.addr == peer.addr) {
                Some(p) => *p = peer,
                None => conf.peers.push(peer),
            },
            None => {
                settings
                    .bgp_peers
                    .insert(String::from(PHY0), BgpPeerConfig { peers: vec![peer] });
            }
        }
        client
            .networking_switch_port_settings_create()
            .body(settings)
            .send()
            .await?;
        Ok(())
    }
}

/// Remove a BGP peer from a given switch port configuration.
///
/// This performs a read-modify-write on the specified switch port
/// configuration.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "delete")]
pub struct CmdBgpPeerDel {
    /// Id of the rack to remove the peer from.
    #[arg(long)]
    rack: Uuid,

    /// Switch to remove the peer from.
    #[arg(long, value_enum)]
    switch: Switch,

    /// Port to remove the peer from.
    #[arg(long, value_enum)]
    port: Port,

    /// Address of the peer to remove.
    #[arg(long)]
    addr: IpAddr,
}

#[async_trait]
impl AuthenticatedCmd for CmdBgpPeerDel {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut settings =
            current_port_settings(client, &self.rack, &self.switch, &self.port).await?;
        if let Some(config) = settings.bgp_peers.get_mut(PHY0) {
            let before = config.peers.len();
            config.peers.retain(|x| x.addr != self.addr);
            let after = config.peers.len();
            if before == after {
                eprintln_nopipe!("no peers match the provided address");
                return Ok(());
            }
        }
        client
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
impl AuthenticatedCmd for CmdPortConfig {
    async fn run(&self, client: &Client) -> Result<()> {
        let ports = client
            .networking_switch_port_list()
            .stream()
            .try_collect::<Vec<_>>()
            .await?;

        let mut tw = TabWriter::new(std::io::stdout()).ansi(true);

        // TODO bad API, having to pull all address lots to work backwards from
        // address reference to address lot block is terribad
        let addr_lots = client
            .networking_address_lot_list()
            .stream()
            .try_collect::<Vec<_>>()
            .await?;

        let mut addr_lot_blocks = Vec::new();
        for a in addr_lots.iter() {
            let blocks = client
                .networking_address_lot_block_list()
                .address_lot(a.id)
                .stream()
                .try_collect::<Vec<_>>()
                .await?;

            for b in blocks.iter() {
                addr_lot_blocks.push((a.clone(), b.clone()));
            }
        }

        for p in &ports {
            if let Some(id) = p.port_settings_id {
                let config = client
                    .networking_switch_port_settings_view()
                    .port(id)
                    .send()
                    .await?
                    .into_inner();

                let bgp_configs: HashMap<Uuid, Name> = client
                    .networking_bgp_config_list()
                    .stream()
                    .try_collect::<Vec<_>>()
                    .await?
                    .into_iter()
                    .map(|x| (x.id, x.name))
                    .collect();

                println_nopipe!(
                    "{}{}{}",
                    p.switch_location.to_string().blue(),
                    "/".dimmed(),
                    p.port_name.blue(),
                );

                println_nopipe!(
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
                println_nopipe!();

                writeln!(
                    &mut tw,
                    "{}\t{}\t{}",
                    "Address".dimmed(),
                    "Lot".dimmed(),
                    "VLAN".dimmed()
                )?;
                for a in &config.addresses {
                    let addr = match &a.address {
                        oxide::types::IpNet::V4(a) => a.to_string(),
                        oxide::types::IpNet::V6(a) => a.to_string(),
                    };

                    let alb = addr_lot_blocks
                        .iter()
                        .find(|x| x.1.id == a.address_lot_block_id)
                        .unwrap();

                    writeln!(&mut tw, "{}\t{}\t{:?}", addr, *alb.0.name, a.vlan_id)?;
                }
                tw.flush()?;
                println_nopipe!();

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
                        "{}\t{}\t[{}]\t[{}]\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
                        p.addr,
                        match &p.bgp_config {
                            NameOrId::Id(id) =>  bgp_configs[id].to_string(),
                            NameOrId::Name(name) => name.to_string(),
                        },
                        match &p.allowed_export {
                            ImportExportPolicy::NoFiltering => String::from("no filtering"),
                            ImportExportPolicy::Allow(list) => list
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(" "),
                        },
                        match &p.allowed_import {
                            ImportExportPolicy::NoFiltering => String::from("no filtering"),
                            ImportExportPolicy::Allow(list) => list
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(" "),
                        },
                        p.communities,
                        p.connect_retry,
                        p.delay_open,
                        p.enforce_first_as,
                        p.hold_time,
                        p.idle_hold_time,
                        p.keepalive,
                        p.local_pref,
                        p.md5_auth_key.as_ref().map(|x| format!("{:x}", md5::compute(x))),
                        p.min_ttl,
                        p.multi_exit_discriminator,
                        p.remote_asn,
                        p.vlan_id,
                    )?;
                }
                tw.flush()?;
                println_nopipe!();

                writeln!(
                    &mut tw,
                    "{}\t{}\t{}\t{}",
                    "Destination".dimmed(),
                    "Nexthop".dimmed(),
                    "Vlan".dimmed(),
                    "Preference".dimmed(),
                )?;

                for r in &config.routes {
                    writeln!(
                        &mut tw,
                        "{}\t{}\t{}\t{}",
                        r.dst,
                        r.gw,
                        r.vlan_id.unwrap_or(0),
                        r.rib_priority.unwrap_or(0),
                    )?;
                }
                tw.flush()?;
                println_nopipe!();

                // Uncomment to see full payload
                //println!("");
                //println!("{:#?}", config);
                //println!("");
            }
        }

        Ok(())
    }
}

/// Get the status of BGP on the rack.
///
/// This will show the peering status for all peers on all switches.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "net bgp status")]
pub struct CmdBgpStatus {}

#[async_trait]
impl AuthenticatedCmd for CmdBgpStatus {
    async fn run(&self, client: &Client) -> Result<()> {
        let status = client.networking_bgp_status().send().await?.into_inner();

        let (sw0, sw1) = status
            .iter()
            .partition(|x| x.switch == SwitchLocation::Switch0);

        println_nopipe!("{}", "switch0".dimmed());
        println_nopipe!("{}", "=======".dimmed());
        show_status(&sw0)?;
        println_nopipe!();

        println_nopipe!("{}", "switch1".dimmed());
        println_nopipe!("{}", "=======".dimmed());
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
impl AuthenticatedCmd for CmdPortStatus {
    async fn run(&self, client: &Client) -> Result<()> {
        let ports = client
            .networking_switch_port_list()
            .stream()
            .try_collect::<Vec<_>>()
            .await?;

        let (mut sw0, mut sw1): (Vec<&SwitchPort>, Vec<&SwitchPort>) = ports
            .iter()
            .partition(|x| x.switch_location.as_str() == "switch0");

        sw0.sort_by_key(|x| x.port_name.as_str());
        sw1.sort_by_key(|x| x.port_name.as_str());

        println_nopipe!("{}", "switch0".dimmed());
        println_nopipe!("{}", "=======".dimmed());
        self.show_switch(client, "switch0", &sw0).await?;

        println_nopipe!("{}", "switch1".dimmed());
        println_nopipe!("{}", "=======".dimmed());
        self.show_switch(client, "switch1", &sw1).await?;

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

            let monitors = status
                .as_ref()
                .and_then(|value| value.get("monitors"))
                .and_then(|value| Monitors::deserialize(value).ok());

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
        println_nopipe!();
        mtw.flush()?;
        println_nopipe!();

        Ok(())
    }
}

// NOTE: This bonanza of befuckerry is needed to translate the current
//       switch port settings view into a corresponding switch port
//       settings create request. It's the preliminary step for a read-
//       modify-write operation.
async fn create_current(settings_id: Uuid, client: &Client) -> Result<SwitchPortSettingsCreate> {
    let list = client
        .networking_switch_port_settings_list()
        .stream()
        .try_collect::<Vec<_>>()
        .await?;

    let name = list
        .iter()
        .find(|x| x.id == settings_id)
        .ok_or(anyhow::anyhow!("settings not found for {}", settings_id))?
        .name
        .clone();

    let current = client
        .networking_switch_port_settings_view()
        .port(settings_id)
        .send()
        .await
        .unwrap()
        .into_inner();

    let mut block_to_lot = HashMap::new();
    let lots = client
        .networking_address_lot_list()
        .stream()
        .try_collect::<Vec<_>>()
        .await?;

    for lot in lots.iter() {
        let lot_blocks = client
            .networking_address_lot_block_list()
            .address_lot(lot.id)
            .stream()
            .try_collect::<Vec<_>>()
            .await?;
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
                    mtu: x.mtu,
                    speed: x.speed,
                    lldp: LldpLinkConfigCreate {
                        enabled: false,
                        link_name: None,
                        link_description: None,
                        chassis_id: None,
                        system_name: None,
                        system_description: None,
                        management_ip: None,
                    },
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
            .map(|x| {
                let gw: oxnet::IpNet = x.gw.to_string().parse().unwrap();
                Route {
                    dst: x.dst.clone(),
                    gw: gw.addr(),
                    vid: x.vlan_id,
                    rib_priority: x.rib_priority,
                }
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
    client: &Client,
    rack_id: &Uuid,
    switch: &Switch,
    port: &Port,
) -> Result<SwitchPort> {
    let ports = client
        .networking_switch_port_list()
        .stream()
        .try_collect::<Vec<_>>()
        .await?;

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
    client: &Client,
    rack_id: &Uuid,
    switch: &Switch,
    port: &Port,
) -> Result<Uuid> {
    let port = get_port(client, rack_id, switch, port).await?;
    let id = port.port_settings_id.ok_or(anyhow::anyhow!(
        "Port settings uninitialized. Initialize by creating a link."
    ))?;
    Ok(id)
}

async fn current_port_settings(
    client: &Client,
    rack_id: &Uuid,
    switch: &Switch,
    port: &Port,
) -> Result<SwitchPortSettingsCreate> {
    let id = get_port_settings_id(client, rack_id, switch, port).await?;
    let settings = create_current(id, client).await?;
    Ok(settings)
}

/// Link forward error correction setting.
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum CliLinkFec {
    /// Firecode forward error correction.
    Firecode,
    /// No forward error correction.
    None,
    /// Reed-Solomon forward error correction.
    Rs,
}

impl From<CliLinkFec> for LinkFec {
    fn from(value: CliLinkFec) -> Self {
        match value {
            CliLinkFec::Firecode => LinkFec::Firecode,
            CliLinkFec::None => LinkFec::None,
            CliLinkFec::Rs => LinkFec::Rs,
        }
    }
}

/// Link speed.
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum CliLinkSpeed {
    /// Zero gigabits per second.
    #[clap(name = "0g")]
    Speed0G,
    /// 1 gigabit per second.
    #[clap(name = "1g")]
    Speed1G,
    /// 10 gigabits per second.
    #[clap(name = "10g")]
    Speed10G,
    /// 25 gigabits per second.
    #[clap(name = "25g")]
    Speed25G,
    /// 40 gigabits per second.
    #[clap(name = "40g")]
    Speed40G,
    /// 50 gigabits per second.
    #[clap(name = "50g")]
    Speed50G,
    /// 100 gigabits per second.
    #[clap(name = "100g")]
    Speed100G,
    /// 200 gigabits per second.
    #[clap(name = "200g")]
    Speed200G,
    /// 400 gigabits per second.
    #[clap(name = "400g")]
    Speed400G,
}

impl From<CliLinkSpeed> for LinkSpeed {
    fn from(value: CliLinkSpeed) -> Self {
        match value {
            CliLinkSpeed::Speed0G => LinkSpeed::Speed0G,
            CliLinkSpeed::Speed1G => LinkSpeed::Speed1G,
            CliLinkSpeed::Speed10G => LinkSpeed::Speed10G,
            CliLinkSpeed::Speed25G => LinkSpeed::Speed25G,
            CliLinkSpeed::Speed40G => LinkSpeed::Speed40G,
            CliLinkSpeed::Speed50G => LinkSpeed::Speed50G,
            CliLinkSpeed::Speed100G => LinkSpeed::Speed100G,
            CliLinkSpeed::Speed200G => LinkSpeed::Speed200G,
            CliLinkSpeed::Speed400G => LinkSpeed::Speed400G,
        }
    }
}
