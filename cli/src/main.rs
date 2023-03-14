use std::collections::HashMap;

use clap::{Command, CommandFactory, FromArgMatches};

use config::Config;
use context::Context;
use generated_cli::{Cli, CliCommand};

mod cmd_api;
#[allow(unused_mut)] // TODO
#[allow(unused)] // TODO
mod cmd_auth;
mod config;
mod context;
#[allow(unused_mut)]
#[allow(unused)] // TODO
#[allow(unused_must_use)] // TODO
mod generated_cli;

#[derive(Debug, Default)]
struct Tree<'a> {
    children: HashMap<&'a str, Tree<'a>>,
    cmd: Option<CliCommand>,
}

impl<'a> Tree<'a> {
    fn cmd(&self, name: &str) -> Command {
        let mut cmd = if let Some(op) = self.cmd {
            // Command node
            Cli::get_command(op).name(name.to_owned())
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
                assert!(node.children.get(ss).is_none());
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

    let mut cmd = root.cmd("oxide");
    cmd = cmd.bin_name("oxide");

    // TODO Example of how to build a fully custom sub-command. Note that this
    // could be placed under another subcommand if needed.
    cmd = cmd.subcommand(cmd_auth::CmdAuth::command());
    cmd = cmd.subcommand(cmd_api::CmdApi::command());

    let matches = cmd.get_matches();

    // Construct the global config
    // TODO I think this has to come between parsing and execution in that the
    // parsed options may change where we get config from.
    let config = Config::default();
    let mut ctx = Context::new(config).unwrap();

    match matches.subcommand() {
        Some(("auth", sub_matches)) => {
            let x = cmd_auth::CmdAuth::from_arg_matches(sub_matches).unwrap();
            x.run(&mut ctx).await.unwrap();
        }

        Some(("api", sub_matches)) => {
            let x = cmd_api::CmdApi::from_arg_matches(sub_matches).unwrap();
            x.run(&mut ctx).await.unwrap();
        }

        _ => {
            let mut node = &root;
            let mut sm = &matches;

            while let Some((sub_name, sub_matches)) = sm.subcommand() {
                node = node.children.get(sub_name).unwrap();
                sm = sub_matches;
            }

            let cli = Cli::new(ctx.client.clone());

            cli.execute(node.cmd.unwrap(), sm).await;
        }
    }
}

fn xxx<'a>(command: CliCommand) -> Option<&'a str> {
    let x = match command {
        CliCommand::InstanceListV1 => Some("instance list"),
        CliCommand::InstanceCreateV1 => Some("instance create"),
        CliCommand::InstanceViewV1 => Some("instance view"),
        CliCommand::InstanceDeleteV1 => Some("instance delete"),
        CliCommand::InstanceMigrateV1 => None, // TODO delete from API?
        CliCommand::InstanceRebootV1 => Some("instance reboot"),
        CliCommand::InstanceSerialConsoleV1 => None, // TODO not sure how to handle this
        CliCommand::InstanceSerialConsoleStreamV1 => None, // Ditto
        CliCommand::InstanceStartV1 => Some("instance start"),
        CliCommand::InstanceStopV1 => Some("instance stop"),

        CliCommand::OrganizationListV1 => Some("org list"),
        CliCommand::OrganizationCreateV1 => Some("org create"),
        CliCommand::OrganizationViewV1 => Some("org view"),
        CliCommand::OrganizationUpdateV1 => Some("org update"),
        CliCommand::OrganizationDeleteV1 => Some("org delete"),
        CliCommand::OrganizationPolicyViewV1 => Some("org policy view"),
        CliCommand::OrganizationPolicyUpdateV1 => Some("org policy update"),

        CliCommand::ProjectListV1 => Some("project list"),
        CliCommand::ProjectCreateV1 => Some("project create"),
        CliCommand::ProjectViewV1 => Some("project view"),
        CliCommand::ProjectUpdateV1 => Some("project update"),
        CliCommand::ProjectDeleteV1 => Some("project delete"),
        CliCommand::ProjectPolicyViewV1 => Some("project policy view"),
        CliCommand::ProjectPolicyUpdateV1 => Some("project policy update"),

        // TODO
        CliCommand::DiskViewById
        | CliCommand::ImageViewById
        | CliCommand::InstanceViewById
        | CliCommand::InstanceNetworkInterfaceViewById
        | CliCommand::OrganizationViewById
        | CliCommand::ProjectViewById
        | CliCommand::SnapshotViewById
        | CliCommand::VpcRouterRouteViewById
        | CliCommand::VpcRouterViewById
        | CliCommand::VpcSubnetViewById
        | CliCommand::VpcViewById
        | CliCommand::DeviceAuthRequest
        | CliCommand::DeviceAuthConfirm
        | CliCommand::DeviceAccessToken
        | CliCommand::GroupList
        | CliCommand::LoginSpoof
        | CliCommand::LoginLocal
        | CliCommand::LoginSamlBegin
        | CliCommand::LoginSaml
        | CliCommand::Logout
        | CliCommand::OrganizationList
        | CliCommand::OrganizationCreate
        | CliCommand::OrganizationView
        | CliCommand::OrganizationUpdate
        | CliCommand::OrganizationDelete
        | CliCommand::OrganizationPolicyView
        | CliCommand::OrganizationPolicyUpdate
        | CliCommand::ProjectList
        | CliCommand::ProjectCreate
        | CliCommand::ProjectView
        | CliCommand::ProjectUpdate
        | CliCommand::ProjectDelete
        | CliCommand::DiskList
        | CliCommand::DiskCreate
        | CliCommand::DiskView
        | CliCommand::DiskDelete
        | CliCommand::DiskMetricsList
        | CliCommand::ImageList
        | CliCommand::ImageCreate
        | CliCommand::ImageView
        | CliCommand::ImageDelete
        | CliCommand::InstanceList
        | CliCommand::InstanceCreate
        | CliCommand::InstanceView
        | CliCommand::InstanceDelete
        | CliCommand::InstanceDiskList
        | CliCommand::InstanceDiskAttach
        | CliCommand::InstanceDiskDetach
        | CliCommand::InstanceExternalIpList
        | CliCommand::InstanceMigrate
        | CliCommand::InstanceNetworkInterfaceList
        | CliCommand::InstanceNetworkInterfaceCreate
        | CliCommand::InstanceNetworkInterfaceView
        | CliCommand::InstanceNetworkInterfaceUpdate
        | CliCommand::InstanceNetworkInterfaceDelete
        | CliCommand::InstanceReboot
        | CliCommand::InstanceSerialConsole
        | CliCommand::InstanceSerialConsoleStream
        | CliCommand::InstanceStart
        | CliCommand::InstanceStop
        | CliCommand::ProjectPolicyView
        | CliCommand::ProjectPolicyUpdate
        | CliCommand::SnapshotList
        | CliCommand::SnapshotCreate
        | CliCommand::SnapshotView
        | CliCommand::SnapshotDelete
        | CliCommand::VpcList
        | CliCommand::VpcCreate
        | CliCommand::VpcView
        | CliCommand::VpcUpdate
        | CliCommand::VpcDelete
        | CliCommand::VpcFirewallRulesView
        | CliCommand::VpcFirewallRulesUpdate
        | CliCommand::VpcRouterList
        | CliCommand::VpcRouterCreate
        | CliCommand::VpcRouterView
        | CliCommand::VpcRouterUpdate
        | CliCommand::VpcRouterDelete
        | CliCommand::VpcRouterRouteList
        | CliCommand::VpcRouterRouteCreate
        | CliCommand::VpcRouterRouteView
        | CliCommand::VpcRouterRouteUpdate
        | CliCommand::VpcRouterRouteDelete
        | CliCommand::VpcSubnetList
        | CliCommand::VpcSubnetCreate
        | CliCommand::VpcSubnetView
        | CliCommand::VpcSubnetUpdate
        | CliCommand::VpcSubnetDelete
        | CliCommand::VpcSubnetListNetworkInterfaces
        | CliCommand::PolicyView
        | CliCommand::PolicyUpdate
        | CliCommand::RoleList
        | CliCommand::RoleView
        | CliCommand::SessionMe
        | CliCommand::SessionMeGroups
        | CliCommand::SessionSshkeyList
        | CliCommand::SessionSshkeyCreate
        | CliCommand::SessionSshkeyView
        | CliCommand::SessionSshkeyDelete
        | CliCommand::SystemImageViewById
        | CliCommand::IpPoolViewById
        | CliCommand::SiloViewById
        | CliCommand::RackList
        | CliCommand::RackView
        | CliCommand::SledList
        | CliCommand::SledView
        | CliCommand::SystemImageList
        | CliCommand::SystemImageCreate
        | CliCommand::SystemImageView
        | CliCommand::SystemImageDelete
        | CliCommand::IpPoolList
        | CliCommand::IpPoolCreate
        | CliCommand::IpPoolView
        | CliCommand::IpPoolUpdate
        | CliCommand::IpPoolDelete
        | CliCommand::IpPoolRangeList
        | CliCommand::IpPoolRangeAdd
        | CliCommand::IpPoolRangeRemove
        | CliCommand::IpPoolServiceView
        | CliCommand::IpPoolServiceRangeList
        | CliCommand::IpPoolServiceRangeAdd
        | CliCommand::IpPoolServiceRangeRemove
        | CliCommand::SystemPolicyView
        | CliCommand::SystemPolicyUpdate
        | CliCommand::SagaList
        | CliCommand::SagaView
        | CliCommand::SiloList
        | CliCommand::SiloCreate
        | CliCommand::SiloView
        | CliCommand::SiloDelete
        | CliCommand::SiloIdentityProviderList
        | CliCommand::LocalIdpUserCreate
        | CliCommand::LocalIdpUserDelete
        | CliCommand::LocalIdpUserSetPassword
        | CliCommand::SamlIdentityProviderCreate
        | CliCommand::SamlIdentityProviderView
        | CliCommand::SiloPolicyView
        | CliCommand::SiloPolicyUpdate
        | CliCommand::SiloUsersList
        | CliCommand::SiloUserView
        | CliCommand::SystemUserList
        | CliCommand::SystemUserView
        | CliCommand::TimeseriesSchemaGet
        | CliCommand::UserList
        | CliCommand::CertificateList
        | CliCommand::CertificateCreate
        | CliCommand::CertificateView
        | CliCommand::CertificateDelete
        | CliCommand::PhysicalDiskList
        | CliCommand::SledPhysicalDiskList
        | CliCommand::SystemMetric
        | CliCommand::DiskListV1
        | CliCommand::DiskCreateV1
        | CliCommand::DiskViewV1
        | CliCommand::DiskDeleteV1
        | CliCommand::GroupListV1
        | CliCommand::GroupView
        | CliCommand::InstanceDiskListV1
        | CliCommand::InstanceDiskAttachV1
        | CliCommand::InstanceDiskDetachV1
        | CliCommand::InstanceNetworkInterfaceListV1
        | CliCommand::InstanceNetworkInterfaceCreateV1
        | CliCommand::InstanceNetworkInterfaceViewV1
        | CliCommand::InstanceNetworkInterfaceUpdateV1
        | CliCommand::InstanceNetworkInterfaceDeleteV1
        | CliCommand::PolicyViewV1
        | CliCommand::PolicyUpdateV1
        | CliCommand::SnapshotListV1
        | CliCommand::SnapshotCreateV1
        | CliCommand::SnapshotViewV1
        | CliCommand::SnapshotDeleteV1
        | CliCommand::CertificateListV1
        | CliCommand::CertificateCreateV1
        | CliCommand::CertificateViewV1
        | CliCommand::CertificateDeleteV1
        | CliCommand::PhysicalDiskListV1
        | CliCommand::RackListV1
        | CliCommand::RackViewV1
        | CliCommand::SledListV1
        | CliCommand::SledViewV1
        | CliCommand::SledPhysicalDiskListV1
        | CliCommand::SystemPolicyViewV1
        | CliCommand::SystemPolicyUpdateV1
        | CliCommand::SagaListV1
        | CliCommand::SagaViewV1
        | CliCommand::SystemComponentVersionList
        | CliCommand::UpdateDeploymentsList
        | CliCommand::UpdateDeploymentView
        | CliCommand::SystemUpdateRefresh
        | CliCommand::SystemUpdateStart
        | CliCommand::SystemUpdateStop
        | CliCommand::SystemUpdateList
        | CliCommand::SystemUpdateView
        | CliCommand::SystemUpdateComponentsList
        | CliCommand::SystemVersion
        | CliCommand::UserListV1
        | CliCommand::VpcFirewallRulesViewV1
        | CliCommand::VpcFirewallRulesUpdateV1
        | CliCommand::VpcRouterRouteListV1
        | CliCommand::VpcRouterRouteCreateV1
        | CliCommand::VpcRouterRouteViewV1
        | CliCommand::VpcRouterRouteUpdateV1
        | CliCommand::VpcRouterRouteDeleteV1
        | CliCommand::VpcRouterListV1
        | CliCommand::VpcRouterCreateV1
        | CliCommand::VpcRouterViewV1
        | CliCommand::VpcRouterUpdateV1
        | CliCommand::VpcRouterDeleteV1
        | CliCommand::VpcSubnetListV1
        | CliCommand::VpcSubnetCreateV1
        | CliCommand::VpcSubnetViewV1
        | CliCommand::VpcSubnetUpdateV1
        | CliCommand::VpcSubnetDeleteV1
        | CliCommand::VpcListV1
        | CliCommand::VpcCreateV1
        | CliCommand::VpcViewV1
        | CliCommand::VpcUpdateV1
        | CliCommand::VpcDeleteV1 => None,
    };

    x
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
