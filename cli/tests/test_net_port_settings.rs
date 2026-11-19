// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

use httpmock::MockServer;
use oxide::types::{
    Address, AddressConfig, AddressLot, AddressLotBlock, AddressLotBlockResultsPage,
    AddressLotResultsPage, BgpPeer, BgpPeerConfig, ImportExportPolicy, LinkConfigCreate, LinkFec,
    LinkSpeed, LldpLinkConfig, LldpLinkConfigCreate, Name, NameOrId, Route, RouteConfig,
    RouterPeerType, SwitchInterfaceConfig, SwitchInterfaceConfigCreate, SwitchPort,
    SwitchPortAddressView, SwitchPortConfig, SwitchPortConfigCreate, SwitchPortGeometry,
    SwitchPortLinkConfig, SwitchPortResultsPage, SwitchPortRouteConfig, SwitchPortSettings,
    SwitchPortSettingsCreate, SwitchPortSettingsGroups, SwitchPortSettingsIdentity,
    SwitchPortSettingsIdentityResultsPage, SwitchSlot, TxEqConfig,
};
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use test_common::JsonMock;
use uuid::Uuid;

const RACK: &str = "c19a698f-c6f9-4a17-ae30-20d711b8f7dc";
const PORT: &str = "qsfp0";
const PEER: &str = "192.0.2.1";
const OTHER_PEER: &str = "192.0.2.2";

fn phy0() -> Name {
    "phy0".parse().unwrap()
}

fn lldp_disabled() -> LldpLinkConfigCreate {
    LldpLinkConfigCreate {
        chassis_id: None,
        enabled: false,
        link_description: None,
        link_name: None,
        management_ip: None,
        system_description: None,
        system_name: None,
    }
}

/// A numbered BGP peer that filters exports and does not filter imports.
fn peer(src: &mut rand::rngs::SmallRng, target_addr: &str) -> BgpPeer {
    BgpPeer {
        addr: RouterPeerType::Numbered {
            src_addr: None,
            target_addr: target_addr.parse().unwrap(),
        },
        allowed_export: ImportExportPolicy::Allow(vec!["198.51.100.0/24".parse().unwrap()]),
        allowed_import: ImportExportPolicy::NoFiltering,
        bgp_config: JsonMock::mock_value(src).unwrap(),
        communities: JsonMock::mock_value(src).unwrap(),
        connect_retry: JsonMock::mock_value(src).unwrap(),
        delay_open: JsonMock::mock_value(src).unwrap(),
        enforce_first_as: JsonMock::mock_value(src).unwrap(),
        hold_time: JsonMock::mock_value(src).unwrap(),
        idle_hold_time: JsonMock::mock_value(src).unwrap(),
        keepalive: JsonMock::mock_value(src).unwrap(),
        local_pref: None,
        md5_auth_key: None,
        min_ttl: JsonMock::mock_value(src).unwrap(),
        multi_exit_discriminator: JsonMock::mock_value(src).unwrap(),
        remote_asn: JsonMock::mock_value(src).unwrap(),
        vlan_id: JsonMock::mock_value(src).unwrap(),
    }
}

/// A mock server whose port `PORT` on switch0 of rack `RACK` has settings
/// applied.
///
/// The settings have one link, two addresses, two routes, and two BGP peers,
/// all on phy0. `JsonMock` does not generate prefixes, so the addresses,
/// routes, and peers are written out. A test changes `settings` before
/// calling `run`.
struct TestPort {
    server: MockServer,
    lot_id: Uuid,
    settings: SwitchPortSettings,
}

impl TestPort {
    fn new() -> Self {
        let mut src = rand::rngs::SmallRng::seed_from_u64(42);
        let server = MockServer::start();
        let rack_id: Uuid = RACK.parse().unwrap();
        let settings_id = Uuid::mock_value(&mut src).unwrap();

        let lot = AddressLot::mock_value(&mut src).unwrap();
        let block = AddressLotBlock::mock_value(&mut src).unwrap();

        let addresses = [("198.51.100.2/30", None), ("2001:db8::2/64", Some(300))]
            .into_iter()
            .map(|(address, vlan_id)| SwitchPortAddressView {
                address: address.parse().unwrap(),
                address_lot_block_id: block.id,
                address_lot_id: lot.id,
                address_lot_name: lot.name.clone(),
                interface_name: phy0(),
                port_settings_id: settings_id,
                vlan_id,
            })
            .collect();

        // The routes differ in VLAN id and RIB priority.
        let routes = [(None, None), (Some(300), Some(5))]
            .into_iter()
            .map(|(vlan_id, rib_priority)| SwitchPortRouteConfig {
                dst: "192.0.2.0/24".parse().unwrap(),
                gw: "198.51.100.1".parse().unwrap(),
                interface_name: phy0(),
                port_settings_id: settings_id,
                rib_priority,
                vlan_id,
            })
            .collect();

        let identity = SwitchPortSettingsIdentity {
            id: settings_id,
            ..SwitchPortSettingsIdentity::mock_value(&mut src).unwrap()
        };
        let settings = SwitchPortSettings {
            addresses,
            bgp_peers: vec![peer(&mut src, PEER), peer(&mut src, OTHER_PEER)],
            description: identity.description.clone(),
            groups: Vec::<SwitchPortSettingsGroups>::mock_value(&mut src)
                .unwrap()
                .into_iter()
                .map(|group| SwitchPortSettingsGroups {
                    port_settings_id: settings_id,
                    ..group
                })
                .collect(),
            id: settings_id,
            interfaces: vec![SwitchInterfaceConfig {
                interface_name: phy0(),
                port_settings_id: settings_id,
                ..SwitchInterfaceConfig::mock_value(&mut src).unwrap()
            }],
            links: vec![SwitchPortLinkConfig {
                link_name: phy0(),
                lldp_link_config: None,
                port_settings_id: settings_id,
                tx_eq_config: None,
                ..SwitchPortLinkConfig::mock_value(&mut src).unwrap()
            }],
            name: identity.name.clone(),
            port: SwitchPortConfig {
                geometry: SwitchPortGeometry::Qsfp28x1,
                port_settings_id: settings_id,
            },
            routes,
            time_created: identity.time_created,
            time_modified: identity.time_modified,
        };

        // The same port name on the other switch and on another rack has
        // other settings.
        let mut port = |rack_id, switch_slot, port_settings_id| SwitchPort {
            port_name: PORT.parse().unwrap(),
            port_settings_id,
            rack_id,
            switch_slot,
            ..SwitchPort::mock_value(&mut src).unwrap()
        };
        let ports = SwitchPortResultsPage {
            items: vec![
                port(rack_id, SwitchSlot::Switch1, Some(Uuid::new_v4())),
                port(Uuid::new_v4(), SwitchSlot::Switch0, Some(Uuid::new_v4())),
                port(rack_id, SwitchSlot::Switch0, Some(settings_id)),
            ],
            next_page: None,
        };
        server.networking_switch_port_list(|when, then| {
            when.into_inner().any_request();
            then.ok(&ports);
        });

        let mut identities = Vec::<SwitchPortSettingsIdentity>::mock_value(&mut src).unwrap();
        identities.push(identity);
        server.networking_switch_port_settings_list(|when, then| {
            when.into_inner().any_request();
            then.ok(&SwitchPortSettingsIdentityResultsPage {
                items: identities,
                next_page: None,
            });
        });

        server.networking_address_lot_list(|when, then| {
            when.into_inner().any_request();
            then.ok(&AddressLotResultsPage {
                items: vec![lot.clone()],
                next_page: None,
            });
        });
        server.networking_address_lot_block_list(|when, then| {
            when.address_lot(&NameOrId::Id(lot.id));
            then.ok(&AddressLotBlockResultsPage {
                items: vec![block],
                next_page: None,
            });
        });

        Self {
            server,
            lot_id: lot.id,
            settings,
        }
    }

    /// The create request that reproduces `settings`.
    fn current(&self) -> SwitchPortSettingsCreate {
        let s = &self.settings;
        SwitchPortSettingsCreate {
            addresses: vec![AddressConfig {
                link_name: phy0(),
                addresses: s
                    .addresses
                    .iter()
                    .map(|a| Address {
                        address: a.address.clone(),
                        address_lot: NameOrId::Id(self.lot_id),
                        vlan_id: a.vlan_id,
                    })
                    .collect(),
            }],
            bgp_peers: vec![BgpPeerConfig {
                link_name: phy0(),
                peers: s.bgp_peers.clone(),
            }],
            description: s.description.clone(),
            groups: s
                .groups
                .iter()
                .map(|g| NameOrId::Id(g.port_settings_group_id))
                .collect(),
            interfaces: s
                .interfaces
                .iter()
                .map(|i| SwitchInterfaceConfigCreate {
                    kind: i.kind.clone(),
                    link_name: i.interface_name.clone(),
                    v6_enabled: i.v6_enabled,
                })
                .collect(),
            links: s
                .links
                .iter()
                .map(|l| LinkConfigCreate {
                    autoneg: l.autoneg,
                    fec: l.fec,
                    link_name: l.link_name.clone(),
                    lldp: l
                        .lldp_link_config
                        .as_ref()
                        .map(|c| LldpLinkConfigCreate {
                            chassis_id: c.chassis_id.clone(),
                            enabled: c.enabled,
                            link_description: c.link_description.clone(),
                            link_name: c.link_name.clone(),
                            management_ip: c.management_ip,
                            system_description: c.system_description.clone(),
                            system_name: c.system_name.clone(),
                        })
                        .unwrap_or_else(lldp_disabled),
                    mtu: l.mtu,
                    speed: l.speed,
                    tx_eq: l.tx_eq_config.clone(),
                })
                .collect(),
            name: s.name.clone(),
            port_config: SwitchPortConfigCreate {
                geometry: s.port.geometry,
            },
            routes: vec![RouteConfig {
                link_name: phy0(),
                routes: s
                    .routes
                    .iter()
                    .map(|r| Route {
                        dst: r.dst.clone(),
                        gw: r.gw,
                        rib_priority: r.rib_priority,
                        vid: r.vlan_id,
                    })
                    .collect(),
            }],
        }
    }

    /// Runs `oxide system networking <args>` against this port and checks
    /// that it reads `settings` and creates `expected`.
    fn run(&self, args: &[&str], expected: &SwitchPortSettingsCreate) {
        let view = self
            .server
            .networking_switch_port_settings_view(|when, then| {
                when.port(&NameOrId::Id(self.settings.id));
                then.ok(&self.settings);
            });
        let create = self
            .server
            .networking_switch_port_settings_create(|when, then| {
                when.body(expected);
                then.created(&self.settings);
            });

        assert_cmd::cargo::cargo_bin_cmd!("oxide")
            .env("RUST_BACKTRACE", "1")
            .env("OXIDE_HOST", self.server.url(""))
            .env("OXIDE_TOKEN", "fake-token")
            .args(["system", "networking"])
            .args(args)
            .args(["--rack", RACK, "--switch", "switch0", "--port", PORT])
            .assert()
            .success();

        view.assert();
        create.assert();
    }
}

// `link add` adds a phy0 link with the given properties to settings that
// have no link.
#[test]
fn test_net_link_add() {
    let mut port = TestPort::new();
    port.settings.links.clear();

    let mut expected = port.current();
    expected.links.push(LinkConfigCreate {
        autoneg: true,
        fec: Some(LinkFec::Rs),
        link_name: phy0(),
        lldp: lldp_disabled(),
        mtu: 9000,
        speed: LinkSpeed::Speed100G,
        tx_eq: None,
    });

    port.run(
        &[
            "link",
            "add",
            "--autoneg",
            "--fec",
            "rs",
            "--mtu",
            "9000",
            "--speed",
            "100g",
        ],
        &expected,
    );
}

// `link delete` removes the link and nothing else.
#[test]
fn test_net_link_delete() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.links.clear();

    port.run(&["link", "delete"], &expected);
}

// `addr add` appends the address, with its lot and VLAN, to the phy0
// addresses.
#[test]
fn test_net_addr_add() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.addresses[0].addresses.push(Address {
        address: "203.0.113.5/24".parse().unwrap(),
        address_lot: NameOrId::Name("uplink-lot".parse().unwrap()),
        vlan_id: Some(400),
    });

    port.run(
        &[
            "addr",
            "add",
            "--addr",
            "203.0.113.5/24",
            "--lot",
            "uplink-lot",
            "--vlan",
            "400",
        ],
        &expected,
    );
}

// `addr delete` removes an IPv4 address.
#[test]
fn test_net_addr_delete() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.addresses[0].addresses.remove(0);

    port.run(&["addr", "delete", "--addr", "198.51.100.2/30"], &expected);
}

// `route set` appends a route that does not match an existing one.
#[test]
fn test_net_route_set_new() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.routes[0].routes.push(Route {
        dst: "203.0.113.0/24".parse().unwrap(),
        gw: "198.51.100.1".parse().unwrap(),
        rib_priority: Some(10),
        vid: Some(200),
    });

    port.run(
        &[
            "route",
            "set",
            "--destination",
            "203.0.113.0/24",
            "--nexthop",
            "198.51.100.1",
            "--vlan-id",
            "200",
            "--rib-priority",
            "10",
        ],
        &expected,
    );
}

// `route set` updates the route with the same destination, nexthop, and VLAN
// id in place.
#[test]
fn test_net_route_set_existing() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.routes[0].routes[1].rib_priority = Some(50);

    port.run(
        &[
            "route",
            "set",
            "--destination",
            "192.0.2.0/24",
            "--nexthop",
            "198.51.100.1",
            "--vlan-id",
            "300",
            "--rib-priority",
            "50",
        ],
        &expected,
    );
}

// `route delete` removes only the route whose destination, nexthop, and VLAN
// id all match.
#[test]
fn test_net_route_delete() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.routes[0].routes.remove(1);

    port.run(
        &[
            "route",
            "delete",
            "--destination",
            "192.0.2.0/24",
            "--nexthop",
            "198.51.100.1",
            "--vlan-id",
            "300",
        ],
        &expected,
    );
}

// A read-modify-write keeps the LLDP and `tx_eq` settings of the link.
#[test]
fn test_net_route_set_preserves_link() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(43);
    let mut port = TestPort::new();
    let link = &mut port.settings.links[0];
    link.lldp_link_config = Some(LldpLinkConfig {
        enabled: true,
        ..LldpLinkConfig::mock_value(&mut src).unwrap()
    });
    link.tx_eq_config = Some(TxEqConfig::mock_value(&mut src).unwrap());

    let mut expected = port.current();
    expected.routes[0].routes.push(Route {
        dst: "203.0.113.0/24".parse().unwrap(),
        gw: "198.51.100.1".parse().unwrap(),
        rib_priority: None,
        vid: None,
    });

    port.run(
        &[
            "route",
            "set",
            "--destination",
            "203.0.113.0/24",
            "--nexthop",
            "198.51.100.1",
        ],
        &expected,
    );
}

// `bgp peer set` appends a peer built from the arguments.
#[test]
fn test_net_bgp_peer_set_new() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.bgp_peers[0].peers.push(BgpPeer {
        addr: RouterPeerType::Numbered {
            src_addr: Some("169.254.20.2".parse().unwrap()),
            target_addr: "169.254.20.1".parse().unwrap(),
        },
        allowed_export: ImportExportPolicy::Allow(vec!["198.51.100.0/24".parse().unwrap()]),
        allowed_import: ImportExportPolicy::Allow(vec![
            "203.0.113.0/24".parse().unwrap(),
            "192.0.2.0/24".parse().unwrap(),
        ]),
        bgp_config: NameOrId::Name("as65547".parse().unwrap()),
        communities: vec![100, 200],
        connect_retry: 3,
        delay_open: 4,
        enforce_first_as: true,
        hold_time: 9,
        idle_hold_time: 5,
        keepalive: 3,
        local_pref: Some(70),
        md5_auth_key: Some("secret".to_string()),
        min_ttl: Some(255),
        multi_exit_discriminator: Some(20),
        remote_asn: Some(65548),
        vlan_id: Some(400),
    });

    port.run(
        &[
            "bgp",
            "peer",
            "set",
            "--addr",
            "169.254.20.1",
            "--src-addr",
            "169.254.20.2",
            "--bgp-config",
            "as65547",
            "--allowed-imports",
            "203.0.113.0/24",
            "--allowed-imports",
            "192.0.2.0/24",
            "--allowed-exports",
            "198.51.100.0/24",
            "--communities",
            "100",
            "--communities",
            "200",
            "--connect-retry",
            "3",
            "--delay-open",
            "4",
            "--enforce-first-as",
            "--hold-time",
            "9",
            "--keepalive",
            "3",
            "--idle-hold-time",
            "5",
            "--local-pref",
            "70",
            "--authstring",
            "secret",
            "--min-ttl",
            "255",
            "--multi-exit-discriminator",
            "20",
            "--remote-asn",
            "65548",
            "--vlan-id",
            "400",
        ],
        &expected,
    );
}

// `bgp peer set` replaces the peer with the same address in place, using the
// documented defaults for omitted options.
#[test]
fn test_net_bgp_peer_set_existing() {
    let port = TestPort::new();

    let bgp_config = Uuid::new_v4();
    let mut expected = port.current();
    expected.bgp_peers[0].peers[0] = BgpPeer {
        addr: RouterPeerType::Numbered {
            src_addr: None,
            target_addr: PEER.parse().unwrap(),
        },
        allowed_export: ImportExportPolicy::NoFiltering,
        allowed_import: ImportExportPolicy::NoFiltering,
        bgp_config: NameOrId::Id(bgp_config),
        communities: Vec::new(),
        connect_retry: 0,
        delay_open: 0,
        enforce_first_as: false,
        hold_time: 6,
        idle_hold_time: 0,
        keepalive: 2,
        local_pref: None,
        md5_auth_key: None,
        min_ttl: None,
        multi_exit_discriminator: None,
        remote_asn: None,
        vlan_id: None,
    };

    port.run(
        &[
            "bgp",
            "peer",
            "set",
            "--addr",
            PEER,
            "--bgp-config",
            &bgp_config.to_string(),
        ],
        &expected,
    );
}

// `bgp peer delete` removes only the peer with the given address.
#[test]
fn test_net_bgp_peer_delete() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.bgp_peers[0].peers.remove(0);

    port.run(&["bgp", "peer", "delete", "--addr", PEER], &expected);
}

// `bgp filter --direction import --allowed` sets the peer's import policy to
// allow the given prefixes.
#[test]
fn test_net_bgp_filter_import_allowed() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.bgp_peers[0].peers[0].allowed_import = ImportExportPolicy::Allow(vec![
        "203.0.113.0/24".parse().unwrap(),
        "198.51.100.0/25".parse().unwrap(),
    ]);

    port.run(
        &[
            "bgp",
            "filter",
            "--peer",
            PEER,
            "--direction",
            "import",
            "--allowed",
            "203.0.113.0/24",
            "--allowed",
            "198.51.100.0/25",
        ],
        &expected,
    );
}

// `bgp filter --direction export --no-filtering` removes the peer's export
// filter.
#[test]
fn test_net_bgp_filter_export_no_filtering() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.bgp_peers[0].peers[0].allowed_export = ImportExportPolicy::NoFiltering;

    port.run(
        &[
            "bgp",
            "filter",
            "--peer",
            PEER,
            "--direction",
            "export",
            "--no-filtering",
        ],
        &expected,
    );
}

// `bgp auth --authstring` sets the peer's TCP-MD5 key.
#[test]
fn test_net_bgp_auth() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.bgp_peers[0].peers[0].md5_auth_key = Some("hunter2".to_string());

    port.run(
        &["bgp", "auth", "--peer", PEER, "--authstring", "hunter2"],
        &expected,
    );
}

// `bgp pref --local-pref` sets the peer's local preference.
#[test]
fn test_net_bgp_pref() {
    let port = TestPort::new();

    let mut expected = port.current();
    expected.bgp_peers[0].peers[0].local_pref = Some(200);

    port.run(
        &["bgp", "pref", "--peer", PEER, "--local-pref", "200"],
        &expected,
    );
}

// `bgp auth` without `--authstring` removes the peer's TCP-MD5 key.
#[test]
fn test_net_bgp_auth_clear() {
    let mut port = TestPort::new();
    port.settings.bgp_peers[0].md5_auth_key = Some("hunter2".to_string());

    let mut expected = port.current();
    expected.bgp_peers[0].peers[0].md5_auth_key = None;

    port.run(&["bgp", "auth", "--peer", PEER], &expected);
}

// `bgp pref` without `--local-pref` removes the peer's local preference.
#[test]
fn test_net_bgp_pref_clear() {
    let mut port = TestPort::new();
    port.settings.bgp_peers[0].local_pref = Some(200);

    let mut expected = port.current();
    expected.bgp_peers[0].peers[0].local_pref = None;

    port.run(&["bgp", "pref", "--peer", PEER], &expected);
}
