// The contents of this file are generated; do not modify them.

use oxide_api::*;

pub struct Cli<T: CliOverride = ()> {
    client: oxide_api::Client,
    over: T,
}

impl Cli {
    pub fn new(client: oxide_api::Client) -> Self {
        Self { client, over: () }
    }

    pub fn get_command(cmd: CliCommand) -> clap::Command {
        match cmd {
            CliCommand::DeviceAuthRequest => Self::cli_device_auth_request(),
            CliCommand::DeviceAuthConfirm => Self::cli_device_auth_confirm(),
            CliCommand::DeviceAccessToken => Self::cli_device_access_token(),
            CliCommand::LoginSpoof => Self::cli_login_spoof(),
            CliCommand::LoginLocal => Self::cli_login_local(),
            CliCommand::LoginSamlBegin => Self::cli_login_saml_begin(),
            CliCommand::LoginSaml => Self::cli_login_saml(),
            CliCommand::Logout => Self::cli_logout(),
            CliCommand::SystemImageViewById => Self::cli_system_image_view_by_id(),
            CliCommand::SystemImageList => Self::cli_system_image_list(),
            CliCommand::SystemImageCreate => Self::cli_system_image_create(),
            CliCommand::SystemImageView => Self::cli_system_image_view(),
            CliCommand::SystemImageDelete => Self::cli_system_image_delete(),
            CliCommand::DiskList => Self::cli_disk_list(),
            CliCommand::DiskCreate => Self::cli_disk_create(),
            CliCommand::DiskView => Self::cli_disk_view(),
            CliCommand::DiskDelete => Self::cli_disk_delete(),
            CliCommand::DiskMetricsList => Self::cli_disk_metrics_list(),
            CliCommand::GroupList => Self::cli_group_list(),
            CliCommand::GroupView => Self::cli_group_view(),
            CliCommand::ImageList => Self::cli_image_list(),
            CliCommand::ImageCreate => Self::cli_image_create(),
            CliCommand::ImageView => Self::cli_image_view(),
            CliCommand::ImageDelete => Self::cli_image_delete(),
            CliCommand::InstanceList => Self::cli_instance_list(),
            CliCommand::InstanceCreate => Self::cli_instance_create(),
            CliCommand::InstanceView => Self::cli_instance_view(),
            CliCommand::InstanceDelete => Self::cli_instance_delete(),
            CliCommand::InstanceDiskList => Self::cli_instance_disk_list(),
            CliCommand::InstanceDiskAttach => Self::cli_instance_disk_attach(),
            CliCommand::InstanceDiskDetach => Self::cli_instance_disk_detach(),
            CliCommand::InstanceExternalIpList => Self::cli_instance_external_ip_list(),
            CliCommand::InstanceMigrate => Self::cli_instance_migrate(),
            CliCommand::InstanceReboot => Self::cli_instance_reboot(),
            CliCommand::InstanceSerialConsole => Self::cli_instance_serial_console(),
            CliCommand::InstanceSerialConsoleStream => Self::cli_instance_serial_console_stream(),
            CliCommand::InstanceStart => Self::cli_instance_start(),
            CliCommand::InstanceStop => Self::cli_instance_stop(),
            CliCommand::CurrentUserView => Self::cli_current_user_view(),
            CliCommand::CurrentUserGroups => Self::cli_current_user_groups(),
            CliCommand::CurrentUserSshKeyList => Self::cli_current_user_ssh_key_list(),
            CliCommand::CurrentUserSshKeyCreate => Self::cli_current_user_ssh_key_create(),
            CliCommand::CurrentUserSshKeyView => Self::cli_current_user_ssh_key_view(),
            CliCommand::CurrentUserSshKeyDelete => Self::cli_current_user_ssh_key_delete(),
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
            CliCommand::CertificateList => Self::cli_certificate_list(),
            CliCommand::CertificateCreate => Self::cli_certificate_create(),
            CliCommand::CertificateView => Self::cli_certificate_view(),
            CliCommand::CertificateDelete => Self::cli_certificate_delete(),
            CliCommand::PhysicalDiskList => Self::cli_physical_disk_list(),
            CliCommand::RackList => Self::cli_rack_list(),
            CliCommand::RackView => Self::cli_rack_view(),
            CliCommand::SledList => Self::cli_sled_list(),
            CliCommand::SledView => Self::cli_sled_view(),
            CliCommand::SledPhysicalDiskList => Self::cli_sled_physical_disk_list(),
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
            CliCommand::IpPoolServiceView => Self::cli_ip_pool_service_view(),
            CliCommand::IpPoolServiceRangeList => Self::cli_ip_pool_service_range_list(),
            CliCommand::IpPoolServiceRangeAdd => Self::cli_ip_pool_service_range_add(),
            CliCommand::IpPoolServiceRangeRemove => Self::cli_ip_pool_service_range_remove(),
            CliCommand::SystemMetric => Self::cli_system_metric(),
            CliCommand::SystemPolicyView => Self::cli_system_policy_view(),
            CliCommand::SystemPolicyUpdate => Self::cli_system_policy_update(),
            CliCommand::RoleList => Self::cli_role_list(),
            CliCommand::RoleView => Self::cli_role_view(),
            CliCommand::SagaList => Self::cli_saga_list(),
            CliCommand::SagaView => Self::cli_saga_view(),
            CliCommand::SiloList => Self::cli_silo_list(),
            CliCommand::SiloCreate => Self::cli_silo_create(),
            CliCommand::SiloView => Self::cli_silo_view(),
            CliCommand::SiloDelete => Self::cli_silo_delete(),
            CliCommand::SiloPolicyView => Self::cli_silo_policy_view(),
            CliCommand::SiloPolicyUpdate => Self::cli_silo_policy_update(),
            CliCommand::SystemComponentVersionList => Self::cli_system_component_version_list(),
            CliCommand::UpdateDeploymentsList => Self::cli_update_deployments_list(),
            CliCommand::UpdateDeploymentView => Self::cli_update_deployment_view(),
            CliCommand::SystemUpdateRefresh => Self::cli_system_update_refresh(),
            CliCommand::SystemUpdateStart => Self::cli_system_update_start(),
            CliCommand::SystemUpdateStop => Self::cli_system_update_stop(),
            CliCommand::SystemUpdateList => Self::cli_system_update_list(),
            CliCommand::SystemUpdateView => Self::cli_system_update_view(),
            CliCommand::SystemUpdateComponentsList => Self::cli_system_update_components_list(),
            CliCommand::SystemVersion => Self::cli_system_version(),
            CliCommand::SiloUserList => Self::cli_silo_user_list(),
            CliCommand::SiloUserView => Self::cli_silo_user_view(),
            CliCommand::UserBuiltinList => Self::cli_user_builtin_list(),
            CliCommand::UserBuiltinView => Self::cli_user_builtin_view(),
            CliCommand::UserList => Self::cli_user_list(),
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
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid)),
            )
            .about(
                "Start an OAuth 2.0 Device Authorization Grant\n\nThis endpoint is designed to be \
                 accessed from an *unauthenticated* API client. It generates and records a \
                 `device_code` and `user_code` which must be verified and confirmed prior to a \
                 token being granted.",
            )
    }

    pub fn cli_device_auth_confirm() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("user-code")
                    .long("user-code")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .about(
                "Confirm an OAuth 2.0 Device Authorization Grant\n\nThis endpoint is designed to \
                 be accessed by the user agent (browser), not the client requesting the token. So \
                 we do not actually return the token here; it will be returned in response to the \
                 poll on `/device/token`.",
            )
    }

    pub fn cli_device_access_token() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client-id")
                    .long("client-id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid)),
            )
            .arg(
                clap::Arg::new("device-code")
                    .long("device-code")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("grant-type")
                    .long("grant-type")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .about(
                "Request a device access token\n\nThis endpoint should be polled by the client \
                 until the user code is verified and the grant is confirmed.",
            )
    }

    pub fn cli_login_spoof() -> clap::Command {
        clap::Command::new("").arg(
            clap::Arg::new("username")
                .long("username")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
    }

    pub fn cli_login_local() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo-name")
                    .long("silo-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("password")
                    .long("password")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Password)),
            )
            .arg(
                clap::Arg::new("username")
                    .long("username")
                    .required(true)
                    .value_parser(clap::value_parser!(types::UserId)),
            )
            .about("Authenticate a user (i.e., log in) via username and password")
    }

    pub fn cli_login_saml_begin() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo-name")
                    .long("silo-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("provider-name")
                    .long("provider-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about(
                "Prompt user login\n\nEither display a page asking a user for their credentials, \
                 or redirect them to their identity provider.",
            )
    }

    pub fn cli_login_saml() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo-name")
                    .long("silo-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("provider-name")
                    .long("provider-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about("Authenticate a user (i.e., log in) via SAML")
    }

    pub fn cli_logout() -> clap::Command {
        clap::Command::new("")
    }

    pub fn cli_system_image_view_by_id() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("id")
                    .long("id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid)),
            )
            .about("Fetch a system-wide image by id")
    }

    pub fn cli_system_image_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameSortMode)),
            )
            .about(
                "List system-wide images\n\nReturns a list of all the system-wide images. \
                 System-wide images are returned sorted by creation date, with the most recent \
                 images appearing first.",
            )
    }

    pub fn cli_system_image_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about(
                "Create a system-wide image\n\nCreate a new system-wide image. This image can \
                 then be used by any user in any silo as a base for instances.",
            )
    }

    pub fn cli_system_image_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image-name")
                    .long("image-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about(
                "Fetch a system-wide image\n\nReturns the details of a specific system-wide image.",
            )
    }

    pub fn cli_system_image_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image-name")
                    .long("image-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about(
                "Delete a system-wide image\n\nPermanently delete a system-wide image. This \
                 operation cannot be undone. Any instances using the system-wide image will \
                 continue to run, however new instances can not be created with this image.",
            )
    }

    pub fn cli_disk_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about("List disks")
    }

    pub fn cli_disk_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("size")
                    .long("size")
                    .required(true)
                    .value_parser(clap::value_parser!(types::ByteCount))
                    .help("total size of the Disk in bytes"),
            )
            .about("Create a disk")
    }

    pub fn cli_disk_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a disk")
    }

    pub fn cli_disk_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete a disk")
    }

    pub fn cli_disk_metrics_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("metric")
                    .long("metric")
                    .required(true)
                    .value_parser(clap::value_parser!(types::DiskMetricName)),
            )
            .arg(
                clap::Arg::new("end-time")
                    .long("end-time")
                    .required(false)
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .help("An exclusive end time of metrics."),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("start-time")
                    .long("start-time")
                    .required(false)
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .help("An inclusive start time of metrics."),
            )
            .about("Fetch disk metrics")
    }

    pub fn cli_group_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List groups")
    }

    pub fn cli_group_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("group")
                    .long("group")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid)),
            )
            .about("Fetch group")
    }

    pub fn cli_image_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about(
                "List images\n\nList images which are global or scoped to the specified project. \
                 The images are returned sorted by creation date, with the most recent images \
                 appearing first.",
            )
    }

    pub fn cli_image_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("os")
                    .long("os")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("The family of the operating system (e.g. Debian, Ubuntu, etc.)"),
            )
            .arg(
                clap::Arg::new("version")
                    .long("version")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("The version of the operating system (e.g. 18.04, 20.04, etc.)"),
            )
            .about("Create an image\n\nCreate a new image in a project.")
    }

    pub fn cli_image_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image")
                    .long("image")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch an image\n\nFetch the details for a specific image in a project.")
    }

    pub fn cli_image_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("image")
                    .long("image")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about(
                "Delete an image\n\nPermanently delete an image from a project. This operation \
                 cannot be undone. Any instances in the project using the image will continue to \
                 run, however new instances can not be created with this image.",
            )
    }

    pub fn cli_instance_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about("List instances")
    }

    pub fn cli_instance_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("hostname")
                    .long("hostname")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("memory")
                    .long("memory")
                    .required(true)
                    .value_parser(clap::value_parser!(types::ByteCount)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("ncpus")
                    .long("ncpus")
                    .required(true)
                    .value_parser(clap::value_parser!(types::InstanceCpuCount)),
            )
            .arg(
                clap::Arg::new("start")
                    .long("start")
                    .required(false)
                    .value_parser(clap::value_parser!(bool))
                    .help("Should this instance be started upon creation; true by default."),
            )
            .arg(
                clap::Arg::new("user-data")
                    .long("user-data")
                    .required(false)
                    .value_parser(clap::value_parser!(String))
                    .help(
                        "User data for instance initialization systems (such as cloud-init). Must \
                         be a Base64-encoded string, as specified in RFC 4648 § 4 (+ and / \
                         characters with padding). Maximum 32 KiB unencoded data.",
                    ),
            )
            .about("Create an instance")
    }

    pub fn cli_instance_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch an instance")
    }

    pub fn cli_instance_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete an instance")
    }

    pub fn cli_instance_disk_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about("List an instance's disks")
    }

    pub fn cli_instance_disk_attach() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Attach a disk to an instance")
    }

    pub fn cli_instance_disk_detach() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Detach a disk from an instance")
    }

    pub fn cli_instance_external_ip_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("List external IP addresses")
    }

    pub fn cli_instance_migrate() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("dst-sled-id")
                    .long("dst-sled-id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid)),
            )
            .about("Migrate an instance")
    }

    pub fn cli_instance_reboot() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Reboot an instance")
    }

    pub fn cli_instance_serial_console() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("from-start")
                    .long("from-start")
                    .required(false)
                    .value_parser(clap::value_parser!(u64))
                    .help(
                        "Character index in the serial buffer from which to read, counting the \
                         bytes output since instance start. If this is not provided, \
                         `most_recent` must be provided, and if this *is* provided, `most_recent` \
                         must *not* be provided.",
                    ),
            )
            .arg(
                clap::Arg::new("max-bytes")
                    .long("max-bytes")
                    .required(false)
                    .value_parser(clap::value_parser!(u64))
                    .help(
                        "Maximum number of bytes of buffered serial console contents to return. \
                         If the requested range runs to the end of the available buffer, the data \
                         returned will be shorter than `max_bytes`.",
                    ),
            )
            .arg(
                clap::Arg::new("most-recent")
                    .long("most-recent")
                    .required(false)
                    .value_parser(clap::value_parser!(u64))
                    .help(
                        "Character index in the serial buffer from which to read, counting \
                         *backward* from the most recently buffered data retrieved from the \
                         instance. (See note on `from_start` about mutual exclusivity)",
                    ),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch an instance's serial console")
    }

    pub fn cli_instance_serial_console_stream() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Stream an instance's serial console")
    }

    pub fn cli_instance_start() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Boot an instance")
    }

    pub fn cli_instance_stop() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Stop an instance")
    }

    pub fn cli_current_user_view() -> clap::Command {
        clap::Command::new("").about("Fetch the user associated with the current session")
    }

    pub fn cli_current_user_groups() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("Fetch the silo\u{a0}groups the current user belongs to")
    }

    pub fn cli_current_user_ssh_key_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about(
                "List SSH public keys\n\nLists SSH public keys for the currently authenticated \
                 user.",
            )
    }

    pub fn cli_current_user_ssh_key_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("public-key")
                    .long("public-key")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("SSH public key, e.g., `\"ssh-ed25519 AAAAC3NzaC...\"`"),
            )
            .about(
                "Create an SSH public key\n\nCreate an SSH public key for the currently \
                 authenticated user.",
            )
    }

    pub fn cli_current_user_ssh_key_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("ssh-key")
                    .long("ssh-key")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about(
                "Fetch an SSH public key\n\nFetch an SSH public key associated with the currently \
                 authenticated user.",
            )
    }

    pub fn cli_current_user_ssh_key_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("ssh-key")
                    .long("ssh-key")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about(
                "Delete an SSH public key\n\nDelete an SSH public key associated with the \
                 currently authenticated user.",
            )
    }

    pub fn cli_instance_network_interface_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about("List network interfaces")
    }

    pub fn cli_instance_network_interface_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("subnet-name")
                    .long("subnet-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name))
                    .help("The VPC Subnet in which to create the interface."),
            )
            .arg(
                clap::Arg::new("vpc-name")
                    .long("vpc-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name))
                    .help("The VPC in which to create the interface."),
            )
            .about("Create a network interface")
    }

    pub fn cli_instance_network_interface_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("interface")
                    .long("interface")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a network interface")
    }

    pub fn cli_instance_network_interface_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("interface")
                    .long("interface")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("primary")
                    .long("primary")
                    .required(false)
                    .value_parser(clap::value_parser!(bool))
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
            .about("Update a network interface")
    }

    pub fn cli_instance_network_interface_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("interface")
                    .long("interface")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("instance")
                    .long("instance")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about(
                "Delete a network interface\n\nNote that the primary interface for an instance \
                 cannot be deleted if there are any secondary interfaces. A new primary interface \
                 must be designated first. The primary interface can be deleted if there are no \
                 secondary interfaces.",
            )
    }

    pub fn cli_policy_view() -> clap::Command {
        clap::Command::new("").about("Fetch the current silo's IAM policy")
    }

    pub fn cli_policy_update() -> clap::Command {
        clap::Command::new("").about("Update the current silo's IAM policy")
    }

    pub fn cli_project_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about("List projects")
    }

    pub fn cli_project_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about("Create a project")
    }

    pub fn cli_project_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a project")
    }

    pub fn cli_project_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Update a project")
    }

    pub fn cli_project_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete a project")
    }

    pub fn cli_project_policy_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a project's IAM policy")
    }

    pub fn cli_project_policy_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Update a project's IAM policy")
    }

    pub fn cli_snapshot_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about("List snapshots")
    }

    pub fn cli_snapshot_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("disk")
                    .long("disk")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name))
                    .help("The name of the disk to be snapshotted"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about("Create a snapshot\n\nCreates a point-in-time snapshot from a disk.")
    }

    pub fn cli_snapshot_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("snapshot")
                    .long("snapshot")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a snapshot")
    }

    pub fn cli_snapshot_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("snapshot")
                    .long("snapshot")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete a snapshot")
    }

    pub fn cli_certificate_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about(
                "List system-wide certificates\n\nReturns a list of all the system-wide \
                 certificates. System-wide certificates are returned sorted by creation date, \
                 with the most recent certificates appearing first.",
            )
    }

    pub fn cli_certificate_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("service")
                    .long("service")
                    .required(true)
                    .value_parser(clap::value_parser!(types::ServiceUsingCertificate))
                    .help("The service using this certificate"),
            )
            .about(
                "Create a new system-wide x.509 certificate.\n\nThis certificate is automatically \
                 used by the Oxide Control plane to serve external connections.",
            )
    }

    pub fn cli_certificate_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("certificate")
                    .long("certificate")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a certificate\n\nReturns the details of a specific certificate")
    }

    pub fn cli_certificate_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("certificate")
                    .long("certificate")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about(
                "Delete a certificate\n\nPermanently delete a certificate. This operation cannot \
                 be undone.",
            )
    }

    pub fn cli_physical_disk_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List physical disks")
    }

    pub fn cli_rack_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List racks")
    }

    pub fn cli_rack_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("rack-id")
                    .long("rack-id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .help("The rack's unique ID."),
            )
            .about("Fetch a rack")
    }

    pub fn cli_sled_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List sleds")
    }

    pub fn cli_sled_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("sled-id")
                    .long("sled-id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .help("The sled's unique ID."),
            )
            .about("Fetch a sled")
    }

    pub fn cli_sled_physical_disk_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("sled-id")
                    .long("sled-id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .help("The sled's unique ID."),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List physical disks attached to sleds")
    }

    pub fn cli_silo_identity_provider_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about("List a silo's IDPs_name")
    }

    pub fn cli_local_idp_user_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("external-id")
                    .long("external-id")
                    .required(true)
                    .value_parser(clap::value_parser!(types::UserId))
                    .help("username used to log in"),
            )
            .about(
                "Create a user\n\nUsers can only be created in Silos with `provision_type` == \
                 `Fixed`. Otherwise, Silo users are just-in-time (JIT) provisioned when a user \
                 first logs in using an external Identity Provider.",
            )
    }

    pub fn cli_local_idp_user_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("user-id")
                    .long("user-id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .help("The user's internal id"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete a user")
    }

    pub fn cli_local_idp_user_set_password() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("user-id")
                    .long("user-id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .help("The user's internal id"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about(
                "Set or invalidate a user's password\n\nPasswords can only be updated for users \
                 in Silos with identity mode `LocalOnly`.",
            )
    }

    pub fn cli_saml_identity_provider_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("acs-url")
                    .long("acs-url")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("service provider endpoint where the response will be sent"),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("idp-entity-id")
                    .long("idp-entity-id")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("idp's entity id"),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("slo-url")
                    .long("slo-url")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("service provider endpoint where the idp should send log out requests"),
            )
            .arg(
                clap::Arg::new("sp-client-id")
                    .long("sp-client-id")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("sp's client id"),
            )
            .arg(
                clap::Arg::new("technical-contact-email")
                    .long("technical-contact-email")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("customer's technical contact for saml configuration"),
            )
            .about("Create a SAML IDP")
    }

    pub fn cli_saml_identity_provider_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("provider")
                    .long("provider")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a SAML IDP")
    }

    pub fn cli_ip_pool_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about("List IP pools")
    }

    pub fn cli_ip_pool_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about("Create an IP pool")
    }

    pub fn cli_ip_pool_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch an IP pool")
    }

    pub fn cli_ip_pool_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Update an IP Pool")
    }

    pub fn cli_ip_pool_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete an IP Pool")
    }

    pub fn cli_ip_pool_range_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .about("List ranges for an IP pool\n\nRanges are ordered by their first address.")
    }

    pub fn cli_ip_pool_range_add() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Add a range to an IP pool")
    }

    pub fn cli_ip_pool_range_remove() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("pool")
                    .long("pool")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Remove a range from an IP pool")
    }

    pub fn cli_ip_pool_service_view() -> clap::Command {
        clap::Command::new("").about("Fetch the IP pool used for Oxide services.")
    }

    pub fn cli_ip_pool_service_range_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .about(
                "List ranges for the IP pool used for Oxide services.\n\nRanges are ordered by \
                 their first address.",
            )
    }

    pub fn cli_ip_pool_service_range_add() -> clap::Command {
        clap::Command::new("").about("Add a range to an IP pool used for Oxide services.")
    }

    pub fn cli_ip_pool_service_range_remove() -> clap::Command {
        clap::Command::new("").about("Remove a range from an IP pool used for Oxide services.")
    }

    pub fn cli_system_metric() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("metric-name")
                    .long("metric-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::SystemMetricName)),
            )
            .arg(
                clap::Arg::new("end-time")
                    .long("end-time")
                    .required(false)
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .help("An exclusive end time of metrics."),
            )
            .arg(
                clap::Arg::new("id")
                    .long("id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .help("The UUID of the container being queried"),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("page-token")
                    .long("page-token")
                    .required(false)
                    .value_parser(clap::value_parser!(String))
                    .help("Token returned by previous call to retrieve the subsequent page"),
            )
            .arg(
                clap::Arg::new("start-time")
                    .long("start-time")
                    .required(false)
                    .value_parser(clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
                    .help("An inclusive start time of metrics."),
            )
            .about("Access metrics data")
    }

    pub fn cli_system_policy_view() -> clap::Command {
        clap::Command::new("").about("Fetch the top-level IAM policy")
    }

    pub fn cli_system_policy_update() -> clap::Command {
        clap::Command::new("").about("Update the top-level IAM policy")
    }

    pub fn cli_role_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .about("List built-in roles")
    }

    pub fn cli_role_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("role-name")
                    .long("role-name")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("The built-in role's unique name."),
            )
            .about("Fetch a built-in role")
    }

    pub fn cli_saga_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List sagas")
    }

    pub fn cli_saga_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("saga-id")
                    .long("saga-id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid)),
            )
            .about("Fetch a saga")
    }

    pub fn cli_silo_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about(
                "List silos\n\nLists silos that are discoverable based on the current permissions.",
            )
    }

    pub fn cli_silo_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("discoverable")
                    .long("discoverable")
                    .required(true)
                    .value_parser(clap::value_parser!(bool)),
            )
            .arg(
                clap::Arg::new("identity-mode")
                    .long("identity-mode")
                    .required(true)
                    .value_parser(clap::value_parser!(types::SiloIdentityMode)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about("Create a silo")
    }

    pub fn cli_silo_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a silo\n\nFetch a silo by name.")
    }

    pub fn cli_silo_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete a silo\n\nDelete a silo by name.")
    }

    pub fn cli_silo_policy_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a silo's IAM policy")
    }

    pub fn cli_silo_policy_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Update a silo's IAM policy")
    }

    pub fn cli_system_component_version_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("View version and update status of component tree")
    }

    pub fn cli_update_deployments_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List all update deployments")
    }

    pub fn cli_update_deployment_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("id")
                    .long("id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid)),
            )
            .about("Fetch a system update deployment")
    }

    pub fn cli_system_update_refresh() -> clap::Command {
        clap::Command::new("").about("Refresh update data")
    }

    pub fn cli_system_update_start() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("version")
                    .long("version")
                    .required(true)
                    .value_parser(clap::value_parser!(types::SemverVersion)),
            )
            .about("Start system update")
    }

    pub fn cli_system_update_stop() -> clap::Command {
        clap::Command::new("")
            .about("Stop system update\n\nIf there is no update in progress, do nothing.")
    }

    pub fn cli_system_update_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List all updates")
    }

    pub fn cli_system_update_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("version")
                    .long("version")
                    .required(true)
                    .value_parser(clap::value_parser!(types::SemverVersion)),
            )
            .about("View system update")
    }

    pub fn cli_system_update_components_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("version")
                    .long("version")
                    .required(true)
                    .value_parser(clap::value_parser!(types::SemverVersion)),
            )
            .about("View system update component tree")
    }

    pub fn cli_system_version() -> clap::Command {
        clap::Command::new("").about("View system version and update status")
    }

    pub fn cli_silo_user_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List users in a silo")
    }

    pub fn cli_silo_user_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("user-id")
                    .long("user-id")
                    .required(true)
                    .value_parser(clap::value_parser!(uuid::Uuid))
                    .help("The user's internal id"),
            )
            .arg(
                clap::Arg::new("silo")
                    .long("silo")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a user")
    }

    pub fn cli_user_builtin_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameSortMode)),
            )
            .about("List built-in users")
    }

    pub fn cli_user_builtin_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("user")
                    .long("user")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a built-in user")
    }

    pub fn cli_user_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("group")
                    .long("group")
                    .required(false)
                    .value_parser(clap::value_parser!(uuid::Uuid)),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::IdSortMode)),
            )
            .about("List users")
    }

    pub fn cli_vpc_firewall_rules_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("List firewall rules")
    }

    pub fn cli_vpc_firewall_rules_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Replace firewall rules")
    }

    pub fn cli_vpc_router_route_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("List routes\n\nList the routes associated with a router in a particular VPC.")
    }

    pub fn cli_vpc_router_route_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about("Create a router")
    }

    pub fn cli_vpc_router_route_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("route")
                    .long("route")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a route")
    }

    pub fn cli_vpc_router_route_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("route")
                    .long("route")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Update a route")
    }

    pub fn cli_vpc_router_route_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("route")
                    .long("route")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete a route")
    }

    pub fn cli_vpc_router_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("List routers")
    }

    pub fn cli_vpc_router_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about("Create a VPC router")
    }

    pub fn cli_vpc_router_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Get a router")
    }

    pub fn cli_vpc_router_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Update a router")
    }

    pub fn cli_vpc_router_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("router")
                    .long("router")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete a router")
    }

    pub fn cli_vpc_subnet_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a subnet")
    }

    pub fn cli_vpc_subnet_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("ipv4-block")
                    .long("ipv4-block")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Ipv4Net))
                    .help(
                        "The IPv4 address range for this subnet.\n\nIt must be allocated from an \
                         RFC 1918 private address range, and must not overlap with any other \
                         existing subnet in the VPC.",
                    ),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about("Create a subnet")
    }

    pub fn cli_vpc_subnet_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("subnet")
                    .long("subnet")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a subnet")
    }

    pub fn cli_vpc_subnet_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("subnet")
                    .long("subnet")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Update a subnet")
    }

    pub fn cli_vpc_subnet_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("subnet")
                    .long("subnet")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete a subnet")
    }

    pub fn cli_vpc_subnet_list_network_interfaces() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("subnet")
                    .long("subnet")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("List network interfaces")
    }

    pub fn cli_vpc_list() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("limit")
                    .long("limit")
                    .required(false)
                    .value_parser(clap::value_parser!(std::num::NonZeroU32))
                    .help("Maximum number of items returned by a single call"),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("sort-by")
                    .long("sort-by")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrIdSortMode)),
            )
            .about("List VPCs")
    }

    pub fn cli_vpc_create() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("description")
                    .long("description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("dns-name")
                    .long("dns-name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .required(true)
                    .value_parser(clap::value_parser!(types::Name)),
            )
            .about("Create a VPC")
    }

    pub fn cli_vpc_view() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Fetch a VPC")
    }

    pub fn cli_vpc_update() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Update a VPC")
    }

    pub fn cli_vpc_delete() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("vpc")
                    .long("vpc")
                    .required(true)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .arg(
                clap::Arg::new("project")
                    .long("project")
                    .required(false)
                    .value_parser(clap::value_parser!(types::NameOrId)),
            )
            .about("Delete a VPC")
    }
}

impl<T: CliOverride> Cli<T> {
    pub fn new_with_override(client: oxide_api::Client, over: T) -> Self {
        Self { client, over }
    }

    pub async fn execute(&self, cmd: CliCommand, matches: &clap::ArgMatches) {
        match cmd {
            CliCommand::DeviceAuthRequest => {
                self.execute_device_auth_request(matches).await;
            }
            CliCommand::DeviceAuthConfirm => {
                self.execute_device_auth_confirm(matches).await;
            }
            CliCommand::DeviceAccessToken => {
                self.execute_device_access_token(matches).await;
            }
            CliCommand::LoginSpoof => {
                self.execute_login_spoof(matches).await;
            }
            CliCommand::LoginLocal => {
                self.execute_login_local(matches).await;
            }
            CliCommand::LoginSamlBegin => {
                self.execute_login_saml_begin(matches).await;
            }
            CliCommand::LoginSaml => {
                self.execute_login_saml(matches).await;
            }
            CliCommand::Logout => {
                self.execute_logout(matches).await;
            }
            CliCommand::SystemImageViewById => {
                self.execute_system_image_view_by_id(matches).await;
            }
            CliCommand::SystemImageList => {
                self.execute_system_image_list(matches).await;
            }
            CliCommand::SystemImageCreate => {
                self.execute_system_image_create(matches).await;
            }
            CliCommand::SystemImageView => {
                self.execute_system_image_view(matches).await;
            }
            CliCommand::SystemImageDelete => {
                self.execute_system_image_delete(matches).await;
            }
            CliCommand::DiskList => {
                self.execute_disk_list(matches).await;
            }
            CliCommand::DiskCreate => {
                self.execute_disk_create(matches).await;
            }
            CliCommand::DiskView => {
                self.execute_disk_view(matches).await;
            }
            CliCommand::DiskDelete => {
                self.execute_disk_delete(matches).await;
            }
            CliCommand::DiskMetricsList => {
                self.execute_disk_metrics_list(matches).await;
            }
            CliCommand::GroupList => {
                self.execute_group_list(matches).await;
            }
            CliCommand::GroupView => {
                self.execute_group_view(matches).await;
            }
            CliCommand::ImageList => {
                self.execute_image_list(matches).await;
            }
            CliCommand::ImageCreate => {
                self.execute_image_create(matches).await;
            }
            CliCommand::ImageView => {
                self.execute_image_view(matches).await;
            }
            CliCommand::ImageDelete => {
                self.execute_image_delete(matches).await;
            }
            CliCommand::InstanceList => {
                self.execute_instance_list(matches).await;
            }
            CliCommand::InstanceCreate => {
                self.execute_instance_create(matches).await;
            }
            CliCommand::InstanceView => {
                self.execute_instance_view(matches).await;
            }
            CliCommand::InstanceDelete => {
                self.execute_instance_delete(matches).await;
            }
            CliCommand::InstanceDiskList => {
                self.execute_instance_disk_list(matches).await;
            }
            CliCommand::InstanceDiskAttach => {
                self.execute_instance_disk_attach(matches).await;
            }
            CliCommand::InstanceDiskDetach => {
                self.execute_instance_disk_detach(matches).await;
            }
            CliCommand::InstanceExternalIpList => {
                self.execute_instance_external_ip_list(matches).await;
            }
            CliCommand::InstanceMigrate => {
                self.execute_instance_migrate(matches).await;
            }
            CliCommand::InstanceReboot => {
                self.execute_instance_reboot(matches).await;
            }
            CliCommand::InstanceSerialConsole => {
                self.execute_instance_serial_console(matches).await;
            }
            CliCommand::InstanceSerialConsoleStream => {
                self.execute_instance_serial_console_stream(matches).await;
            }
            CliCommand::InstanceStart => {
                self.execute_instance_start(matches).await;
            }
            CliCommand::InstanceStop => {
                self.execute_instance_stop(matches).await;
            }
            CliCommand::CurrentUserView => {
                self.execute_current_user_view(matches).await;
            }
            CliCommand::CurrentUserGroups => {
                self.execute_current_user_groups(matches).await;
            }
            CliCommand::CurrentUserSshKeyList => {
                self.execute_current_user_ssh_key_list(matches).await;
            }
            CliCommand::CurrentUserSshKeyCreate => {
                self.execute_current_user_ssh_key_create(matches).await;
            }
            CliCommand::CurrentUserSshKeyView => {
                self.execute_current_user_ssh_key_view(matches).await;
            }
            CliCommand::CurrentUserSshKeyDelete => {
                self.execute_current_user_ssh_key_delete(matches).await;
            }
            CliCommand::InstanceNetworkInterfaceList => {
                self.execute_instance_network_interface_list(matches).await;
            }
            CliCommand::InstanceNetworkInterfaceCreate => {
                self.execute_instance_network_interface_create(matches)
                    .await;
            }
            CliCommand::InstanceNetworkInterfaceView => {
                self.execute_instance_network_interface_view(matches).await;
            }
            CliCommand::InstanceNetworkInterfaceUpdate => {
                self.execute_instance_network_interface_update(matches)
                    .await;
            }
            CliCommand::InstanceNetworkInterfaceDelete => {
                self.execute_instance_network_interface_delete(matches)
                    .await;
            }
            CliCommand::PolicyView => {
                self.execute_policy_view(matches).await;
            }
            CliCommand::PolicyUpdate => {
                self.execute_policy_update(matches).await;
            }
            CliCommand::ProjectList => {
                self.execute_project_list(matches).await;
            }
            CliCommand::ProjectCreate => {
                self.execute_project_create(matches).await;
            }
            CliCommand::ProjectView => {
                self.execute_project_view(matches).await;
            }
            CliCommand::ProjectUpdate => {
                self.execute_project_update(matches).await;
            }
            CliCommand::ProjectDelete => {
                self.execute_project_delete(matches).await;
            }
            CliCommand::ProjectPolicyView => {
                self.execute_project_policy_view(matches).await;
            }
            CliCommand::ProjectPolicyUpdate => {
                self.execute_project_policy_update(matches).await;
            }
            CliCommand::SnapshotList => {
                self.execute_snapshot_list(matches).await;
            }
            CliCommand::SnapshotCreate => {
                self.execute_snapshot_create(matches).await;
            }
            CliCommand::SnapshotView => {
                self.execute_snapshot_view(matches).await;
            }
            CliCommand::SnapshotDelete => {
                self.execute_snapshot_delete(matches).await;
            }
            CliCommand::CertificateList => {
                self.execute_certificate_list(matches).await;
            }
            CliCommand::CertificateCreate => {
                self.execute_certificate_create(matches).await;
            }
            CliCommand::CertificateView => {
                self.execute_certificate_view(matches).await;
            }
            CliCommand::CertificateDelete => {
                self.execute_certificate_delete(matches).await;
            }
            CliCommand::PhysicalDiskList => {
                self.execute_physical_disk_list(matches).await;
            }
            CliCommand::RackList => {
                self.execute_rack_list(matches).await;
            }
            CliCommand::RackView => {
                self.execute_rack_view(matches).await;
            }
            CliCommand::SledList => {
                self.execute_sled_list(matches).await;
            }
            CliCommand::SledView => {
                self.execute_sled_view(matches).await;
            }
            CliCommand::SledPhysicalDiskList => {
                self.execute_sled_physical_disk_list(matches).await;
            }
            CliCommand::SiloIdentityProviderList => {
                self.execute_silo_identity_provider_list(matches).await;
            }
            CliCommand::LocalIdpUserCreate => {
                self.execute_local_idp_user_create(matches).await;
            }
            CliCommand::LocalIdpUserDelete => {
                self.execute_local_idp_user_delete(matches).await;
            }
            CliCommand::LocalIdpUserSetPassword => {
                self.execute_local_idp_user_set_password(matches).await;
            }
            CliCommand::SamlIdentityProviderCreate => {
                self.execute_saml_identity_provider_create(matches).await;
            }
            CliCommand::SamlIdentityProviderView => {
                self.execute_saml_identity_provider_view(matches).await;
            }
            CliCommand::IpPoolList => {
                self.execute_ip_pool_list(matches).await;
            }
            CliCommand::IpPoolCreate => {
                self.execute_ip_pool_create(matches).await;
            }
            CliCommand::IpPoolView => {
                self.execute_ip_pool_view(matches).await;
            }
            CliCommand::IpPoolUpdate => {
                self.execute_ip_pool_update(matches).await;
            }
            CliCommand::IpPoolDelete => {
                self.execute_ip_pool_delete(matches).await;
            }
            CliCommand::IpPoolRangeList => {
                self.execute_ip_pool_range_list(matches).await;
            }
            CliCommand::IpPoolRangeAdd => {
                self.execute_ip_pool_range_add(matches).await;
            }
            CliCommand::IpPoolRangeRemove => {
                self.execute_ip_pool_range_remove(matches).await;
            }
            CliCommand::IpPoolServiceView => {
                self.execute_ip_pool_service_view(matches).await;
            }
            CliCommand::IpPoolServiceRangeList => {
                self.execute_ip_pool_service_range_list(matches).await;
            }
            CliCommand::IpPoolServiceRangeAdd => {
                self.execute_ip_pool_service_range_add(matches).await;
            }
            CliCommand::IpPoolServiceRangeRemove => {
                self.execute_ip_pool_service_range_remove(matches).await;
            }
            CliCommand::SystemMetric => {
                self.execute_system_metric(matches).await;
            }
            CliCommand::SystemPolicyView => {
                self.execute_system_policy_view(matches).await;
            }
            CliCommand::SystemPolicyUpdate => {
                self.execute_system_policy_update(matches).await;
            }
            CliCommand::RoleList => {
                self.execute_role_list(matches).await;
            }
            CliCommand::RoleView => {
                self.execute_role_view(matches).await;
            }
            CliCommand::SagaList => {
                self.execute_saga_list(matches).await;
            }
            CliCommand::SagaView => {
                self.execute_saga_view(matches).await;
            }
            CliCommand::SiloList => {
                self.execute_silo_list(matches).await;
            }
            CliCommand::SiloCreate => {
                self.execute_silo_create(matches).await;
            }
            CliCommand::SiloView => {
                self.execute_silo_view(matches).await;
            }
            CliCommand::SiloDelete => {
                self.execute_silo_delete(matches).await;
            }
            CliCommand::SiloPolicyView => {
                self.execute_silo_policy_view(matches).await;
            }
            CliCommand::SiloPolicyUpdate => {
                self.execute_silo_policy_update(matches).await;
            }
            CliCommand::SystemComponentVersionList => {
                self.execute_system_component_version_list(matches).await;
            }
            CliCommand::UpdateDeploymentsList => {
                self.execute_update_deployments_list(matches).await;
            }
            CliCommand::UpdateDeploymentView => {
                self.execute_update_deployment_view(matches).await;
            }
            CliCommand::SystemUpdateRefresh => {
                self.execute_system_update_refresh(matches).await;
            }
            CliCommand::SystemUpdateStart => {
                self.execute_system_update_start(matches).await;
            }
            CliCommand::SystemUpdateStop => {
                self.execute_system_update_stop(matches).await;
            }
            CliCommand::SystemUpdateList => {
                self.execute_system_update_list(matches).await;
            }
            CliCommand::SystemUpdateView => {
                self.execute_system_update_view(matches).await;
            }
            CliCommand::SystemUpdateComponentsList => {
                self.execute_system_update_components_list(matches).await;
            }
            CliCommand::SystemVersion => {
                self.execute_system_version(matches).await;
            }
            CliCommand::SiloUserList => {
                self.execute_silo_user_list(matches).await;
            }
            CliCommand::SiloUserView => {
                self.execute_silo_user_view(matches).await;
            }
            CliCommand::UserBuiltinList => {
                self.execute_user_builtin_list(matches).await;
            }
            CliCommand::UserBuiltinView => {
                self.execute_user_builtin_view(matches).await;
            }
            CliCommand::UserList => {
                self.execute_user_list(matches).await;
            }
            CliCommand::VpcFirewallRulesView => {
                self.execute_vpc_firewall_rules_view(matches).await;
            }
            CliCommand::VpcFirewallRulesUpdate => {
                self.execute_vpc_firewall_rules_update(matches).await;
            }
            CliCommand::VpcRouterRouteList => {
                self.execute_vpc_router_route_list(matches).await;
            }
            CliCommand::VpcRouterRouteCreate => {
                self.execute_vpc_router_route_create(matches).await;
            }
            CliCommand::VpcRouterRouteView => {
                self.execute_vpc_router_route_view(matches).await;
            }
            CliCommand::VpcRouterRouteUpdate => {
                self.execute_vpc_router_route_update(matches).await;
            }
            CliCommand::VpcRouterRouteDelete => {
                self.execute_vpc_router_route_delete(matches).await;
            }
            CliCommand::VpcRouterList => {
                self.execute_vpc_router_list(matches).await;
            }
            CliCommand::VpcRouterCreate => {
                self.execute_vpc_router_create(matches).await;
            }
            CliCommand::VpcRouterView => {
                self.execute_vpc_router_view(matches).await;
            }
            CliCommand::VpcRouterUpdate => {
                self.execute_vpc_router_update(matches).await;
            }
            CliCommand::VpcRouterDelete => {
                self.execute_vpc_router_delete(matches).await;
            }
            CliCommand::VpcSubnetList => {
                self.execute_vpc_subnet_list(matches).await;
            }
            CliCommand::VpcSubnetCreate => {
                self.execute_vpc_subnet_create(matches).await;
            }
            CliCommand::VpcSubnetView => {
                self.execute_vpc_subnet_view(matches).await;
            }
            CliCommand::VpcSubnetUpdate => {
                self.execute_vpc_subnet_update(matches).await;
            }
            CliCommand::VpcSubnetDelete => {
                self.execute_vpc_subnet_delete(matches).await;
            }
            CliCommand::VpcSubnetListNetworkInterfaces => {
                self.execute_vpc_subnet_list_network_interfaces(matches)
                    .await;
            }
            CliCommand::VpcList => {
                self.execute_vpc_list(matches).await;
            }
            CliCommand::VpcCreate => {
                self.execute_vpc_create(matches).await;
            }
            CliCommand::VpcView => {
                self.execute_vpc_view(matches).await;
            }
            CliCommand::VpcUpdate => {
                self.execute_vpc_update(matches).await;
            }
            CliCommand::VpcDelete => {
                self.execute_vpc_delete(matches).await;
            }
        }
    }

    pub async fn execute_device_auth_request(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.device_auth_request();
        if let Some(value) = matches.get_one::<uuid::Uuid>("client-id") {
            request = request.body_map(|body| body.client_id(value.clone()))
        }

        self.over
            .execute_device_auth_request(matches, &mut request)
            .unwrap();
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

    pub async fn execute_device_auth_confirm(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.device_auth_confirm();
        if let Some(value) = matches.get_one::<String>("user-code") {
            request = request.body_map(|body| body.user_code(value.clone()))
        }

        self.over
            .execute_device_auth_confirm(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_device_access_token(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_device_access_token(matches, &mut request)
            .unwrap();
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

    pub async fn execute_login_spoof(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.login_spoof();
        if let Some(value) = matches.get_one::<String>("username") {
            request = request.body_map(|body| body.username(value.clone()))
        }

        self.over
            .execute_login_spoof(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_login_local(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.login_local();
        if let Some(value) = matches.get_one::<types::Name>("silo-name") {
            request = request.silo_name(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Password>("password") {
            request = request.body_map(|body| body.password(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::UserId>("username") {
            request = request.body_map(|body| body.username(value.clone()))
        }

        self.over
            .execute_login_local(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_login_saml_begin(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.login_saml_begin();
        if let Some(value) = matches.get_one::<types::Name>("silo-name") {
            request = request.silo_name(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("provider-name") {
            request = request.provider_name(value.clone());
        }

        self.over
            .execute_login_saml_begin(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_login_saml(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.login_saml();
        if let Some(value) = matches.get_one::<types::Name>("silo-name") {
            request = request.silo_name(value.clone());
        }

        if let Some(value) = matches.get_one::<types::Name>("provider-name") {
            request = request.provider_name(value.clone());
        }

        self.over.execute_login_saml(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_logout(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.logout();
        self.over.execute_logout(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_image_view_by_id(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_image_view_by_id();
        if let Some(value) = matches.get_one::<uuid::Uuid>("id") {
            request = request.id(value.clone());
        }

        self.over
            .execute_system_image_view_by_id(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_image_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_image_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_system_image_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_image_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_image_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        self.over
            .execute_system_image_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_image_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_image_view();
        if let Some(value) = matches.get_one::<types::Name>("image-name") {
            request = request.image_name(value.clone());
        }

        self.over
            .execute_system_image_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_image_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_image_delete();
        if let Some(value) = matches.get_one::<types::Name>("image-name") {
            request = request.image_name(value.clone());
        }

        self.over
            .execute_system_image_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_disk_list(&self, matches: &clap::ArgMatches) {
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

        self.over.execute_disk_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_disk_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.disk_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::ByteCount>("size") {
            request = request.body_map(|body| body.size(value.clone()))
        }

        self.over
            .execute_disk_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_disk_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.disk_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over.execute_disk_view(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_disk_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.disk_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_disk_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_disk_metrics_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.disk_metrics_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.disk(value.clone());
        }

        if let Some(value) = matches.get_one::<types::DiskMetricName>("metric") {
            request = request.metric(value.clone());
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("end-time") {
            request = request.end_time(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("start-time")
        {
            request = request.start_time(value.clone());
        }

        self.over
            .execute_disk_metrics_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_group_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.group_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over.execute_group_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_group_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.group_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("group") {
            request = request.group(value.clone());
        }

        self.over.execute_group_view(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_image_list(&self, matches: &clap::ArgMatches) {
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

        self.over.execute_image_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_image_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.image_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("os") {
            request = request.body_map(|body| body.os(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("version") {
            request = request.body_map(|body| body.version(value.clone()))
        }

        self.over
            .execute_image_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_image_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.image_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("image") {
            request = request.image(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over.execute_image_view(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_image_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.image_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("image") {
            request = request.image(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_image_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_list(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_instance_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("hostname") {
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

        if let Some(value) = matches.get_one::<bool>("start") {
            request = request.body_map(|body| body.start(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("user-data") {
            request = request.body_map(|body| body.user_data(value.clone()))
        }

        self.over
            .execute_instance_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_instance_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_instance_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_disk_list(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_instance_disk_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_disk_attach(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_disk_attach();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.body_map(|body| body.disk(value.clone()))
        }

        self.over
            .execute_instance_disk_attach(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_disk_detach(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_disk_detach();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("disk") {
            request = request.body_map(|body| body.disk(value.clone()))
        }

        self.over
            .execute_instance_disk_detach(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_external_ip_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_external_ip_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_instance_external_ip_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_migrate(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_migrate();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("dst-sled-id") {
            request = request.body_map(|body| body.dst_sled_id(value.clone()))
        }

        self.over
            .execute_instance_migrate(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_reboot(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_reboot();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_instance_reboot(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_serial_console(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_serial_console();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<u64>("from-start") {
            request = request.from_start(value.clone());
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

        self.over
            .execute_instance_serial_console(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_serial_console_stream(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_serial_console_stream();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_instance_serial_console_stream(matches, &mut request)
            .unwrap();
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

    pub async fn execute_instance_start(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_start();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_instance_start(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_stop(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_stop();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_instance_stop(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_current_user_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.current_user_view();
        self.over
            .execute_current_user_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_current_user_groups(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.current_user_groups();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_current_user_groups(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_current_user_ssh_key_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.current_user_ssh_key_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_current_user_ssh_key_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_current_user_ssh_key_create(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_current_user_ssh_key_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_current_user_ssh_key_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.current_user_ssh_key_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("ssh-key") {
            request = request.ssh_key(value.clone());
        }

        self.over
            .execute_current_user_ssh_key_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_current_user_ssh_key_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.current_user_ssh_key_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("ssh-key") {
            request = request.ssh_key(value.clone());
        }

        self.over
            .execute_current_user_ssh_key_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_network_interface_list(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_instance_network_interface_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_network_interface_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_network_interface_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("subnet-name") {
            request = request.body_map(|body| body.subnet_name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("vpc-name") {
            request = request.body_map(|body| body.vpc_name(value.clone()))
        }

        self.over
            .execute_instance_network_interface_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_network_interface_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_network_interface_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("interface") {
            request = request.interface(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_instance_network_interface_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_network_interface_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_network_interface_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("interface") {
            request = request.interface(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<bool>("primary") {
            request = request.body_map(|body| body.primary(value.clone()))
        }

        self.over
            .execute_instance_network_interface_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_instance_network_interface_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.instance_network_interface_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("interface") {
            request = request.interface(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("instance") {
            request = request.instance(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_instance_network_interface_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_policy_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.policy_view();
        self.over
            .execute_policy_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_policy_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.policy_update();
        self.over
            .execute_policy_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_project_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.project_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_project_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_project_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.project_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        self.over
            .execute_project_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_project_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.project_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_project_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_project_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.project_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_project_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_project_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.project_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_project_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_project_policy_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.project_policy_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_project_policy_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_project_policy_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.project_policy_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_project_policy_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_snapshot_list(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_snapshot_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_snapshot_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.snapshot_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("disk") {
            request = request.body_map(|body| body.disk(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        self.over
            .execute_snapshot_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_snapshot_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.snapshot_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("snapshot") {
            request = request.snapshot(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_snapshot_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_snapshot_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.snapshot_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("snapshot") {
            request = request.snapshot(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over
            .execute_snapshot_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_certificate_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.certificate_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_certificate_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_certificate_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.certificate_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::ServiceUsingCertificate>("service") {
            request = request.body_map(|body| body.service(value.clone()))
        }

        self.over
            .execute_certificate_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_certificate_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.certificate_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("certificate") {
            request = request.certificate(value.clone());
        }

        self.over
            .execute_certificate_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_certificate_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.certificate_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("certificate") {
            request = request.certificate(value.clone());
        }

        self.over
            .execute_certificate_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_physical_disk_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.physical_disk_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_physical_disk_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_rack_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.rack_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over.execute_rack_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_rack_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.rack_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("rack-id") {
            request = request.rack_id(value.clone());
        }

        self.over.execute_rack_view(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_sled_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.sled_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over.execute_sled_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_sled_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.sled_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("sled-id") {
            request = request.sled_id(value.clone());
        }

        self.over.execute_sled_view(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_sled_physical_disk_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.sled_physical_disk_list();
        if let Some(value) = matches.get_one::<uuid::Uuid>("sled-id") {
            request = request.sled_id(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_sled_physical_disk_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_silo_identity_provider_list(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_silo_identity_provider_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_local_idp_user_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.local_idp_user_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<types::UserId>("external-id") {
            request = request.body_map(|body| body.external_id(value.clone()))
        }

        self.over
            .execute_local_idp_user_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_local_idp_user_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.local_idp_user_delete();
        if let Some(value) = matches.get_one::<uuid::Uuid>("user-id") {
            request = request.user_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.over
            .execute_local_idp_user_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_local_idp_user_set_password(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.local_idp_user_set_password();
        if let Some(value) = matches.get_one::<uuid::Uuid>("user-id") {
            request = request.user_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.over
            .execute_local_idp_user_set_password(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_saml_identity_provider_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.saml_identity_provider_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("acs-url") {
            request = request.body_map(|body| body.acs_url(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("idp-entity-id") {
            request = request.body_map(|body| body.idp_entity_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
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

        self.over
            .execute_saml_identity_provider_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_saml_identity_provider_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.saml_identity_provider_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("provider") {
            request = request.provider(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.over
            .execute_saml_identity_provider_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_ip_pool_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_create();
        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        self.over
            .execute_ip_pool_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.over
            .execute_ip_pool_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.over
            .execute_ip_pool_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.over
            .execute_ip_pool_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_range_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_range_list();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        self.over
            .execute_ip_pool_range_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_range_add(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_range_add();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.over
            .execute_ip_pool_range_add(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_range_remove(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_range_remove();
        if let Some(value) = matches.get_one::<types::NameOrId>("pool") {
            request = request.pool(value.clone());
        }

        self.over
            .execute_ip_pool_range_remove(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_service_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_service_view();
        self.over
            .execute_ip_pool_service_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_service_range_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_service_range_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        self.over
            .execute_ip_pool_service_range_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_service_range_add(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_service_range_add();
        self.over
            .execute_ip_pool_service_range_add(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_ip_pool_service_range_remove(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.ip_pool_service_range_remove();
        self.over
            .execute_ip_pool_service_range_remove(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_metric(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_metric();
        if let Some(value) = matches.get_one::<types::SystemMetricName>("metric-name") {
            request = request.metric_name(value.clone());
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("end-time") {
            request = request.end_time(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("id") {
            request = request.id(value.clone());
        }

        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("page-token") {
            request = request.page_token(value.clone());
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("start-time")
        {
            request = request.start_time(value.clone());
        }

        self.over
            .execute_system_metric(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_policy_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_policy_view();
        self.over
            .execute_system_policy_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_policy_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_policy_update();
        self.over
            .execute_system_policy_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_role_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.role_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        self.over.execute_role_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_role_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.role_view();
        if let Some(value) = matches.get_one::<String>("role-name") {
            request = request.role_name(value.clone());
        }

        self.over.execute_role_view(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_saga_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.saga_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over.execute_saga_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_saga_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.saga_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("saga-id") {
            request = request.saga_id(value.clone());
        }

        self.over.execute_saga_view(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_silo_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.silo_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrIdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over.execute_silo_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_silo_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.silo_create();
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

        self.over
            .execute_silo_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_silo_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.silo_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.over.execute_silo_view(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_silo_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.silo_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.over
            .execute_silo_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_silo_policy_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.silo_policy_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.over
            .execute_silo_policy_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_silo_policy_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.silo_policy_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.over
            .execute_silo_policy_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_component_version_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_component_version_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_system_component_version_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_update_deployments_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.update_deployments_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_update_deployments_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_update_deployment_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.update_deployment_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("id") {
            request = request.id(value.clone());
        }

        self.over
            .execute_update_deployment_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_update_refresh(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_update_refresh();
        self.over
            .execute_system_update_refresh(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_update_start(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_update_start();
        if let Some(value) = matches.get_one::<types::SemverVersion>("version") {
            request = request.body_map(|body| body.version(value.clone()))
        }

        self.over
            .execute_system_update_start(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_update_stop(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_update_stop();
        self.over
            .execute_system_update_stop(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_update_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_update_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::IdSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_system_update_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_update_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_update_view();
        if let Some(value) = matches.get_one::<types::SemverVersion>("version") {
            request = request.version(value.clone());
        }

        self.over
            .execute_system_update_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_update_components_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_update_components_list();
        if let Some(value) = matches.get_one::<types::SemverVersion>("version") {
            request = request.version(value.clone());
        }

        self.over
            .execute_system_update_components_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_system_version(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.system_version();
        self.over
            .execute_system_version(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_silo_user_list(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_silo_user_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_silo_user_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.silo_user_view();
        if let Some(value) = matches.get_one::<uuid::Uuid>("user-id") {
            request = request.user_id(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("silo") {
            request = request.silo(value.clone());
        }

        self.over
            .execute_silo_user_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_user_builtin_list(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.user_builtin_list();
        if let Some(value) = matches.get_one::<std::num::NonZeroU32>("limit") {
            request = request.limit(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameSortMode>("sort-by") {
            request = request.sort_by(value.clone());
        }

        self.over
            .execute_user_builtin_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_user_builtin_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.user_builtin_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("user") {
            request = request.user(value.clone());
        }

        self.over
            .execute_user_builtin_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_user_list(&self, matches: &clap::ArgMatches) {
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

        self.over.execute_user_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_firewall_rules_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_firewall_rules_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.over
            .execute_vpc_firewall_rules_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_firewall_rules_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_firewall_rules_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.over
            .execute_vpc_firewall_rules_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_route_list(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_vpc_router_route_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_route_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_router_route_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        self.over
            .execute_vpc_router_route_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_route_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_router_route_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("route") {
            request = request.route(value.clone());
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

        self.over
            .execute_vpc_router_route_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_route_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_router_route_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("route") {
            request = request.route(value.clone());
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

        self.over
            .execute_vpc_router_route_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_route_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_router_route_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("route") {
            request = request.route(value.clone());
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

        self.over
            .execute_vpc_router_route_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_list(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_vpc_router_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_router_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        self.over
            .execute_vpc_router_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_router_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.over
            .execute_vpc_router_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_router_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.over
            .execute_vpc_router_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_router_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_router_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("router") {
            request = request.router(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.over
            .execute_vpc_router_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_subnet_list(&self, matches: &clap::ArgMatches) {
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

        self.over
            .execute_vpc_subnet_list(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_subnet_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_subnet_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Ipv4Net>("ipv4-block") {
            request = request.body_map(|body| body.ipv4_block(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        self.over
            .execute_vpc_subnet_create(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_subnet_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_subnet_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("subnet") {
            request = request.subnet(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.over
            .execute_vpc_subnet_view(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_subnet_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_subnet_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("subnet") {
            request = request.subnet(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.over
            .execute_vpc_subnet_update(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_subnet_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_subnet_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("subnet") {
            request = request.subnet(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        self.over
            .execute_vpc_subnet_delete(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_subnet_list_network_interfaces(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_subnet_list_network_interfaces();
        if let Some(value) = matches.get_one::<types::NameOrId>("subnet") {
            request = request.subnet(value.clone());
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

        self.over
            .execute_vpc_subnet_list_network_interfaces(matches, &mut request)
            .unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_list(&self, matches: &clap::ArgMatches) {
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

        self.over.execute_vpc_list(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_create(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_create();
        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("description") {
            request = request.body_map(|body| body.description(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("dns-name") {
            request = request.body_map(|body| body.dns_name(value.clone()))
        }

        if let Some(value) = matches.get_one::<types::Name>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        self.over.execute_vpc_create(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_view(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_view();
        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over.execute_vpc_view(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_update(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_update();
        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over.execute_vpc_update(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }

    pub async fn execute_vpc_delete(&self, matches: &clap::ArgMatches) {
        let mut request = self.client.vpc_delete();
        if let Some(value) = matches.get_one::<types::NameOrId>("vpc") {
            request = request.vpc(value.clone());
        }

        if let Some(value) = matches.get_one::<types::NameOrId>("project") {
            request = request.project(value.clone());
        }

        self.over.execute_vpc_delete(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("error\n{:#?}", r)
            }
        }
    }
}

pub trait CliOverride {
    fn execute_device_auth_request(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeviceAuthRequest,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_device_auth_confirm(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeviceAuthConfirm,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_device_access_token(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DeviceAccessToken,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_login_spoof(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LoginSpoof,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_login_local(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LoginLocal,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_login_saml_begin(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LoginSamlBegin,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_login_saml(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LoginSaml,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_logout(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::Logout,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_image_view_by_id(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemImageViewById,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_image_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemImageList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_image_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemImageCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_image_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemImageView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_image_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemImageDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_disk_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_disk_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_disk_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_disk_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_disk_metrics_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::DiskMetricsList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_group_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GroupList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_group_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::GroupView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_image_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImageList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_image_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImageCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_image_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImageView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_image_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ImageDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_disk_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceDiskList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_disk_attach(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceDiskAttach,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_disk_detach(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceDiskDetach,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_external_ip_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceExternalIpList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_migrate(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceMigrate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_reboot(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceReboot,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_serial_console(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceSerialConsole,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_serial_console_stream(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceSerialConsoleStream,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_start(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceStart,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_stop(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceStop,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_current_user_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_current_user_groups(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserGroups,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_current_user_ssh_key_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserSshKeyList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_current_user_ssh_key_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserSshKeyCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_current_user_ssh_key_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserSshKeyView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_current_user_ssh_key_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CurrentUserSshKeyDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_network_interface_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_network_interface_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_network_interface_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_network_interface_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_instance_network_interface_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::InstanceNetworkInterfaceDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_policy_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::PolicyView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_policy_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::PolicyUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_project_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_project_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_project_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_project_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_project_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_project_policy_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectPolicyView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_project_policy_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::ProjectPolicyUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_snapshot_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SnapshotList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_snapshot_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SnapshotCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_snapshot_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SnapshotView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_snapshot_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SnapshotDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_certificate_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CertificateList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_certificate_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CertificateCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_certificate_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CertificateView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_certificate_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::CertificateDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_physical_disk_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::PhysicalDiskList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_rack_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RackList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_rack_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RackView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_sled_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_sled_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_sled_physical_disk_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SledPhysicalDiskList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_silo_identity_provider_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloIdentityProviderList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_local_idp_user_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LocalIdpUserCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_local_idp_user_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LocalIdpUserDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_local_idp_user_set_password(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::LocalIdpUserSetPassword,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_saml_identity_provider_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SamlIdentityProviderCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_saml_identity_provider_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SamlIdentityProviderView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_range_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolRangeList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_range_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolRangeAdd,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_range_remove(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolRangeRemove,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_service_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolServiceView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_service_range_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolServiceRangeList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_service_range_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolServiceRangeAdd,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_ip_pool_service_range_remove(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::IpPoolServiceRangeRemove,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_metric(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemMetric,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_policy_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemPolicyView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_policy_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemPolicyUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_role_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RoleList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_role_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::RoleView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_saga_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SagaList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_saga_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SagaView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_silo_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_silo_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_silo_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_silo_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_silo_policy_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloPolicyView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_silo_policy_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloPolicyUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_component_version_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemComponentVersionList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_update_deployments_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UpdateDeploymentsList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_update_deployment_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UpdateDeploymentView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_update_refresh(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemUpdateRefresh,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_update_start(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemUpdateStart,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_update_stop(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemUpdateStop,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_update_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemUpdateList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_update_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemUpdateView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_update_components_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemUpdateComponentsList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_system_version(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SystemVersion,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_silo_user_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloUserList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_silo_user_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::SiloUserView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_user_builtin_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UserBuiltinList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_user_builtin_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UserBuiltinView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_user_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::UserList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_firewall_rules_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcFirewallRulesView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_firewall_rules_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcFirewallRulesUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_route_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_route_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_route_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_route_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_route_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterRouteDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_router_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcRouterDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_subnet_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_subnet_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_subnet_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_subnet_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_subnet_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetDelete,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_subnet_list_network_interfaces(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcSubnetListNetworkInterfaces,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_list(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcList,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcCreate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_view(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcView,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcUpdate,
    ) -> Result<(), String> {
        Ok(())
    }

    fn execute_vpc_delete(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::VpcDelete,
    ) -> Result<(), String> {
        Ok(())
    }
}

impl CliOverride for () {}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    DeviceAuthRequest,
    DeviceAuthConfirm,
    DeviceAccessToken,
    LoginSpoof,
    LoginLocal,
    LoginSamlBegin,
    LoginSaml,
    Logout,
    SystemImageViewById,
    SystemImageList,
    SystemImageCreate,
    SystemImageView,
    SystemImageDelete,
    DiskList,
    DiskCreate,
    DiskView,
    DiskDelete,
    DiskMetricsList,
    GroupList,
    GroupView,
    ImageList,
    ImageCreate,
    ImageView,
    ImageDelete,
    InstanceList,
    InstanceCreate,
    InstanceView,
    InstanceDelete,
    InstanceDiskList,
    InstanceDiskAttach,
    InstanceDiskDetach,
    InstanceExternalIpList,
    InstanceMigrate,
    InstanceReboot,
    InstanceSerialConsole,
    InstanceSerialConsoleStream,
    InstanceStart,
    InstanceStop,
    CurrentUserView,
    CurrentUserGroups,
    CurrentUserSshKeyList,
    CurrentUserSshKeyCreate,
    CurrentUserSshKeyView,
    CurrentUserSshKeyDelete,
    InstanceNetworkInterfaceList,
    InstanceNetworkInterfaceCreate,
    InstanceNetworkInterfaceView,
    InstanceNetworkInterfaceUpdate,
    InstanceNetworkInterfaceDelete,
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
    CertificateList,
    CertificateCreate,
    CertificateView,
    CertificateDelete,
    PhysicalDiskList,
    RackList,
    RackView,
    SledList,
    SledView,
    SledPhysicalDiskList,
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
    IpPoolServiceView,
    IpPoolServiceRangeList,
    IpPoolServiceRangeAdd,
    IpPoolServiceRangeRemove,
    SystemMetric,
    SystemPolicyView,
    SystemPolicyUpdate,
    RoleList,
    RoleView,
    SagaList,
    SagaView,
    SiloList,
    SiloCreate,
    SiloView,
    SiloDelete,
    SiloPolicyView,
    SiloPolicyUpdate,
    SystemComponentVersionList,
    UpdateDeploymentsList,
    UpdateDeploymentView,
    SystemUpdateRefresh,
    SystemUpdateStart,
    SystemUpdateStop,
    SystemUpdateList,
    SystemUpdateView,
    SystemUpdateComponentsList,
    SystemVersion,
    SiloUserList,
    SiloUserView,
    UserBuiltinList,
    UserBuiltinView,
    UserList,
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
            CliCommand::LoginSpoof,
            CliCommand::LoginLocal,
            CliCommand::LoginSamlBegin,
            CliCommand::LoginSaml,
            CliCommand::Logout,
            CliCommand::SystemImageViewById,
            CliCommand::SystemImageList,
            CliCommand::SystemImageCreate,
            CliCommand::SystemImageView,
            CliCommand::SystemImageDelete,
            CliCommand::DiskList,
            CliCommand::DiskCreate,
            CliCommand::DiskView,
            CliCommand::DiskDelete,
            CliCommand::DiskMetricsList,
            CliCommand::GroupList,
            CliCommand::GroupView,
            CliCommand::ImageList,
            CliCommand::ImageCreate,
            CliCommand::ImageView,
            CliCommand::ImageDelete,
            CliCommand::InstanceList,
            CliCommand::InstanceCreate,
            CliCommand::InstanceView,
            CliCommand::InstanceDelete,
            CliCommand::InstanceDiskList,
            CliCommand::InstanceDiskAttach,
            CliCommand::InstanceDiskDetach,
            CliCommand::InstanceExternalIpList,
            CliCommand::InstanceMigrate,
            CliCommand::InstanceReboot,
            CliCommand::InstanceSerialConsole,
            CliCommand::InstanceSerialConsoleStream,
            CliCommand::InstanceStart,
            CliCommand::InstanceStop,
            CliCommand::CurrentUserView,
            CliCommand::CurrentUserGroups,
            CliCommand::CurrentUserSshKeyList,
            CliCommand::CurrentUserSshKeyCreate,
            CliCommand::CurrentUserSshKeyView,
            CliCommand::CurrentUserSshKeyDelete,
            CliCommand::InstanceNetworkInterfaceList,
            CliCommand::InstanceNetworkInterfaceCreate,
            CliCommand::InstanceNetworkInterfaceView,
            CliCommand::InstanceNetworkInterfaceUpdate,
            CliCommand::InstanceNetworkInterfaceDelete,
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
            CliCommand::CertificateList,
            CliCommand::CertificateCreate,
            CliCommand::CertificateView,
            CliCommand::CertificateDelete,
            CliCommand::PhysicalDiskList,
            CliCommand::RackList,
            CliCommand::RackView,
            CliCommand::SledList,
            CliCommand::SledView,
            CliCommand::SledPhysicalDiskList,
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
            CliCommand::IpPoolServiceView,
            CliCommand::IpPoolServiceRangeList,
            CliCommand::IpPoolServiceRangeAdd,
            CliCommand::IpPoolServiceRangeRemove,
            CliCommand::SystemMetric,
            CliCommand::SystemPolicyView,
            CliCommand::SystemPolicyUpdate,
            CliCommand::RoleList,
            CliCommand::RoleView,
            CliCommand::SagaList,
            CliCommand::SagaView,
            CliCommand::SiloList,
            CliCommand::SiloCreate,
            CliCommand::SiloView,
            CliCommand::SiloDelete,
            CliCommand::SiloPolicyView,
            CliCommand::SiloPolicyUpdate,
            CliCommand::SystemComponentVersionList,
            CliCommand::UpdateDeploymentsList,
            CliCommand::UpdateDeploymentView,
            CliCommand::SystemUpdateRefresh,
            CliCommand::SystemUpdateStart,
            CliCommand::SystemUpdateStop,
            CliCommand::SystemUpdateList,
            CliCommand::SystemUpdateView,
            CliCommand::SystemUpdateComponentsList,
            CliCommand::SystemVersion,
            CliCommand::SiloUserList,
            CliCommand::SiloUserView,
            CliCommand::UserBuiltinList,
            CliCommand::UserBuiltinView,
            CliCommand::UserList,
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
