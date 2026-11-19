// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

use httpmock::MockServer;
use oxide::types::{
    BgpPeerState, BgpPeerStatus, SwitchLinkState, SwitchPort, SwitchPortResultsPage, SwitchSlot,
};
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use serde_json::json;
use test_common::JsonMock;

// Each peer appears under its switch, in the order the API returns them.
#[test]
fn test_bgp_show_status() {
    let server = MockServer::start();

    let peers = [
        (
            SwitchSlot::Switch1,
            "10.1.0.1",
            "sw1-a",
            BgpPeerState::Established,
            3_723_004,
        ),
        (
            SwitchSlot::Switch0,
            "10.0.0.1",
            "sw0-a",
            BgpPeerState::Active,
            1_000,
        ),
        (
            SwitchSlot::Switch1,
            "fd00::11",
            "sw1-b",
            BgpPeerState::Idle,
            0,
        ),
        (
            SwitchSlot::Switch0,
            "10.0.0.2",
            "sw0-b",
            BgpPeerState::OpenSent,
            90_061_000,
        ),
    ]
    .into_iter()
    .enumerate()
    .map(
        |(ii, (switch, addr, peer_id, state, millis))| BgpPeerStatus {
            switch,
            addr: addr.parse().unwrap(),
            peer_id: peer_id.to_string(),
            local_asn: 65000 + ii as u32,
            remote_asn: 64500 + ii as u32,
            state,
            state_duration_millis: millis,
        },
    )
    .collect::<Vec<_>>();

    let mock = server.networking_bgp_status(|when, then| {
        when.into_inner().any_request();
        then.ok(&peers);
    });

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("system")
        .arg("networking")
        .arg("bgp")
        .arg("show-status")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_net_status_bgp_show_status.stdout",
        ));

    mock.assert();
}

// Each port appears under its switch, sorted by port number, with the status
// of that port on that switch. Both switches have a qsfp0 and a qsfp10.
#[test]
fn test_switch_port_show_status() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let rack_id = uuid::Uuid::mock_value(&mut src).unwrap();

    let ports = [
        (SwitchSlot::Switch1, "qsfp10", true),
        (SwitchSlot::Switch0, "qsfp2", true),
        (SwitchSlot::Switch1, "qsfp0", false),
        (SwitchSlot::Switch0, "qsfp10", false),
        (SwitchSlot::Switch0, "qsfp0", true),
    ]
    .into_iter()
    .map(|(switch_slot, name, configured)| SwitchPort {
        rack_id,
        switch_slot,
        port_name: name.parse().unwrap(),
        port_settings_id: configured.then(|| uuid::Uuid::mock_value(&mut src).unwrap()),
        ..JsonMock::mock_value(&mut src).unwrap()
    })
    .collect::<Vec<_>>();

    let list_mock = server.networking_switch_port_list(|when, then| {
        when.into_inner().any_request();
        then.ok(&SwitchPortResultsPage {
            items: ports.clone(),
            next_page: None,
        });
    });

    let status_mocks = ports
        .iter()
        .enumerate()
        .map(|(ii, port)| {
            let n = ii as u8;
            let link_state = if ii % 2 == 0 {
                json!("Up")
            } else {
                json!({ "Faulted": format!("fault{ii}") })
            };
            let status = SwitchLinkState(json!({
                "link": {
                    "address": { "a": [0xa8, 0x40, 0x25, 0x00, n, 0x01] },
                    "enabled": ii % 2 == 0,
                    "autoneg": ii % 3 == 0,
                    "fec": format!("Fec{ii}"),
                    "link_state": link_state,
                    "fsm_state": format!("Fsm{ii}"),
                    "media": format!("Media{ii}"),
                    "speed": format!("Speed{ii}00G"),
                },
                "monitors": {
                    "receiver_power": [{ "average": ii as f32 + 0.5 }],
                    "transmitter_bias_current": [ii as f32 + 0.25],
                    "transmitter_power": [ii as f32 + 0.75],
                },
            }));
            server.networking_switch_port_status(|when, then| {
                when.port(&port.port_name)
                    .rack_id(&rack_id)
                    .switch_slot(port.switch_slot);
                then.ok(&status);
            })
        })
        .collect::<Vec<_>>();

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("system")
        .arg("hardware")
        .arg("switch-port")
        .arg("show-status")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_net_status_switch_port_show_status.stdout",
        ));

    list_mock.assert();
    for mock in status_mocks {
        mock.assert();
    }
}
