// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use std::{collections::BTreeMap, marker::PhantomData, path::PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use clap::{ArgMatches, Command, CommandFactory, FromArgMatches};

use crate::{
    config::Config,
    context::Context,
    generated_cli::{Cli, CliCommand},
    OxideOverride, RunnableCmd,
};

#[derive(clap::Parser, Debug, Clone)]
#[command(name = "oxide")]
pub struct OxideCli {
    #[clap(long)]
    debug: bool,

    #[clap(long)]
    config_dir: Option<PathBuf>,
}

#[async_trait]
trait RunIt: Send + Sync {
    async fn run_cmd(&self, matches: &ArgMatches, ctx: &Context) -> Result<()>;
    fn is_subtree(&self) -> bool;
}

#[derive(Default)]
struct CommandBuilder<'a> {
    children: BTreeMap<&'a str, CommandBuilder<'a>>,
    cmd: Option<Box<dyn RunIt>>,
    terminal: bool,
}

pub struct NewCli<'a> {
    parser: Command,
    runner: CommandBuilder<'a>,
}

impl<'a> Default for NewCli<'a> {
    fn default() -> Self {
        let mut parser = OxideCli::command().name("oxide").subcommand_required(true);
        // let mut parser = Command::new("oxide").subcommand_required(true);
        let mut runner = CommandBuilder::default();
        for op in CliCommand::iter() {
            let Some(path) = xxx(op) else {
                continue
            };
            runner.add_cmd(path, GeneratedCmd(op));

            let cmd = Cli::get_command(op);
            let cmd = match op {
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

                CliCommand::SamlIdentityProviderCreate => cmd
                    // We're filling in the missing required field so the full
                    // body is no longer required.
                    .mut_arg("json-body", |arg| arg.required(false))
                    .arg(
                        clap::Arg::new("metadata-url")
                            .long("metadata-url")
                            .value_name("url")
                            .value_parser(clap::value_parser!(String)),
                    )
                    .arg(
                        clap::Arg::new("metadata-value")
                            .long("metadata-value")
                            .value_name("xml")
                            .value_parser(clap::value_parser!(String)),
                    )
                    .group(
                        clap::ArgGroup::new("idp_metadata_source")
                            .args(["metadata-url", "metadata-value"])
                            .required(true)
                            .multiple(false),
                    ),

                // Command is fine as-is.
                _ => cmd,
            };

            parser = parser.add_subcommand(path, cmd);
            // print_cmd(&parser, 0);
        }

        Self { parser, runner }
    }
}

#[async_trait]
impl<C> RunIt for CustomCmd<C>
where
    C: Send + Sync + FromArgMatches + RunnableCmd,
{
    async fn run_cmd(&self, matches: &ArgMatches, ctx: &Context) -> Result<()> {
        let cmd = C::from_arg_matches(matches).unwrap();
        cmd.run(ctx).await
    }

    fn is_subtree(&self) -> bool {
        true
    }
}

impl<'a> NewCli<'a> {
    pub fn add_custom<N>(mut self, path: &'a str) -> Self
    where
        N: Send + Sync + FromArgMatches + RunnableCmd + CommandFactory + 'static,
    {
        self.runner.add_cmd(path, CustomCmd::<N>::new());
        self.parser = self.parser.add_subcommand(path, N::command());

        self
    }

    pub async fn run(self) -> Result<()> {
        let Self { parser, runner } = self;
        let matches = parser.get_matches();

        let top = OxideCli::from_arg_matches(&matches).unwrap();

        // Construct the global config. We do this **after** parsing options,
        // anticipating that top-level options may impact e.g. where we look for
        // config files.
        let config = Config::default();
        let ctx = Context::new(config).unwrap();

        let mut node = &runner;
        let mut sm = &matches;
        while let Some((sub_name, sub_matches)) = sm.subcommand() {
            node = node.children.get(sub_name).unwrap();
            sm = sub_matches;
            if node.terminal {
                break;
            }
        }
        node.cmd.as_ref().unwrap().run_cmd(sm, &ctx).await
    }

    pub fn command(&self) -> &Command {
        &self.parser
    }
}

impl<'a> CommandBuilder<'a> {
    pub fn add_cmd(&mut self, path: &'a str, cmd: impl RunIt + 'static) {
        let mut node = self;
        for component in path.split(' ') {
            node = node.children.entry(component).or_default();
        }
        node.terminal = cmd.is_subtree();
        node.cmd = Some(Box::new(cmd));
    }
}

struct GeneratedCmd(CliCommand);
#[async_trait]
impl RunIt for GeneratedCmd {
    async fn run_cmd(&self, matches: &ArgMatches, ctx: &Context) -> Result<()> {
        let cli = Cli::new_with_override(ctx.client().clone(), OxideOverride);
        cli.execute(self.0, matches).await;
        Ok(())
    }

    fn is_subtree(&self) -> bool {
        false
    }
}

struct CustomCmd<C> {
    _cmd: PhantomData<C>,
}

impl<C> CustomCmd<C> {
    pub fn new() -> Self {
        Self { _cmd: PhantomData }
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

trait CommandExt {
    fn add_subcommand(self, path: &str, subcmd: impl Into<Command>) -> Self;
}

impl CommandExt for Command {
    fn add_subcommand(self, path: &str, subcmd: impl Into<Command>) -> Self {
        if let Some(index) = path.find(' ') {
            let first = &path[..index];
            let rest = &path[index + 1..];

            let has_subcommand = self.find_subcommand(first).is_some();

            if has_subcommand {
                self.mut_subcommand(first, |cmd| cmd.add_subcommand(rest, subcmd))
            } else {
                self.subcommand(
                    Command::new(first.to_owned())
                        .subcommand_required(true)
                        .add_subcommand(rest, subcmd),
                )
            }
        } else {
            let new_subcmd = subcmd.into().name(path.to_owned());
            let has_subcommand = self.find_subcommand(path).is_some();
            if has_subcommand {
                self.mut_subcommand(path, |cmd| {
                    // Replace the subcommand, but retain its subcommands.
                    new_subcmd.subcommands(cmd.get_subcommands())
                })
            } else {
                self.subcommand(new_subcmd)
            }
        }
    }
}
