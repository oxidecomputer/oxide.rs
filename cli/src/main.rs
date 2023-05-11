// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use std::{collections::BTreeMap, marker::PhantomData, net::IpAddr};

use clap::{ArgMatches, Command, CommandFactory, FromArgMatches};

use anyhow::Result;
use async_trait::async_trait;
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

#[async_trait]
trait RunIt: Send + Sync {
    async fn run_cmd(&self, matches: &ArgMatches, ctx: Context) -> Result<()>;
    fn is_terminal(&self) -> bool {
        false
    }
}

#[derive(Default)]
struct Tree<'a> {
    children: BTreeMap<&'a str, Tree<'a>>,
    cmd: Option<CliCommand>,
    new_cmd: Option<Box<dyn RunIt>>,
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

impl<'a> NewCli<'a> {
    pub fn new() -> Self {
        let mut parser = Command::new("oxide").subcommand_required(true);
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

                // Command is fine as-is.
                _ => cmd,
            };

            parser = parser.add_subcommand(path, cmd);
            // print_cmd(&parser, 0);
        }

        Self { parser, runner }
    }

    pub fn add_custom<C>(mut self, path: &'a str) -> Self
    where
        C: Send + Sync + FromArgMatches + RunnableCmd + CommandFactory + 'static,
    {
        self.runner.add_cmd(path, CustomCmd::<C>::new());
        self.parser = self.parser.add_subcommand(path, C::command());

        // print_cmd(&self.parser, 0);
        self
    }

    pub async fn run(self, ctx: Context) -> Result<()> {
        let Self { parser, runner } = self;
        let matches = parser.get_matches();

        let mut node = &runner;
        let mut sm = &matches;
        while let Some((sub_name, sub_matches)) = sm.subcommand() {
            node = node.children.get(sub_name).unwrap();
            sm = sub_matches;
            if node.terminal {
                break;
            }
        }
        node.cmd.as_ref().unwrap().run_cmd(sm, ctx).await
    }

    pub fn command(&self) -> &Command {
        &self.parser
    }
}

fn print_cmd(cmd: &Command, indent: usize) {
    println!("{:width$} {}", "", cmd.get_name(), width = indent);
    cmd.get_subcommands()
        .for_each(|subcmd| print_cmd(subcmd, indent + 2));
}

impl<'a> CommandBuilder<'a> {
    fn print(&self, indent: usize) {
        self.children.iter().for_each(|(key, value)| {
            println!("{:width$} {}", "", key, width = indent);
            value.print(indent + 2);
        });
    }
    pub fn add_cmd(&mut self, path: &'a str, cmd: impl RunIt + 'static) {
        let mut node = self;
        let mut components = path.split(' ').peekable();
        while let (Some(component), next) = (components.next(), components.peek()) {
            if next.is_some() {
                node = node.children.entry(component).or_default();
            } else {
                let x = node.children.entry(component).or_default();
                x.terminal = cmd.is_terminal();
                x.cmd = Some(Box::new(cmd));
                // assert!(
                //     node.children.get(component).is_none(),
                //     "two identical subcommands {}",
                //     path,
                // );
                // node.children.insert(
                //     component,
                //     CommandBuilder {
                //         children: Default::default(),
                //         cmd: Some(Box::new(cmd)),
                //     },
                // );
                break;
            }
        }
        // loop {
        //     let Some(component) = components.next() else {panic!()};
        //     if components.peek().is_some() {
        //         node = node.children.entry(component).or_default();
        //     } else {
        //         assert!(
        //             node.children.get(component).is_none(),
        //             "two identical subcommands {}",
        //             path,
        //         );
        //         node.children.insert(
        //             component,
        //             CommandBuilder {
        //                 children: Default::default(),
        //                 cmd: Some(Box::new(cmd)),
        //             },
        //         );
        //         break;
        //     }
        // }
    }

    pub async fn run(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}

struct GeneratedCmd(CliCommand);
#[async_trait]
impl RunIt for GeneratedCmd {
    async fn run_cmd(&self, matches: &ArgMatches, ctx: Context) -> Result<()> {
        let cli = Cli::new_with_override(ctx.client().clone(), OxideOverride);
        cli.execute(self.0, matches).await;
        Ok(())
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

#[async_trait]
pub trait RunnableCmd: Send + Sync {
    async fn run(&self, ctx: Context) -> Result<()>;
}

#[async_trait]
impl<C> RunIt for CustomCmd<C>
where
    C: Send + Sync + FromArgMatches + RunnableCmd,
{
    async fn run_cmd(&self, matches: &ArgMatches, ctx: Context) -> Result<()> {
        let cmd = C::from_arg_matches(matches).unwrap();
        cmd.run(ctx).await
    }

    fn is_terminal(&self) -> bool {
        true
    }
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

pub fn make_cli() -> NewCli<'static> {
    NewCli::new()
        .add_custom::<cmd_auth::CmdAuth>("auth")
        .add_custom::<cmd_api::CmdApi>("api")
        .add_custom::<cmd_docs::CmdDocs>("docs")
        .add_custom::<cmd_version::CmdVersion>("version")
        .add_custom::<cmd_disk::CmdDiskImport>("disk import")
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // let mut root = Tree::default();
    // for op in CliCommand::iter() {
    //     let mut node = &mut root;
    //     let Some(subcmd_str) = xxx(op) else {
    //         continue
    //     };

    //     let mut path = subcmd_str.split(' ').peekable();
    //     while let Some(ss) = path.next() {
    //         if path.peek().is_some() {
    //             node = node.children.entry(ss).or_default();
    //         } else {
    //             assert!(
    //                 node.children.get(ss).is_none(),
    //                 "two identical subcommands {}",
    //                 subcmd_str,
    //             );
    //             node.children.insert(
    //                 ss,
    //                 Tree {
    //                     cmd: Some(op),
    //                     ..Default::default()
    //                 },
    //             );
    //         }
    //     }
    // }

    // let cmd = root
    //     .cmd("oxide")
    //     .bin_name("oxide")
    //     .subcommand(cmd_auth::CmdAuth::command())
    //     .subcommand(cmd_api::CmdApi::command())
    //     .subcommand(cmd_docs::CmdDocs::command())
    //     .subcommand(cmd_version::CmdVersion::command())
    //     .add_subcommand("disk import", cmd_disk::CmdDiskImport::command());

    // let matches = cmd.clone().get_matches();

    // XXX
    let new_cli = make_cli();

    // Construct the global config. We do this **after** parsing options,
    // anticipating that top-level options may impact e.g. where we look for
    // config files.
    let config = Config::default();
    let ctx = Context::new(config).unwrap();

    let result = new_cli.run(ctx).await;
    if let Err(e) = result {
        println!("error: {}", e);
        std::process::exit(1)
    }

    // let result = match matches.subcommand() {
    //     Some(("api", sub_matches)) => {
    //         cmd_api::CmdApi::from_arg_matches(sub_matches)
    //             .unwrap()
    //             .run(&mut ctx)
    //             .await
    //     }

    //     Some(("auth", sub_matches)) => {
    //         cmd_auth::CmdAuth::from_arg_matches(sub_matches)
    //             .unwrap()
    //             .run(&mut ctx)
    //             .await
    //     }

    //     // TODO this could be improved
    //     Some(("disk", sub_matches)) if sub_matches.subcommand_name() == Some("import") => {
    //         cmd_disk::CmdDiskImport::from_arg_matches(sub_matches)
    //             .unwrap()
    //             .run(&mut ctx)
    //             .await
    //     }

    //     Some(("docs", sub_matches)) => {
    //         cmd_docs::CmdDocs::from_arg_matches(sub_matches)
    //             .unwrap()
    //             .run(&cmd)
    //             .await
    //     }

    //     Some(("version", sub_matches)) => {
    //         cmd_version::CmdVersion::from_arg_matches(sub_matches)
    //             .unwrap()
    //             .run()
    //             .await
    //     }

    //     _ => {
    //         // Spawn a task so we get this potentially chunky Future off the
    //         // main thread's stack.
    //         tokio::spawn(async move {
    //             let mut node = &root;
    //             let mut sm = &matches;

    //             while let Some((sub_name, sub_matches)) = sm.subcommand() {
    //                 node = node.children.get(sub_name).unwrap();
    //                 sm = sub_matches;
    //             }

    //             let cli = Cli::new_with_override(ctx.client().clone(), OxideOverride);

    //             // TODO error handling
    //             cli.execute(node.cmd.unwrap(), sm).await;
    //         })
    //         .await
    //         .unwrap();
    //         Ok(())
    //     }
    // };

    // if let Err(e) = result {
    //     println!("error: {}", e);
    //     std::process::exit(1)
    // }
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

        CliCommand::DiskMetricsList => Some("disk metrics list"),
        CliCommand::ImageList => Some("image list"),
        CliCommand::ImageCreate => Some("image create"),
        CliCommand::ImageView => Some("image view"),
        CliCommand::ImageDelete => Some("image delete"),
        CliCommand::ImagePromote => Some("image promote"),

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
        CliCommand::PhysicalDiskList => Some("physical-disk list"),
        CliCommand::RackList => Some("rack list"),
        CliCommand::RackView => Some("rack view"),
        CliCommand::SledList => Some("sled list"),
        CliCommand::SledView => Some("sled view"),
        CliCommand::SledPhysicalDiskList => Some("sled disk-led"),
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
            let has_subcommand = self.find_subcommand(path).is_some();
            if has_subcommand {
                self.mut_subcommand(path, |cmd| {
                    subcmd
                        .into()
                        .name(path.to_owned())
                        .subcommands(cmd.get_subcommands().cloned())
                })
            } else {
                self.subcommand(subcmd.into().name(path.to_owned()))
            }
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
