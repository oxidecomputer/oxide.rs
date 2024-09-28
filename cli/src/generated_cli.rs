// The contents of this file are generated; do not modify them.

use oxide::*;
pub struct Cli<T: CliConfig> {
    client: Client,
    config: T,
}

impl<T: CliConfig> Cli<T> {
    pub fn new(client: Client, config: T) -> Self {
        Self { client, config }
    }

    pub fn get_command(cmd: CliCommand) -> clap::Command {
        match cmd {
            CliCommand::DeviceAuthRequest => Self::cli_device_auth_request(),
            CliCommand::DeviceAuthConfirm => Self::cli_device_auth_confirm(),
            CliCommand::DeviceAccessToken => Self::cli_device_access_token(),
            CliCommand::ProbeList => Self::cli_probe_list(),
            CliCommand::ProbeCreate => Self::cli_probe_create(),
            CliCommand::ProbeView => Self::cli_probe_view(),
            CliCommand::ProbeDelete => Self::cli_probe_delete(),
            CliCommand::LoginSaml => Self::cli_login_saml(),
            CliCommand::CertificateList => Self::cli_certificate_list(),
            CliCommand::CertificateCreate => Self::cli_certificate_create(),
            CliCommand::CertificateView => Self::cli_certificate_view(),
            CliCommand::CertificateDelete => Self::cli_certificate_delete(),
            CliCommand::DiskList => Self::cli_disk_list(),
            CliCommand::DiskCreate => Self::cli_disk_create(),
            CliCommand::DiskView => Self::cli_disk_view(),
            CliCommand::DiskDelete => Self::cli_disk_delete(),
            CliCommand::DiskBulkWriteImport => Self::cli_disk_bulk_write_import(),
            CliCommand::DiskBulkWriteImportStart => Self::cli_disk_bulk_write_import_start(),
            CliCommand::DiskBulkWriteImportStop => Self::cli_disk_bulk_write_import_stop(),
            CliCommand::DiskFinalizeImport => Self::cli_disk_finalize_import(),
            CliCommand::DiskMetricsList => Self::cli_disk_metrics_list(),
            CliCommand::FloatingIpList => Self::cli_floating_ip_list(),
            CliCommand::FloatingIpCreate => Self::cli_floating_ip_create(),
            CliCommand::FloatingIpView => Self::cli_floating_ip_view(),
            CliCommand::FloatingIpUpdate => Self::cli_floating_ip_update(),
            CliCommand::FloatingIpDelete => Self::cli_floating_ip_delete(),
            CliCommand::FloatingIpAttach => Self::cli_floating_ip_attach(),
            CliCommand::FloatingIpDetach => Self::cli_floating_ip_detach(),
            CliCommand::GroupList => Self::cli_group_list(),
            CliCommand::GroupView => Self::cli_group_view(),
            CliCommand::ImageList => Self::cli_image_list(),
            CliCommand::ImageCreate => Self::cli_image_create(),
            CliCommand::ImageView => Self::cli_image_view(),
            CliCommand::ImageDelete => Self::cli_image_delete(),
            CliCommand::ImageDemote => Self::cli_image_demote(),
            CliCommand::ImagePromote => Self::cli_image_promote(),
            CliCommand::InstanceList => Self::cli_instance_list(),
            CliCommand::InstanceCreate => Self::cli_instance_create(),
            CliCommand::InstanceView => Self::cli_instance_view(),
            CliCommand::InstanceDelete => Self::cli_instance_delete(),
            CliCommand::InstanceDiskList => Self::cli_instance_disk_list(),
            CliCommand::InstanceDiskAttach => Self::cli_instance_disk_attach(),
            CliCommand::InstanceDiskDetach => Self::cli_instance_disk_detach(),
            CliCommand::InstanceExternalIpList => Self::cli_instance_external_ip_list(),
            CliCommand::InstanceEphemeralIpAttach => Self::cli_instance_ephemeral_ip_attach(),
            CliCommand::InstanceEphemeralIpDetach => Self::cli_instance_ephemeral_ip_detach(),
            CliCommand::InstanceReboot => Self::cli_instance_reboot(),
            CliCommand::InstanceSerialConsole => Self::cli_instance_serial_console(),
            CliCommand::InstanceSerialConsoleStream => Self::cli_instance_serial_console_stream(),
            CliCommand::InstanceSshPublicKeyList => Self::cli_instance_ssh_public_key_list(),
            CliCommand::InstanceStart => Self::cli_instance_start(),
            CliCommand::InstanceStop => Self::cli_instance_stop(),
            CliCommand::InternetGatewayIpAddressList => {
                Self::cli_internet_gateway_ip_address_list()
            }
            CliCommand::InternetGatewayIpAddressCreate => {
                Self::cli_internet_gateway_ip_address_create()
            }
            CliCommand::InternetGatewayIpAddressDelete => {
                Self::cli_internet_gateway_ip_address_delete()
            }
            CliCommand::InternetGatewayIpPoolList => Self::cli_internet_gateway_ip_pool_list(),
            CliCommand::InternetGatewayIpPoolCreate => Self::cli_internet_gateway_ip_pool_create(),
            CliCommand::InternetGatewayIpPoolDelete => Self::cli_internet_gateway_ip_pool_delete(),
            CliCommand::InternetGatewayList => Self::cli_internet_gateway_list(),
            CliCommand::InternetGatewayCreate => Self::cli_internet_gateway_create(),
            CliCommand::InternetGatewayView => Self::cli_internet_gateway_view(),
            CliCommand::InternetGatewayDelete => Self::cli_internet_gateway_delete(),
            CliCommand::ProjectIpPoolList => Self::cli_project_ip_pool_list(),
            CliCommand::ProjectIpPoolView => Self::cli_project_ip_pool_view(),
            CliCommand::LoginLocal => Self::cli_login_local(),
            CliCommand::Logout => Self::cli_logout(),
            CliCommand::CurrentUserView => Self::cli_current_user_view(),
            CliCommand::CurrentUserGroups => Self::cli_current_user_groups(),
            CliCommand::CurrentUserSshKeyList => Self::cli_current_user_ssh_key_list(),
            CliCommand::CurrentUserSshKeyCreate => Self::cli_current_user_ssh_key_create(),
            CliCommand::CurrentUserSshKeyView => Self::cli_current_user_ssh_key_view(),
            CliCommand::CurrentUserSshKeyDelete => Self::cli_current_user_ssh_key_delete(),
            CliCommand::SiloMetric => Self::cli_silo_metric(),
            CliCommand::InstanceNetworkInterfaceList => Self::cli_instance_network_interface_list(),
            CliCommand::InstanceNetworkInterfaceCreate => {
                Self::cli_instance_network_interface_create()
            }
            CliCommand::InstanceNetworkInterfaceView => Self::cli_instance_network_interface_view(),
            CliCommand::InstanceNetworkInterfaceUpdate => {
                Self::cli_instance_network_interface_update()
            }
            CliCommand::InstanceNetworkInterfaceDelete => {
                Self::cli_instance_network_interface_delete()
            }
            CliCommand::Ping => Self::cli_ping(),
            CliCommand::PolicyView => Self::cli_policy_view(),
            CliCommand::PolicyUpdate => Self::cli_policy_update(),
            CliCommand::ProjectList => Self::cli_project_list(),
            CliCommand::ProjectCreate => Self::cli_project_create(),
            CliCommand::ProjectView => Self::cli_project_view(),
            CliCommand::ProjectUpdate => Self::cli_project_update(),
            CliCommand::ProjectDelete => Self::cli_project_delete(),
            CliCommand::ProjectPolicyView => Self::cli_project_policy_view(),
            CliCommand::ProjectPolicyUpdate => Self::cli_project_policy_update(),
            CliCommand::SnapshotList => Self::cli_snapshot_list(),
            CliCommand::SnapshotCreate => Self::cli_snapshot_create(),
            CliCommand::SnapshotView => Self::cli_snapshot_view(),
            CliCommand::SnapshotDelete => Self::cli_snapshot_delete(),
            CliCommand::PhysicalDiskList => Self::cli_physical_disk_list(),
            CliCommand::PhysicalDiskView => Self::cli_physical_disk_view(),
            CliCommand::RackList => Self::cli_rack_list(),
            CliCommand::RackView => Self::cli_rack_view(),
            CliCommand::SledList => Self::cli_sled_list(),
            CliCommand::SledAdd => Self::cli_sled_add(),
            CliCommand::SledView => Self::cli_sled_view(),
            CliCommand::SledPhysicalDiskList => Self::cli_sled_physical_disk_list(),
            CliCommand::SledInstanceList => Self::cli_sled_instance_list(),
            CliCommand::SledSetProvisionPolicy => Self::cli_sled_set_provision_policy(),
            CliCommand::SledListUninitialized => Self::cli_sled_list_uninitialized(),
            CliCommand::NetworkingSwitchPortList => Self::cli_networking_switch_port_list(),
            CliCommand::NetworkingSwitchPortApplySettings => {
                Self::cli_networking_switch_port_apply_settings()
            }
            CliCommand::NetworkingSwitchPortClearSettings => {
                Self::cli_networking_switch_port_clear_settings()
            }
            CliCommand::NetworkingSwitchPortStatus => Self::cli_networking_switch_port_status(),
            CliCommand::SwitchList => Self::cli_switch_list(),
            CliCommand::SwitchView => Self::cli_switch_view(),
            CliCommand::SiloIdentityProviderList => Self::cli_silo_identity_provider_list(),
            CliCommand::LocalIdpUserCreate => Self::cli_local_idp_user_create(),
            CliCommand::LocalIdpUserDelete => Self::cli_local_idp_user_delete(),
            CliCommand::LocalIdpUserSetPassword => Self::cli_local_idp_user_set_password(),
            CliCommand::SamlIdentityProviderCreate => Self::cli_saml_identity_provider_create(),
            CliCommand::SamlIdentityProviderView => Self::cli_saml_identity_provider_view(),
            CliCommand::IpPoolList => Self::cli_ip_pool_list(),
            CliCommand::IpPoolCreate => Self::cli_ip_pool_create(),
            CliCommand::IpPoolView => Self::cli_ip_pool_view(),
            CliCommand::IpPoolUpdate => Self::cli_ip_pool_update(),
            CliCommand::IpPoolDelete => Self::cli_ip_pool_delete(),
            CliCommand::IpPoolRangeList => Self::cli_ip_pool_range_list(),
            CliCommand::IpPoolRangeAdd => Self::cli_ip_pool_range_add(),
            CliCommand::IpPoolRangeRemove => Self::cli_ip_pool_range_remove(),
            CliCommand::IpPoolSiloList => Self::cli_ip_pool_silo_list(),
            CliCommand::IpPoolSiloLink => Self::cli_ip_pool_silo_link(),
            CliCommand::IpPoolSiloUpdate => Self::cli_ip_pool_silo_update(),
            CliCommand::IpPoolSiloUnlink => Self::cli_ip_pool_silo_unlink(),
            CliCommand::IpPoolUtilizationView => Self::cli_ip_pool_utilization_view(),
            CliCommand::IpPoolServiceView => Self::cli_ip_pool_service_view(),
            CliCommand::IpPoolServiceRangeList => Self::cli_ip_pool_service_range_list(),
            CliCommand::IpPoolServiceRangeAdd => Self::cli_ip_pool_service_range_add(),
            CliCommand::IpPoolServiceRangeRemove => Self::cli_ip_pool_service_range_remove(),
            CliCommand::SystemMetric => Self::cli_system_metric(),
            CliCommand::NetworkingAddressLotList => Self::cli_networking_address_lot_list(),
            CliCommand::NetworkingAddressLotCreate => Self::cli_networking_address_lot_create(),
            CliCommand::NetworkingAddressLotDelete => Self::cli_networking_address_lot_delete(),
            CliCommand::NetworkingAddressLotBlockList => {
                Self::cli_networking_address_lot_block_list()
            }
            CliCommand::NetworkingAllowListView => Self::cli_networking_allow_list_view(),
            CliCommand::NetworkingAllowListUpdate => Self::cli_networking_allow_list_update(),
            CliCommand::NetworkingBfdDisable => Self::cli_networking_bfd_disable(),
            CliCommand::NetworkingBfdEnable => Self::cli_networking_bfd_enable(),
            CliCommand::NetworkingBfdStatus => Self::cli_networking_bfd_status(),
            CliCommand::NetworkingBgpConfigList => Self::cli_networking_bgp_config_list(),
            CliCommand::NetworkingBgpConfigCreate => Self::cli_networking_bgp_config_create(),
            CliCommand::NetworkingBgpConfigDelete => Self::cli_networking_bgp_config_delete(),
            CliCommand::NetworkingBgpAnnounceSetList => {
                Self::cli_networking_bgp_announce_set_list()
            }
            CliCommand::NetworkingBgpAnnounceSetUpdate => {
                Self::cli_networking_bgp_announce_set_update()
            }
            CliCommand::NetworkingBgpAnnounceSetDelete => {
                Self::cli_networking_bgp_announce_set_delete()
            }
            CliCommand::NetworkingBgpAnnouncementList => {
                Self::cli_networking_bgp_announcement_list()
            }
            CliCommand::NetworkingBgpExported => Self::cli_networking_bgp_exported(),
            CliCommand::NetworkingBgpMessageHistory => Self::cli_networking_bgp_message_history(),
            CliCommand::NetworkingBgpImportedRoutesIpv4 => {
                Self::cli_networking_bgp_imported_routes_ipv4()
            }
            CliCommand::NetworkingBgpStatus => Self::cli_networking_bgp_status(),
            CliCommand::NetworkingLoopbackAddressList => {
                Self::cli_networking_loopback_address_list()
            }
            CliCommand::NetworkingLoopbackAddressCreate => {
                Self::cli_networking_loopback_address_create()
            }
            CliCommand::NetworkingLoopbackAddressDelete => {
                Self::cli_networking_loopback_address_delete()
            }
            CliCommand::NetworkingSwitchPortSettingsList => {
                Self::cli_networking_switch_port_settings_list()
            }
            CliCommand::NetworkingSwitchPortSettingsCreate => {
                Self::cli_networking_switch_port_settings_create()
            }
            CliCommand::NetworkingSwitchPortSettingsDelete => {
                Self::cli_networking_switch_port_settings_delete()
            }
            CliCommand::NetworkingSwitchPortSettingsView => {
                Self::cli_networking_switch_port_settings_view()
            }
            CliCommand::SystemPolicyView => Self::cli_system_policy_view(),
            CliCommand::SystemPolicyUpdate => Self::cli_system_policy_update(),
            CliCommand::RoleList => Self::cli_role_list(),
            CliCommand::RoleView => Self::cli_role_view(),
            CliCommand::SystemQuotasList => Self::cli_system_quotas_list(),
            CliCommand::SiloList => Self::cli_silo_list(),
            CliCommand::SiloCreate => Self::cli_silo_create(),
            CliCommand::SiloView => Self::cli_silo_view(),
            CliCommand::SiloDelete => Self::cli_silo_delete(),
            CliCommand::SiloIpPoolList => Self::cli_silo_ip_pool_list(),
            CliCommand::SiloPolicyView => Self::cli_silo_policy_view(),
            CliCommand::SiloPolicyUpdate => Self::cli_silo_policy_update(),
            CliCommand::SiloQuotasView => Self::cli_silo_quotas_view(),
            CliCommand::SiloQuotasUpdate => Self::cli_silo_quotas_update(),
            CliCommand::SiloUserList => Self::cli_silo_user_list(),
            CliCommand::SiloUserView => Self::cli_silo_user_view(),
            CliCommand::UserBuiltinList => Self::cli_user_builtin_list(),
            CliCommand::UserBuiltinView => Self::cli_user_builtin_view(),
            CliCommand::SiloUtilizationList => Self::cli_silo_utilization_list(),
            CliCommand::SiloUtilizationView => Self::cli_silo_utilization_view(),
            CliCommand::TimeseriesQuery => Self::cli_timeseries_query(),
            CliCommand::TimeseriesSchemaList => Self::cli_timeseries_schema_list(),
            CliCommand::UserList => Self::cli_user_list(),
            CliCommand::UtilizationView => Self::cli_utilization_view(),
            CliCommand::VpcFirewallRulesView => Self::cli_vpc_firewall_rules_view(),
            CliCommand::VpcFirewallRulesUpdate => Self::cli_vpc_firewall_rules_update(),
            CliCommand::VpcRouterRouteList => Self::cli_vpc_router_route_list(),
            CliCommand::VpcRouterRouteCreate => Self::cli_vpc_router_route_create(),
            CliCommand::VpcRouterRouteView => Self::cli_vpc_router_route_view(),
            CliCommand::VpcRouterRouteUpdate => Self::cli_vpc_router_route_update(),
            CliCommand::VpcRouterRouteDelete => Self::cli_vpc_router_route_delete(),
            CliCommand::VpcRouterList => Self::cli_vpc_router_list(),
            CliCommand::VpcRouterCreate => Self::cli_vpc_router_create(),
            CliCommand::VpcRouterView => Self::cli_vpc_router_view(),
            CliCommand::VpcRouterUpdate => Self::cli_vpc_router_update(),
            CliCommand::VpcRouterDelete => Self::cli_vpc_router_delete(),
            CliCommand::VpcSubnetList => Self::cli_vpc_subnet_list(),
            CliCommand::VpcSubnetCreate => Self::cli_vpc_subnet_create(),
            CliCommand::VpcSubnetView => Self::cli_vpc_subnet_view(),
            CliCommand::VpcSubnetUpdate => Self::cli_vpc_subnet_update(),
            CliCommand::VpcSubnetDelete => Self::cli_vpc_subnet_delete(),
            CliCommand::VpcSubnetListNetworkInterfaces => {
                Self::cli_vpc_subnet_list_network_interfaces()
            }
            CliCommand::VpcList => Self::cli_vpc_list(),
            CliCommand::VpcCreate => Self::cli_vpc_create(),
            CliCommand::VpcView => Self::cli_vpc_view(),
            CliCommand::VpcUpdate => Self::cli_vpc_update(),
            CliCommand::VpcDelete => Self::cli_vpc_delete(),
        }
    }

    pub fn cli_device_auth_request() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Start an OAuth 2.0 Device Authorization Grant")
            .long_about(
                "This endpoint is designed to be accessed from an *unauthenticated* API client. \
                 It generates and records a `device_code` and `user_code` which must be verified \
                 and confirmed prior to a token being granted.",
            )
    }

    pub fn cli_device_auth_confirm() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("user-code")
                    .long("user-code")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Confirm an OAuth 2.0 Device Authorization Grant")
            .long_about(
                "This endpoint is designed to be accessed by the user agent (browser), not the \
                 client requesting the token. So we do not actually return the token here; it \
                 will be returned in response to the poll on `/device/token`.",
            )
    }

    pub fn cli_device_access_token() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("device-code")
                    .long("device-code")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("grant-type")
                    .long("grant-type")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Request a device access token")
            .long_about(
                "This endpoint should be polled by the client until the user code is verified and \
                 the grant is confirmed.",
            )
    }

    pub fn cli_probe_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List instrumentation probes")
    }

    pub fn cli_probe_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("ip-pool")
                    .long("ip-pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sled")
                    .long("sled")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create instrumentation probe")
    }

    pub fn cli_probe_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("probe")
                    .long("probe")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the probe"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .about("View instrumentation probe")
    }

    pub fn cli_probe_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("probe")
                    .long("probe")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the probe"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .about("Delete instrumentation probe")
    }

    pub fn cli_login_saml() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("provider-name")
                    .long("provider-name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true),
            )
            .arg(
                clap::Arg::new("silo-name")
                    .long("silo-name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true),
            )
            .about("Authenticate a user via SAML")
    }

    pub fn cli_certificate_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List certificates for external endpoints")
            .long_about(
                "Returns a list of TLS certificates used for the external API (for the current \
                 Silo).  These are sorted by creation date, with the most recent certificates \
                 appearing first.",
            )
    }

    pub fn cli_certificate_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("cert")
                    .long("cert")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("PEM-formatted string containing public certificate chain"),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("key")
                    .long("key")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("PEM-formatted string containing private key"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("service")
                    .long("service")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::ServiceUsingCertificate::ExternalApi.to_string(),
                        ]),
                        |s| types::ServiceUsingCertificate::try_from(s).unwrap(),
                    ))
                    .required_unless_present("json-body")
                    .help("The service using this certificate"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create new system-wide x.509 certificate")
            .long_about(
                "This certificate is automatically used by the Oxide Control plane to serve \
                 external connections.",
            )
    }

    pub fn cli_certificate_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("certificate")
                    .long("certificate")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the certificate"),
            )
            .about("Fetch certificate")
            .long_about("Returns the details of a specific certificate")
    }

    pub fn cli_certificate_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("certificate")
                    .long("certificate")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the certificate"),
            )
            .about("Delete certificate")
            .long_about("Permanently delete a certificate. This operation cannot be undone.")
    }

    pub fn cli_disk_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List disks")
    }

    pub fn cli_disk_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("size")
                    .long("size")
                    .value_parser(clap::value_parser!(types::ByteCount))
                    .required_unless_present("json-body")
                    .help("total size of the Disk in bytes"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create a disk")
    }

    pub fn cli_disk_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the disk"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Fetch disk")
    }

    pub fn cli_disk_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the disk"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Delete disk")
    }

    pub fn cli_disk_bulk_write_import() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("base64-encoded-data")
                    .long("base64-encoded-data")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the disk"),
            )
            .arg(
                clap::Arg::new("offset")
                    .long("offset")
                    .value_parser(clap::value_parser!(u64))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Import blocks into disk")
    }

    pub fn cli_disk_bulk_write_import_start() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the disk"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Start importing blocks into disk")
            .long_about("Start the process of importing blocks into a disk")
    }

    pub fn cli_disk_bulk_write_import_stop() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the disk"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Stop importing blocks into disk")
            .long_about("Stop the process of importing blocks into a disk")
    }

    pub fn cli_disk_finalize_import() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the disk"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("snapshot-name")
                    .long("snapshot-name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false)
                    .help(
                        "If specified a snapshot of the disk will be created with the given name \
                         during finalization. If not specified, a snapshot for the disk will \
                         _not_ be created. A snapshot can be manually created once the disk \
                         transitions into the `Detached` state.",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Confirm disk block import completion")
    }

    pub fn cli_disk_metrics_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true),
            )
            .arg(
                clap::Arg::new("end-time")
                    .long("end-time")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required(true)
                    .help("An exclusive end time of metrics."),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("metric")
                    .long("metric")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::DiskMetricName::Activated.to_string(),
                            types::DiskMetricName::Flush.to_string(),
                            types::DiskMetricName::Read.to_string(),
                            types::DiskMetricName::ReadBytes.to_string(),
                            types::DiskMetricName::Write.to_string(),
                            types::DiskMetricName::WriteBytes.to_string(),
                        ]),
                        |s| types::DiskMetricName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("order")
                    .long("order")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::PaginationOrder::Ascending.to_string(),
                            types::PaginationOrder::Descending.to_string(),
                        ]),
                        |s| types::PaginationOrder::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help("Query result order"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("start-time")
                    .long("start-time")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required(true)
                    .help("An inclusive start time of metrics."),
            )
            .about("Fetch disk metrics")
    }

    pub fn cli_floating_ip_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List floating IPs")
    }

    pub fn cli_floating_ip_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("ip")
                    .long("ip")
                    .value_parser(clap::value_parser!(std::net::IpAddr))
                    .required(false)
                    .help(
                        "An IP address to reserve for use as a floating IP. This field is \
                         optional: when not set, an address will be automatically chosen from \
                         `pool`. If set, then the IP must be available in the resolved `pool`.",
                    ),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "The parent IP pool that a floating IP is pulled from. If unset, the \
                         default pool is selected.",
                    ),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create floating IP")
    }

    pub fn cli_floating_ip_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("floating-ip")
                    .long("floating-ip")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the floating IP"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Fetch floating IP")
    }

    pub fn cli_floating_ip_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("floating-ip")
                    .long("floating-ip")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the floating IP"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update floating IP")
    }

    pub fn cli_floating_ip_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("floating-ip")
                    .long("floating-ip")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the floating IP"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Delete floating IP")
    }

    pub fn cli_floating_ip_attach() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("floating-ip")
                    .long("floating-ip")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the floating IP"),
            )
            .arg(
                clap::Arg::new("kind")
                    .long("kind")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::FloatingIpParentKind::Instance.to_string(),
                        ]),
                        |s| types::FloatingIpParentKind::try_from(s).unwrap(),
                    ))
                    .required_unless_present("json-body")
                    .help("The type of `parent`'s resource"),
            )
            .arg(
                clap::Arg::new("parent")
                    .long("parent")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body")
                    .help("Name or ID of the resource that this IP address should be attached to"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Attach floating IP")
            .long_about("Attach floating IP to an instance or other resource.")
    }

    pub fn cli_floating_ip_detach() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("floating-ip")
                    .long("floating-ip")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the floating IP"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Detach floating IP")
    }

    pub fn cli_group_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List groups")
    }

    pub fn cli_group_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("group-id")
                    .long("group-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("ID of the group"),
            )
            .about("Fetch group")
    }

    pub fn cli_image_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List images")
            .long_about(
                "List images which are global or scoped to the specified project. The images are \
                 returned sorted by creation date, with the most recent images appearing first.",
            )
    }

    pub fn cli_image_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("os")
                    .long("os")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("The family of the operating system (e.g. Debian, Ubuntu, etc.)"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("version")
                    .long("version")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("The version of the operating system (e.g. 18.04, 20.04, etc.)"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create image")
            .long_about("Create a new image in a project.")
    }

    pub fn cli_image_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image")
                    .long("image")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the image"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Fetch image")
            .long_about("Fetch the details for a specific image in a project.")
    }

    pub fn cli_image_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image")
                    .long("image")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the image"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Delete image")
            .long_about(
                "Permanently delete an image from a project. This operation cannot be undone. Any \
                 instances in the project using the image will continue to run, however new \
                 instances can not be created with this image.",
            )
    }

    pub fn cli_image_demote() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image")
                    .long("image")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the image"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .about("Demote silo image")
            .long_about("Demote silo image to be visible only to a specified project")
    }

    pub fn cli_image_promote() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image")
                    .long("image")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the image"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Promote project image")
            .long_about("Promote project image to be visible to all projects in the silo")
    }

    pub fn cli_instance_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List instances")
    }

    pub fn cli_instance_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("auto-restart-policy")
                    .long("auto-restart-policy")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::InstanceAutoRestartPolicy::Never.to_string(),
                            types::InstanceAutoRestartPolicy::BestEffort.to_string(),
                        ]),
                        |s| types::InstanceAutoRestartPolicy::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help(
                        "The auto-restart policy for this instance.\n\nThis indicates whether the \
                         instance should be automatically restarted by the control plane on \
                         failure. If this is `null`, no auto-restart policy has been configured \
                         for this instance by the user.",
                    ),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("hostname")
                    .long("hostname")
                    .value_parser(clap::value_parser!(types::Hostname))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("memory")
                    .long("memory")
                    .value_parser(clap::value_parser!(types::ByteCount))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("ncpus")
                    .long("ncpus")
                    .value_parser(clap::value_parser!(types::InstanceCpuCount))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("start")
                    .long("start")
                    .value_parser(clap::value_parser!(bool))
                    .required(false)
                    .help("Should this instance be started upon creation; true by default."),
            )
            .arg(
                clap::Arg::new("user-data")
                    .long("user-data")
                    .value_parser(clap::value_parser!(String))
                    .required(false)
                    .help(
                        "User data for instance initialization systems (such as cloud-init). Must \
                         be a Base64-encoded string, as specified in RFC 4648 § 4 (+ and / \
                         characters with padding). Maximum 32 KiB unencoded data.",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create instance")
    }

    pub fn cli_instance_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Fetch instance")
    }

    pub fn cli_instance_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Delete instance")
    }

    pub fn cli_instance_disk_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List disks for instance")
    }

    pub fn cli_instance_disk_attach() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body")
                    .help("Name or ID of the disk"),
            )
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Attach disk to instance")
    }

    pub fn cli_instance_disk_detach() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body")
                    .help("Name or ID of the disk"),
            )
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Detach disk from instance")
    }

    pub fn cli_instance_external_ip_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("List external IP addresses")
    }

    pub fn cli_instance_ephemeral_ip_attach() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the IP pool used to allocate an address"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Allocate and attach ephemeral IP to instance")
    }

    pub fn cli_instance_ephemeral_ip_detach() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Detach and deallocate ephemeral IP from instance")
    }

    pub fn cli_instance_reboot() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Reboot an instance")
    }

    pub fn cli_instance_serial_console() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("from-start")
                    .long("from-start")
                    .value_parser(clap::value_parser!(u64))
                    .required(false)
                    .help(
                        "Character index in the serial buffer from which to read, counting the \
                         bytes output since instance start. If this is not provided, \
                         `most_recent` must be provided, and if this *is* provided, `most_recent` \
                         must *not* be provided.",
                    ),
            )
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("max-bytes")
                    .long("max-bytes")
                    .value_parser(clap::value_parser!(u64))
                    .required(false)
                    .help(
                        "Maximum number of bytes of buffered serial console contents to return. \
                         If the requested range runs to the end of the available buffer, the data \
                         returned will be shorter than `max_bytes`.",
                    ),
            )
            .arg(
                clap::Arg::new("most-recent")
                    .long("most-recent")
                    .value_parser(clap::value_parser!(u64))
                    .required(false)
                    .help(
                        "Character index in the serial buffer from which to read, counting \
                         *backward* from the most recently buffered data retrieved from the \
                         instance. (See note on `from_start` about mutual exclusivity)",
                    ),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `instance` is provided as a \
                         `Name`",
                    ),
            )
            .about("Fetch instance serial console")
    }

    pub fn cli_instance_serial_console_stream() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("most-recent")
                    .long("most-recent")
                    .value_parser(clap::value_parser!(u64))
                    .required(false)
                    .help(
                        "Character index in the serial buffer from which to read, counting \
                         *backward* from the most recently buffered data retrieved from the \
                         instance.",
                    ),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `instance` is provided as a \
                         `Name`",
                    ),
            )
            .about("Stream instance serial console")
    }

    pub fn cli_instance_ssh_public_key_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List SSH public keys for instance")
            .long_about(
                "List SSH public keys injected via cloud-init during instance creation. Note that \
                 this list is a snapshot in time and will not reflect updates made after the \
                 instance is created.",
            )
    }

    pub fn cli_instance_start() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Boot instance")
    }

    pub fn cli_instance_stop() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .about("Stop instance")
    }

    pub fn cli_internet_gateway_ip_address_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("gateway")
                    .long("gateway")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the internet gateway"),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `gateway` is provided as a `Name`",
                    ),
            )
            .about("List addresses attached to an internet gateway.")
    }

    pub fn cli_internet_gateway_ip_address_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("address")
                    .long("address")
                    .value_parser(clap::value_parser!(std::net::IpAddr))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("gateway")
                    .long("gateway")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `gateway` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Attach ip pool to internet gateway")
    }

    pub fn cli_internet_gateway_ip_address_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("address")
                    .long("address")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP address"),
            )
            .arg(
                clap::Arg::new("cascade")
                    .long("cascade")
                    .value_parser(clap::value_parser!(bool))
                    .required(false)
                    .help("Also delete routes targeting this gateway element."),
            )
            .arg(
                clap::Arg::new("gateway")
                    .long("gateway")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the internet gateway"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `gateway` is provided as a `Name`",
                    ),
            )
            .about("Detach ip pool from internet gateway")
    }

    pub fn cli_internet_gateway_ip_pool_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("gateway")
                    .long("gateway")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the internet gateway"),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `gateway` is provided as a `Name`",
                    ),
            )
            .about("List IP pools attached to an internet gateway.")
    }

    pub fn cli_internet_gateway_ip_pool_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("gateway")
                    .long("gateway")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("ip-pool")
                    .long("ip-pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `gateway` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Attach ip pool to internet gateway")
    }

    pub fn cli_internet_gateway_ip_pool_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("cascade")
                    .long("cascade")
                    .value_parser(clap::value_parser!(bool))
                    .required(false)
                    .help("Also delete routes targeting this gateway element."),
            )
            .arg(
                clap::Arg::new("gateway")
                    .long("gateway")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the internet gateway"),
            )
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `gateway` is provided as a `Name`",
                    ),
            )
            .about("Detach ip pool from internet gateway")
    }

    pub fn cli_internet_gateway_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .about("List internet gateways")
    }

    pub fn cli_internet_gateway_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create VPC internet gateway")
    }

    pub fn cli_internet_gateway_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("gateway")
                    .long("gateway")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the gateway"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the VPC"),
            )
            .about("Fetch internet gateway")
    }

    pub fn cli_internet_gateway_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("cascade")
                    .long("cascade")
                    .value_parser(clap::value_parser!(bool))
                    .required(false)
                    .help("Also delete routes targeting this gateway."),
            )
            .arg(
                clap::Arg::new("gateway")
                    .long("gateway")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the gateway"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the VPC"),
            )
            .about("Delete internet gateway")
    }

    pub fn cli_project_ip_pool_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List IP pools")
    }

    pub fn cli_project_ip_pool_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .about("Fetch IP pool")
    }

    pub fn cli_login_local() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("password")
                    .long("password")
                    .value_parser(clap::value_parser!(types::Password))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("silo-name")
                    .long("silo-name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true),
            )
            .arg(
                clap::Arg::new("username")
                    .long("username")
                    .value_parser(clap::value_parser!(types::UserId))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Authenticate a user via username and password")
    }

    pub fn cli_logout() -> clap::Command {
        clap::Command::new("")
            .about("Log user out of web console by deleting session on client and server")
    }

    pub fn cli_current_user_view() -> clap::Command {
        clap::Command::new("").about("Fetch user for current session")
    }

    pub fn cli_current_user_groups() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("Fetch current user's groups")
    }

    pub fn cli_current_user_ssh_key_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List SSH public keys")
            .long_about("Lists SSH public keys for the currently authenticated user.")
    }

    pub fn cli_current_user_ssh_key_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("public-key")
                    .long("public-key")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("SSH public key, e.g., `\"ssh-ed25519 AAAAC3NzaC...\"`"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create SSH public key")
            .long_about("Create an SSH public key for the currently authenticated user.")
    }

    pub fn cli_current_user_ssh_key_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("ssh-key")
                    .long("ssh-key")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the SSH key"),
            )
            .about("Fetch SSH public key")
            .long_about("Fetch SSH public key associated with the currently authenticated user.")
    }

    pub fn cli_current_user_ssh_key_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("ssh-key")
                    .long("ssh-key")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the SSH key"),
            )
            .about("Delete SSH public key")
            .long_about(
                "Delete an SSH public key associated with the currently authenticated user.",
            )
    }

    pub fn cli_silo_metric() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("end-time")
                    .long("end-time")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required(true)
                    .help("An exclusive end time of metrics."),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("metric-name")
                    .long("metric-name")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::SystemMetricName::VirtualDiskSpaceProvisioned.to_string(),
                            types::SystemMetricName::CpusProvisioned.to_string(),
                            types::SystemMetricName::RamProvisioned.to_string(),
                        ]),
                        |s| types::SystemMetricName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("order")
                    .long("order")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::PaginationOrder::Ascending.to_string(),
                            types::PaginationOrder::Descending.to_string(),
                        ]),
                        |s| types::PaginationOrder::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help("Query result order"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("start-time")
                    .long("start-time")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required(true)
                    .help("An inclusive start time of metrics."),
            )
            .about("View metrics")
            .long_about(
                "View CPU, memory, or storage utilization metrics at the silo or project level.",
            )
    }

    pub fn cli_instance_network_interface_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `instance` is provided as a \
                         `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List network interfaces")
    }

    pub fn cli_instance_network_interface_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("ip")
                    .long("ip")
                    .value_parser(clap::value_parser!(std::net::IpAddr))
                    .required(false)
                    .help(
                        "The IP address for the interface. One will be auto-assigned if not \
                         provided.",
                    ),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `instance` is provided as a \
                         `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("subnet-name")
                    .long("subnet-name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body")
                    .help("The VPC Subnet in which to create the interface."),
            )
            .arg(
                clap::Arg::new("vpc-name")
                    .long("vpc-name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body")
                    .help("The VPC in which to create the interface."),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create network interface")
    }

    pub fn cli_instance_network_interface_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("interface")
                    .long("interface")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the network interface"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `instance` is provided as a \
                         `Name`",
                    ),
            )
            .about("Fetch network interface")
    }

    pub fn cli_instance_network_interface_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("interface")
                    .long("interface")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the network interface"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false),
            )
            .arg(
                clap::Arg::new("primary")
                    .long("primary")
                    .value_parser(clap::value_parser!(bool))
                    .required(false)
                    .help(
                        "Make a secondary interface the instance's primary interface.\n\nIf \
                         applied to a secondary interface, that interface will become the primary \
                         on the next reboot of the instance. Note that this may have implications \
                         for routing between instances, as the new primary interface will be on a \
                         distinct subnet from the previous primary interface.\n\nNote that this \
                         can only be used to select a new primary interface for an instance. \
                         Requests to change the primary interface into a secondary will return an \
                         error.",
                    ),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `instance` is provided as a \
                         `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update network interface")
    }

    pub fn cli_instance_network_interface_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the instance"),
            )
            .arg(
                clap::Arg::new("interface")
                    .long("interface")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the network interface"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `instance` is provided as a \
                         `Name`",
                    ),
            )
            .about("Delete network interface")
            .long_about(
                "Note that the primary interface for an instance cannot be deleted if there are \
                 any secondary interfaces. A new primary interface must be designated first. The \
                 primary interface can be deleted if there are no secondary interfaces.",
            )
    }

    pub fn cli_ping() -> clap::Command {
        clap::Command::new("")
            .about("Ping API")
            .long_about("Always responds with Ok if it responds at all.")
    }

    pub fn cli_policy_view() -> clap::Command {
        clap::Command::new("").about("Fetch current silo's IAM policy")
    }

    pub fn cli_policy_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update current silo's IAM policy")
    }

    pub fn cli_project_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List projects")
    }

    pub fn cli_project_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create project")
    }

    pub fn cli_project_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .about("Fetch project")
    }

    pub fn cli_project_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update a project")
    }

    pub fn cli_project_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .about("Delete project")
    }

    pub fn cli_project_policy_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .about("Fetch project's IAM policy")
    }

    pub fn cli_project_policy_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update project's IAM policy")
    }

    pub fn cli_snapshot_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List snapshots")
    }

    pub fn cli_snapshot_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body")
                    .help("The disk to be snapshotted"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create snapshot")
            .long_about("Creates a point-in-time snapshot from a disk.")
    }

    pub fn cli_snapshot_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("snapshot")
                    .long("snapshot")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the snapshot"),
            )
            .about("Fetch snapshot")
    }

    pub fn cli_snapshot_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("snapshot")
                    .long("snapshot")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the snapshot"),
            )
            .about("Delete snapshot")
    }

    pub fn cli_physical_disk_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List physical disks")
    }

    pub fn cli_physical_disk_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk-id")
                    .long("disk-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("ID of the physical disk"),
            )
            .about("Get a physical disk")
    }

    pub fn cli_rack_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List racks")
    }

    pub fn cli_rack_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("rack-id")
                    .long("rack-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("ID of the rack"),
            )
            .about("Fetch rack")
    }

    pub fn cli_sled_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List sleds")
    }

    pub fn cli_sled_add() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("part")
                    .long("part")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("serial")
                    .long("serial")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Add sled to initialized rack")
    }

    pub fn cli_sled_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("sled-id")
                    .long("sled-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("ID of the sled"),
            )
            .about("Fetch sled")
    }

    pub fn cli_sled_physical_disk_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sled-id")
                    .long("sled-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("ID of the sled"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List physical disks attached to sleds")
    }

    pub fn cli_sled_instance_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sled-id")
                    .long("sled-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("ID of the sled"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List instances running on given sled")
    }

    pub fn cli_sled_set_provision_policy() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("sled-id")
                    .long("sled-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("ID of the sled"),
            )
            .arg(
                clap::Arg::new("state")
                    .long("state")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::SledProvisionPolicy::Provisionable.to_string(),
                            types::SledProvisionPolicy::NonProvisionable.to_string(),
                        ]),
                        |s| types::SledProvisionPolicy::try_from(s).unwrap(),
                    ))
                    .required_unless_present("json-body")
                    .help("The provision state."),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Set sled provision policy")
    }

    pub fn cli_sled_list_uninitialized() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .about("List uninitialized sleds")
    }

    pub fn cli_networking_switch_port_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .arg(
                clap::Arg::new("switch-port-id")
                    .long("switch-port-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(false)
                    .help("An optional switch port id to use when listing switch ports."),
            )
            .about("List switch ports")
    }

    pub fn cli_networking_switch_port_apply_settings() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("port")
                    .long("port")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true)
                    .help("A name to use when selecting switch ports."),
            )
            .arg(
                clap::Arg::new("port-settings")
                    .long("port-settings")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body")
                    .help("A name or id to use when applying switch port settings."),
            )
            .arg(
                clap::Arg::new("rack-id")
                    .long("rack-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("A rack id to use when selecting switch ports."),
            )
            .arg(
                clap::Arg::new("switch-location")
                    .long("switch-location")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true)
                    .help("A switch location to use when selecting switch ports."),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Apply switch port settings")
    }

    pub fn cli_networking_switch_port_clear_settings() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("port")
                    .long("port")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true)
                    .help("A name to use when selecting switch ports."),
            )
            .arg(
                clap::Arg::new("rack-id")
                    .long("rack-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("A rack id to use when selecting switch ports."),
            )
            .arg(
                clap::Arg::new("switch-location")
                    .long("switch-location")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true)
                    .help("A switch location to use when selecting switch ports."),
            )
            .about("Clear switch port settings")
    }

    pub fn cli_networking_switch_port_status() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("port")
                    .long("port")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true)
                    .help("A name to use when selecting switch ports."),
            )
            .arg(
                clap::Arg::new("rack-id")
                    .long("rack-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("A rack id to use when selecting switch ports."),
            )
            .arg(
                clap::Arg::new("switch-location")
                    .long("switch-location")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true)
                    .help("A switch location to use when selecting switch ports."),
            )
            .about("Get switch port status")
    }

    pub fn cli_switch_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List switches")
    }

    pub fn cli_switch_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("switch-id")
                    .long("switch-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("ID of the switch"),
            )
            .about("Fetch switch")
    }

    pub fn cli_silo_identity_provider_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List a silo's IdP's name")
    }

    pub fn cli_local_idp_user_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("external-id")
                    .long("external-id")
                    .value_parser(clap::value_parser!(types::UserId))
                    .required_unless_present("json-body")
                    .help("username used to log in"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create user")
            .long_about(
                "Users can only be created in Silos with `provision_type` == `Fixed`. Otherwise, \
                 Silo users are just-in-time (JIT) provisioned when a user first logs in using an \
                 external Identity Provider.",
            )
    }

    pub fn cli_local_idp_user_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("The user's internal ID"),
            )
            .about("Delete user")
    }

    pub fn cli_local_idp_user_set_password() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("The user's internal ID"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Set or invalidate user's password")
            .long_about(
                "Passwords can only be updated for users in Silos with identity mode `LocalOnly`.",
            )
    }

    pub fn cli_saml_identity_provider_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("acs-url")
                    .long("acs-url")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("service provider endpoint where the response will be sent"),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("group-attribute-name")
                    .long("group-attribute-name")
                    .value_parser(clap::value_parser!(String))
                    .required(false)
                    .help(
                        "If set, SAML attributes with this name will be considered to denote a \
                         user's group membership, where the attribute value(s) should be a \
                         comma-separated list of group names.",
                    ),
            )
            .arg(
                clap::Arg::new("idp-entity-id")
                    .long("idp-entity-id")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("idp's entity id"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("slo-url")
                    .long("slo-url")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("service provider endpoint where the idp should send log out requests"),
            )
            .arg(
                clap::Arg::new("sp-client-id")
                    .long("sp-client-id")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("sp's client id"),
            )
            .arg(
                clap::Arg::new("technical-contact-email")
                    .long("technical-contact-email")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("customer's technical contact for saml configuration"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create SAML IdP")
    }

    pub fn cli_saml_identity_provider_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("provider")
                    .long("provider")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the SAML identity provider"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .about("Fetch SAML IdP")
    }

    pub fn cli_ip_pool_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List IP pools")
    }

    pub fn cli_ip_pool_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create IP pool")
    }

    pub fn cli_ip_pool_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .about("Fetch IP pool")
    }

    pub fn cli_ip_pool_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false),
            )
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update IP pool")
    }

    pub fn cli_ip_pool_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .about("Delete IP pool")
    }

    pub fn cli_ip_pool_range_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .about("List ranges for IP pool")
            .long_about("Ranges are ordered by their first address.")
    }

    pub fn cli_ip_pool_range_add() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Add range to IP pool")
            .long_about("IPv6 ranges are not allowed yet.")
    }

    pub fn cli_ip_pool_range_remove() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Remove range from IP pool")
    }

    pub fn cli_ip_pool_silo_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List IP pool's linked silos")
    }

    pub fn cli_ip_pool_silo_link() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("is-default")
                    .long("is-default")
                    .value_parser(clap::value_parser!(bool))
                    .required_unless_present("json-body")
                    .help(
                        "When a pool is the default for a silo, floating IPs and instance \
                         ephemeral IPs will come from that pool when no other pool is specified. \
                         There can be at most one default for a given silo.",
                    ),
            )
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Link IP pool to silo")
            .long_about(
                "Users in linked silos can allocate external IPs from this pool for their \
                 instances. A silo can have at most one default pool. IPs are allocated from the \
                 default pool when users ask for one without specifying a pool.",
            )
    }

    pub fn cli_ip_pool_silo_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("is-default")
                    .long("is-default")
                    .value_parser(clap::value_parser!(bool))
                    .required_unless_present("json-body")
                    .help(
                        "When a pool is the default for a silo, floating IPs and instance \
                         ephemeral IPs will come from that pool when no other pool is specified. \
                         There can be at most one default for a given silo, so when a pool is \
                         made default, an existing default will remain linked but will no longer \
                         be the default.",
                    ),
            )
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Make IP pool default for silo")
            .long_about(
                "When a user asks for an IP (e.g., at instance create time) without specifying a \
                 pool, the IP comes from the default pool if a default is configured. When a pool \
                 is made the default for a silo, any existing default will remain linked to the \
                 silo, but will no longer be the default.",
            )
    }

    pub fn cli_ip_pool_silo_unlink() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true),
            )
            .about("Unlink IP pool from silo")
            .long_about("Will fail if there are any outstanding IPs allocated in the silo.")
    }

    pub fn cli_ip_pool_utilization_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the IP pool"),
            )
            .about("Fetch IP pool utilization")
    }

    pub fn cli_ip_pool_service_view() -> clap::Command {
        clap::Command::new("").about("Fetch Oxide service IP pool")
    }

    pub fn cli_ip_pool_service_range_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .about("List IP ranges for the Oxide service pool")
            .long_about("Ranges are ordered by their first address.")
    }

    pub fn cli_ip_pool_service_range_add() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Add IP range to Oxide service pool")
            .long_about("IPv6 ranges are not allowed yet.")
    }

    pub fn cli_ip_pool_service_range_remove() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Remove IP range from Oxide service pool")
    }

    pub fn cli_system_metric() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("end-time")
                    .long("end-time")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required(true)
                    .help("An exclusive end time of metrics."),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("metric-name")
                    .long("metric-name")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::SystemMetricName::VirtualDiskSpaceProvisioned.to_string(),
                            types::SystemMetricName::CpusProvisioned.to_string(),
                            types::SystemMetricName::RamProvisioned.to_string(),
                        ]),
                        |s| types::SystemMetricName::try_from(s).unwrap(),
                    ))
                    .required(true),
            )
            .arg(
                clap::Arg::new("order")
                    .long("order")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::PaginationOrder::Ascending.to_string(),
                            types::PaginationOrder::Descending.to_string(),
                        ]),
                        |s| types::PaginationOrder::try_from(s).unwrap(),
                    ))
                    .required(false)
                    .help("Query result order"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("start-time")
                    .long("start-time")
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .required(true)
                    .help("An inclusive start time of metrics."),
            )
            .about("View metrics")
            .long_about(
                "View CPU, memory, or storage utilization metrics at the fleet or silo level.",
            )
    }

    pub fn cli_networking_address_lot_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List address lots")
    }

    pub fn cli_networking_address_lot_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("kind")
                    .long("kind")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::AddressLotKind::Infra.to_string(),
                            types::AddressLotKind::Pool.to_string(),
                        ]),
                        |s| types::AddressLotKind::try_from(s).unwrap(),
                    ))
                    .required_unless_present("json-body")
                    .help("The kind of address lot to create."),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create address lot")
    }

    pub fn cli_networking_address_lot_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("address-lot")
                    .long("address-lot")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the address lot"),
            )
            .about("Delete address lot")
    }

    pub fn cli_networking_address_lot_block_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("address-lot")
                    .long("address-lot")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the address lot"),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List blocks in address lot")
    }

    pub fn cli_networking_allow_list_view() -> clap::Command {
        clap::Command::new("").about("Get user-facing services IP allowlist")
    }

    pub fn cli_networking_allow_list_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update user-facing services IP allowlist")
    }

    pub fn cli_networking_bfd_disable() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("remote")
                    .long("remote")
                    .value_parser(clap::value_parser!(std::net::IpAddr))
                    .required_unless_present("json-body")
                    .help("Address of the remote peer to disable a BFD session for."),
            )
            .arg(
                clap::Arg::new("switch")
                    .long("switch")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body")
                    .help("The switch to enable this session on. Must be `switch0` or `switch1`."),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Disable a BFD session")
    }

    pub fn cli_networking_bfd_enable() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("detection-threshold")
                    .long("detection-threshold")
                    .value_parser(clap::value_parser!(u8))
                    .required_unless_present("json-body")
                    .help(
                        "The negotiated Control packet transmission interval, multiplied by this \
                         variable, will be the Detection Time for this session (as seen by the \
                         remote system)",
                    ),
            )
            .arg(
                clap::Arg::new("local")
                    .long("local")
                    .value_parser(clap::value_parser!(std::net::IpAddr))
                    .required(false)
                    .help(
                        "Address the Oxide switch will listen on for BFD traffic. If `None` then \
                         the unspecified address (0.0.0.0 or ::) is used.",
                    ),
            )
            .arg(
                clap::Arg::new("mode")
                    .long("mode")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::BfdMode::SingleHop.to_string(),
                            types::BfdMode::MultiHop.to_string(),
                        ]),
                        |s| types::BfdMode::try_from(s).unwrap(),
                    ))
                    .required_unless_present("json-body")
                    .help("Select either single-hop (RFC 5881) or multi-hop (RFC 5883)"),
            )
            .arg(
                clap::Arg::new("remote")
                    .long("remote")
                    .value_parser(clap::value_parser!(std::net::IpAddr))
                    .required_unless_present("json-body")
                    .help("Address of the remote peer to establish a BFD session with."),
            )
            .arg(
                clap::Arg::new("required-rx")
                    .long("required-rx")
                    .value_parser(clap::value_parser!(u64))
                    .required_unless_present("json-body")
                    .help(
                        "The minimum interval, in microseconds, between received BFD Control \
                         packets that this system requires",
                    ),
            )
            .arg(
                clap::Arg::new("switch")
                    .long("switch")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body")
                    .help("The switch to enable this session on. Must be `switch0` or `switch1`."),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Enable a BFD session")
    }

    pub fn cli_networking_bfd_status() -> clap::Command {
        clap::Command::new("").about("Get BFD status")
    }

    pub fn cli_networking_bgp_config_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List BGP configurations")
    }

    pub fn cli_networking_bgp_config_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("asn")
                    .long("asn")
                    .value_parser(clap::value_parser!(u32))
                    .required_unless_present("json-body")
                    .help("The autonomous system number of this BGP configuration."),
            )
            .arg(
                clap::Arg::new("bgp-announce-set-id")
                    .long("bgp-announce-set-id")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("vrf")
                    .long("vrf")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false)
                    .help(
                        "Optional virtual routing and forwarding identifier for this BGP \
                         configuration.",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create new BGP configuration")
    }

    pub fn cli_networking_bgp_config_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("name-or-id")
                    .long("name-or-id")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("A name or id to use when selecting BGP config."),
            )
            .about("Delete BGP configuration")
    }

    pub fn cli_networking_bgp_announce_set_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("page-token")
                    .long("page-token")
                    .value_parser(clap::value_parser!(String))
                    .required(false)
                    .help("Token returned by previous call to retrieve the subsequent page"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List BGP announce sets")
    }

    pub fn cli_networking_bgp_announce_set_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update BGP announce set")
            .long_about(
                "If the announce set exists, this endpoint replaces the existing announce set \
                 with the one specified.",
            )
    }

    pub fn cli_networking_bgp_announce_set_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("announce-set")
                    .long("announce-set")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the announce set"),
            )
            .about("Delete BGP announce set")
    }

    pub fn cli_networking_bgp_announcement_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("announce-set")
                    .long("announce-set")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the announce set"),
            )
            .about("Get originated routes for a specified BGP announce set")
    }

    pub fn cli_networking_bgp_exported() -> clap::Command {
        clap::Command::new("").about("Get BGP exported routes")
    }

    pub fn cli_networking_bgp_message_history() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("asn")
                    .long("asn")
                    .value_parser(clap::value_parser!(u32))
                    .required(true)
                    .help("The ASN to filter on. Required."),
            )
            .about("Get BGP router message history")
    }

    pub fn cli_networking_bgp_imported_routes_ipv4() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("asn")
                    .long("asn")
                    .value_parser(clap::value_parser!(u32))
                    .required(true)
                    .help("The ASN to filter on. Required."),
            )
            .about("Get imported IPv4 BGP routes")
    }

    pub fn cli_networking_bgp_status() -> clap::Command {
        clap::Command::new("").about("Get BGP peer status")
    }

    pub fn cli_networking_loopback_address_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List loopback addresses")
    }

    pub fn cli_networking_loopback_address_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("address")
                    .long("address")
                    .value_parser(clap::value_parser!(std::net::IpAddr))
                    .required_unless_present("json-body")
                    .help("The address to create."),
            )
            .arg(
                clap::Arg::new("address-lot")
                    .long("address-lot")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required_unless_present("json-body")
                    .help(
                        "The name or id of the address lot this loopback address will pull an \
                         address from.",
                    ),
            )
            .arg(
                clap::Arg::new("anycast")
                    .long("anycast")
                    .value_parser(clap::value_parser!(bool))
                    .required_unless_present("json-body")
                    .help(
                        "Address is an anycast address. This allows the address to be assigned to \
                         multiple locations simultaneously.",
                    ),
            )
            .arg(
                clap::Arg::new("mask")
                    .long("mask")
                    .value_parser(clap::value_parser!(u8))
                    .required_unless_present("json-body")
                    .help("The subnet mask to use for the address."),
            )
            .arg(
                clap::Arg::new("rack-id")
                    .long("rack-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required_unless_present("json-body")
                    .help("The containing the switch this loopback address will be configured on."),
            )
            .arg(
                clap::Arg::new("switch-location")
                    .long("switch-location")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body")
                    .help(
                        "The location of the switch within the rack this loopback address will be \
                         configured on.",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create loopback address")
    }

    pub fn cli_networking_loopback_address_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("address")
                    .long("address")
                    .value_parser(clap::value_parser!(std::net::IpAddr))
                    .required(true)
                    .help(
                        "The IP address and subnet mask to use when selecting the loopback \
                         address.",
                    ),
            )
            .arg(
                clap::Arg::new("rack-id")
                    .long("rack-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("The rack to use when selecting the loopback address."),
            )
            .arg(
                clap::Arg::new("subnet-mask")
                    .long("subnet-mask")
                    .value_parser(clap::value_parser!(u8))
                    .required(true)
                    .help(
                        "The IP address and subnet mask to use when selecting the loopback \
                         address.",
                    ),
            )
            .arg(
                clap::Arg::new("switch-location")
                    .long("switch-location")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(true)
                    .help("The switch location to use when selecting the loopback address."),
            )
            .about("Delete loopback address")
    }

    pub fn cli_networking_switch_port_settings_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("port-settings")
                    .long("port-settings")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("An optional name or id to use when selecting port settings."),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List switch port settings")
    }

    pub fn cli_networking_switch_port_settings_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create switch port settings")
    }

    pub fn cli_networking_switch_port_settings_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("port-settings")
                    .long("port-settings")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("An optional name or id to use when selecting port settings."),
            )
            .about("Delete switch port settings")
    }

    pub fn cli_networking_switch_port_settings_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("port")
                    .long("port")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("A name or id to use when selecting switch port settings info objects."),
            )
            .about("Get information about switch port")
    }

    pub fn cli_system_policy_view() -> clap::Command {
        clap::Command::new("").about("Fetch top-level IAM policy")
    }

    pub fn cli_system_policy_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update top-level IAM policy")
    }

    pub fn cli_role_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .about("List built-in roles")
    }

    pub fn cli_role_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("role-name")
                    .long("role-name")
                    .value_parser(clap::value_parser!(String))
                    .required(true)
                    .help("The built-in role's unique name."),
            )
            .about("Fetch built-in role")
    }

    pub fn cli_system_quotas_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("Lists resource quotas for all silos")
    }

    pub fn cli_silo_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List silos")
            .long_about("Lists silos that are discoverable based on the current permissions.")
    }

    pub fn cli_silo_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("admin-group-name")
                    .long("admin-group-name")
                    .value_parser(clap::value_parser!(String))
                    .required(false)
                    .help(
                        "If set, this group will be created during Silo creation and granted the \
                         \"Silo Admin\" role. Identity providers can assert that users belong to \
                         this group and those users can log in and further initialize the \
                         Silo.\n\nNote that if configuring a SAML based identity provider, \
                         group_attribute_name must be set for users to be considered part of a \
                         group. See `SamlIdentityProviderCreate` for more information.",
                    ),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("discoverable")
                    .long("discoverable")
                    .value_parser(clap::value_parser!(bool))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("identity-mode")
                    .long("identity-mode")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::SiloIdentityMode::SamlJit.to_string(),
                            types::SiloIdentityMode::LocalOnly.to_string(),
                        ]),
                        |s| types::SiloIdentityMode::try_from(s).unwrap(),
                    ))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create a silo")
    }

    pub fn cli_silo_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .about("Fetch silo")
            .long_about("Fetch silo by name or ID.")
    }

    pub fn cli_silo_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .about("Delete a silo")
            .long_about("Delete a silo by name or ID.")
    }

    pub fn cli_silo_ip_pool_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List IP pools linked to silo")
            .long_about(
                "Linked IP pools are available to users in the specified silo. A silo can have at \
                 most one default pool. IPs are allocated from the default pool when users ask \
                 for one without specifying a pool.",
            )
    }

    pub fn cli_silo_policy_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .about("Fetch silo IAM policy")
    }

    pub fn cli_silo_policy_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update silo IAM policy")
    }

    pub fn cli_silo_quotas_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .about("Fetch resource quotas for silo")
    }

    pub fn cli_silo_quotas_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("cpus")
                    .long("cpus")
                    .value_parser(clap::value_parser!(i64))
                    .required(false)
                    .help("The amount of virtual CPUs available for running instances in the Silo"),
            )
            .arg(
                clap::Arg::new("memory")
                    .long("memory")
                    .value_parser(clap::value_parser!(types::ByteCount))
                    .required(false)
                    .help(
                        "The amount of RAM (in bytes) available for running instances in the Silo",
                    ),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("storage")
                    .long("storage")
                    .value_parser(clap::value_parser!(types::ByteCount))
                    .required(false)
                    .help("The amount of storage (in bytes) available for disks or snapshots"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update resource quotas for silo")
            .long_about("If a quota value is not specified, it will remain unchanged.")
    }

    pub fn cli_silo_user_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List built-in (system) users in silo")
    }

    pub fn cli_silo_user_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .arg(
                clap::Arg::new("user-id")
                    .long("user-id")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(true)
                    .help("The user's internal ID"),
            )
            .about("Fetch built-in (system) user")
    }

    pub fn cli_user_builtin_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameSortMode::NameAscending.to_string(),
                        ]),
                        |s| types::NameSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List built-in users")
    }

    pub fn cli_user_builtin_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("user")
                    .long("user")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true),
            )
            .about("Fetch built-in user")
    }

    pub fn cli_silo_utilization_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List current utilization state for all silos")
    }

    pub fn cli_silo_utilization_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the silo"),
            )
            .about("Fetch current utilization for given silo")
    }

    pub fn cli_timeseries_query() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("query")
                    .long("query")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body")
                    .help("A timeseries query string, written in the Oximeter query language."),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Run timeseries query")
            .long_about("Queries are written in OxQL.")
    }

    pub fn cli_timeseries_schema_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .about("List timeseries schemas")
    }

    pub fn cli_user_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("group")
                    .long("group")
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .required(false),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::IdSortMode::IdAscending.to_string()
                        ]),
                        |s| types::IdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List users")
    }

    pub fn cli_utilization_view() -> clap::Command {
        clap::Command::new("").about("Fetch resource utilization for user's current silo")
    }

    pub fn cli_vpc_firewall_rules_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .about("List firewall rules")
    }

    pub fn cli_vpc_firewall_rules_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Replace firewall rules")
            .long_about(
                "The maximum number of rules per VPC is 1024.\n\nTargets are used to specify the \
                 set of instances to which a firewall rule applies. You can target instances \
                 directly by name, or specify a VPC, VPC subnet, IP, or IP subnet, which will \
                 apply the rule to traffic going to all matching instances. Targets are additive: \
                 the rule applies to instances matching ANY target. The maximum number of targets \
                 is 256.\n\nFilters reduce the scope of a firewall rule. Without filters, the \
                 rule applies to all packets to the targets (or from the targets, if it's an \
                 outbound rule). With multiple filters, the rule applies only to packets matching \
                 ALL filters. The maximum number of each type of filter is 256.",
            )
    }

    pub fn cli_vpc_router_route_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the router"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `router` is provided as a `Name`",
                    ),
            )
            .about("List routes")
            .long_about("List the routes associated with a router in a particular VPC.")
    }

    pub fn cli_vpc_router_route_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the router"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `router` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create route")
    }

    pub fn cli_vpc_router_route_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("route")
                    .long("route")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the route"),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the router"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `router` is provided as a `Name`",
                    ),
            )
            .about("Fetch route")
    }

    pub fn cli_vpc_router_route_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("route")
                    .long("route")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the route"),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the router"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `router` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update route")
    }

    pub fn cli_vpc_router_route_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("route")
                    .long("route")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the route"),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the router"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the VPC, only required if `router` is provided as a `Name`",
                    ),
            )
            .about("Delete route")
    }

    pub fn cli_vpc_router_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .about("List routers")
    }

    pub fn cli_vpc_router_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create VPC router")
    }

    pub fn cli_vpc_router_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the router"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the VPC"),
            )
            .about("Fetch router")
    }

    pub fn cli_vpc_router_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the router"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the VPC"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update router")
    }

    pub fn cli_vpc_router_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the router"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the VPC"),
            )
            .about("Delete router")
    }

    pub fn cli_vpc_subnet_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .about("List subnets")
    }

    pub fn cli_vpc_subnet_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("custom-router")
                    .long("custom-router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "An optional router, used to direct packets sent from hosts in this \
                         subnet to any destination address.\n\nCustom routers apply in addition \
                         to the VPC-wide *system* router, and have higher priority than the \
                         system router for an otherwise equal-prefix-length match.",
                    ),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("ipv4-block")
                    .long("ipv4-block")
                    .value_parser(clap::value_parser!(types::Ipv4Net))
                    .required_unless_present("json-body")
                    .help(
                        "The IPv4 address range for this subnet.\n\nIt must be allocated from an \
                         RFC 1918 private address range, and must not overlap with any other \
                         existing subnet in the VPC.",
                    ),
            )
            .arg(
                clap::Arg::new("ipv6-block")
                    .long("ipv6-block")
                    .value_parser(clap::value_parser!(types::Ipv6Net))
                    .required(false)
                    .help(
                        "The IPv6 address range for this subnet.\n\nIt must be allocated from the \
                         RFC 4193 Unique Local Address range, with the prefix equal to the parent \
                         VPC's prefix. A random `/64` block will be assigned if one is not \
                         provided. It must not overlap with any existing subnet in the VPC.",
                    ),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create subnet")
    }

    pub fn cli_vpc_subnet_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("subnet")
                    .long("subnet")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the subnet"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the VPC"),
            )
            .about("Fetch subnet")
    }

    pub fn cli_vpc_subnet_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("custom-router")
                    .long("custom-router")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "An optional router, used to direct packets sent from hosts in this \
                         subnet to any destination address.",
                    ),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("subnet")
                    .long("subnet")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the subnet"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the VPC"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update subnet")
    }

    pub fn cli_vpc_subnet_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("subnet")
                    .long("subnet")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the subnet"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the VPC"),
            )
            .about("Delete subnet")
    }

    pub fn cli_vpc_subnet_list_network_interfaces() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help(
                        "Name or ID of the project, only required if `vpc` is provided as a `Name`",
                    ),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .arg(
                clap::Arg::new("subnet")
                    .long("subnet")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the subnet"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the VPC"),
            )
            .about("List network interfaces")
    }

    pub fn cli_vpc_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .required(false)
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .value_parser(clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            types::NameOrIdSortMode::NameAscending.to_string(),
                            types::NameOrIdSortMode::NameDescending.to_string(),
                            types::NameOrIdSortMode::IdAscending.to_string(),
                        ]),
                        |s| types::NameOrIdSortMode::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("List VPCs")
    }

    pub fn cli_vpc_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("dns-name")
                    .long("dns-name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("ipv6-prefix")
                    .long("ipv6-prefix")
                    .value_parser(clap::value_parser!(types::Ipv6Net))
                    .required(false)
                    .help(
                        "The IPv6 prefix for this VPC\n\nAll IPv6 subnets created from this VPC \
                         must be taken from this range, which should be a Unique Local Address in \
                         the range `fd00::/48`. The default VPC Subnet will have the first `/64` \
                         range from this prefix.",
                    ),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required_unless_present("json-body"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Create VPC")
    }

    pub fn cli_vpc_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .about("Fetch VPC")
    }

    pub fn cli_vpc_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            )
            .arg(
                clap::Arg::new("dns-name")
                    .long("dns-name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(types::Name))
                    .required(false),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .arg(
                clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Update a VPC")
    }

    pub fn cli_vpc_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(false)
                    .help("Name or ID of the project"),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .value_parser(clap::value_parser!(types::NameOrId))
                    .required(true)
                    .help("Name or ID of the VPC"),
            )
            .about("Delete VPC")
    }

    pub async fn execute(&self, cmd: CliCommand, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        match cmd {
            CliCommand::DeviceAuthRequest => self.execute_device_auth_request(matches).await,
            CliCommand::DeviceAuthConfirm => self.execute_device_auth_confirm(matches).await,
            CliCommand::DeviceAccessToken => self.execute_device_access_token(matches).await,
            CliCommand::ProbeList => self.execute_probe_list(matches).await,
            CliCommand::ProbeCreate => self.execute_probe_create(matches).await,
            CliCommand::ProbeView => self.execute_probe_view(matches).await,
            CliCommand::ProbeDelete => self.execute_probe_delete(matches).await,
            CliCommand::LoginSaml => self.execute_login_saml(matches).await,
            CliCommand::CertificateList => self.execute_certificate_list(matches).await,
            CliCommand::CertificateCreate => self.execute_certificate_create(matches).await,
            CliCommand::CertificateView => self.execute_certificate_view(matches).await,
            CliCommand::CertificateDelete => self.execute_certificate_delete(matches).await,
            CliCommand::DiskList => self.execute_disk_list(matches).await,
            CliCommand::DiskCreate => self.execute_disk_create(matches).await,
            CliCommand::DiskView => self.execute_disk_view(matches).await,
            CliCommand::DiskDelete => self.execute_disk_delete(matches).await,
            CliCommand::DiskBulkWriteImport => self.execute_disk_bulk_write_import(matches).await,
            CliCommand::DiskBulkWriteImportStart => {
                self.execute_disk_bulk_write_import_start(matches).await
            }
            CliCommand::DiskBulkWriteImportStop => {
                self.execute_disk_bulk_write_import_stop(matches).await
            }
            CliCommand::DiskFinalizeImport => self.execute_disk_finalize_import(matches).await,
            CliCommand::DiskMetricsList => self.execute_disk_metrics_list(matches).await,
            CliCommand::FloatingIpList => self.execute_floating_ip_list(matches).await,
            CliCommand::FloatingIpCreate => self.execute_floating_ip_create(matches).await,
            CliCommand::FloatingIpView => self.execute_floating_ip_view(matches).await,
            CliCommand::FloatingIpUpdate => self.execute_floating_ip_update(matches).await,
            CliCommand::FloatingIpDelete => self.execute_floating_ip_delete(matches).await,
            CliCommand::FloatingIpAttach => self.execute_floating_ip_attach(matches).await,
            CliCommand::FloatingIpDetach => self.execute_floating_ip_detach(matches).await,
            CliCommand::GroupList => self.execute_group_list(matches).await,
            CliCommand::GroupView => self.execute_group_view(matches).await,
            CliCommand::ImageList => self.execute_image_list(matches).await,
            CliCommand::ImageCreate => self.execute_image_create(matches).await,
            CliCommand::ImageView => self.execute_image_view(matches).await,
            CliCommand::ImageDelete => self.execute_image_delete(matches).await,
            CliCommand::ImageDemote => self.execute_image_demote(matches).await,
            CliCommand::ImagePromote => self.execute_image_promote(matches).await,
            CliCommand::InstanceList => self.execute_instance_list(matches).await,
            CliCommand::InstanceCreate => self.execute_instance_create(matches).await,
            CliCommand::InstanceView => self.execute_instance_view(matches).await,
            CliCommand::InstanceDelete => self.execute_instance_delete(matches).await,
            CliCommand::InstanceDiskList => self.execute_instance_disk_list(matches).await,
            CliCommand::InstanceDiskAttach => self.execute_instance_disk_attach(matches).await,
            CliCommand::InstanceDiskDetach => self.execute_instance_disk_detach(matches).await,
            CliCommand::InstanceExternalIpList => {
                self.execute_instance_external_ip_list(matches).await
            }
            CliCommand::InstanceEphemeralIpAttach => {
                self.execute_instance_ephemeral_ip_attach(matches).await
            }
            CliCommand::InstanceEphemeralIpDetach => {
                self.execute_instance_ephemeral_ip_detach(matches).await
            }
            CliCommand::InstanceReboot => self.execute_instance_reboot(matches).await,
            CliCommand::InstanceSerialConsole => {
                self.execute_instance_serial_console(matches).await
            }
            CliCommand::InstanceSerialConsoleStream => {
                self.execute_instance_serial_console_stream(matches).await
            }
            CliCommand::InstanceSshPublicKeyList => {
                self.execute_instance_ssh_public_key_list(matches).await
            }
            CliCommand::InstanceStart => self.execute_instance_start(matches).await,
            CliCommand::InstanceStop => self.execute_instance_stop(matches).await,
            CliCommand::InternetGatewayIpAddressList => {
                self.execute_internet_gateway_ip_address_list(matches).await
            }
            CliCommand::InternetGatewayIpAddressCreate => {
                self.execute_internet_gateway_ip_address_create(matches)
                    .await
            }
            CliCommand::InternetGatewayIpAddressDelete => {
                self.execute_internet_gateway_ip_address_delete(matches)
                    .await
            }
            CliCommand::InternetGatewayIpPoolList => {
                self.execute_internet_gateway_ip_pool_list(matches).await
            }
            CliCommand::InternetGatewayIpPoolCreate => {
                self.execute_internet_gateway_ip_pool_create(matches).await
            }
            CliCommand::InternetGatewayIpPoolDelete => {
                self.execute_internet_gateway_ip_pool_delete(matches).await
            }
            CliCommand::InternetGatewayList => self.execute_internet_gateway_list(matches).await,
            CliCommand::InternetGatewayCreate => {
                self.execute_internet_gateway_create(matches).await
            }
            CliCommand::InternetGatewayView => self.execute_internet_gateway_view(matches).await,
            CliCommand::InternetGatewayDelete => {
                self.execute_internet_gateway_delete(matches).await
            }
            CliCommand::ProjectIpPoolList => self.execute_project_ip_pool_list(matches).await,
            CliCommand::ProjectIpPoolView => self.execute_project_ip_pool_view(matches).await,
            CliCommand::LoginLocal => self.execute_login_local(matches).await,
            CliCommand::Logout => self.execute_logout(matches).await,
            CliCommand::CurrentUserView => self.execute_current_user_view(matches).await,
            CliCommand::CurrentUserGroups => self.execute_current_user_groups(matches).await,
            CliCommand::CurrentUserSshKeyList => {
                self.execute_current_user_ssh_key_list(matches).await
            }
            CliCommand::CurrentUserSshKeyCreate => {
                self.execute_current_user_ssh_key_create(matches).await
            }
            CliCommand::CurrentUserSshKeyView => {
                self.execute_current_user_ssh_key_view(matches).await
            }
            CliCommand::CurrentUserSshKeyDelete => {
                self.execute_current_user_ssh_key_delete(matches).await
            }
            CliCommand::SiloMetric => self.execute_silo_metric(matches).await,
            CliCommand::InstanceNetworkInterfaceList => {
                self.execute_instance_network_interface_list(matches).await
            }
            CliCommand::InstanceNetworkInterfaceCreate => {
                self.execute_instance_network_interface_create(matches)
                    .await
            }
            CliCommand::InstanceNetworkInterfaceView => {
                self.execute_instance_network_interface_view(matches).await
            }
            CliCommand::InstanceNetworkInterfaceUpdate => {
                self.execute_instance_network_interface_update(matches)
                    .await
            }
            CliCommand::InstanceNetworkInterfaceDelete => {
                self.execute_instance_network_interface_delete(matches)
                    .await
            }
            CliCommand::Ping => self.execute_ping(matches).await,
            CliCommand::PolicyView => self.execute_policy_view(matches).await,
            CliCommand::PolicyUpdate => self.execute_policy_update(matches).await,
            CliCommand::ProjectList => self.execute_project_list(matches).await,
            CliCommand::ProjectCreate => self.execute_project_create(matches).await,
            CliCommand::ProjectView => self.execute_project_view(matches).await,
            CliCommand::ProjectUpdate => self.execute_project_update(matches).await,
            CliCommand::ProjectDelete => self.execute_project_delete(matches).await,
            CliCommand::ProjectPolicyView => self.execute_project_policy_view(matches).await,
            CliCommand::ProjectPolicyUpdate => self.execute_project_policy_update(matches).await,
            CliCommand::SnapshotList => self.execute_snapshot_list(matches).await,
            CliCommand::SnapshotCreate => self.execute_snapshot_create(matches).await,
            CliCommand::SnapshotView => self.execute_snapshot_view(matches).await,
            CliCommand::SnapshotDelete => self.execute_snapshot_delete(matches).await,
            CliCommand::PhysicalDiskList => self.execute_physical_disk_list(matches).await,
            CliCommand::PhysicalDiskView => self.execute_physical_disk_view(matches).await,
            CliCommand::RackList => self.execute_rack_list(matches).await,
            CliCommand::RackView => self.execute_rack_view(matches).await,
            CliCommand::SledList => self.execute_sled_list(matches).await,
            CliCommand::SledAdd => self.execute_sled_add(matches).await,
            CliCommand::SledView => self.execute_sled_view(matches).await,
            CliCommand::SledPhysicalDiskList => self.execute_sled_physical_disk_list(matches).await,
            CliCommand::SledInstanceList => self.execute_sled_instance_list(matches).await,
            CliCommand::SledSetProvisionPolicy => {
                self.execute_sled_set_provision_policy(matches).await
            }
            CliCommand::SledListUninitialized => {
                self.execute_sled_list_uninitialized(matches).await
            }
            CliCommand::NetworkingSwitchPortList => {
                self.execute_networking_switch_port_list(matches).await
            }
            CliCommand::NetworkingSwitchPortApplySettings => {
                self.execute_networking_switch_port_apply_settings(matches)
                    .await
            }
            CliCommand::NetworkingSwitchPortClearSettings => {
                self.execute_networking_switch_port_clear_settings(matches)
                    .await
            }
            CliCommand::NetworkingSwitchPortStatus => {
                self.execute_networking_switch_port_status(matches).await
            }
            CliCommand::SwitchList => self.execute_switch_list(matches).await,
            CliCommand::SwitchView => self.execute_switch_view(matches).await,
            CliCommand::SiloIdentityProviderList => {
                self.execute_silo_identity_provider_list(matches).await
            }
            CliCommand::LocalIdpUserCreate => self.execute_local_idp_user_create(matches).await,
            CliCommand::LocalIdpUserDelete => self.execute_local_idp_user_delete(matches).await,
            CliCommand::LocalIdpUserSetPassword => {
                self.execute_local_idp_user_set_password(matches).await
            }
            CliCommand::SamlIdentityProviderCreate => {
                self.execute_saml_identity_provider_create(matches).await
            }
            CliCommand::SamlIdentityProviderView => {
                self.execute_saml_identity_provider_view(matches).await
            }
            CliCommand::IpPoolList => self.execute_ip_pool_list(matches).await,
            CliCommand::IpPoolCreate => self.execute_ip_pool_create(matches).await,
            CliCommand::IpPoolView => self.execute_ip_pool_view(matches).await,
            CliCommand::IpPoolUpdate => self.execute_ip_pool_update(matches).await,
            CliCommand::IpPoolDelete => self.execute_ip_pool_delete(matches).await,
            CliCommand::IpPoolRangeList => self.execute_ip_pool_range_list(matches).await,
            CliCommand::IpPoolRangeAdd => self.execute_ip_pool_range_add(matches).await,
            CliCommand::IpPoolRangeRemove => self.execute_ip_pool_range_remove(matches).await,
            CliCommand::IpPoolSiloList => self.execute_ip_pool_silo_list(matches).await,
            CliCommand::IpPoolSiloLink => self.execute_ip_pool_silo_link(matches).await,
            CliCommand::IpPoolSiloUpdate => self.execute_ip_pool_silo_update(matches).await,
            CliCommand::IpPoolSiloUnlink => self.execute_ip_pool_silo_unlink(matches).await,
            CliCommand::IpPoolUtilizationView => {
                self.execute_ip_pool_utilization_view(matches).await
            }
            CliCommand::IpPoolServiceView => self.execute_ip_pool_service_view(matches).await,
            CliCommand::IpPoolServiceRangeList => {
                self.execute_ip_pool_service_range_list(matches).await
            }
            CliCommand::IpPoolServiceRangeAdd => {
                self.execute_ip_pool_service_range_add(matches).await
            }
            CliCommand::IpPoolServiceRangeRemove => {
                self.execute_ip_pool_service_range_remove(matches).await
            }
            CliCommand::SystemMetric => self.execute_system_metric(matches).await,
            CliCommand::NetworkingAddressLotList => {
                self.execute_networking_address_lot_list(matches).await
            }
            CliCommand::NetworkingAddressLotCreate => {
                self.execute_networking_address_lot_create(matches).await
            }
            CliCommand::NetworkingAddressLotDelete => {
                self.execute_networking_address_lot_delete(matches).await
            }
            CliCommand::NetworkingAddressLotBlockList => {
                self.execute_networking_address_lot_block_list(matches)
                    .await
            }
            CliCommand::NetworkingAllowListView => {
                self.execute_networking_allow_list_view(matches).await
            }
            CliCommand::NetworkingAllowListUpdate => {
                self.execute_networking_allow_list_update(matches).await
            }
            CliCommand::NetworkingBfdDisable => self.execute_networking_bfd_disable(matches).await,
            CliCommand::NetworkingBfdEnable => self.execute_networking_bfd_enable(matches).await,
            CliCommand::NetworkingBfdStatus => self.execute_networking_bfd_status(matches).await,
            CliCommand::NetworkingBgpConfigList => {
                self.execute_networking_bgp_config_list(matches).await
            }
            CliCommand::NetworkingBgpConfigCreate => {
                self.execute_networking_bgp_config_create(matches).await
            }
            CliCommand::NetworkingBgpConfigDelete => {
                self.execute_networking_bgp_config_delete(matches).await
            }
            CliCommand::NetworkingBgpAnnounceSetList => {
                self.execute_networking_bgp_announce_set_list(matches).await
            }
            CliCommand::NetworkingBgpAnnounceSetUpdate => {
                self.execute_networking_bgp_announce_set_update(matches)
                    .await
            }
            CliCommand::NetworkingBgpAnnounceSetDelete => {
                self.execute_networking_bgp_announce_set_delete(matches)
                    .await
            }
            CliCommand::NetworkingBgpAnnouncementList => {
                self.execute_networking_bgp_announcement_list(matches).await
            }
            CliCommand::NetworkingBgpExported => {
                self.execute_networking_bgp_exported(matches).await
            }
            CliCommand::NetworkingBgpMessageHistory => {
                self.execute_networking_bgp_message_history(matches).await
            }
            CliCommand::NetworkingBgpImportedRoutesIpv4 => {
                self.execute_networking_bgp_imported_routes_ipv4(matches)
                    .await
            }
            CliCommand::NetworkingBgpStatus => self.execute_networking_bgp_status(matches).await,
            CliCommand::NetworkingLoopbackAddressList => {
                self.execute_networking_loopback_address_list(matches).await
            }
            CliCommand::NetworkingLoopbackAddressCreate => {
                self.execute_networking_loopback_address_create(matches)
                    .await
            }
            CliCommand::NetworkingLoopbackAddressDelete => {
                self.execute_networking_loopback_address_delete(matches)
                    .await
            }
            CliCommand::NetworkingSwitchPortSettingsList => {
                self.execute_networking_switch_port_settings_list(matches)
                    .await
            }
            CliCommand::NetworkingSwitchPortSettingsCreate => {
                self.execute_networking_switch_port_settings_create(matches)
                    .await
            }
            CliCommand::NetworkingSwitchPortSettingsDelete => {
                self.execute_networking_switch_port_settings_delete(matches)
                    .await
            }
            CliCommand::NetworkingSwitchPortSettingsView => {
                self.execute_networking_switch_port_settings_view(matches)
                    .await
            }
            CliCommand::SystemPolicyView => self.execute_system_policy_view(matches).await,
            CliCommand::SystemPolicyUpdate => self.execute_system_policy_update(matches).await,
            CliCommand::RoleList => self.execute_role_list(matches).await,
            CliCommand::RoleView => self.execute_role_view(matches).await,
            CliCommand::SystemQuotasList => self.execute_system_quotas_list(matches).await,
            CliCommand::SiloList => self.execute_silo_list(matches).await,
            CliCommand::SiloCreate => self.execute_silo_create(matches).await,
            CliCommand::SiloView => self.execute_silo_view(matches).await,
            CliCommand::SiloDelete => self.execute_silo_delete(matches).await,
            CliCommand::SiloIpPoolList => self.execute_silo_ip_pool_list(matches).await,
            CliCommand::SiloPolicyView => self.execute_silo_policy_view(matches).await,
            CliCommand::SiloPolicyUpdate => self.execute_silo_policy_update(matches).await,
            CliCommand::SiloQuotasView => self.execute_silo_quotas_view(matches).await,
            CliCommand::SiloQuotasUpdate => self.execute_silo_quotas_update(matches).await,
            CliCommand::SiloUserList => self.execute_silo_user_list(matches).await,
            CliCommand::SiloUserView => self.execute_silo_user_view(matches).await,
            CliCommand::UserBuiltinList => self.execute_user_builtin_list(matches).await,
            CliCommand::UserBuiltinView => self.execute_user_builtin_view(matches).await,
            CliCommand::SiloUtilizationList => self.execute_silo_utilization_list(matches).await,
            CliCommand::SiloUtilizationView => self.execute_silo_utilization_view(matches).await,
            CliCommand::TimeseriesQuery => self.execute_timeseries_query(matches).await,
            CliCommand::TimeseriesSchemaList => self.execute_timeseries_schema_list(matches).await,
            CliCommand::UserList => self.execute_user_list(matches).await,
            CliCommand::UtilizationView => self.execute_utilization_view(matches).await,
            CliCommand::VpcFirewallRulesView => self.execute_vpc_firewall_rules_view(matches).await,
            CliCommand::VpcFirewallRulesUpdate => {
                self.execute_vpc_firewall_rules_update(matches).await
            }
            CliCommand::VpcRouterRouteList => self.execute_vpc_router_route_list(matches).await,
            CliCommand::VpcRouterRouteCreate => self.execute_vpc_router_route_create(matches).await,
            CliCommand::VpcRouterRouteView => self.execute_vpc_router_route_view(matches).await,
            CliCommand::VpcRouterRouteUpdate => self.execute_vpc_router_route_update(matches).await,
            CliCommand::VpcRouterRouteDelete => self.execute_vpc_router_route_delete(matches).await,
            CliCommand::VpcRouterList => self.execute_vpc_router_list(matches).await,
            CliCommand::VpcRouterCreate => self.execute_vpc_router_create(matches).await,
            CliCommand::VpcRouterView => self.execute_vpc_router_view(matches).await,
            CliCommand::VpcRouterUpdate => self.execute_vpc_router_update(matches).await,
            CliCommand::VpcRouterDelete => self.execute_vpc_router_delete(matches).await,
            CliCommand::VpcSubnetList => self.execute_vpc_subnet_list(matches).await,
            CliCommand::VpcSubnetCreate => self.execute_vpc_subnet_create(matches).await,
            CliCommand::VpcSubnetView => self.execute_vpc_subnet_view(matches).await,
            CliCommand::VpcSubnetUpdate => self.execute_vpc_subnet_update(matches).await,
            CliCommand::VpcSubnetDelete => self.execute_vpc_subnet_delete(matches).await,
            CliCommand::VpcSubnetListNetworkInterfaces => {
                self.execute_vpc_subnet_list_network_interfaces(matches)
                    .await
            }
            CliCommand::VpcList => self.execute_vpc_list(matches).await,
            CliCommand::VpcCreate => self.execute_vpc_create(matches).await,
            CliCommand::VpcView => self.execute_vpc_view(matches).await,
            CliCommand::VpcUpdate => self.execute_vpc_update(matches).await,
            CliCommand::VpcDelete => self.execute_vpc_delete(matches).await,
        }
    }

    pub async fn execute_device_auth_request(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.device_auth_request();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.body_map(|body| body.client_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::DeviceAuthRequest>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_device_auth_request(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                todo!()
            }
        }
    }

    pub async fn execute_device_auth_confirm(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.device_auth_confirm();
        if let Some(value) = matches.get_one::<String>("user-code") {
            request = request.body_map(|body| body.user_code(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::DeviceAuthVerify>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_device_auth_confirm(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_device_access_token(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.device_access_token();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.body_map(|body| body.client_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("device-code") {
            request = request.body_map(|body| body.device_code(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("grant-type") {
            request = request.body_map(|body| body.grant_type(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::DeviceAccessTokenRequest>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_device_access_token(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                todo!()
            }
        }
    }

    pub async fn execute_probe_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.probe_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_probe_list(matches, &mut request)?;
        self.config.list_start::<types::ProbeInfoResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::ProbeInfoResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_probe_create(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.probe_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("ip-pool") {
            request = request.body_map(|body| body.ip_pool(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("sled") {
            request = request.body_map(|body| body.sled(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ProbeCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_probe_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_probe_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.probe_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("probe") {
            request = request.probe(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_probe_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_probe_delete(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.probe_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("probe") {
            request = request.probe(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_probe_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_login_saml(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.login_saml();
        if let Some(value) = matches.get_one::<types::Name>("provider-name") {
            request = request.provider_name(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("silo-name") {
            request = request.silo_name(value.clone());
        }

        self.config.execute_login_saml(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_certificate_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.certificate_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_certificate_list(matches, &mut request)?;
        self.config.list_start::<types::CertificateResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::CertificateResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_certificate_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.certificate_create();
        if let Some(value) = matches.get_one::<String>("cert") {
            request = request.body_map(|body| body.cert(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("key") {
            request = request.body_map(|body| body.key(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::ServiceUsingCertificate>("service") {
            request = request.body_map(|body| body.service(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::CertificateCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_certificate_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_certificate_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.certificate_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("certificate") {
            request = request.certificate(value.clone());
        }

        self.config
            .execute_certificate_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_certificate_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.certificate_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("certificate") {
            request = request.certificate(value.clone());
        }

        self.config
            .execute_certificate_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_disk_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.disk_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_disk_list(matches, &mut request)?;
        self.config.list_start::<types::DiskResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::DiskResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_disk_create(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.disk_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::ByteCount>("size") {
            request = request.body_map(|body| body.size(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::DiskCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_disk_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_disk_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.disk_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_disk_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_disk_delete(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.disk_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_disk_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_disk_bulk_write_import(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.disk_bulk_write_import();
        if let Some(value) = matches.get_one::<String>("base64-encoded-data") {
            request = request.body_map(|body| body.base64_encoded_data(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<u64>("offset") {
            request = request.body_map(|body| body.offset(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::ImportBlocksBulkWrite>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_disk_bulk_write_import(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_disk_bulk_write_import_start(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.disk_bulk_write_import_start();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_disk_bulk_write_import_start(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_disk_bulk_write_import_stop(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.disk_bulk_write_import_stop();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_disk_bulk_write_import_stop(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_disk_finalize_import(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.disk_finalize_import();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("snapshot-name") {
            request = request.body_map(|body| body.snapshot_name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::FinalizeDisk>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_disk_finalize_import(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_disk_metrics_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.disk_metrics_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("end-time") {
            request = request.end_time(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::DiskMetricName>("metric") {
            request = request.metric(value.clone());
        }

        if let Some(value) = matches.get_one::<types::PaginationOrder>("order") {
            request = request.order(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("start-time")
        {
            request = request.start_time(value.clone());
        }

        self.config
            .execute_disk_metrics_list(matches, &mut request)?;
        self.config.list_start::<types::MeasurementResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::MeasurementResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_floating_ip_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.floating_ip_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_floating_ip_list(matches, &mut request)?;
        self.config.list_start::<types::FloatingIpResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::FloatingIpResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_floating_ip_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.floating_ip_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::net::IpAddr>("ip") {
            request = request.body_map(|body| body.ip(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.body_map(|body| body.pool(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::FloatingIpCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_floating_ip_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_floating_ip_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.floating_ip_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("floating-ip") {
            request = request.floating_ip(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_floating_ip_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_floating_ip_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.floating_ip_update();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("floating-ip") {
            request = request.floating_ip(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::FloatingIpUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_floating_ip_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_floating_ip_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.floating_ip_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("floating-ip") {
            request = request.floating_ip(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_floating_ip_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_floating_ip_attach(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.floating_ip_attach();
        if let Some(value) = matches.get_one::<types::NameOrId>("floating-ip") {
            request = request.floating_ip(value.clone());
        }

        if let Some(value) = matches.get_one::<types::FloatingIpParentKind>("kind") {
            request = request.body_map(|body| body.kind(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("parent") {
            request = request.body_map(|body| body.parent(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::FloatingIpAttach>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_floating_ip_attach(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_floating_ip_detach(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.floating_ip_detach();
        if let Some(value) = matches.get_one::<types::NameOrId>("floating-ip") {
            request = request.floating_ip(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_floating_ip_detach(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_group_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.group_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_group_list(matches, &mut request)?;
        self.config.list_start::<types::GroupResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::GroupResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_group_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.group_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("group-id") {
            request = request.group_id(value.clone());
        }

        self.config.execute_group_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_image_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.image_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_image_list(matches, &mut request)?;
        self.config.list_start::<types::ImageResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::ImageResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_image_create(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.image_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("os") {
            request = request.body_map(|body| body.os(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("version") {
            request = request.body_map(|body| body.version(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ImageCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_image_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_image_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.image_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("image") {
            request = request.image(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_image_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_image_delete(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.image_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("image") {
            request = request.image(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_image_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_image_demote(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.image_demote();
        if let Some(value) = matches.get_one::<types::NameOrId>("image") {
            request = request.image(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_image_demote(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_image_promote(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.image_promote();
        if let Some(value) = matches.get_one::<types::NameOrId>("image") {
            request = request.image(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_image_promote(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.instance_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_instance_list(matches, &mut request)?;
        self.config.list_start::<types::InstanceResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::InstanceResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_instance_create(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.instance_create();
        if let Some(value) =
            matches.get_one::<types::InstanceAutoRestartPolicy>("auto-restart-policy")
        {
            request = request.body_map(|body| body.auto_restart_policy(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Hostname>("hostname") {
            request = request.body_map(|body| body.hostname(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::ByteCount>("memory") {
            request = request.body_map(|body| body.memory(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::InstanceCpuCount>("ncpus") {
            request = request.body_map(|body| body.ncpus(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<bool>("start") {
            request = request.body_map(|body| body.start(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("user-data") {
            request = request.body_map(|body| body.user_data(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::InstanceCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_instance_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.instance_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_instance_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_delete(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.instance_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_instance_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_disk_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_disk_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_instance_disk_list(matches, &mut request)?;
        self.config.list_start::<types::DiskResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::DiskResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_instance_disk_attach(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_disk_attach();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.body_map(|body| body.disk(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::DiskPath>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_instance_disk_attach(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_disk_detach(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_disk_detach();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.body_map(|body| body.disk(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::DiskPath>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_instance_disk_detach(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_external_ip_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_external_ip_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_instance_external_ip_list(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_ephemeral_ip_attach(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_ephemeral_ip_attach();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.body_map(|body| body.pool(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::EphemeralIpCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_instance_ephemeral_ip_attach(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_ephemeral_ip_detach(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_ephemeral_ip_detach();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_instance_ephemeral_ip_detach(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_reboot(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.instance_reboot();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_instance_reboot(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_serial_console(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_serial_console();
        if let Some(value) = matches.get_one::<u64>("from-start") {
            request = request.from_start(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<u64>("max-bytes") {
            request = request.max_bytes(value.clone());
        }

        if let Some(value) = matches.get_one::<u64>("most-recent") {
            request = request.most_recent(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_instance_serial_console(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_serial_console_stream(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_serial_console_stream();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<u64>("most-recent") {
            request = request.most_recent(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_instance_serial_console_stream(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                todo!()
            }
        }
    }

    pub async fn execute_instance_ssh_public_key_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_ssh_public_key_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_instance_ssh_public_key_list(matches, &mut request)?;
        self.config.list_start::<types::SshKeyResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::SshKeyResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_instance_start(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.instance_start();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_instance_start(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_stop(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.instance_stop();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_instance_stop(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_internet_gateway_ip_address_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_ip_address_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("gateway") {
            request = request.gateway(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_internet_gateway_ip_address_list(matches, &mut request)?;
        self.config
            .list_start::<types::InternetGatewayIpAddressResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::InternetGatewayIpAddressResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_internet_gateway_ip_address_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_ip_address_create();
        if let Some(value) = matches.get_one::<std::net::IpAddr>("address") {
            request = request.body_map(|body| body.address(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("gateway") {
            request = request.body_map(|body| body.gateway(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InternetGatewayIpAddressCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_internet_gateway_ip_address_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_internet_gateway_ip_address_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_ip_address_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("address") {
            request = request.address(value.clone());
        }

        if let Some(value) = matches.get_one::<bool>("cascade") {
            request = request.cascade(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("gateway") {
            request = request.gateway(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_internet_gateway_ip_address_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_internet_gateway_ip_pool_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_ip_pool_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("gateway") {
            request = request.gateway(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_internet_gateway_ip_pool_list(matches, &mut request)?;
        self.config
            .list_start::<types::InternetGatewayIpPoolResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::InternetGatewayIpPoolResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_internet_gateway_ip_pool_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_ip_pool_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("gateway") {
            request = request.body_map(|body| body.gateway(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("ip-pool") {
            request = request.body_map(|body| body.ip_pool(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InternetGatewayIpPoolCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_internet_gateway_ip_pool_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_internet_gateway_ip_pool_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_ip_pool_delete();
        if let Some(value) = matches.get_one::<bool>("cascade") {
            request = request.cascade(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("gateway") {
            request = request.gateway(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_internet_gateway_ip_pool_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_internet_gateway_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_internet_gateway_list(matches, &mut request)?;
        self.config
            .list_start::<types::InternetGatewayResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::InternetGatewayResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_internet_gateway_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InternetGatewayCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_internet_gateway_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_internet_gateway_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("gateway") {
            request = request.gateway(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_internet_gateway_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_internet_gateway_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.internet_gateway_delete();
        if let Some(value) = matches.get_one::<bool>("cascade") {
            request = request.cascade(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("gateway") {
            request = request.gateway(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_internet_gateway_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_project_ip_pool_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.project_ip_pool_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_project_ip_pool_list(matches, &mut request)?;
        self.config.list_start::<types::SiloIpPoolResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::SiloIpPoolResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_project_ip_pool_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.project_ip_pool_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.config
            .execute_project_ip_pool_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_login_local(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.login_local();
        if let Some(value) = matches.get_one::<types::Password>("password") {
            request = request.body_map(|body| body.password(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("silo-name") {
            request = request.silo_name(value.clone());
        }

        if let Some(value) = matches.get_one::<types::UserId>("username") {
            request = request.body_map(|body| body.username(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::UsernamePasswordCredentials>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_login_local(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_logout(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.logout();
        self.config.execute_logout(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_current_user_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.current_user_view();
        self.config
            .execute_current_user_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_current_user_groups(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.current_user_groups();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_current_user_groups(matches, &mut request)?;
        self.config.list_start::<types::GroupResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::GroupResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_current_user_ssh_key_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.current_user_ssh_key_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_current_user_ssh_key_list(matches, &mut request)?;
        self.config.list_start::<types::SshKeyResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::SshKeyResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_current_user_ssh_key_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.current_user_ssh_key_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("public-key") {
            request = request.body_map(|body| body.public_key(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::SshKeyCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_current_user_ssh_key_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_current_user_ssh_key_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.current_user_ssh_key_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("ssh-key") {
            request = request.ssh_key(value.clone());
        }

        self.config
            .execute_current_user_ssh_key_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_current_user_ssh_key_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.current_user_ssh_key_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("ssh-key") {
            request = request.ssh_key(value.clone());
        }

        self.config
            .execute_current_user_ssh_key_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_metric(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.silo_metric();
        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("end-time") {
            request = request.end_time(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::SystemMetricName>("metric-name") {
            request = request.metric_name(value.clone());
        }

        if let Some(value) = matches.get_one::<types::PaginationOrder>("order") {
            request = request.order(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("start-time")
        {
            request = request.start_time(value.clone());
        }

        self.config.execute_silo_metric(matches, &mut request)?;
        self.config.list_start::<types::MeasurementResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::MeasurementResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_instance_network_interface_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_network_interface_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_instance_network_interface_list(matches, &mut request)?;
        self.config
            .list_start::<types::InstanceNetworkInterfaceResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::InstanceNetworkInterfaceResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_instance_network_interface_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_network_interface_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<std::net::IpAddr>("ip") {
            request = request.body_map(|body| body.ip(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("subnet-name") {
            request = request.body_map(|body| body.subnet_name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("vpc-name") {
            request = request.body_map(|body| body.vpc_name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InstanceNetworkInterfaceCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_instance_network_interface_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_network_interface_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_network_interface_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("interface") {
            request = request.interface(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_instance_network_interface_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_network_interface_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_network_interface_update();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("interface") {
            request = request.interface(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<bool>("primary") {
            request = request.body_map(|body| body.primary(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InstanceNetworkInterfaceUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_instance_network_interface_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_instance_network_interface_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_network_interface_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("interface") {
            request = request.interface(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_instance_network_interface_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ping(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ping();
        self.config.execute_ping(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_policy_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.policy_view();
        self.config.execute_policy_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_policy_update(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.policy_update();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::SiloRolePolicy>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_policy_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_project_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.project_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_project_list(matches, &mut request)?;
        self.config.list_start::<types::ProjectResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::ProjectResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_project_create(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.project_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ProjectCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_project_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_project_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.project_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_project_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_project_update(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.project_update();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ProjectUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_project_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_project_delete(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.project_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config.execute_project_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_project_policy_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.project_policy_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.config
            .execute_project_policy_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_project_policy_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.project_policy_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ProjectRolePolicy>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_project_policy_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_snapshot_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.snapshot_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_snapshot_list(matches, &mut request)?;
        self.config.list_start::<types::SnapshotResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::SnapshotResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_snapshot_create(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.snapshot_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.body_map(|body| body.disk(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::SnapshotCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_snapshot_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_snapshot_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.snapshot_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("snapshot") {
            request = request.snapshot(value.clone());
        }

        self.config.execute_snapshot_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_snapshot_delete(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.snapshot_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("snapshot") {
            request = request.snapshot(value.clone());
        }

        self.config.execute_snapshot_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_physical_disk_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.physical_disk_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_physical_disk_list(matches, &mut request)?;
        self.config.list_start::<types::PhysicalDiskResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::PhysicalDiskResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_physical_disk_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.physical_disk_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("disk-id") {
            request = request.disk_id(value.clone());
        }

        self.config
            .execute_physical_disk_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_rack_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.rack_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_rack_list(matches, &mut request)?;
        self.config.list_start::<types::RackResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::RackResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_rack_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.rack_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("rack-id") {
            request = request.rack_id(value.clone());
        }

        self.config.execute_rack_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_sled_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.sled_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_sled_list(matches, &mut request)?;
        self.config.list_start::<types::SledResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::SledResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_sled_add(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.sled_add();
        if let Some(value) = matches.get_one::<String>("part") {
            request = request.body_map(|body| body.part(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("serial") {
            request = request.body_map(|body| body.serial(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::UninitializedSledId>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_sled_add(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_sled_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.sled_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("sled-id") {
            request = request.sled_id(value.clone());
        }

        self.config.execute_sled_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_sled_physical_disk_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.sled_physical_disk_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("sled-id") {
            request = request.sled_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_sled_physical_disk_list(matches, &mut request)?;
        self.config.list_start::<types::PhysicalDiskResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::PhysicalDiskResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_sled_instance_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.sled_instance_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("sled-id") {
            request = request.sled_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_sled_instance_list(matches, &mut request)?;
        self.config.list_start::<types::SledInstanceResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::SledInstanceResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_sled_set_provision_policy(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.sled_set_provision_policy();
        if let Some(value) = matches.get_one::<uuid::Uuid>("sled-id") {
            request = request.sled_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::SledProvisionPolicy>("state") {
            request = request.body_map(|body| body.state(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::SledProvisionPolicyParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_sled_set_provision_policy(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_sled_list_uninitialized(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.sled_list_uninitialized();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        self.config
            .execute_sled_list_uninitialized(matches, &mut request)?;
        self.config
            .list_start::<types::UninitializedSledResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::UninitializedSledResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_networking_switch_port_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_switch_port_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("switch-port-id") {
            request = request.switch_port_id(value.clone());
        }

        self.config
            .execute_networking_switch_port_list(matches, &mut request)?;
        self.config.list_start::<types::SwitchPortResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::SwitchPortResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_networking_switch_port_apply_settings(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_switch_port_apply_settings();
        if let Some(value) = matches.get_one::<types::Name>("port") {
            request = request.port(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("port-settings") {
            request = request.body_map(|body| body.port_settings(value.clone()))
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("rack-id") {
            request = request.rack_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("switch-location") {
            request = request.switch_location(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::SwitchPortApplySettings>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_networking_switch_port_apply_settings(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_switch_port_clear_settings(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_switch_port_clear_settings();
        if let Some(value) = matches.get_one::<types::Name>("port") {
            request = request.port(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("rack-id") {
            request = request.rack_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("switch-location") {
            request = request.switch_location(value.clone());
        }

        self.config
            .execute_networking_switch_port_clear_settings(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_switch_port_status(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_switch_port_status();
        if let Some(value) = matches.get_one::<types::Name>("port") {
            request = request.port(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("rack-id") {
            request = request.rack_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("switch-location") {
            request = request.switch_location(value.clone());
        }

        self.config
            .execute_networking_switch_port_status(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_switch_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.switch_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_switch_list(matches, &mut request)?;
        self.config.list_start::<types::SwitchResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::SwitchResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_switch_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.switch_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("switch-id") {
            request = request.switch_id(value.clone());
        }

        self.config.execute_switch_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_identity_provider_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.silo_identity_provider_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_silo_identity_provider_list(matches, &mut request)?;
        self.config
            .list_start::<types::IdentityProviderResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::IdentityProviderResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_local_idp_user_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.local_idp_user_create();
        if let Some(value) = matches.get_one::<types::UserId>("external-id") {
            request = request.body_map(|body| body.external_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::UserCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_local_idp_user_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_local_idp_user_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.local_idp_user_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("user-id") {
            request = request.user_id(value.clone());
        }

        self.config
            .execute_local_idp_user_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_local_idp_user_set_password(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.local_idp_user_set_password();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("user-id") {
            request = request.user_id(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::UserPassword>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_local_idp_user_set_password(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_saml_identity_provider_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.saml_identity_provider_create();
        if let Some(value) = matches.get_one::<String>("acs-url") {
            request = request.body_map(|body| body.acs_url(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("group-attribute-name") {
            request = request.body_map(|body| body.group_attribute_name(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("idp-entity-id") {
            request = request.body_map(|body| body.idp_entity_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("slo-url") {
            request = request.body_map(|body| body.slo_url(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("sp-client-id") {
            request = request.body_map(|body| body.sp_client_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("technical-contact-email") {
            request = request.body_map(|body| body.technical_contact_email(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::SamlIdentityProviderCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_saml_identity_provider_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_saml_identity_provider_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.saml_identity_provider_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.config
            .execute_saml_identity_provider_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_ip_pool_list(matches, &mut request)?;
        self.config.list_start::<types::IpPoolResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::IpPoolResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_ip_pool_create(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::IpPoolCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_ip_pool_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.config.execute_ip_pool_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_update(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_update();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::IpPoolUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_ip_pool_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_delete(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.config.execute_ip_pool_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_range_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_range_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.config
            .execute_ip_pool_range_list(matches, &mut request)?;
        self.config.list_start::<types::IpPoolRangeResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::IpPoolRangeResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_ip_pool_range_add(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_range_add();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::IpRange>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_ip_pool_range_add(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_range_remove(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_range_remove();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::IpRange>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_ip_pool_range_remove(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_silo_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_silo_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_ip_pool_silo_list(matches, &mut request)?;
        self.config.list_start::<types::IpPoolSiloLinkResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::IpPoolSiloLinkResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_ip_pool_silo_link(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_silo_link();
        if let Some(value) = matches.get_one::<bool>("is-default") {
            request = request.body_map(|body| body.is_default(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.body_map(|body| body.silo(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::IpPoolLinkSilo>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_ip_pool_silo_link(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_silo_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_silo_update();
        if let Some(value) = matches.get_one::<bool>("is-default") {
            request = request.body_map(|body| body.is_default(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::IpPoolSiloUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_ip_pool_silo_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_silo_unlink(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_silo_unlink();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.config
            .execute_ip_pool_silo_unlink(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_utilization_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_utilization_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.config
            .execute_ip_pool_utilization_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_service_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_service_view();
        self.config
            .execute_ip_pool_service_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_service_range_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_service_range_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        self.config
            .execute_ip_pool_service_range_list(matches, &mut request)?;
        self.config.list_start::<types::IpPoolRangeResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::IpPoolRangeResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_ip_pool_service_range_add(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_service_range_add();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::IpRange>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_ip_pool_service_range_add(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_ip_pool_service_range_remove(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.ip_pool_service_range_remove();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::IpRange>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_ip_pool_service_range_remove(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_system_metric(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.system_metric();
        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("end-time") {
            request = request.end_time(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::SystemMetricName>("metric-name") {
            request = request.metric_name(value.clone());
        }

        if let Some(value) = matches.get_one::<types::PaginationOrder>("order") {
            request = request.order(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("start-time")
        {
            request = request.start_time(value.clone());
        }

        self.config.execute_system_metric(matches, &mut request)?;
        self.config.list_start::<types::MeasurementResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::MeasurementResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_networking_address_lot_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_address_lot_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_networking_address_lot_list(matches, &mut request)?;
        self.config.list_start::<types::AddressLotResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::AddressLotResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_networking_address_lot_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_address_lot_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::AddressLotKind>("kind") {
            request = request.body_map(|body| body.kind(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::AddressLotCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_networking_address_lot_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_address_lot_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_address_lot_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("address-lot") {
            request = request.address_lot(value.clone());
        }

        self.config
            .execute_networking_address_lot_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_address_lot_block_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_address_lot_block_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("address-lot") {
            request = request.address_lot(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_networking_address_lot_block_list(matches, &mut request)?;
        self.config
            .list_start::<types::AddressLotBlockResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::AddressLotBlockResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_networking_allow_list_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_allow_list_view();
        self.config
            .execute_networking_allow_list_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_allow_list_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_allow_list_update();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::AllowListUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_networking_allow_list_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bfd_disable(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bfd_disable();
        if let Some(value) = matches.get_one::<std::net::IpAddr>("remote") {
            request = request.body_map(|body| body.remote(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("switch") {
            request = request.body_map(|body| body.switch(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::BfdSessionDisable>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_networking_bfd_disable(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bfd_enable(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bfd_enable();
        if let Some(value) = matches.get_one::<u8>("detection-threshold") {
            request = request.body_map(|body| body.detection_threshold(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::net::IpAddr>("local") {
            request = request.body_map(|body| body.local(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::BfdMode>("mode") {
            request = request.body_map(|body| body.mode(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::net::IpAddr>("remote") {
            request = request.body_map(|body| body.remote(value.clone()))
        }

        if let Some(value) = matches.get_one::<u64>("required-rx") {
            request = request.body_map(|body| body.required_rx(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("switch") {
            request = request.body_map(|body| body.switch(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::BfdSessionEnable>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_networking_bfd_enable(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bfd_status(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bfd_status();
        self.config
            .execute_networking_bfd_status(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_config_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_config_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_networking_bgp_config_list(matches, &mut request)?;
        self.config.list_start::<types::BgpConfigResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::BgpConfigResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_networking_bgp_config_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_config_create();
        if let Some(value) = matches.get_one::<u32>("asn") {
            request = request.body_map(|body| body.asn(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("bgp-announce-set-id") {
            request = request.body_map(|body| body.bgp_announce_set_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("vrf") {
            request = request.body_map(|body| body.vrf(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::BgpConfigCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_networking_bgp_config_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_config_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_config_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("name-or-id") {
            request = request.name_or_id(value.clone());
        }

        self.config
            .execute_networking_bgp_config_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_announce_set_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_announce_set_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("page-token") {
            request = request.page_token(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_networking_bgp_announce_set_list(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_announce_set_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_announce_set_update();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::BgpAnnounceSetCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_networking_bgp_announce_set_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_announce_set_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_announce_set_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("announce-set") {
            request = request.announce_set(value.clone());
        }

        self.config
            .execute_networking_bgp_announce_set_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_announcement_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_announcement_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("announce-set") {
            request = request.announce_set(value.clone());
        }

        self.config
            .execute_networking_bgp_announcement_list(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_exported(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_exported();
        self.config
            .execute_networking_bgp_exported(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_message_history(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_message_history();
        if let Some(value) = matches.get_one::<u32>("asn") {
            request = request.asn(value.clone());
        }

        self.config
            .execute_networking_bgp_message_history(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_imported_routes_ipv4(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_imported_routes_ipv4();
        if let Some(value) = matches.get_one::<u32>("asn") {
            request = request.asn(value.clone());
        }

        self.config
            .execute_networking_bgp_imported_routes_ipv4(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_bgp_status(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_bgp_status();
        self.config
            .execute_networking_bgp_status(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_loopback_address_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_loopback_address_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_networking_loopback_address_list(matches, &mut request)?;
        self.config
            .list_start::<types::LoopbackAddressResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::LoopbackAddressResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_networking_loopback_address_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_loopback_address_create();
        if let Some(value) = matches.get_one::<std::net::IpAddr>("address") {
            request = request.body_map(|body| body.address(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("address-lot") {
            request = request.body_map(|body| body.address_lot(value.clone()))
        }

        if let Some(value) = matches.get_one::<bool>("anycast") {
            request = request.body_map(|body| body.anycast(value.clone()))
        }

        if let Some(value) = matches.get_one::<u8>("mask") {
            request = request.body_map(|body| body.mask(value.clone()))
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("rack-id") {
            request = request.body_map(|body| body.rack_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("switch-location") {
            request = request.body_map(|body| body.switch_location(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::LoopbackAddressCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_networking_loopback_address_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_loopback_address_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_loopback_address_delete();
        if let Some(value) = matches.get_one::<std::net::IpAddr>("address") {
            request = request.address(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("rack-id") {
            request = request.rack_id(value.clone());
        }

        if let Some(value) = matches.get_one::<u8>("subnet-mask") {
            request = request.subnet_mask(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("switch-location") {
            request = request.switch_location(value.clone());
        }

        self.config
            .execute_networking_loopback_address_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_switch_port_settings_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_switch_port_settings_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("port-settings") {
            request = request.port_settings(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_networking_switch_port_settings_list(matches, &mut request)?;
        self.config
            .list_start::<types::SwitchPortSettingsResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::SwitchPortSettingsResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_networking_switch_port_settings_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_switch_port_settings_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::SwitchPortSettingsCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_networking_switch_port_settings_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_switch_port_settings_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_switch_port_settings_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("port-settings") {
            request = request.port_settings(value.clone());
        }

        self.config
            .execute_networking_switch_port_settings_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_networking_switch_port_settings_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.networking_switch_port_settings_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("port") {
            request = request.port(value.clone());
        }

        self.config
            .execute_networking_switch_port_settings_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_system_policy_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.system_policy_view();
        self.config
            .execute_system_policy_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_system_policy_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.system_policy_update();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::FleetRolePolicy>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_system_policy_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_role_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.role_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        self.config.execute_role_list(matches, &mut request)?;
        self.config.list_start::<types::RoleResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::RoleResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_role_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.role_view();
        if let Some(value) = matches.get_one::<String>("role-name") {
            request = request.role_name(value.clone());
        }

        self.config.execute_role_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_system_quotas_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.system_quotas_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_system_quotas_list(matches, &mut request)?;
        self.config.list_start::<types::SiloQuotasResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::SiloQuotasResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_silo_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.silo_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_silo_list(matches, &mut request)?;
        self.config.list_start::<types::SiloResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::SiloResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_silo_create(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.silo_create();
        if let Some(value) = matches.get_one::<String>("admin-group-name") {
            request = request.body_map(|body| body.admin_group_name(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<bool>("discoverable") {
            request = request.body_map(|body| body.discoverable(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::SiloIdentityMode>("identity-mode") {
            request = request.body_map(|body| body.identity_mode(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::SiloCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_silo_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.silo_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.config.execute_silo_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_delete(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.silo_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.config.execute_silo_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_ip_pool_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.silo_ip_pool_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_silo_ip_pool_list(matches, &mut request)?;
        self.config.list_start::<types::SiloIpPoolResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::SiloIpPoolResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_silo_policy_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.silo_policy_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.config
            .execute_silo_policy_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_policy_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.silo_policy_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::SiloRolePolicy>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_silo_policy_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_quotas_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.silo_quotas_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.config
            .execute_silo_quotas_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_quotas_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.silo_quotas_update();
        if let Some(value) = matches.get_one::<i64>("cpus") {
            request = request.body_map(|body| body.cpus(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::ByteCount>("memory") {
            request = request.body_map(|body| body.memory(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<types::ByteCount>("storage") {
            request = request.body_map(|body| body.storage(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::SiloQuotasUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_silo_quotas_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_user_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.silo_user_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_silo_user_list(matches, &mut request)?;
        self.config.list_start::<types::UserResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::UserResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_silo_user_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.silo_user_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("user-id") {
            request = request.user_id(value.clone());
        }

        self.config.execute_silo_user_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_user_builtin_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.user_builtin_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_user_builtin_list(matches, &mut request)?;
        self.config.list_start::<types::UserBuiltinResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::UserBuiltinResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_user_builtin_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.user_builtin_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("user") {
            request = request.user(value.clone());
        }

        self.config
            .execute_user_builtin_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_silo_utilization_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.silo_utilization_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config
            .execute_silo_utilization_list(matches, &mut request)?;
        self.config
            .list_start::<types::SiloUtilizationResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::SiloUtilizationResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_silo_utilization_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.silo_utilization_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.config
            .execute_silo_utilization_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_timeseries_query(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.timeseries_query();
        if let Some(value) = matches.get_one::<String>("query") {
            request = request.body_map(|body| body.query(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::TimeseriesQuery>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_timeseries_query(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_timeseries_schema_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.timeseries_schema_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        self.config
            .execute_timeseries_schema_list(matches, &mut request)?;
        self.config
            .list_start::<types::TimeseriesSchemaResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::TimeseriesSchemaResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_user_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.user_list();
        if let Some(value) = matches.get_one::<uuid::Uuid>("group") {
            request = request.group(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_user_list(matches, &mut request)?;
        self.config.list_start::<types::UserResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::UserResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_utilization_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.utilization_view();
        self.config
            .execute_utilization_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_firewall_rules_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_firewall_rules_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_vpc_firewall_rules_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_firewall_rules_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_firewall_rules_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::VpcFirewallRuleUpdateParams>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_vpc_firewall_rules_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_router_route_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_route_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_vpc_router_route_list(matches, &mut request)?;
        self.config.list_start::<types::RouterRouteResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::RouterRouteResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_vpc_router_route_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_route_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::RouterRouteCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_vpc_router_route_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_router_route_view(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_route_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("route") {
            request = request.route(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_vpc_router_route_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_router_route_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_route_update();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("route") {
            request = request.route(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::RouterRouteUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_vpc_router_route_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_router_route_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_route_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("route") {
            request = request.route(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_vpc_router_route_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_router_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config.execute_vpc_router_list(matches, &mut request)?;
        self.config.list_start::<types::VpcRouterResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::VpcRouterResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_vpc_router_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::VpcRouterCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_vpc_router_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_router_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config.execute_vpc_router_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_router_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_update();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::VpcRouterUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_vpc_router_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_router_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_router_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_vpc_router_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_subnet_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.vpc_subnet_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config.execute_vpc_subnet_list(matches, &mut request)?;
        self.config.list_start::<types::VpcSubnetResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::VpcSubnetResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_vpc_subnet_create(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_subnet_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("custom-router") {
            request = request.body_map(|body| body.custom_router(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Ipv4Net>("ipv4-block") {
            request = request.body_map(|body| body.ipv4_block(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Ipv6Net>("ipv6-block") {
            request = request.body_map(|body| body.ipv6_block(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::VpcSubnetCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_vpc_subnet_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_subnet_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.vpc_subnet_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("subnet") {
            request = request.subnet(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config.execute_vpc_subnet_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_subnet_update(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_subnet_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("custom-router") {
            request = request.body_map(|body| body.custom_router(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("subnet") {
            request = request.subnet(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::VpcSubnetUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_vpc_subnet_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_subnet_delete(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_subnet_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("subnet") {
            request = request.subnet(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_vpc_subnet_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_subnet_list_network_interfaces(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.vpc_subnet_list_network_interfaces();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("subnet") {
            request = request.subnet(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config
            .execute_vpc_subnet_list_network_interfaces(matches, &mut request)?;
        self.config
            .list_start::<types::InstanceNetworkInterfaceResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config
                        .list_end_success::<types::InstanceNetworkInterfaceResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_vpc_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.vpc_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.config.execute_vpc_list(matches, &mut request)?;
        self.config.list_start::<types::VpcResultsPage>();
        let mut stream = futures::StreamExt::take(
            request.stream(),
            matches
                .get_one::<std::num::NonZeroU32>("limit")
                .map_or(usize::MAX, |x| x.get() as usize),
        );
        loop {
            match futures::TryStreamExt::try_next(&mut stream).await {
                Err(r) => {
                    self.config.list_end_error(&r);
                    return Err(anyhow::Error::new(r));
                }
                Ok(None) => {
                    self.config.list_end_success::<types::VpcResultsPage>();
                    return Ok(());
                }
                Ok(Some(value)) => {
                    self.config.list_item(&value);
                }
            }
        }
    }

    pub async fn execute_vpc_create(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.vpc_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("dns-name") {
            request = request.body_map(|body| body.dns_name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Ipv6Net>("ipv6-prefix") {
            request = request.body_map(|body| body.ipv6_prefix(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::VpcCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_vpc_create(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_view(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.vpc_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config.execute_vpc_view(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_update(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.vpc_update();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("dns-name") {
            request = request.body_map(|body| body.dns_name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::VpcUpdate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_vpc_update(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }

    pub async fn execute_vpc_delete(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.vpc_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.config.execute_vpc_delete(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
}

pub trait CliConfig {
    fn success_item<T>(&self, value: &ResponseValue<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn success_no_item(&self, value: &ResponseValue<()>);
    fn error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_start<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn execute_device_auth_request(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeviceAuthRequest,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_device_auth_confirm(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeviceAuthConfirm,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_device_access_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeviceAccessToken,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_probe_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProbeList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_probe_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProbeCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_probe_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProbeView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_probe_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProbeDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_login_saml(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LoginSaml,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_certificate_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CertificateList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_certificate_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CertificateCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_certificate_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CertificateView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_certificate_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CertificateDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_disk_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_disk_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_disk_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_disk_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_disk_bulk_write_import(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskBulkWriteImport,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_disk_bulk_write_import_start(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskBulkWriteImportStart,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_disk_bulk_write_import_stop(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskBulkWriteImportStop,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_disk_finalize_import(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskFinalizeImport,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_disk_metrics_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskMetricsList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_floating_ip_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::FloatingIpList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_floating_ip_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::FloatingIpCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_floating_ip_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::FloatingIpView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_floating_ip_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::FloatingIpUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_floating_ip_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::FloatingIpDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_floating_ip_attach(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::FloatingIpAttach,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_floating_ip_detach(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::FloatingIpDetach,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_group_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GroupList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_group_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GroupView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_image_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImageList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_image_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImageCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_image_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImageView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_image_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImageDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_image_demote(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImageDemote,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_image_promote(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImagePromote,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_disk_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceDiskList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_disk_attach(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceDiskAttach,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_disk_detach(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceDiskDetach,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_external_ip_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceExternalIpList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_ephemeral_ip_attach(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceEphemeralIpAttach,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_ephemeral_ip_detach(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceEphemeralIpDetach,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_reboot(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceReboot,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_serial_console(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceSerialConsole,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_serial_console_stream(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceSerialConsoleStream,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_ssh_public_key_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceSshPublicKeyList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_start(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceStart,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_stop(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceStop,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_ip_address_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayIpAddressList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_ip_address_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayIpAddressCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_ip_address_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayIpAddressDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_ip_pool_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayIpPoolList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_ip_pool_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayIpPoolCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_ip_pool_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayIpPoolDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_internet_gateway_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InternetGatewayDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_project_ip_pool_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectIpPoolList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_project_ip_pool_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectIpPoolView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_login_local(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LoginLocal,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_logout(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::Logout,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_current_user_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_current_user_groups(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserGroups,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_current_user_ssh_key_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserSshKeyList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_current_user_ssh_key_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserSshKeyCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_current_user_ssh_key_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserSshKeyView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_current_user_ssh_key_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserSshKeyDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_metric(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloMetric,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_network_interface_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_network_interface_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_network_interface_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_network_interface_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_network_interface_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ping(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::Ping,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_policy_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::PolicyView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_policy_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::PolicyUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_project_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_project_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_project_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_project_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_project_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_project_policy_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectPolicyView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_project_policy_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectPolicyUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_snapshot_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SnapshotList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_snapshot_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SnapshotCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_snapshot_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SnapshotView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_snapshot_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SnapshotDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_physical_disk_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::PhysicalDiskList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_physical_disk_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::PhysicalDiskView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_rack_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RackList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_rack_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RackView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_sled_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_sled_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledAdd,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_sled_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_sled_physical_disk_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledPhysicalDiskList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_sled_instance_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledInstanceList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_sled_set_provision_policy(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledSetProvisionPolicy,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_sled_list_uninitialized(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledListUninitialized,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_switch_port_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingSwitchPortList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_switch_port_apply_settings(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingSwitchPortApplySettings,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_switch_port_clear_settings(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingSwitchPortClearSettings,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_switch_port_status(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingSwitchPortStatus,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_switch_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SwitchList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_switch_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SwitchView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_identity_provider_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloIdentityProviderList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_local_idp_user_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LocalIdpUserCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_local_idp_user_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LocalIdpUserDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_local_idp_user_set_password(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LocalIdpUserSetPassword,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_saml_identity_provider_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SamlIdentityProviderCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_saml_identity_provider_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SamlIdentityProviderView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_range_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolRangeList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_range_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolRangeAdd,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_range_remove(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolRangeRemove,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_silo_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolSiloList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_silo_link(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolSiloLink,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_silo_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolSiloUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_silo_unlink(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolSiloUnlink,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_utilization_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolUtilizationView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_service_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolServiceView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_service_range_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolServiceRangeList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_service_range_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolServiceRangeAdd,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ip_pool_service_range_remove(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolServiceRangeRemove,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_system_metric(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemMetric,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_address_lot_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingAddressLotList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_address_lot_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingAddressLotCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_address_lot_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingAddressLotDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_address_lot_block_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingAddressLotBlockList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_allow_list_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingAllowListView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_allow_list_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingAllowListUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bfd_disable(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBfdDisable,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bfd_enable(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBfdEnable,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bfd_status(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBfdStatus,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_config_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpConfigList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_config_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpConfigCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_config_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpConfigDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_announce_set_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpAnnounceSetList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_announce_set_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpAnnounceSetUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_announce_set_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpAnnounceSetDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_announcement_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpAnnouncementList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_exported(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpExported,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_message_history(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpMessageHistory,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_imported_routes_ipv4(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpImportedRoutesIpv4,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_bgp_status(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingBgpStatus,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_loopback_address_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingLoopbackAddressList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_loopback_address_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingLoopbackAddressCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_loopback_address_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingLoopbackAddressDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_switch_port_settings_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingSwitchPortSettingsList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_switch_port_settings_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingSwitchPortSettingsCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_switch_port_settings_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingSwitchPortSettingsDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_networking_switch_port_settings_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::NetworkingSwitchPortSettingsView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_system_policy_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemPolicyView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_system_policy_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemPolicyUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_role_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RoleList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_role_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RoleView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_system_quotas_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemQuotasList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_ip_pool_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloIpPoolList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_policy_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloPolicyView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_policy_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloPolicyUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_quotas_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloQuotasView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_quotas_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloQuotasUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_user_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloUserList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_user_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloUserView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_user_builtin_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UserBuiltinList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_user_builtin_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UserBuiltinView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_utilization_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloUtilizationList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_silo_utilization_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloUtilizationView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_timeseries_query(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::TimeseriesQuery,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_timeseries_schema_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::TimeseriesSchemaList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_user_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UserList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_utilization_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UtilizationView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_firewall_rules_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcFirewallRulesView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_firewall_rules_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcFirewallRulesUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_route_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_route_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_route_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_route_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_route_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_router_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_subnet_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_subnet_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_subnet_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_subnet_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_subnet_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_subnet_list_network_interfaces(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetListNetworkInterfaces,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcView,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcUpdate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_vpc_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcDelete,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    DeviceAuthRequest,
    DeviceAuthConfirm,
    DeviceAccessToken,
    ProbeList,
    ProbeCreate,
    ProbeView,
    ProbeDelete,
    LoginSaml,
    CertificateList,
    CertificateCreate,
    CertificateView,
    CertificateDelete,
    DiskList,
    DiskCreate,
    DiskView,
    DiskDelete,
    DiskBulkWriteImport,
    DiskBulkWriteImportStart,
    DiskBulkWriteImportStop,
    DiskFinalizeImport,
    DiskMetricsList,
    FloatingIpList,
    FloatingIpCreate,
    FloatingIpView,
    FloatingIpUpdate,
    FloatingIpDelete,
    FloatingIpAttach,
    FloatingIpDetach,
    GroupList,
    GroupView,
    ImageList,
    ImageCreate,
    ImageView,
    ImageDelete,
    ImageDemote,
    ImagePromote,
    InstanceList,
    InstanceCreate,
    InstanceView,
    InstanceDelete,
    InstanceDiskList,
    InstanceDiskAttach,
    InstanceDiskDetach,
    InstanceExternalIpList,
    InstanceEphemeralIpAttach,
    InstanceEphemeralIpDetach,
    InstanceReboot,
    InstanceSerialConsole,
    InstanceSerialConsoleStream,
    InstanceSshPublicKeyList,
    InstanceStart,
    InstanceStop,
    InternetGatewayIpAddressList,
    InternetGatewayIpAddressCreate,
    InternetGatewayIpAddressDelete,
    InternetGatewayIpPoolList,
    InternetGatewayIpPoolCreate,
    InternetGatewayIpPoolDelete,
    InternetGatewayList,
    InternetGatewayCreate,
    InternetGatewayView,
    InternetGatewayDelete,
    ProjectIpPoolList,
    ProjectIpPoolView,
    LoginLocal,
    Logout,
    CurrentUserView,
    CurrentUserGroups,
    CurrentUserSshKeyList,
    CurrentUserSshKeyCreate,
    CurrentUserSshKeyView,
    CurrentUserSshKeyDelete,
    SiloMetric,
    InstanceNetworkInterfaceList,
    InstanceNetworkInterfaceCreate,
    InstanceNetworkInterfaceView,
    InstanceNetworkInterfaceUpdate,
    InstanceNetworkInterfaceDelete,
    Ping,
    PolicyView,
    PolicyUpdate,
    ProjectList,
    ProjectCreate,
    ProjectView,
    ProjectUpdate,
    ProjectDelete,
    ProjectPolicyView,
    ProjectPolicyUpdate,
    SnapshotList,
    SnapshotCreate,
    SnapshotView,
    SnapshotDelete,
    PhysicalDiskList,
    PhysicalDiskView,
    RackList,
    RackView,
    SledList,
    SledAdd,
    SledView,
    SledPhysicalDiskList,
    SledInstanceList,
    SledSetProvisionPolicy,
    SledListUninitialized,
    NetworkingSwitchPortList,
    NetworkingSwitchPortApplySettings,
    NetworkingSwitchPortClearSettings,
    NetworkingSwitchPortStatus,
    SwitchList,
    SwitchView,
    SiloIdentityProviderList,
    LocalIdpUserCreate,
    LocalIdpUserDelete,
    LocalIdpUserSetPassword,
    SamlIdentityProviderCreate,
    SamlIdentityProviderView,
    IpPoolList,
    IpPoolCreate,
    IpPoolView,
    IpPoolUpdate,
    IpPoolDelete,
    IpPoolRangeList,
    IpPoolRangeAdd,
    IpPoolRangeRemove,
    IpPoolSiloList,
    IpPoolSiloLink,
    IpPoolSiloUpdate,
    IpPoolSiloUnlink,
    IpPoolUtilizationView,
    IpPoolServiceView,
    IpPoolServiceRangeList,
    IpPoolServiceRangeAdd,
    IpPoolServiceRangeRemove,
    SystemMetric,
    NetworkingAddressLotList,
    NetworkingAddressLotCreate,
    NetworkingAddressLotDelete,
    NetworkingAddressLotBlockList,
    NetworkingAllowListView,
    NetworkingAllowListUpdate,
    NetworkingBfdDisable,
    NetworkingBfdEnable,
    NetworkingBfdStatus,
    NetworkingBgpConfigList,
    NetworkingBgpConfigCreate,
    NetworkingBgpConfigDelete,
    NetworkingBgpAnnounceSetList,
    NetworkingBgpAnnounceSetUpdate,
    NetworkingBgpAnnounceSetDelete,
    NetworkingBgpAnnouncementList,
    NetworkingBgpExported,
    NetworkingBgpMessageHistory,
    NetworkingBgpImportedRoutesIpv4,
    NetworkingBgpStatus,
    NetworkingLoopbackAddressList,
    NetworkingLoopbackAddressCreate,
    NetworkingLoopbackAddressDelete,
    NetworkingSwitchPortSettingsList,
    NetworkingSwitchPortSettingsCreate,
    NetworkingSwitchPortSettingsDelete,
    NetworkingSwitchPortSettingsView,
    SystemPolicyView,
    SystemPolicyUpdate,
    RoleList,
    RoleView,
    SystemQuotasList,
    SiloList,
    SiloCreate,
    SiloView,
    SiloDelete,
    SiloIpPoolList,
    SiloPolicyView,
    SiloPolicyUpdate,
    SiloQuotasView,
    SiloQuotasUpdate,
    SiloUserList,
    SiloUserView,
    UserBuiltinList,
    UserBuiltinView,
    SiloUtilizationList,
    SiloUtilizationView,
    TimeseriesQuery,
    TimeseriesSchemaList,
    UserList,
    UtilizationView,
    VpcFirewallRulesView,
    VpcFirewallRulesUpdate,
    VpcRouterRouteList,
    VpcRouterRouteCreate,
    VpcRouterRouteView,
    VpcRouterRouteUpdate,
    VpcRouterRouteDelete,
    VpcRouterList,
    VpcRouterCreate,
    VpcRouterView,
    VpcRouterUpdate,
    VpcRouterDelete,
    VpcSubnetList,
    VpcSubnetCreate,
    VpcSubnetView,
    VpcSubnetUpdate,
    VpcSubnetDelete,
    VpcSubnetListNetworkInterfaces,
    VpcList,
    VpcCreate,
    VpcView,
    VpcUpdate,
    VpcDelete,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::DeviceAuthRequest,
            CliCommand::DeviceAuthConfirm,
            CliCommand::DeviceAccessToken,
            CliCommand::ProbeList,
            CliCommand::ProbeCreate,
            CliCommand::ProbeView,
            CliCommand::ProbeDelete,
            CliCommand::LoginSaml,
            CliCommand::CertificateList,
            CliCommand::CertificateCreate,
            CliCommand::CertificateView,
            CliCommand::CertificateDelete,
            CliCommand::DiskList,
            CliCommand::DiskCreate,
            CliCommand::DiskView,
            CliCommand::DiskDelete,
            CliCommand::DiskBulkWriteImport,
            CliCommand::DiskBulkWriteImportStart,
            CliCommand::DiskBulkWriteImportStop,
            CliCommand::DiskFinalizeImport,
            CliCommand::DiskMetricsList,
            CliCommand::FloatingIpList,
            CliCommand::FloatingIpCreate,
            CliCommand::FloatingIpView,
            CliCommand::FloatingIpUpdate,
            CliCommand::FloatingIpDelete,
            CliCommand::FloatingIpAttach,
            CliCommand::FloatingIpDetach,
            CliCommand::GroupList,
            CliCommand::GroupView,
            CliCommand::ImageList,
            CliCommand::ImageCreate,
            CliCommand::ImageView,
            CliCommand::ImageDelete,
            CliCommand::ImageDemote,
            CliCommand::ImagePromote,
            CliCommand::InstanceList,
            CliCommand::InstanceCreate,
            CliCommand::InstanceView,
            CliCommand::InstanceDelete,
            CliCommand::InstanceDiskList,
            CliCommand::InstanceDiskAttach,
            CliCommand::InstanceDiskDetach,
            CliCommand::InstanceExternalIpList,
            CliCommand::InstanceEphemeralIpAttach,
            CliCommand::InstanceEphemeralIpDetach,
            CliCommand::InstanceReboot,
            CliCommand::InstanceSerialConsole,
            CliCommand::InstanceSerialConsoleStream,
            CliCommand::InstanceSshPublicKeyList,
            CliCommand::InstanceStart,
            CliCommand::InstanceStop,
            CliCommand::InternetGatewayIpAddressList,
            CliCommand::InternetGatewayIpAddressCreate,
            CliCommand::InternetGatewayIpAddressDelete,
            CliCommand::InternetGatewayIpPoolList,
            CliCommand::InternetGatewayIpPoolCreate,
            CliCommand::InternetGatewayIpPoolDelete,
            CliCommand::InternetGatewayList,
            CliCommand::InternetGatewayCreate,
            CliCommand::InternetGatewayView,
            CliCommand::InternetGatewayDelete,
            CliCommand::ProjectIpPoolList,
            CliCommand::ProjectIpPoolView,
            CliCommand::LoginLocal,
            CliCommand::Logout,
            CliCommand::CurrentUserView,
            CliCommand::CurrentUserGroups,
            CliCommand::CurrentUserSshKeyList,
            CliCommand::CurrentUserSshKeyCreate,
            CliCommand::CurrentUserSshKeyView,
            CliCommand::CurrentUserSshKeyDelete,
            CliCommand::SiloMetric,
            CliCommand::InstanceNetworkInterfaceList,
            CliCommand::InstanceNetworkInterfaceCreate,
            CliCommand::InstanceNetworkInterfaceView,
            CliCommand::InstanceNetworkInterfaceUpdate,
            CliCommand::InstanceNetworkInterfaceDelete,
            CliCommand::Ping,
            CliCommand::PolicyView,
            CliCommand::PolicyUpdate,
            CliCommand::ProjectList,
            CliCommand::ProjectCreate,
            CliCommand::ProjectView,
            CliCommand::ProjectUpdate,
            CliCommand::ProjectDelete,
            CliCommand::ProjectPolicyView,
            CliCommand::ProjectPolicyUpdate,
            CliCommand::SnapshotList,
            CliCommand::SnapshotCreate,
            CliCommand::SnapshotView,
            CliCommand::SnapshotDelete,
            CliCommand::PhysicalDiskList,
            CliCommand::PhysicalDiskView,
            CliCommand::RackList,
            CliCommand::RackView,
            CliCommand::SledList,
            CliCommand::SledAdd,
            CliCommand::SledView,
            CliCommand::SledPhysicalDiskList,
            CliCommand::SledInstanceList,
            CliCommand::SledSetProvisionPolicy,
            CliCommand::SledListUninitialized,
            CliCommand::NetworkingSwitchPortList,
            CliCommand::NetworkingSwitchPortApplySettings,
            CliCommand::NetworkingSwitchPortClearSettings,
            CliCommand::NetworkingSwitchPortStatus,
            CliCommand::SwitchList,
            CliCommand::SwitchView,
            CliCommand::SiloIdentityProviderList,
            CliCommand::LocalIdpUserCreate,
            CliCommand::LocalIdpUserDelete,
            CliCommand::LocalIdpUserSetPassword,
            CliCommand::SamlIdentityProviderCreate,
            CliCommand::SamlIdentityProviderView,
            CliCommand::IpPoolList,
            CliCommand::IpPoolCreate,
            CliCommand::IpPoolView,
            CliCommand::IpPoolUpdate,
            CliCommand::IpPoolDelete,
            CliCommand::IpPoolRangeList,
            CliCommand::IpPoolRangeAdd,
            CliCommand::IpPoolRangeRemove,
            CliCommand::IpPoolSiloList,
            CliCommand::IpPoolSiloLink,
            CliCommand::IpPoolSiloUpdate,
            CliCommand::IpPoolSiloUnlink,
            CliCommand::IpPoolUtilizationView,
            CliCommand::IpPoolServiceView,
            CliCommand::IpPoolServiceRangeList,
            CliCommand::IpPoolServiceRangeAdd,
            CliCommand::IpPoolServiceRangeRemove,
            CliCommand::SystemMetric,
            CliCommand::NetworkingAddressLotList,
            CliCommand::NetworkingAddressLotCreate,
            CliCommand::NetworkingAddressLotDelete,
            CliCommand::NetworkingAddressLotBlockList,
            CliCommand::NetworkingAllowListView,
            CliCommand::NetworkingAllowListUpdate,
            CliCommand::NetworkingBfdDisable,
            CliCommand::NetworkingBfdEnable,
            CliCommand::NetworkingBfdStatus,
            CliCommand::NetworkingBgpConfigList,
            CliCommand::NetworkingBgpConfigCreate,
            CliCommand::NetworkingBgpConfigDelete,
            CliCommand::NetworkingBgpAnnounceSetList,
            CliCommand::NetworkingBgpAnnounceSetUpdate,
            CliCommand::NetworkingBgpAnnounceSetDelete,
            CliCommand::NetworkingBgpAnnouncementList,
            CliCommand::NetworkingBgpExported,
            CliCommand::NetworkingBgpMessageHistory,
            CliCommand::NetworkingBgpImportedRoutesIpv4,
            CliCommand::NetworkingBgpStatus,
            CliCommand::NetworkingLoopbackAddressList,
            CliCommand::NetworkingLoopbackAddressCreate,
            CliCommand::NetworkingLoopbackAddressDelete,
            CliCommand::NetworkingSwitchPortSettingsList,
            CliCommand::NetworkingSwitchPortSettingsCreate,
            CliCommand::NetworkingSwitchPortSettingsDelete,
            CliCommand::NetworkingSwitchPortSettingsView,
            CliCommand::SystemPolicyView,
            CliCommand::SystemPolicyUpdate,
            CliCommand::RoleList,
            CliCommand::RoleView,
            CliCommand::SystemQuotasList,
            CliCommand::SiloList,
            CliCommand::SiloCreate,
            CliCommand::SiloView,
            CliCommand::SiloDelete,
            CliCommand::SiloIpPoolList,
            CliCommand::SiloPolicyView,
            CliCommand::SiloPolicyUpdate,
            CliCommand::SiloQuotasView,
            CliCommand::SiloQuotasUpdate,
            CliCommand::SiloUserList,
            CliCommand::SiloUserView,
            CliCommand::UserBuiltinList,
            CliCommand::UserBuiltinView,
            CliCommand::SiloUtilizationList,
            CliCommand::SiloUtilizationView,
            CliCommand::TimeseriesQuery,
            CliCommand::TimeseriesSchemaList,
            CliCommand::UserList,
            CliCommand::UtilizationView,
            CliCommand::VpcFirewallRulesView,
            CliCommand::VpcFirewallRulesUpdate,
            CliCommand::VpcRouterRouteList,
            CliCommand::VpcRouterRouteCreate,
            CliCommand::VpcRouterRouteView,
            CliCommand::VpcRouterRouteUpdate,
            CliCommand::VpcRouterRouteDelete,
            CliCommand::VpcRouterList,
            CliCommand::VpcRouterCreate,
            CliCommand::VpcRouterView,
            CliCommand::VpcRouterUpdate,
            CliCommand::VpcRouterDelete,
            CliCommand::VpcSubnetList,
            CliCommand::VpcSubnetCreate,
            CliCommand::VpcSubnetView,
            CliCommand::VpcSubnetUpdate,
            CliCommand::VpcSubnetDelete,
            CliCommand::VpcSubnetListNetworkInterfaces,
            CliCommand::VpcList,
            CliCommand::VpcCreate,
            CliCommand::VpcView,
            CliCommand::VpcUpdate,
            CliCommand::VpcDelete,
        ]
        .into_iter()
    }
}
