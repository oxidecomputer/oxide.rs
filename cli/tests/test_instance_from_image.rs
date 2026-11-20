// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2026 Oxide Computer Company

use httpmock::MockServer;
use oxide::types::{
    DiskBackend, DiskSource, ExternalIpCreate, Image, Instance, InstanceCpuPlatform,
    InstanceCreate, InstanceDiskAttachment, IpVersion, NameOrId, PoolSelector,
};
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use test_common::JsonMock;

const PROJECT: &str = "projname";

fn from_image_cmd(server: &MockServer, image: &str) -> assert_cmd::Command {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("oxide");
    cmd.env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .args([
            "instance",
            "from-image",
            "--name",
            "bingo",
            "--project",
            PROJECT,
            "--description",
            "a dog",
            "--hostname",
            "bingo",
            "--memory",
            "8g",
            "--ncpus",
            "2",
            "--image",
            image,
            "--size",
            "20g",
        ]);
    cmd
}

/// The body that the arguments in `from_image_cmd` produce.
fn base_body(
    image_id: uuid::Uuid,
    pool_selector: PoolSelector,
) -> oxide::types::builder::InstanceCreate {
    InstanceCreate::builder()
        .name("bingo".parse::<oxide::types::Name>().unwrap())
        .description("a dog")
        .boot_disk(Some(InstanceDiskAttachment::Create {
            description: "bingo disk".to_string(),
            disk_backend: DiskBackend::Distributed(DiskSource::Image {
                image_id,
                read_only: false,
            }),
            name: "bingo-disk".parse().unwrap(),
            size: (20u64 * 1024 * 1024 * 1024).into(),
        }))
        .external_ips(vec![ExternalIpCreate::Ephemeral { pool_selector }])
        .hostname("bingo".parse::<oxide::types::Hostname>().unwrap())
        .memory(8u64 * 1024 * 1024 * 1024)
        .ncpus(2)
        .start(false)
}

/// Runs `from_image_cmd` with `extra` arguments and checks that it looks up
/// the image by name in `PROJECT` and creates an instance with `body`.
fn run_by_name(
    extra: &[&str],
    body: impl FnOnce(uuid::Uuid) -> oxide::types::builder::InstanceCreate,
) {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let image = Image::mock_value(&mut src).unwrap();
    let image_view = server.image_view(|when, then| {
        when.image(&"blank".parse().unwrap())
            .project(Some(&PROJECT.parse().unwrap()));
        then.ok(&image);
    });

    let body: InstanceCreate = body(image.id).try_into().unwrap();
    let instance = Instance::mock_value(&mut src).unwrap();
    let instance_create = server.instance_create(|when, then| {
        when.project(&PROJECT.parse().unwrap()).body(&body);
        then.created(&instance);
    });

    from_image_cmd(&server, "blank")
        .args(extra)
        .assert()
        .success()
        .stdout(format!("instance {} created\n", instance.id));

    image_view.assert();
    instance_create.assert();
}

fn auto_v4() -> PoolSelector {
    PoolSelector::Auto {
        ip_version: Some(IpVersion::V4),
    }
}

#[test]
fn test_instance_from_image_defaults() {
    run_by_name(&[], |id| base_body(id, auto_v4()));
}

#[test]
fn test_instance_from_image_ip_pool() {
    run_by_name(&["--ip-pool", "mypool"], |id| {
        base_body(
            id,
            PoolSelector::Explicit {
                pool: "mypool".parse().unwrap(),
            },
        )
    });
}

#[test]
fn test_instance_from_image_options() {
    run_by_name(
        &[
            "--start",
            "--cpu-platform",
            "amd_milan",
            "--anti-affinity-group",
            "group-a",
            "--anti-affinity-group",
            "group-b",
            "--enable-jumbo-frames",
        ],
        |id| {
            base_body(id, auto_v4())
                .start(true)
                .cpu_platform(Some(InstanceCpuPlatform::AmdMilan))
                .anti_affinity_groups(vec![
                    "group-a".parse::<NameOrId>().unwrap(),
                    "group-b".parse::<NameOrId>().unwrap(),
                ])
                .enable_jumbo_frames(true)
        },
    );
}

// An image given by id is looked up without a project.
#[test]
fn test_instance_from_image_image_id() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let image = Image::mock_value(&mut src).unwrap();
    let image_view = server.image_view(|when, then| {
        when.image(&NameOrId::Id(image.id)).project(None);
        then.ok(&image);
    });

    let body: InstanceCreate = base_body(image.id, auto_v4()).try_into().unwrap();
    let instance = Instance::mock_value(&mut src).unwrap();
    let instance_create = server.instance_create(|when, then| {
        when.project(&PROJECT.parse().unwrap()).body(&body);
        then.created(&instance);
    });

    from_image_cmd(&server, &image.id.to_string())
        .assert()
        .success();

    image_view.assert();
    instance_create.assert();
}

#[test]
fn test_instance_from_image_ip_pool_and_ip_version() {
    let server = MockServer::start();
    let any = server.mock(|when, then| {
        when.any_request();
        then.status(500);
    });

    from_image_cmd(&server, "blank")
        .args(["--ip-pool", "mypool", "--ip-version", "v6"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicates::str::contains("cannot be used with"));

    any.assert_calls(0);
}
