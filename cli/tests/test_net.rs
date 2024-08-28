// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use assert_cmd::Command;
use chrono::prelude::*;
use httpmock::MockServer;
use oxide::types::{
    AddressLot, AddressLotBlock, AddressLotBlockResultsPage, AddressLotKind, AddressLotResultsPage,
    BgpConfig, BgpConfigResultsPage, BgpPeer, ImportExportPolicy, LinkFec, LinkSpeed, NameOrId,
    SwitchPort, SwitchPortAddressConfig, SwitchPortConfig, SwitchPortGeometry2,
    SwitchPortLinkConfig, SwitchPortResultsPage, SwitchPortRouteConfig, SwitchPortSettings,
    SwitchPortSettingsView,
};
use oxide_httpmock::MockServerExt;
use uuid::Uuid;

#[test]
fn test_port_config() {
    let server = MockServer::start();

    let rack_id = Uuid::new_v4();
    let switch0_qsfp0_settings_id = Uuid::new_v4();
    let switch1_qsfp0_settings_id = Uuid::new_v4();
    let lot_id = Uuid::new_v4();
    let lot_block_id = Uuid::new_v4();

    let ports = SwitchPortResultsPage {
        items: vec![
            SwitchPort {
                id: Uuid::new_v4(),
                port_name: String::from("qsfp0"),
                port_settings_id: Some(switch0_qsfp0_settings_id),
                rack_id,
                switch_location: String::from("switch0"),
            },
            SwitchPort {
                id: Uuid::new_v4(),
                port_name: String::from("qsfp0"),
                port_settings_id: Some(switch1_qsfp0_settings_id),
                rack_id,
                switch_location: String::from("switch1"),
            },
        ],
        next_page: None,
    };

    let mock_ports = server.networking_switch_port_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&ports);
    });

    let lots = AddressLotResultsPage {
        items: vec![AddressLot {
            description: String::from("Initial infrastructure address lot"),
            id: lot_id,
            kind: AddressLotKind::Infra,
            name: "initial-infra".parse().unwrap(),
            time_created: Utc.with_ymd_and_hms(2024, 7, 8, 9, 10, 11).unwrap(),
            time_modified: Utc.with_ymd_and_hms(2024, 7, 8, 9, 10, 11).unwrap(),
        }],
        next_page: None,
    };

    let mock_lots = server.networking_address_lot_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&lots);
    });

    let lot_blocks = AddressLotBlockResultsPage {
        items: vec![AddressLotBlock {
            first_address: "198.51.100.0".parse().unwrap(),
            last_address: "198.51.100.254".parse().unwrap(),
            id: lot_block_id,
        }],
        next_page: None,
    };

    let mock_lot_blocks = server.networking_address_lot_block_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&lot_blocks);
    });

    let bgp_configs = BgpConfigResultsPage {
        items: vec![BgpConfig {
            asn: 65547,
            description: String::from("as65547"),
            id: Uuid::new_v4(),
            name: "as65547".parse().unwrap(),
            vrf: None,
            time_created: Utc.with_ymd_and_hms(2024, 7, 8, 9, 10, 11).unwrap(),
            time_modified: Utc.with_ymd_and_hms(2024, 7, 8, 9, 10, 11).unwrap(),
        }],
        next_page: None,
    };

    let mock_bgp_configs = server.networking_bgp_config_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&bgp_configs);
    });

    let switch0_qsfp0_view = SwitchPortSettingsView {
        addresses: vec![
            SwitchPortAddressConfig {
                address: "169.254.10.2/30".parse().unwrap(),
                address_lot_block_id: lot_block_id,
                interface_name: String::from("phy0"),
                port_settings_id: switch0_qsfp0_settings_id,
                vlan_id: None,
            },
            SwitchPortAddressConfig {
                address: "169.254.30.2/30".parse().unwrap(),
                address_lot_block_id: lot_block_id,
                interface_name: String::from("phy0"),
                port_settings_id: switch0_qsfp0_settings_id,
                vlan_id: Some(300),
            },
        ],
        bgp_peers: vec![
            BgpPeer {
                interface_name: String::from("phy0"),
                addr: "169.254.10.1".parse().unwrap(),
                bgp_config: NameOrId::Id(bgp_configs.items[0].id),
                allowed_export: ImportExportPolicy::Allow(vec!["198.51.100.0/24".parse().unwrap()]),
                allowed_import: ImportExportPolicy::NoFiltering,
                communities: Vec::new(),
                connect_retry: 3,
                delay_open: 3,
                enforce_first_as: false,
                hold_time: 6,
                idle_hold_time: 3,
                keepalive: 2,
                local_pref: None,
                md5_auth_key: None,
                min_ttl: None,
                multi_exit_discriminator: None,
                remote_asn: None,
                vlan_id: None,
            },
            BgpPeer {
                interface_name: String::from("phy0"),
                addr: "169.254.30.1".parse().unwrap(),
                bgp_config: NameOrId::Id(bgp_configs.items[0].id),
                allowed_export: ImportExportPolicy::Allow(vec!["203.0.113.0/24".parse().unwrap()]),
                allowed_import: ImportExportPolicy::NoFiltering,
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
                vlan_id: Some(300),
            },
        ],
        groups: Vec::new(),
        interfaces: Vec::new(),
        link_lldp: Vec::new(),
        links: vec![SwitchPortLinkConfig {
            autoneg: false,
            fec: LinkFec::None,
            link_name: String::from("phy0"),
            lldp_link_config_id: None,
            mtu: 1500,
            port_settings_id: switch1_qsfp0_settings_id,
            speed: LinkSpeed::Speed100G,
        }],
        port: SwitchPortConfig {
            geometry: SwitchPortGeometry2::Qsfp28x1,
            port_settings_id: switch1_qsfp0_settings_id,
        },
        routes: vec![SwitchPortRouteConfig {
            dst: "1.2.3.0/24".parse().unwrap(),
            gw: "1.2.3.4/32".parse().unwrap(),
            interface_name: "qsfp0".to_owned(),
            local_pref: Some(10),
            port_settings_id: Uuid::new_v4(),
            vlan_id: Some(1701),
        }],
        settings: SwitchPortSettings {
            description: String::from("default uplink 0 switch port settings"),
            id: switch1_qsfp0_settings_id,
            name: "default-uplink0".parse().unwrap(),
            time_created: Utc.with_ymd_and_hms(2024, 7, 8, 9, 10, 11).unwrap(),
            time_modified: Utc.with_ymd_and_hms(2024, 7, 8, 9, 10, 11).unwrap(),
        },
        vlan_interfaces: Vec::new(),
    };
    let switch1_qsfp0_view = SwitchPortSettingsView {
        addresses: vec![
            SwitchPortAddressConfig {
                address: "169.254.20.2/30".parse().unwrap(),
                address_lot_block_id: lot_block_id,
                interface_name: String::from("phy0"),
                port_settings_id: switch0_qsfp0_settings_id,
                vlan_id: None,
            },
            SwitchPortAddressConfig {
                address: "169.254.40.2/30".parse().unwrap(),
                address_lot_block_id: lot_block_id,
                interface_name: String::from("phy0"),
                port_settings_id: switch0_qsfp0_settings_id,
                vlan_id: Some(400),
            },
        ],
        bgp_peers: vec![
            BgpPeer {
                interface_name: String::from("phy0"),
                addr: "169.254.20.1".parse().unwrap(),
                bgp_config: NameOrId::Id(bgp_configs.items[0].id),
                allowed_export: ImportExportPolicy::Allow(vec!["198.51.100.0/24".parse().unwrap()]),
                allowed_import: ImportExportPolicy::NoFiltering,
                communities: Vec::new(),
                connect_retry: 3,
                delay_open: 3,
                enforce_first_as: false,
                hold_time: 6,
                idle_hold_time: 3,
                keepalive: 2,
                local_pref: None,
                md5_auth_key: None,
                min_ttl: None,
                multi_exit_discriminator: None,
                remote_asn: None,
                vlan_id: None,
            },
            BgpPeer {
                interface_name: String::from("phy0"),
                addr: "169.254.40.1".parse().unwrap(),
                bgp_config: NameOrId::Id(bgp_configs.items[0].id),
                allowed_export: ImportExportPolicy::Allow(vec!["203.0.113.0/24".parse().unwrap()]),
                allowed_import: ImportExportPolicy::NoFiltering,
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
                vlan_id: Some(400),
            },
        ],
        groups: Vec::new(),
        interfaces: Vec::new(),
        link_lldp: Vec::new(),
        links: vec![SwitchPortLinkConfig {
            autoneg: false,
            fec: LinkFec::None,
            link_name: String::from("phy0"),
            lldp_link_config_id: None,
            mtu: 1500,
            port_settings_id: switch1_qsfp0_settings_id,
            speed: LinkSpeed::Speed100G,
        }],
        port: SwitchPortConfig {
            geometry: SwitchPortGeometry2::Qsfp28x1,
            port_settings_id: switch1_qsfp0_settings_id,
        },
        routes: Vec::new(),
        settings: SwitchPortSettings {
            description: String::from("default uplink 1 switch port settings"),
            id: switch1_qsfp0_settings_id,
            name: "default-uplink1".parse().unwrap(),
            time_created: Utc.with_ymd_and_hms(2024, 7, 8, 9, 10, 11).unwrap(),
            time_modified: Utc.with_ymd_and_hms(2024, 7, 8, 9, 10, 11).unwrap(),
        },
        vlan_interfaces: Vec::new(),
    };

    let mock_switch0_qsfp0_settings_view =
        server.networking_switch_port_settings_view(|when, then| {
            when.port(&NameOrId::Id(ports.items[0].port_settings_id.unwrap()));
            then.ok(&switch0_qsfp0_view);
        });

    let mock_switch1_qsfp0_settings_view =
        server.networking_switch_port_settings_view(|when, then| {
            when.port(&NameOrId::Id(ports.items[1].port_settings_id.unwrap()));
            then.ok(&switch1_qsfp0_view);
        });

    env_logger::init();

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("system")
        .arg("networking")
        .arg("switch-port-settings")
        .arg("show")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_switch_port_settings_show.stdout",
        ));

    mock_ports.assert();
    mock_lots.assert();
    mock_lot_blocks.assert();
    mock_bgp_configs.assert_hits(2);
    mock_switch0_qsfp0_settings_view.assert();
    mock_switch1_qsfp0_settings_view.assert();
}
