// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use std::{collections::BTreeMap, net::IpAddr};

use clap::{Command, CommandFactory, FromArgMatches};

use config::Config;
use context::Context;
use generated_cli::{Cli, CliCommand, CliOverride};
use oxide_api::types::{IpRange, Ipv4Range, Ipv6Range};

mod cmd_api;
mod cmd_auth;
mod cmd_disk;
mod cmd_docs;
mod cmd_version;
mod config;
mod context;
#[allow(unused_mut)]
#[allow(unused)] // TODO
#[allow(unused_must_use)] // TODO
#[allow(clippy::clone_on_copy)]
mod generated_cli;

#[derive(Debug, Default)]
struct Tree<'a> {
    children: BTreeMap<&'a str, Tree<'a>>,
    cmd: Option<CliCommand>,
}

impl<'a> Tree<'a> {
    fn cmd(&self, name: &str) -> Command {
        let mut cmd = if let Some(op) = self.cmd {
            // Command node
            // TODO
            let cmd = Cli::get_command(op).name(name.to_owned());
            match op {
                CliCommand::IpPoolRangeAdd => cmd
                    .arg(
                        clap::Arg::new("first")
                            .long("first")
                            .value_name("ip-addr")
                            .required(true)
                            .value_parser(clap::value_parser!(std::net::IpAddr)),
                    )
                    .arg(
                        clap::Arg::new("last")
                            .long("last")
                            .value_name("ip-addr")
                            .required(true)
                            .value_parser(clap::value_parser!(std::net::IpAddr)),
                    ),

                // Command is fine as-is.
                _ => cmd,
            }
        } else {
            // Internal node
            Command::new(name.to_owned()).subcommand_required(true)
        };

        for (sub_name, sub_tree) in self.children.iter() {
            cmd = cmd.subcommand(sub_tree.cmd(sub_name));
        }

        cmd
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut root = Tree::default();
    for op in CliCommand::iter() {
        let mut node = &mut root;
        let Some(subcmd_str) = xxx(op) else {
            continue
        };

        let mut path = subcmd_str.split(' ').peekable();
        while let Some(ss) = path.next() {
            if path.peek().is_some() {
                node = node.children.entry(ss).or_default();
            } else {
                assert!(
                    node.children.get(ss).is_none(),
                    "two identical subcommands {}",
                    subcmd_str,
                );
                node.children.insert(
                    ss,
                    Tree {
                        cmd: Some(op),
                        ..Default::default()
                    },
                );
            }
        }
    }

    let cmd = root
        .cmd("oxide")
        .bin_name("oxide")
        .subcommand(cmd_auth::CmdAuth::command())
        .subcommand(cmd_api::CmdApi::command())
        .subcommand(cmd_docs::CmdDocs::command())
        .subcommand(cmd_version::CmdVersion::command())
        .add_subcommand("disk import", cmd_disk::CmdDiskImport::command());

    let matches = cmd.clone().get_matches();

    // Construct the global config. We do this **after** parsing options,
    // anticipating that top-level options may impact e.g. where we look for
    // config files.
    let config = Config::default();
    let mut ctx = Context::new(config).unwrap();

    let result = match matches.subcommand() {
        Some(("api", sub_matches)) => {
            cmd_api::CmdApi::from_arg_matches(sub_matches)
                .unwrap()
                .run(&mut ctx)
                .await
        }

        Some(("auth", sub_matches)) => {
            cmd_auth::CmdAuth::from_arg_matches(sub_matches)
                .unwrap()
                .run(&mut ctx)
                .await
        }

        // TODO this could be improved
        Some(("disk", sub_matches)) if sub_matches.subcommand_name() == Some("import") => {
            let real_sub_matches = sub_matches.subcommand().unwrap().1;
            cmd_disk::CmdDiskImport::from_arg_matches(real_sub_matches)
                .unwrap()
                .run(&mut ctx)
                .await
        }

        Some(("docs", sub_matches)) => {
            cmd_docs::CmdDocs::from_arg_matches(sub_matches)
                .unwrap()
                .run(&cmd)
                .await
        }

        Some(("version", sub_matches)) => {
            cmd_version::CmdVersion::from_arg_matches(sub_matches)
                .unwrap()
                .run()
                .await
        }

        _ => {
            // Spawn a task so we get this potentially chunky Future off the
            // main thread's stack.
            tokio::spawn(async move {
                let mut node = &root;
                let mut sm = &matches;

                while let Some((sub_name, sub_matches)) = sm.subcommand() {
                    node = node.children.get(sub_name).unwrap();
                    sm = sub_matches;
                }

                let cli = Cli::new_with_override(ctx.client().clone(), OxideOverride);

                // TODO error handling
                cli.execute(node.cmd.unwrap(), sm).await;
            })
            .await
            .unwrap();
            Ok(())
        }
    };

    if let Err(e) = result {
        println!("error: {}", e);
        std::process::exit(1)
    }
}

fn xxx<'a>(command: CliCommand) -> Option<&'a str> {
    match command {
        CliCommand::InstanceList => Some("instance list"),
        CliCommand::InstanceCreate => Some("instance create"),
        CliCommand::InstanceView => Some("instance view"),
        CliCommand::InstanceDelete => Some("instance delete"),
        CliCommand::InstanceMigrate => None, // TODO delete from API?
        CliCommand::InstanceReboot => Some("instance reboot"),
        CliCommand::InstanceSerialConsole => None, // TODO not sure how to handle this
        CliCommand::InstanceSerialConsoleStream => None, // Ditto
        CliCommand::InstanceStart => Some("instance start"),
        CliCommand::InstanceStop => Some("instance stop"),
        CliCommand::InstanceExternalIpList => Some("instance external-ip list"),

        CliCommand::ProjectList => Some("project list"),
        CliCommand::ProjectCreate => Some("project create"),
        CliCommand::ProjectView => Some("project view"),
        CliCommand::ProjectUpdate => Some("project update"),
        CliCommand::ProjectDelete => Some("project delete"),
        CliCommand::ProjectPolicyView => Some("project policy view"),
        CliCommand::ProjectPolicyUpdate => Some("project policy update"),

        CliCommand::ImageList => Some("image list"),
        CliCommand::ImageCreate => Some("image create"),
        CliCommand::ImageView => Some("image view"),
        CliCommand::ImageDelete => Some("image delete"),
        CliCommand::ImagePromote => Some("image promote"),
        CliCommand::ImageDemote => Some("image demote"),

        CliCommand::SiloIdentityProviderList => Some("silo idp list"),
        CliCommand::LocalIdpUserCreate => Some("silo idp local user create"),
        CliCommand::LocalIdpUserDelete => Some("silo idp local user delete"),
        CliCommand::LocalIdpUserSetPassword => Some("silo idp local user set-password"),
        CliCommand::SamlIdentityProviderCreate => Some("silo idp saml create"),
        CliCommand::SamlIdentityProviderView => Some("silo idp saml view"),

        CliCommand::IpPoolList => Some("ip-pool list"),
        CliCommand::IpPoolCreate => Some("ip-pool create"),
        CliCommand::IpPoolView => Some("ip-pool view"),
        CliCommand::IpPoolUpdate => Some("ip-pool update"),
        CliCommand::IpPoolDelete => Some("ip-pool delete"),
        CliCommand::IpPoolRangeList => Some("ip-pool range list"),
        CliCommand::IpPoolRangeAdd => Some("ip-pool range add"),
        CliCommand::IpPoolRangeRemove => Some("ip-pool range remove"),
        CliCommand::IpPoolServiceView => Some("ip-pool service view"),
        CliCommand::IpPoolServiceRangeList => Some("ip-pool service range list"),
        CliCommand::IpPoolServiceRangeAdd => Some("ip-pool service range add"),
        CliCommand::IpPoolServiceRangeRemove => Some("ip-pool service remove"),
        CliCommand::SiloList => Some("silo list"),
        CliCommand::SiloCreate => Some("silo create"),
        CliCommand::SiloView => Some("silo view"),
        CliCommand::SiloDelete => Some("silo delete"),
        CliCommand::SiloPolicyView => Some("silo policy view"),
        CliCommand::SiloPolicyUpdate => Some("silo policy update"),
        CliCommand::SiloUserList => Some("silo user list"),
        CliCommand::SiloUserView => Some("silo user view"),

        CliCommand::UserList => Some("user list"),

        // VPCs
        CliCommand::VpcList => Some("vpc list"),
        CliCommand::VpcCreate => Some("vpc create"),
        CliCommand::VpcView => Some("vpc view"),
        CliCommand::VpcUpdate => Some("vpc update"),
        CliCommand::VpcDelete => Some("vpc delete"),

        CliCommand::VpcFirewallRulesView => Some("vpc firewall-rules view"),
        CliCommand::VpcFirewallRulesUpdate => Some("vpc firewall-rules update"),

        CliCommand::VpcRouterList => Some("vpc router list"),
        CliCommand::VpcRouterCreate => Some("vpc router create"),
        CliCommand::VpcRouterView => Some("vpc router view"),
        CliCommand::VpcRouterUpdate => Some("vpc router update"),
        CliCommand::VpcRouterDelete => Some("vpc router delete"),

        CliCommand::VpcRouterRouteList => Some("vpc router route list"),
        CliCommand::VpcRouterRouteCreate => Some("vpc router route create"),
        CliCommand::VpcRouterRouteView => Some("vpc router route view"),
        CliCommand::VpcRouterRouteUpdate => Some("vpc router route update"),
        CliCommand::VpcRouterRouteDelete => Some("vpc router route delete"),

        CliCommand::VpcSubnetList => Some("vpc subnet list"),
        CliCommand::VpcSubnetCreate => Some("vpc subnet create"),
        CliCommand::VpcSubnetView => Some("vpc subnet view"),
        CliCommand::VpcSubnetUpdate => Some("vpc subnet update"),
        CliCommand::VpcSubnetDelete => Some("vpc subnet delete"),
        CliCommand::VpcSubnetListNetworkInterfaces => Some("vpc subnet nic list"),

        // Subcommand: disk
        CliCommand::DiskList => Some("disk list"),
        CliCommand::DiskCreate => Some("disk create"),
        CliCommand::DiskView => Some("disk view"),
        CliCommand::DiskDelete => Some("disk delete"),
        CliCommand::DiskMetricsList => Some("disk metrics list"),

        CliCommand::DiskBulkWriteImport
        | CliCommand::DiskBulkWriteImportStart
        | CliCommand::DiskBulkWriteImportStop
        | CliCommand::DiskFinalizeImport
        | CliCommand::DiskImportBlocksFromUrl => None,

        CliCommand::GroupList => Some("group list"),
        CliCommand::InstanceDiskList => Some("instance disk list"),
        CliCommand::InstanceDiskAttach => Some("instance disk attach"),
        CliCommand::InstanceDiskDetach => Some("instance disk detach"),
        CliCommand::InstanceNetworkInterfaceList => Some("instance nic list"),
        CliCommand::InstanceNetworkInterfaceCreate => Some("instance nic create"),
        CliCommand::InstanceNetworkInterfaceView => Some("instance nic view"),
        CliCommand::InstanceNetworkInterfaceUpdate => Some("instance nic update"),
        CliCommand::InstanceNetworkInterfaceDelete => Some("instance nic delete"),
        CliCommand::PolicyView => Some("policy view"),
        CliCommand::PolicyUpdate => Some("policy update"),
        CliCommand::SnapshotList => Some("snapshot list"),
        CliCommand::SnapshotCreate => Some("snapshot create"),
        CliCommand::SnapshotView => Some("snapshot view"),
        CliCommand::SnapshotDelete => Some("snapshot delete"),
        CliCommand::CertificateList => Some("certificate list"),
        CliCommand::CertificateCreate => Some("certificate create"),
        CliCommand::CertificateView => Some("certificate view"),
        CliCommand::CertificateDelete => Some("certificate delete"),

        CliCommand::SwitchList => Some("hardware switch list"),
        CliCommand::SwitchView => Some("hardware switch view"),
        CliCommand::RackList => Some("hardware rack list"),
        CliCommand::RackView => Some("hardware rack view"),
        CliCommand::SledList => Some("hardware sled list"),
        CliCommand::SledView => Some("hardware sled view"),
        CliCommand::PhysicalDiskList => Some("hardware disk list"),
        CliCommand::SledPhysicalDiskList => Some("hardware sled disk-led"),

        CliCommand::SystemPolicyView => Some("system policy view"),
        CliCommand::SystemPolicyUpdate => Some("system policy update"),

        CliCommand::CurrentUserView => Some("current-user view"),
        CliCommand::CurrentUserGroups => Some("current-user groups"),
        CliCommand::CurrentUserSshKeyList => Some("current-user ssh-key list"),
        CliCommand::CurrentUserSshKeyCreate => Some("current-user ssh-key create"),
        CliCommand::CurrentUserSshKeyView => Some("current-user ssh-key view"),
        CliCommand::CurrentUserSshKeyDelete => Some("current-user ssh-key delete"),

        // Commands not yet implemented
        CliCommand::DeviceAccessToken
        | CliCommand::DeviceAuthConfirm
        | CliCommand::DeviceAuthRequest
        | CliCommand::GroupView
        | CliCommand::LoginLocal
        | CliCommand::LoginSaml
        | CliCommand::LoginSamlBegin
        | CliCommand::LoginSpoof
        | CliCommand::Logout
        | CliCommand::RoleList
        | CliCommand::RoleView
        | CliCommand::SystemComponentVersionList
        | CliCommand::SystemMetric
        | CliCommand::SystemUpdateComponentsList
        | CliCommand::SystemUpdateList
        | CliCommand::SystemUpdateRefresh
        | CliCommand::SystemUpdateStart
        | CliCommand::SystemUpdateStop
        | CliCommand::SystemUpdateView
        | CliCommand::UpdateDeploymentsList
        | CliCommand::UpdateDeploymentView
        | CliCommand::UserBuiltinList
        | CliCommand::UserBuiltinView
        | CliCommand::SystemVersion => None,
    }
}

struct OxideOverride;

impl CliOverride for OxideOverride {
    fn execute_ip_pool_range_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide_api::builder::IpPoolRangeAdd,
    ) -> Result<(), String> {
        let first = matches.get_one::<IpAddr>("first").unwrap();
        let last = matches.get_one::<IpAddr>("last").unwrap();

        let range: IpRange = match (first, last) {
            (IpAddr::V4(first), IpAddr::V4(last)) => {
                let range: Ipv4Range = Ipv4Range::builder().first(*first).last(*last).try_into()?;
                range.into()
            }
            (IpAddr::V6(first), IpAddr::V6(last)) => {
                let range: Ipv6Range = Ipv6Range::builder().first(*first).last(*last).try_into()?;
                range.into()
            }
            _ => {
                return Err("first and last must either both be ipv4 or ipv6 addresses".to_string())
            }
        };

        *request = request.to_owned().body(range);

        Ok(())
    }
}

trait CommandExt {
    fn add_subcommand(self, path: &str, subcmd: impl Into<Command>) -> Self;
}

impl CommandExt for Command {
    fn add_subcommand(self, path: &str, subcmd: impl Into<Command>) -> Self {
        if let Some(index) = path.find(' ') {
            let first = &path[..index];
            let rest = &path[index + 1..];

            self.mut_subcommand(first, |cmd| cmd.add_subcommand(rest, subcmd))
        } else {
            self.subcommand(subcmd)
        }
    }
}

#[cfg(test)]
mod tests {
    use oxide_api::types::ByteCount;

    // This is the real trait that we're going to tell people about
    trait MyFromStr: Sized {
        type Err;
        fn my_from_str(value: &str) -> Result<Self, Self::Err>;
    }

    // This is the trait we implement by rote for a type that we are interested
    // in.
    // trait AutoRefFromStr: Sized {
    //     fn auto_ref_from_str(value: &str) -> Option<Self>;
    // }

    // Trait that **only** AutoRefFromStrTarget impls (twice).
    trait AutoRefFromStrTargetTrait<T> {
        fn auto_ref_from_str(self, value: &str) -> Option<T>;
    }

    // The struct that we'll either refer to directly or by "auto" reference.
    struct AutoRefTarget<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> AutoRefTarget<T> {
        fn new() -> Self {
            Self {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T: MyFromStr> AutoRefFromStrTargetTrait<T> for AutoRefTarget<T> {
        fn auto_ref_from_str(self, value: &str) -> Option<T> {
            T::my_from_str(value).ok()
        }
    }

    impl<T: std::str::FromStr> AutoRefFromStrTargetTrait<T> for &AutoRefTarget<T> {
        fn auto_ref_from_str(self, value: &str) -> Option<T> {
            T::from_str(value).ok()
        }
    }

    // this is the thing that may or may not exist
    impl MyFromStr for oxide_api::types::ByteCount {
        type Err = &'static str;

        fn my_from_str(_value: &str) -> Result<Self, Self::Err> {
            Ok(Self(42))
        }
    }

    #[test]
    fn test_autoref() {
        let y = {
            // this is what we're going to copy everywhere we use a type.
            AutoRefTarget::<ByteCount>::new().auto_ref_from_str("900")
        };
        println!("{:?}", y)
    }
}
