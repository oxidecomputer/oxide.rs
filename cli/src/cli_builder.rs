// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use std::{any::TypeId, collections::BTreeMap, marker::PhantomData, net::IpAddr, path::PathBuf};

use anyhow::{bail, Result};
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command, CommandFactory, FromArgMatches};
use tracing_subscriber::EnvFilter;

use crate::{
    context::Context,
    generated_cli::{Cli, CliCommand},
    OxideOverride, RunnableCmd,
};
use oxide::{types::ByteCount, ClientConfig};

/// Control an Oxide environment
#[derive(clap::Parser, Debug, Clone)]
#[command(name = "oxide", verbatim_doc_comment)]
struct OxideCli {
    /// Enable debug output
    #[clap(long)]
    pub debug: bool,

    /// Configuration profile to use for commands
    #[clap(long, global = true, help_heading = "Global Options")]
    pub profile: Option<String>,

    /// Directory to use for configuration
    #[clap(long, value_name = "DIR")]
    pub config_dir: Option<PathBuf>,

    /// Modify name resolution
    #[clap(long, value_name = "HOST:PORT:ADDR")]
    pub resolve: Option<ResolveValue>,

    /// Specify a trusted CA cert
    #[clap(long, value_name = "FILE")]
    pub cacert: Option<PathBuf>,

    /// Disable certificate validation and hostname verification
    #[clap(long)]
    pub insecure: bool,

    /// Timeout value for individual API invocations
    #[clap(long, value_name = "SECONDS")]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolveValue {
    pub host: String,
    pub port: u16,
    pub addr: IpAddr,
}

impl std::str::FromStr for ResolveValue {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let values = s.splitn(3, ':').collect::<Vec<_>>();
        let [host, port, addr] = values.as_slice() else {
            return Err(r#"value must be "host:port:addr"#.to_string());
        };

        let host = host.to_string();
        let port = port
            .parse()
            .map_err(|_| format!("error parsing port '{}'", port))?;

        // `IpAddr::parse()` does not accept enclosing brackets on IPv6
        // addresses; strip them off if they exist.
        let addr = addr
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap_or(addr);
        let addr = addr
            .parse()
            .map_err(|_| format!("error parsing address '{}'", addr))?;

        Ok(Self { host, port, addr })
    }
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

impl Default for NewCli<'_> {
    fn default() -> Self {
        let mut parser = OxideCli::command().name("oxide").subcommand_required(true);
        let mut runner = CommandBuilder::default();
        for op in CliCommand::iter() {
            let Some(path) = xxx(op) else { continue };
            runner.add_cmd(path, GeneratedCmd(op));

            let cmd = Cli::<OxideOverride>::get_command(op);
            let cmd = match op {
                CliCommand::IpPoolRangeAdd
                | CliCommand::IpPoolRangeRemove
                | CliCommand::IpPoolServiceRangeAdd
                | CliCommand::IpPoolServiceRangeRemove => cmd
                    .mut_arg("json-body", |arg| arg.required(false))
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
                    .mut_arg("json-body", |arg| arg.required(false))
                    .arg(
                        clap::Arg::new("metadata-url")
                            .long("metadata-url")
                            .value_name("url")
                            .value_parser(clap::value_parser!(String))
                            .help("the URL of an identity provider metadata descriptor"),
                    )
                    .arg(
                        clap::Arg::new("metadata-value")
                            .long("metadata-value")
                            .value_name("xml-file")
                            .value_parser(clap::value_parser!(PathBuf))
                            .help("path to an XML file containing an identity provider metadata descriptor"),
                    )
                    .arg(
                        clap::Arg::new("private-key")
                            .long("private-key")
                            .value_name("key-file")
                            .value_parser(clap::value_parser!(PathBuf))
                            .help("path to the request signing RSA private key in PKCS#1 DER format"),
                    )
                    .arg(
                        clap::Arg::new("public-cert")
                            .long("public-cert")
                            .value_name("cert-file")
                            .value_parser(clap::value_parser!(PathBuf))
                            .help("path to the request signing public certificate in DER format"),
                    )
                    .group(
                        clap::ArgGroup::new("signing_keypair")
                            .args(["private-key", "public-cert"])
                            .requires_all(["private-key", "public-cert"])
                            .multiple(true),
                    )
                    .group(
                        clap::ArgGroup::new("idp_metadata_source")
                            .args(["metadata-url", "metadata-value"])
                            .required(true)
                            .multiple(false),
                    ),

                CliCommand::NetworkingAllowListUpdate => cmd
                    .mut_arg("json-body", |arg| arg.required(false))
                    .arg(
                        clap::Arg::new("any")
                            .long("any")
                            .action(clap::ArgAction::SetTrue)
                            .value_parser(clap::value_parser!(bool)),
                    )
                    .arg(
                        clap::Arg::new("ips")
                            .long("ip")
                            .action(clap::ArgAction::Append)
                            .value_name("IP or IPNET")
                            .value_parser(clap::value_parser!(crate::IpOrNet)),
                    )
                    .group(
                        clap::ArgGroup::new("allow-list")
                            .args(["ips", "any"])
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
        <C as RunnableCmd>::is_subtree()
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

        let OxideCli {
            profile,
            debug,
            config_dir,
            resolve,
            cacert,
            insecure,
            timeout,
        } = OxideCli::from_arg_matches(&matches).expect("failed to parse OxideCli from args");

        let env_filter = if debug {
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("oxide=debug"))
        } else {
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
        };

        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_writer(std::io::stderr)
            .json()
            .flatten_event(true)
            .with_current_span(false)
            .with_span_list(false)
            .init();

        let mut client_config = ClientConfig::default();

        if let Some(profile_name) = profile {
            client_config = client_config.with_profile(profile_name);
        }
        if let Some(config_dir) = config_dir {
            client_config = client_config.with_config_dir(config_dir);
        }
        if let Some(resolve) = resolve {
            client_config = client_config.with_resolve(resolve.host, resolve.addr);
        }
        if let Some(cacert_path) = cacert {
            let extension = cacert_path
                .extension()
                .map(std::ffi::OsStr::to_ascii_lowercase);
            let contents = std::fs::read(cacert_path)?;
            let cert = match extension.as_ref().and_then(|ex| ex.to_str()) {
                Some("pem") => reqwest::tls::Certificate::from_pem(&contents),
                Some("der") => reqwest::tls::Certificate::from_der(&contents),
                _ => bail!("--cacert path must be a 'pem' or 'der' file".to_string()),
            }?;

            client_config = client_config.with_cert(cert);
        }
        client_config = client_config.with_insecure(insecure);
        if let Some(timeout) = timeout {
            client_config = client_config.with_timeout(timeout);
        }

        // Set longer timeouts for potentially slow support-bundle subcommands.
        if matches.subcommand_matches("bundle").is_some_and(|bundle| {
            matches!(bundle.subcommand_name(), Some("download") | Some("inspect"))
        }) {
            // Keep a reasonable timeout for initial connection.
            client_config = client_config.with_connect_timeout(15);

            // Bundles may be tens of gigabytes, set a one hour timeout by default.
            if timeout.is_none() {
                client_config = client_config.with_timeout(60 * 60);
            }

            // Kill the connection if we stop receiving data before the connection timeout.
            client_config = client_config.with_read_timeout(30);
        }

        let ctx = Context::new(client_config)?;

        let mut node = &runner;
        let mut sm = &matches;
        while let Some((sub_name, sub_matches)) = sm.subcommand() {
            node = node
                .children
                .get(sub_name)
                .expect("child subcommand not found");
            sm = sub_matches;
            if node.terminal {
                break;
            }
        }
        node.cmd
            .as_ref()
            .expect("no cmd for node")
            .run_cmd(sm, &ctx)
            .await
    }

    pub fn command(&self) -> &Command {
        &self.parser
    }

    pub fn command_take(self) -> Command {
        self.parser
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
        let client = oxide::Client::new_authenticated_config(ctx.client_config())?;
        let cli = Cli::new(client, OxideOverride::default());
        cli.execute(self.0, matches).await
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
        CliCommand::InstanceUpdate => Some("instance update"),
        CliCommand::InstanceDelete => Some("instance delete"),
        CliCommand::InstanceReboot => Some("instance reboot"),
        CliCommand::InstanceSerialConsole => None, // Special-cased
        CliCommand::InstanceSerialConsoleStream => None, // Ditto
        CliCommand::InstanceStart => Some("instance start"),
        CliCommand::InstanceStop => Some("instance stop"),
        CliCommand::InstanceExternalIpList => Some("instance external-ip list"),
        CliCommand::InstanceEphemeralIpAttach => Some("instance external-ip attach-ephemeral"),
        CliCommand::InstanceEphemeralIpDetach => Some("instance external-ip detach-ephemeral"),

        // Properties of the instance that--for one reason or another--are not
        // available in the default "view" object.
        CliCommand::InstanceSshPublicKeyList => Some("instance property ssh-key"),
        CliCommand::InstanceAffinityGroupList => Some("instance property affinity"),
        CliCommand::InstanceAntiAffinityGroupList => Some("instance property anti-affinity"),

        CliCommand::AffinityGroupList => Some("experimental affinity list"),
        CliCommand::AffinityGroupCreate => Some("experimental affinity create"),
        CliCommand::AffinityGroupView => Some("experimental affinity view"),
        CliCommand::AffinityGroupUpdate => Some("experimental affinity update"),
        CliCommand::AffinityGroupDelete => Some("experimental affinity delete"),
        CliCommand::AffinityGroupMemberList => Some("experimental affinity member list"),
        CliCommand::AffinityGroupMemberInstanceView => {
            Some("experimental affinity member view-instance")
        }
        CliCommand::AffinityGroupMemberInstanceAdd => {
            Some("experimental affinity member add-instance")
        }
        CliCommand::AffinityGroupMemberInstanceDelete => {
            Some("experimental affinity member remove-instance")
        }
        CliCommand::AntiAffinityGroupList => Some("instance anti-affinity list"),
        CliCommand::AntiAffinityGroupCreate => Some("instance anti-affinity create"),
        CliCommand::AntiAffinityGroupView => Some("instance anti-affinity view"),
        CliCommand::AntiAffinityGroupUpdate => Some("instance anti-affinity update"),
        CliCommand::AntiAffinityGroupDelete => Some("instance anti-affinity delete"),
        CliCommand::AntiAffinityGroupMemberList => Some("instance anti-affinity member list"),
        CliCommand::AntiAffinityGroupMemberInstanceView => {
            Some("instance anti-affinity member view-instance")
        }
        CliCommand::AntiAffinityGroupMemberInstanceAdd => {
            Some("instance anti-affinity member add-instance")
        }
        CliCommand::AntiAffinityGroupMemberInstanceDelete => {
            Some("instance anti-affinity member remove-instance")
        }

        CliCommand::ProjectList => Some("project list"),
        CliCommand::ProjectCreate => Some("project create"),
        CliCommand::ProjectView => Some("project view"),
        CliCommand::ProjectUpdate => Some("project update"),
        CliCommand::ProjectDelete => Some("project delete"),
        CliCommand::ProjectPolicyView => Some("project policy view"),
        CliCommand::ProjectPolicyUpdate => Some("project policy update"),
        CliCommand::ProjectIpPoolList => Some("project ip-pool list"),
        CliCommand::ProjectIpPoolView => Some("project ip-pool view"),

        CliCommand::ImageList => Some("image list"),
        CliCommand::ImageCreate => Some("image create"),
        CliCommand::ImageView => Some("image view"),
        CliCommand::ImageDelete => Some("image delete"),
        CliCommand::ImagePromote => Some("image promote"),
        CliCommand::ImageDemote => Some("image demote"),

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
        CliCommand::IpPoolSiloList => Some("ip-pool silo list"),
        CliCommand::IpPoolSiloLink => Some("ip-pool silo link"),
        CliCommand::IpPoolSiloUpdate => Some("ip-pool silo update"),
        CliCommand::IpPoolSiloUnlink => Some("ip-pool silo unlink"),
        CliCommand::IpPoolUtilizationView => Some("ip-pool utilization"),

        CliCommand::SiloList => Some("silo list"),
        CliCommand::SiloCreate => Some("silo create"),
        CliCommand::SiloView => Some("silo view"),
        CliCommand::SiloDelete => Some("silo delete"),

        CliCommand::SiloPolicyView => Some("silo policy view"),
        CliCommand::SiloPolicyUpdate => Some("silo policy update"),
        CliCommand::SiloUserList => Some("silo user list"),
        CliCommand::SiloUserView => Some("silo user view"),

        CliCommand::SiloIdentityProviderList => Some("silo idp list"),
        CliCommand::LocalIdpUserCreate => Some("silo idp local user create"),
        CliCommand::LocalIdpUserDelete => Some("silo idp local user delete"),
        CliCommand::LocalIdpUserSetPassword => Some("silo idp local user set-password"),
        CliCommand::SamlIdentityProviderCreate => Some("silo idp saml create"),
        CliCommand::SamlIdentityProviderView => Some("silo idp saml view"),

        CliCommand::SystemQuotasList => Some("silo quotas list"),
        CliCommand::SiloQuotasView => Some("silo quotas view"),
        CliCommand::SiloQuotasUpdate => Some("silo quotas update"),
        CliCommand::SiloUtilizationList => Some("silo utilization list"),
        CliCommand::SiloUtilizationView => Some("silo utilization view"),
        CliCommand::SiloIpPoolList => Some("silo ip-pool list"),

        CliCommand::UtilizationView => Some("utilization"),
        CliCommand::UserList => Some("user list"),

        // VPCs
        CliCommand::VpcList => Some("vpc list"),
        CliCommand::VpcCreate => Some("vpc create"),
        CliCommand::VpcView => Some("vpc view"),
        CliCommand::VpcUpdate => Some("vpc update"),
        CliCommand::VpcDelete => Some("vpc delete"),

        CliCommand::VpcFirewallRulesView => Some("vpc firewall-rules view"),
        CliCommand::VpcFirewallRulesUpdate => Some("vpc firewall-rules update"),

        CliCommand::VpcSubnetList => Some("vpc subnet list"),
        CliCommand::VpcSubnetCreate => Some("vpc subnet create"),
        CliCommand::VpcSubnetView => Some("vpc subnet view"),
        CliCommand::VpcSubnetUpdate => Some("vpc subnet update"),
        CliCommand::VpcSubnetDelete => Some("vpc subnet delete"),
        CliCommand::VpcSubnetListNetworkInterfaces => Some("vpc subnet nic list"),

        CliCommand::VpcRouterRouteList => Some("vpc router route list"),
        CliCommand::VpcRouterRouteCreate => Some("vpc router route create"),
        CliCommand::VpcRouterRouteView => Some("vpc router route view"),
        CliCommand::VpcRouterRouteUpdate => Some("vpc router route update"),
        CliCommand::VpcRouterRouteDelete => Some("vpc router route delete"),
        CliCommand::VpcRouterList => Some("vpc router list"),
        CliCommand::VpcRouterCreate => Some("vpc router create"),
        CliCommand::VpcRouterView => Some("vpc router view"),
        CliCommand::VpcRouterUpdate => Some("vpc router update"),
        CliCommand::VpcRouterDelete => Some("vpc router delete"),

        CliCommand::InternetGatewayIpAddressList => Some("internet-gateway address list"),
        CliCommand::InternetGatewayIpAddressCreate => Some("internet-gateway address create"),
        CliCommand::InternetGatewayIpAddressDelete => Some("internet-gateway address delete"),
        CliCommand::InternetGatewayIpPoolList => Some("internet-gateway ip-pool list"),
        CliCommand::InternetGatewayIpPoolCreate => Some("internet-gateway ip-pool attach"),
        CliCommand::InternetGatewayIpPoolDelete => Some("internet-gateway ip-pool detach"),
        CliCommand::InternetGatewayList => Some("internet-gateway list"),
        CliCommand::InternetGatewayCreate => Some("internet-gateway create"),
        CliCommand::InternetGatewayView => Some("internet-gateway view"),
        CliCommand::InternetGatewayDelete => Some("internet-gateway delete"),

        CliCommand::NetworkingAddressLotList => Some("system networking address-lot list"),
        CliCommand::NetworkingAddressLotCreate => Some("system networking address-lot create"),
        CliCommand::NetworkingAddressLotDelete => Some("system networking address-lot delete"),
        CliCommand::NetworkingAddressLotBlockList => {
            Some("system networking address-lot block list")
        }
        CliCommand::NetworkingLoopbackAddressList => {
            Some("system networking loopback-address list")
        }
        CliCommand::NetworkingLoopbackAddressCreate => {
            Some("system networking loopback-address create")
        }
        CliCommand::NetworkingLoopbackAddressDelete => {
            Some("system networking loopback-address delete")
        }

        CliCommand::NetworkingSwitchPortApplySettings => {
            Some("system hardware switch-port apply-settings")
        }
        CliCommand::NetworkingSwitchPortClearSettings => {
            Some("system hardware switch-port clear-settings")
        }
        CliCommand::NetworkingSwitchPortList => Some("system hardware switch-port list"),
        CliCommand::NetworkingSwitchPortStatus => Some("system hardware switch-port status"),
        CliCommand::NetworkingSwitchPortSettingsList => {
            Some("system networking switch-port-settings list")
        }
        CliCommand::NetworkingSwitchPortSettingsCreate => {
            Some("system networking switch-port-settings create")
        }
        CliCommand::NetworkingSwitchPortSettingsDelete => {
            Some("system networking switch-port-settings delete")
        }
        CliCommand::NetworkingSwitchPortSettingsView => {
            Some("system networking switch-port-settings view")
        }
        CliCommand::NetworkingSwitchPortLldpConfigView => Some("system networking lldp view"),
        CliCommand::NetworkingSwitchPortLldpConfigUpdate => Some("system networking lldp update"),
        CliCommand::NetworkingSwitchPortLldpNeighbors => Some("system networking lldp neighbors"),

        CliCommand::NetworkingBfdStatus => Some("system networking bfd status"),
        CliCommand::NetworkingBfdEnable => Some("system networking bfd enable"),
        CliCommand::NetworkingBfdDisable => Some("system networking bfd disable"),

        CliCommand::NetworkingBgpStatus => Some("system networking bgp status"),
        CliCommand::NetworkingBgpMessageHistory => Some("system networking bgp history"),
        CliCommand::NetworkingBgpConfigCreate => Some("system networking bgp config create"),
        CliCommand::NetworkingBgpConfigDelete => Some("system networking bgp config delete"),
        CliCommand::NetworkingBgpConfigList => Some("system networking bgp config list"),
        CliCommand::NetworkingBgpAnnounceSetUpdate => {
            Some("system networking bgp announce-set update")
        }
        CliCommand::NetworkingBgpAnnounceSetDelete => {
            Some("system networking bgp announce-set delete")
        }
        CliCommand::NetworkingBgpAnnounceSetList => Some("system networking bgp announce-set list"),
        CliCommand::NetworkingBgpAnnouncementList => {
            Some("system networking bgp announcement list")
        }
        CliCommand::NetworkingBgpExported => Some("system networking bgp exported ipv4"),
        CliCommand::NetworkingBgpImportedRoutesIpv4 => Some("system networking bgp imported ipv4"),

        // Subcommand: disk
        CliCommand::DiskList => Some("disk list"),
        CliCommand::DiskCreate => Some("disk create"),
        CliCommand::DiskView => Some("disk view"),
        CliCommand::DiskDelete => Some("disk delete"),
        CliCommand::DiskMetricsList => Some("disk metrics list"),

        CliCommand::DiskBulkWriteImportStart => Some("disk import start"),
        CliCommand::DiskBulkWriteImport => Some("disk import write"),
        CliCommand::DiskBulkWriteImportStop => Some("disk import stop"),
        CliCommand::DiskFinalizeImport => Some("disk import finalize"),

        CliCommand::GroupList => Some("group list"),

        // Subcommand: instance
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

        CliCommand::AuthSettingsView => Some("auth-settings view"),
        CliCommand::AuthSettingsUpdate => Some("auth-settings update"),

        CliCommand::SwitchList => Some("system hardware switch list"),
        CliCommand::SwitchView => Some("system hardware switch view"),
        CliCommand::RackList => Some("system hardware rack list"),
        CliCommand::RackView => Some("system hardware rack view"),
        CliCommand::SledList => Some("system hardware sled list"),
        CliCommand::SledListUninitialized => Some("system hardware sled list-uninitialized"),
        CliCommand::SledView => Some("system hardware sled view"),
        CliCommand::SledAdd => Some("system hardware sled add"),
        CliCommand::SledSetProvisionPolicy => Some("system hardware sled set-provision-policy"),
        CliCommand::SledInstanceList => Some("system hardware sled instance-list"),
        CliCommand::PhysicalDiskList => Some("system hardware disk list"),
        CliCommand::PhysicalDiskView => Some("system hardware disk view"),
        CliCommand::SledPhysicalDiskList => Some("system hardware sled disk-led"),

        CliCommand::SystemPolicyView => Some("system policy view"),
        CliCommand::SystemPolicyUpdate => Some("system policy update"),

        CliCommand::NetworkingAllowListView => Some("system networking allow-list view"),
        CliCommand::NetworkingAllowListUpdate => Some("system networking allow-list update"),

        CliCommand::CurrentUserView => Some("current-user view"),
        CliCommand::CurrentUserGroups => Some("current-user groups"),
        CliCommand::CurrentUserSshKeyList => Some("current-user ssh-key list"),
        CliCommand::CurrentUserSshKeyCreate => Some("current-user ssh-key create"),
        CliCommand::CurrentUserSshKeyView => Some("current-user ssh-key view"),
        CliCommand::CurrentUserSshKeyDelete => Some("current-user ssh-key delete"),
        CliCommand::CurrentUserAccessTokenList => Some("current-user access-token list"),
        CliCommand::CurrentUserAccessTokenDelete => Some("current-user access-token delete"),

        CliCommand::FloatingIpAttach => Some("floating-ip attach"),
        CliCommand::FloatingIpCreate => Some("floating-ip create"),
        CliCommand::FloatingIpDelete => Some("floating-ip delete"),
        CliCommand::FloatingIpDetach => Some("floating-ip detach"),
        CliCommand::FloatingIpList => Some("floating-ip list"),
        CliCommand::FloatingIpUpdate => Some("floating-ip update"),
        CliCommand::FloatingIpView => Some("floating-ip view"),

        // Alert subcommands
        CliCommand::AlertClassList => Some("alert class list"),
        CliCommand::AlertReceiverList => Some("alert receiver list"),
        CliCommand::AlertDeliveryList => Some("alert receiver log"),
        CliCommand::AlertReceiverProbe => Some("alert receiver probe"),
        CliCommand::AlertDeliveryResend => Some("alert receiver resend"),
        CliCommand::AlertReceiverView => Some("alert receiver view"),
        CliCommand::AlertReceiverDelete => Some("alert receiver delete"),
        CliCommand::AlertReceiverSubscriptionAdd => Some("alert receiver subscribe"),
        CliCommand::AlertReceiverSubscriptionRemove => Some("alert receiver unsubscribe"),

        // Webhook specific subcommands (including secret management)
        CliCommand::WebhookReceiverCreate => Some("alert receiver webhook create"),
        CliCommand::WebhookReceiverUpdate => Some("alert receiver webhook update"),
        CliCommand::WebhookSecretsList => Some("alert receiver webhook secret list"),
        CliCommand::WebhookSecretsAdd => Some("alert receiver webhook secret add"),
        CliCommand::WebhookSecretsDelete => Some("alert receiver webhook secret delete"),

        CliCommand::Ping => Some("ping"),

        CliCommand::ProbeCreate => Some("experimental probe create"),
        CliCommand::ProbeDelete => Some("experimental probe delete"),
        CliCommand::ProbeList => Some("experimental probe list"),
        CliCommand::ProbeView => Some("experimental probe view"),

        // Metrics-related subcommands
        CliCommand::TimeseriesQuery => Some("experimental timeseries query"),
        CliCommand::SystemTimeseriesQuery => Some("experimental system timeseries query"),
        CliCommand::SystemTimeseriesSchemaList => {
            Some("experimental system timeseries schema list")
        }

        // Support bundle commands
        CliCommand::SupportBundleList => Some("bundle list"),
        CliCommand::SupportBundleCreate => Some("bundle create"),
        CliCommand::SupportBundleView => Some("bundle view"),
        CliCommand::SupportBundleDelete => Some("bundle delete"),

        // Implemented as custom command to specify output file
        CliCommand::SupportBundleDownload => None,

        // Support bundles are not fully implemented.
        CliCommand::SupportBundleHead
        | CliCommand::SupportBundleDownloadFile
        | CliCommand::SupportBundleHeadFile
        | CliCommand::SupportBundleIndex => None,

        // Update is not fully implemented.
        CliCommand::TargetReleaseView | CliCommand::TargetReleaseUpdate => None,
        CliCommand::SystemUpdatePutRepository | CliCommand::SystemUpdateGetRepository => None,

        // Commands not yet implemented
        CliCommand::DeviceAccessToken
        | CliCommand::DeviceAuthConfirm
        | CliCommand::DeviceAuthRequest
        | CliCommand::GroupView
        | CliCommand::LoginLocal
        | CliCommand::LoginSaml
        | CliCommand::Logout
        | CliCommand::RoleList
        | CliCommand::RoleView
        | CliCommand::SiloMetric
        | CliCommand::SystemMetric
        | CliCommand::UserBuiltinList
        | CliCommand::UserBuiltinView => None,
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
                        .display_order(0)
                        .mut_args(update_byte_count_help)
                        .add_subcommand(rest, subcmd),
                )
            }
        } else {
            let new_subcmd = subcmd
                .into()
                .name(path.to_owned())
                .display_order(0)
                .mut_args(update_byte_count_help);
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

/// For Args that take a `ByteCount`, append a message on the unit formatting accepted and remove
/// any reference to the field taking bytes as an argument.
fn update_byte_count_help(arg: Arg) -> Arg {
    const UNIT_HINT: &str =
        "unit suffixes are in powers of two (1k = 1024 bytes) for example: 6GiB, 512k, 2048mib";

    let parser = arg.get_value_parser();
    if parser.type_id() == TypeId::of::<ByteCount>() {
        let old_help = arg
            .get_help()
            .cloned()
            .map(|h| h.to_string().replace(" (in bytes)", ""));

        arg.help(match old_help {
            Some(old) => format!("{old}; {UNIT_HINT}"),
            None => UNIT_HINT.to_string(),
        })
    } else {
        arg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_value_parses_ipv4_addr() {
        let addr = "192.168.1.1";
        let host = "oxide.computer";
        let port = 12345;

        let parsed: ResolveValue = format!("{host}:{port}:{addr}").parse().unwrap();

        assert_eq!(
            parsed,
            ResolveValue {
                host: host.to_string(),
                port,
                addr: addr.parse().unwrap(),
            }
        );
    }

    #[test]
    fn resolve_value_parses_ipv6_addr() {
        let addr = "fdb2:a840:2504:355::1";
        let host = "oxide.computer";
        let port = 12345;

        let parsed: ResolveValue = format!("{host}:{port}:[{addr}]").parse().unwrap();

        assert_eq!(
            parsed,
            ResolveValue {
                host: host.to_string(),
                port,
                addr: addr.parse().unwrap(),
            }
        );
    }
}
