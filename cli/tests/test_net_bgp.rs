// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

use httpmock::MockServer;
use oxide::types::{
    BgpAnnounceSet, BgpAnnounceSetCreate, BgpAnnouncement, BgpAnnouncementCreate, NameOrId,
};
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use test_common::JsonMock;
use uuid::Uuid;

// `bgp announce` adds the prefix to the announcements already in the set.
#[test]
fn test_net_bgp_announce() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let set = BgpAnnounceSet {
        name: "as-set".parse().unwrap(),
        ..JsonMock::mock_value(&mut src).unwrap()
    };
    let existing = BgpAnnouncement {
        announce_set_id: set.id,
        network: "198.51.100.0/24".parse().unwrap(),
        address_lot_block_id: Uuid::mock_value(&mut src).unwrap(),
    };

    let list = server.networking_bgp_announcement_list(|when, then| {
        when.announce_set(&NameOrId::Name(set.name.clone()));
        then.ok(&vec![existing.clone()]);
    });
    let update = server.networking_bgp_announce_set_update(|when, then| {
        when.body(&BgpAnnounceSetCreate {
            announcement: vec![
                BgpAnnouncementCreate {
                    address_lot_block: NameOrId::Id(existing.address_lot_block_id),
                    network: existing.network.clone(),
                },
                BgpAnnouncementCreate {
                    address_lot_block: NameOrId::Name("lot-a".parse().unwrap()),
                    network: "203.0.113.0/24".parse().unwrap(),
                },
            ],
            description: "announcements for as-set".to_string(),
            name: set.name.clone(),
        });
        then.ok(&set);
    });

    assert_cmd::cargo::cargo_bin_cmd!("oxide")
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .args(["system", "networking", "bgp", "announce"])
        .args(["--announce-set", "as-set"])
        .args(["--address-lot", "lot-a"])
        .args(["--prefix", "203.0.113.0/24"])
        .args(["--description", "announcements for as-set"])
        .assert()
        .success();

    list.assert();
    update.assert();
}
