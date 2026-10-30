// The contents of this file are generated; do not modify them.

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder;
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types;
#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Items consumers will typically use such as the Client and
/// extension traits.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
    pub use super::ClientAffinityExt;
    pub use super::ClientConsoleAuthExt;
    pub use super::ClientCurrentUserExt;
    pub use super::ClientDisksExt;
    pub use super::ClientExperimentalExt;
    pub use super::ClientExternalSubnetsExt;
    pub use super::ClientFloatingIpsExt;
    pub use super::ClientImagesExt;
    pub use super::ClientInstancesExt;
    pub use super::ClientIpPoolsExt;
    pub use super::ClientLoginExt;
    pub use super::ClientMetricsExt;
    pub use super::ClientPolicyExt;
    pub use super::ClientProjectsExt;
    pub use super::ClientSilosExt;
    pub use super::ClientSnapshotsExt;
    pub use super::ClientSubnetPoolsExt;
    pub use super::ClientSystemAlertsExt;
    pub use super::ClientSystemAuditLogExt;
    pub use super::ClientSystemHardwareExt;
    pub use super::ClientSystemIpPoolsExt;
    pub use super::ClientSystemMetricsExt;
    pub use super::ClientSystemNetworkingExt;
    pub use super::ClientSystemSilosExt;
    pub use super::ClientSystemStatusExt;
    pub use super::ClientSystemSubnetPoolsExt;
    pub use super::ClientSystemSupportBundlesExt;
    pub use super::ClientSystemUpdateExt;
    pub use super::ClientTokensExt;
    pub use super::ClientVpcsExt;
}

#[derive(Clone, Debug)]
/// Client for Oxide Region API
///
/// API for interacting with the Oxide control plane
///
/// Version: 2026102100.0.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "2026102100.0.0"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
/// Anti-affinity groups give control over instance placement.
pub trait ClientAffinityExt {
    /// List anti-affinity groups
    ///
    /// Sends a `GET` request to `/v1/anti-affinity-groups`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.anti_affinity_group_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn anti_affinity_group_list(&self) -> builder::AntiAffinityGroupList<'_>;
    /// Create anti-affinity group
    ///
    /// Sends a `POST` request to `/v1/anti-affinity-groups`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.anti_affinity_group_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn anti_affinity_group_create(&self) -> builder::AntiAffinityGroupCreate<'_>;
    /// Fetch anti-affinity group
    ///
    /// Sends a `GET` request to
    /// `/v1/anti-affinity-groups/{anti_affinity_group}`
    ///
    /// Arguments:
    /// - `anti_affinity_group`: Name or ID of the anti affinity group
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.anti_affinity_group_view()
    ///    .anti_affinity_group(anti_affinity_group)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn anti_affinity_group_view(&self) -> builder::AntiAffinityGroupView<'_>;
    /// Update anti-affinity group
    ///
    /// Sends a `PUT` request to
    /// `/v1/anti-affinity-groups/{anti_affinity_group}`
    ///
    /// Arguments:
    /// - `anti_affinity_group`: Name or ID of the anti affinity group
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.anti_affinity_group_update()
    ///    .anti_affinity_group(anti_affinity_group)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn anti_affinity_group_update(&self) -> builder::AntiAffinityGroupUpdate<'_>;
    /// Delete anti-affinity group
    ///
    /// Sends a `DELETE` request to
    /// `/v1/anti-affinity-groups/{anti_affinity_group}`
    ///
    /// Arguments:
    /// - `anti_affinity_group`: Name or ID of the anti affinity group
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.anti_affinity_group_delete()
    ///    .anti_affinity_group(anti_affinity_group)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn anti_affinity_group_delete(&self) -> builder::AntiAffinityGroupDelete<'_>;
    /// List anti-affinity group members
    ///
    /// Sends a `GET` request to
    /// `/v1/anti-affinity-groups/{anti_affinity_group}/members`
    ///
    /// Arguments:
    /// - `anti_affinity_group`: Name or ID of the anti affinity group
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.anti_affinity_group_member_list()
    ///    .anti_affinity_group(anti_affinity_group)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn anti_affinity_group_member_list(&self) -> builder::AntiAffinityGroupMemberList<'_>;
    /// Fetch anti-affinity group member
    ///
    /// Sends a `GET` request to
    /// `/v1/anti-affinity-groups/{anti_affinity_group}/members/instance/
    /// {instance}`
    ///
    /// Arguments:
    /// - `anti_affinity_group`
    /// - `instance`
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.anti_affinity_group_member_instance_view()
    ///    .anti_affinity_group(anti_affinity_group)
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn anti_affinity_group_member_instance_view(
        &self,
    ) -> builder::AntiAffinityGroupMemberInstanceView<'_>;
    /// Add member to anti-affinity group
    ///
    /// Sends a `POST` request to
    /// `/v1/anti-affinity-groups/{anti_affinity_group}/members/instance/
    /// {instance}`
    ///
    /// Arguments:
    /// - `anti_affinity_group`
    /// - `instance`
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.anti_affinity_group_member_instance_add()
    ///    .anti_affinity_group(anti_affinity_group)
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn anti_affinity_group_member_instance_add(
        &self,
    ) -> builder::AntiAffinityGroupMemberInstanceAdd<'_>;
    /// Remove member from anti-affinity group
    ///
    /// Sends a `DELETE` request to
    /// `/v1/anti-affinity-groups/{anti_affinity_group}/members/instance/
    /// {instance}`
    ///
    /// Arguments:
    /// - `anti_affinity_group`
    /// - `instance`
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.anti_affinity_group_member_instance_delete()
    ///    .anti_affinity_group(anti_affinity_group)
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn anti_affinity_group_member_instance_delete(
        &self,
    ) -> builder::AntiAffinityGroupMemberInstanceDelete<'_>;
}

impl ClientAffinityExt for Client {
    fn anti_affinity_group_list(&self) -> builder::AntiAffinityGroupList<'_> {
        builder::AntiAffinityGroupList::new(self)
    }

    fn anti_affinity_group_create(&self) -> builder::AntiAffinityGroupCreate<'_> {
        builder::AntiAffinityGroupCreate::new(self)
    }

    fn anti_affinity_group_view(&self) -> builder::AntiAffinityGroupView<'_> {
        builder::AntiAffinityGroupView::new(self)
    }

    fn anti_affinity_group_update(&self) -> builder::AntiAffinityGroupUpdate<'_> {
        builder::AntiAffinityGroupUpdate::new(self)
    }

    fn anti_affinity_group_delete(&self) -> builder::AntiAffinityGroupDelete<'_> {
        builder::AntiAffinityGroupDelete::new(self)
    }

    fn anti_affinity_group_member_list(&self) -> builder::AntiAffinityGroupMemberList<'_> {
        builder::AntiAffinityGroupMemberList::new(self)
    }

    fn anti_affinity_group_member_instance_view(
        &self,
    ) -> builder::AntiAffinityGroupMemberInstanceView<'_> {
        builder::AntiAffinityGroupMemberInstanceView::new(self)
    }

    fn anti_affinity_group_member_instance_add(
        &self,
    ) -> builder::AntiAffinityGroupMemberInstanceAdd<'_> {
        builder::AntiAffinityGroupMemberInstanceAdd::new(self)
    }

    fn anti_affinity_group_member_instance_delete(
        &self,
    ) -> builder::AntiAffinityGroupMemberInstanceDelete<'_> {
        builder::AntiAffinityGroupMemberInstanceDelete::new(self)
    }
}

/// API for console authentication
pub trait ClientConsoleAuthExt {
    /// Start an OAuth 2.0 Device Authorization Grant
    ///
    /// This endpoint is designed to be accessed from an *unauthenticated* API
    /// client. It generates and records a `device_code` and `user_code` which
    /// must be verified and confirmed prior to a token being granted.
    ///
    /// Sends a `POST` request to `/device/auth`
    ///
    /// ```ignore
    /// let response = client.device_auth_request()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn device_auth_request(&self) -> builder::DeviceAuthRequest<'_>;
    /// Confirm an OAuth 2.0 Device Authorization Grant
    ///
    /// This endpoint is designed to be accessed by the user agent (browser),
    /// not the client requesting the token. So we do not actually return the
    /// token here; it will be returned in response to the poll on
    /// `/device/token`.
    ///
    /// Some special logic applies when authenticating this request with an
    /// existing device token instead of a console session: the requested TTL
    /// must not produce an expiration time later than the authenticating
    /// token's expiration. If no TTL was specified in the initial grant
    /// request, the expiration will be the lesser of the silo max and the
    /// authenticating token's expiration time. To get the longest allowed
    /// lifetime, omit the TTL and authenticate with a web console session.
    ///
    /// Sends a `POST` request to `/device/confirm`
    ///
    /// ```ignore
    /// let response = client.device_auth_confirm()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn device_auth_confirm(&self) -> builder::DeviceAuthConfirm<'_>;
    /// Request a device access token
    ///
    /// This endpoint should be polled by the client until the user code is
    /// verified and the grant is confirmed.
    ///
    /// Sends a `POST` request to `/device/token`
    ///
    /// ```ignore
    /// let response = client.device_access_token()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn device_access_token(&self) -> builder::DeviceAccessToken<'_>;
    /// Log user out of web console by deleting session on client and server
    ///
    /// Sends a `POST` request to `/v1/logout`
    ///
    /// ```ignore
    /// let response = client.logout()
    ///    .send()
    ///    .await;
    /// ```
    fn logout(&self) -> builder::Logout<'_>;
}

impl ClientConsoleAuthExt for Client {
    fn device_auth_request(&self) -> builder::DeviceAuthRequest<'_> {
        builder::DeviceAuthRequest::new(self)
    }

    fn device_auth_confirm(&self) -> builder::DeviceAuthConfirm<'_> {
        builder::DeviceAuthConfirm::new(self)
    }

    fn device_access_token(&self) -> builder::DeviceAccessToken<'_> {
        builder::DeviceAccessToken::new(self)
    }

    fn logout(&self) -> builder::Logout<'_> {
        builder::Logout::new(self)
    }
}

/// Information pertaining to the current user.
pub trait ClientCurrentUserExt {
    /// Fetch user for current session
    ///
    /// Sends a `GET` request to `/v1/me`
    ///
    /// ```ignore
    /// let response = client.current_user_view()
    ///    .send()
    ///    .await;
    /// ```
    fn current_user_view(&self) -> builder::CurrentUserView<'_>;
    /// Fetch current user's groups
    ///
    /// Sends a `GET` request to `/v1/me/groups`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.current_user_groups()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn current_user_groups(&self) -> builder::CurrentUserGroups<'_>;
    /// List SSH public keys
    ///
    /// Lists SSH public keys for the currently authenticated user.
    ///
    /// Sends a `GET` request to `/v1/me/ssh-keys`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.current_user_ssh_key_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn current_user_ssh_key_list(&self) -> builder::CurrentUserSshKeyList<'_>;
    /// Create SSH public key
    ///
    /// Create an SSH public key for the currently authenticated user.
    ///
    /// Sends a `POST` request to `/v1/me/ssh-keys`
    ///
    /// ```ignore
    /// let response = client.current_user_ssh_key_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn current_user_ssh_key_create(&self) -> builder::CurrentUserSshKeyCreate<'_>;
    /// Fetch SSH public key
    ///
    /// Fetch SSH public key associated with the currently authenticated user.
    ///
    /// Sends a `GET` request to `/v1/me/ssh-keys/{ssh_key}`
    ///
    /// Arguments:
    /// - `ssh_key`: Name or ID of the SSH key
    /// ```ignore
    /// let response = client.current_user_ssh_key_view()
    ///    .ssh_key(ssh_key)
    ///    .send()
    ///    .await;
    /// ```
    fn current_user_ssh_key_view(&self) -> builder::CurrentUserSshKeyView<'_>;
    /// Delete SSH public key
    ///
    /// Delete an SSH public key associated with the currently authenticated
    /// user.
    ///
    /// Sends a `DELETE` request to `/v1/me/ssh-keys/{ssh_key}`
    ///
    /// Arguments:
    /// - `ssh_key`: Name or ID of the SSH key
    /// ```ignore
    /// let response = client.current_user_ssh_key_delete()
    ///    .ssh_key(ssh_key)
    ///    .send()
    ///    .await;
    /// ```
    fn current_user_ssh_key_delete(&self) -> builder::CurrentUserSshKeyDelete<'_>;
}

impl ClientCurrentUserExt for Client {
    fn current_user_view(&self) -> builder::CurrentUserView<'_> {
        builder::CurrentUserView::new(self)
    }

    fn current_user_groups(&self) -> builder::CurrentUserGroups<'_> {
        builder::CurrentUserGroups::new(self)
    }

    fn current_user_ssh_key_list(&self) -> builder::CurrentUserSshKeyList<'_> {
        builder::CurrentUserSshKeyList::new(self)
    }

    fn current_user_ssh_key_create(&self) -> builder::CurrentUserSshKeyCreate<'_> {
        builder::CurrentUserSshKeyCreate::new(self)
    }

    fn current_user_ssh_key_view(&self) -> builder::CurrentUserSshKeyView<'_> {
        builder::CurrentUserSshKeyView::new(self)
    }

    fn current_user_ssh_key_delete(&self) -> builder::CurrentUserSshKeyDelete<'_> {
        builder::CurrentUserSshKeyDelete::new(self)
    }
}

/// Virtual disks are used to store instance-local data which includes the
/// operating system.
pub trait ClientDisksExt {
    /// List disks
    ///
    /// Sends a `GET` request to `/v1/disks`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.disk_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn disk_list(&self) -> builder::DiskList<'_>;
    /// Create disk
    ///
    /// Sends a `POST` request to `/v1/disks`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.disk_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn disk_create(&self) -> builder::DiskCreate<'_>;
    /// Fetch disk
    ///
    /// Sends a `GET` request to `/v1/disks/{disk}`
    ///
    /// Arguments:
    /// - `disk`: Name or ID of the disk
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.disk_view()
    ///    .disk(disk)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn disk_view(&self) -> builder::DiskView<'_>;
    /// Delete disk
    ///
    /// Sends a `DELETE` request to `/v1/disks/{disk}`
    ///
    /// Arguments:
    /// - `disk`: Name or ID of the disk
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.disk_delete()
    ///    .disk(disk)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn disk_delete(&self) -> builder::DiskDelete<'_>;
    /// Import blocks into disk
    ///
    /// Sends a `POST` request to `/v1/disks/{disk}/bulk-write`
    ///
    /// Arguments:
    /// - `disk`: Name or ID of the disk
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.disk_bulk_write_import()
    ///    .disk(disk)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn disk_bulk_write_import(&self) -> builder::DiskBulkWriteImport<'_>;
    /// Start importing blocks into disk
    ///
    /// Start the process of importing blocks into a disk
    ///
    /// Sends a `POST` request to `/v1/disks/{disk}/bulk-write-start`
    ///
    /// Arguments:
    /// - `disk`: Name or ID of the disk
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.disk_bulk_write_import_start()
    ///    .disk(disk)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn disk_bulk_write_import_start(&self) -> builder::DiskBulkWriteImportStart<'_>;
    /// Stop importing blocks into disk
    ///
    /// Stop the process of importing blocks into a disk
    ///
    /// Sends a `POST` request to `/v1/disks/{disk}/bulk-write-stop`
    ///
    /// Arguments:
    /// - `disk`: Name or ID of the disk
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.disk_bulk_write_import_stop()
    ///    .disk(disk)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn disk_bulk_write_import_stop(&self) -> builder::DiskBulkWriteImportStop<'_>;
    /// Confirm disk block import completion
    ///
    /// Sends a `POST` request to `/v1/disks/{disk}/finalize`
    ///
    /// Arguments:
    /// - `disk`: Name or ID of the disk
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.disk_finalize_import()
    ///    .disk(disk)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn disk_finalize_import(&self) -> builder::DiskFinalizeImport<'_>;
}

impl ClientDisksExt for Client {
    fn disk_list(&self) -> builder::DiskList<'_> {
        builder::DiskList::new(self)
    }

    fn disk_create(&self) -> builder::DiskCreate<'_> {
        builder::DiskCreate::new(self)
    }

    fn disk_view(&self) -> builder::DiskView<'_> {
        builder::DiskView::new(self)
    }

    fn disk_delete(&self) -> builder::DiskDelete<'_> {
        builder::DiskDelete::new(self)
    }

    fn disk_bulk_write_import(&self) -> builder::DiskBulkWriteImport<'_> {
        builder::DiskBulkWriteImport::new(self)
    }

    fn disk_bulk_write_import_start(&self) -> builder::DiskBulkWriteImportStart<'_> {
        builder::DiskBulkWriteImportStart::new(self)
    }

    fn disk_bulk_write_import_stop(&self) -> builder::DiskBulkWriteImportStop<'_> {
        builder::DiskBulkWriteImportStop::new(self)
    }

    fn disk_finalize_import(&self) -> builder::DiskFinalizeImport<'_> {
        builder::DiskFinalizeImport::new(self)
    }
}

/// Experimental, unstable interfaces, primarily for use by Oxide personnel
pub trait ClientExperimentalExt {
    /// List instrumentation probes
    ///
    /// Sends a `GET` request to `/experimental/v1/probes`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.probe_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn probe_list(&self) -> builder::ProbeList<'_>;
    /// Create instrumentation probe
    ///
    /// Sends a `POST` request to `/experimental/v1/probes`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.probe_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn probe_create(&self) -> builder::ProbeCreate<'_>;
    /// View instrumentation probe
    ///
    /// Sends a `GET` request to `/experimental/v1/probes/{probe}`
    ///
    /// Arguments:
    /// - `probe`: Name or ID of the probe
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.probe_view()
    ///    .probe(probe)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn probe_view(&self) -> builder::ProbeView<'_>;
    /// Delete instrumentation probe
    ///
    /// Sends a `DELETE` request to `/experimental/v1/probes/{probe}`
    ///
    /// Arguments:
    /// - `probe`: Name or ID of the probe
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.probe_delete()
    ///    .probe(probe)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn probe_delete(&self) -> builder::ProbeDelete<'_>;
    /// List affinity groups
    ///
    /// Sends a `GET` request to `/v1/affinity-groups`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.affinity_group_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn affinity_group_list(&self) -> builder::AffinityGroupList<'_>;
    /// Create affinity group
    ///
    /// Sends a `POST` request to `/v1/affinity-groups`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.affinity_group_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn affinity_group_create(&self) -> builder::AffinityGroupCreate<'_>;
    /// Fetch affinity group
    ///
    /// Sends a `GET` request to `/v1/affinity-groups/{affinity_group}`
    ///
    /// Arguments:
    /// - `affinity_group`: Name or ID of the affinity group
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.affinity_group_view()
    ///    .affinity_group(affinity_group)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn affinity_group_view(&self) -> builder::AffinityGroupView<'_>;
    /// Update affinity group
    ///
    /// Sends a `PUT` request to `/v1/affinity-groups/{affinity_group}`
    ///
    /// Arguments:
    /// - `affinity_group`: Name or ID of the affinity group
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.affinity_group_update()
    ///    .affinity_group(affinity_group)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn affinity_group_update(&self) -> builder::AffinityGroupUpdate<'_>;
    /// Delete affinity group
    ///
    /// Sends a `DELETE` request to `/v1/affinity-groups/{affinity_group}`
    ///
    /// Arguments:
    /// - `affinity_group`: Name or ID of the affinity group
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.affinity_group_delete()
    ///    .affinity_group(affinity_group)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn affinity_group_delete(&self) -> builder::AffinityGroupDelete<'_>;
    /// List affinity group members
    ///
    /// Sends a `GET` request to `/v1/affinity-groups/{affinity_group}/members`
    ///
    /// Arguments:
    /// - `affinity_group`: Name or ID of the affinity group
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.affinity_group_member_list()
    ///    .affinity_group(affinity_group)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn affinity_group_member_list(&self) -> builder::AffinityGroupMemberList<'_>;
    /// Fetch affinity group member
    ///
    /// Sends a `GET` request to
    /// `/v1/affinity-groups/{affinity_group}/members/instance/{instance}`
    ///
    /// Arguments:
    /// - `affinity_group`
    /// - `instance`
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.affinity_group_member_instance_view()
    ///    .affinity_group(affinity_group)
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn affinity_group_member_instance_view(&self) -> builder::AffinityGroupMemberInstanceView<'_>;
    /// Add member to affinity group
    ///
    /// Sends a `POST` request to
    /// `/v1/affinity-groups/{affinity_group}/members/instance/{instance}`
    ///
    /// Arguments:
    /// - `affinity_group`
    /// - `instance`
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.affinity_group_member_instance_add()
    ///    .affinity_group(affinity_group)
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn affinity_group_member_instance_add(&self) -> builder::AffinityGroupMemberInstanceAdd<'_>;
    /// Remove member from affinity group
    ///
    /// Sends a `DELETE` request to
    /// `/v1/affinity-groups/{affinity_group}/members/instance/{instance}`
    ///
    /// Arguments:
    /// - `affinity_group`
    /// - `instance`
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.affinity_group_member_instance_delete()
    ///    .affinity_group(affinity_group)
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn affinity_group_member_instance_delete(
        &self,
    ) -> builder::AffinityGroupMemberInstanceDelete<'_>;
    /// List affinity groups containing instance
    ///
    /// Sends a `GET` request to `/v1/instances/{instance}/affinity-groups`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.instance_affinity_group_list()
    ///    .instance(instance)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_affinity_group_list(&self) -> builder::InstanceAffinityGroupList<'_>;
    /// List multicast groups for an instance
    ///
    /// Sends a `GET` request to `/v1/instances/{instance}/multicast-groups`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.instance_multicast_group_list()
    ///    .instance(instance)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_multicast_group_list(&self) -> builder::InstanceMulticastGroupList<'_>;
    /// Join multicast group by name, IP address, or UUID
    ///
    /// Groups can be referenced by name, IP address, or UUID. If the group
    /// doesn't exist, it's implicitly created with an auto-allocated IP from a
    /// multicast pool linked to the caller's silo. When referencing by UUID,
    /// the group must already exist.
    ///
    /// Source IPs are optional for ASM addresses but required for SSM addresses
    /// (232.0.0.0/8 for IPv4, ff3x::/32 for IPv6). Duplicate IPs in the request
    /// are automatically deduplicated, with a maximum of 64 source IPs allowed.
    ///
    /// Sends a `PUT` request to
    /// `/v1/instances/{instance}/multicast-groups/{multicast_group}`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `multicast_group`: Name, ID, or IP address of the multicast group
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.instance_multicast_group_join()
    ///    .instance(instance)
    ///    .multicast_group(multicast_group)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_multicast_group_join(&self) -> builder::InstanceMulticastGroupJoin<'_>;
    /// Leave multicast group by name, IP address, or UUID
    ///
    /// Sends a `DELETE` request to
    /// `/v1/instances/{instance}/multicast-groups/{multicast_group}`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `multicast_group`: Name, ID, or IP address of the multicast group
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.instance_multicast_group_leave()
    ///    .instance(instance)
    ///    .multicast_group(multicast_group)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_multicast_group_leave(&self) -> builder::InstanceMulticastGroupLeave<'_>;
    /// List multicast groups
    ///
    /// Sends a `GET` request to `/v1/multicast-groups`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.multicast_group_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn multicast_group_list(&self) -> builder::MulticastGroupList<'_>;
    /// Fetch multicast group
    ///
    /// The group can be specified by name, UUID, or multicast IP address.
    /// (e.g., "224.1.2.3" or "ff38::1").
    ///
    /// Sends a `GET` request to `/v1/multicast-groups/{multicast_group}`
    ///
    /// Arguments:
    /// - `multicast_group`: Name, ID, or IP address of the multicast group
    /// ```ignore
    /// let response = client.multicast_group_view()
    ///    .multicast_group(multicast_group)
    ///    .send()
    ///    .await;
    /// ```
    fn multicast_group_view(&self) -> builder::MulticastGroupView<'_>;
    /// List members of multicast group
    ///
    /// The group can be specified by name, UUID, or multicast IP address.
    ///
    /// Sends a `GET` request to
    /// `/v1/multicast-groups/{multicast_group}/members`
    ///
    /// Arguments:
    /// - `multicast_group`: Name, ID, or IP address of the multicast group
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.multicast_group_member_list()
    ///    .multicast_group(multicast_group)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn multicast_group_member_list(&self) -> builder::MulticastGroupMemberList<'_>;
    /// Fetch rack cluster membership status
    ///
    /// Returns the status for the most recent change, or a specific version if
    /// one is specified.
    ///
    /// Sends a `GET` request to
    /// `/v1/system/hardware/racks/{rack_id}/membership`
    ///
    /// ```ignore
    /// let response = client.rack_membership_status()
    ///    .rack_id(rack_id)
    ///    .version(version)
    ///    .send()
    ///    .await;
    /// ```
    fn rack_membership_status(&self) -> builder::RackMembershipStatus<'_>;
    /// Abort the latest rack membership change
    ///
    /// This operation is synchronous. Upon returning from the API call, a
    /// success response indicates that the prior membership change was aborted.
    /// An error response indicates that there is no active membership change in
    /// progress (previous changes have completed) or that the current
    /// membership change could not be aborted.
    ///
    /// Sends a `POST` request to
    /// `/v1/system/hardware/racks/{rack_id}/membership/abort`
    ///
    /// Arguments:
    /// - `rack_id`: ID of the rack
    /// ```ignore
    /// let response = client.rack_membership_abort()
    ///    .rack_id(rack_id)
    ///    .send()
    ///    .await;
    /// ```
    fn rack_membership_abort(&self) -> builder::RackMembershipAbort<'_>;
    /// Add new sleds to rack membership
    ///
    /// Sends a `POST` request to
    /// `/v1/system/hardware/racks/{rack_id}/membership/add`
    ///
    /// Arguments:
    /// - `rack_id`: ID of the rack
    /// - `body`
    /// ```ignore
    /// let response = client.rack_membership_add_sleds()
    ///    .rack_id(rack_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn rack_membership_add_sleds(&self) -> builder::RackMembershipAddSleds<'_>;
    /// Run project-scoped timeseries query
    ///
    /// Queries are written in OxQL. Project must be specified by name or ID in
    /// URL query parameter. The OxQL query will only return timeseries data
    /// from the specified project.
    ///
    /// Sends a `POST` request to `/v1/timeseries/query`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.timeseries_query()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn timeseries_query(&self) -> builder::TimeseriesQuery<'_>;
}

impl ClientExperimentalExt for Client {
    fn probe_list(&self) -> builder::ProbeList<'_> {
        builder::ProbeList::new(self)
    }

    fn probe_create(&self) -> builder::ProbeCreate<'_> {
        builder::ProbeCreate::new(self)
    }

    fn probe_view(&self) -> builder::ProbeView<'_> {
        builder::ProbeView::new(self)
    }

    fn probe_delete(&self) -> builder::ProbeDelete<'_> {
        builder::ProbeDelete::new(self)
    }

    fn affinity_group_list(&self) -> builder::AffinityGroupList<'_> {
        builder::AffinityGroupList::new(self)
    }

    fn affinity_group_create(&self) -> builder::AffinityGroupCreate<'_> {
        builder::AffinityGroupCreate::new(self)
    }

    fn affinity_group_view(&self) -> builder::AffinityGroupView<'_> {
        builder::AffinityGroupView::new(self)
    }

    fn affinity_group_update(&self) -> builder::AffinityGroupUpdate<'_> {
        builder::AffinityGroupUpdate::new(self)
    }

    fn affinity_group_delete(&self) -> builder::AffinityGroupDelete<'_> {
        builder::AffinityGroupDelete::new(self)
    }

    fn affinity_group_member_list(&self) -> builder::AffinityGroupMemberList<'_> {
        builder::AffinityGroupMemberList::new(self)
    }

    fn affinity_group_member_instance_view(&self) -> builder::AffinityGroupMemberInstanceView<'_> {
        builder::AffinityGroupMemberInstanceView::new(self)
    }

    fn affinity_group_member_instance_add(&self) -> builder::AffinityGroupMemberInstanceAdd<'_> {
        builder::AffinityGroupMemberInstanceAdd::new(self)
    }

    fn affinity_group_member_instance_delete(
        &self,
    ) -> builder::AffinityGroupMemberInstanceDelete<'_> {
        builder::AffinityGroupMemberInstanceDelete::new(self)
    }

    fn instance_affinity_group_list(&self) -> builder::InstanceAffinityGroupList<'_> {
        builder::InstanceAffinityGroupList::new(self)
    }

    fn instance_multicast_group_list(&self) -> builder::InstanceMulticastGroupList<'_> {
        builder::InstanceMulticastGroupList::new(self)
    }

    fn instance_multicast_group_join(&self) -> builder::InstanceMulticastGroupJoin<'_> {
        builder::InstanceMulticastGroupJoin::new(self)
    }

    fn instance_multicast_group_leave(&self) -> builder::InstanceMulticastGroupLeave<'_> {
        builder::InstanceMulticastGroupLeave::new(self)
    }

    fn multicast_group_list(&self) -> builder::MulticastGroupList<'_> {
        builder::MulticastGroupList::new(self)
    }

    fn multicast_group_view(&self) -> builder::MulticastGroupView<'_> {
        builder::MulticastGroupView::new(self)
    }

    fn multicast_group_member_list(&self) -> builder::MulticastGroupMemberList<'_> {
        builder::MulticastGroupMemberList::new(self)
    }

    fn rack_membership_status(&self) -> builder::RackMembershipStatus<'_> {
        builder::RackMembershipStatus::new(self)
    }

    fn rack_membership_abort(&self) -> builder::RackMembershipAbort<'_> {
        builder::RackMembershipAbort::new(self)
    }

    fn rack_membership_add_sleds(&self) -> builder::RackMembershipAddSleds<'_> {
        builder::RackMembershipAddSleds::new(self)
    }

    fn timeseries_query(&self) -> builder::TimeseriesQuery<'_> {
        builder::TimeseriesQuery::new(self)
    }
}

/// External subnets that can be attached to instances.
pub trait ClientExternalSubnetsExt {
    /// List external subnets
    ///
    /// Sends a `GET` request to `/v1/external-subnets`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.external_subnet_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn external_subnet_list(&self) -> builder::ExternalSubnetList<'_>;
    /// Create external subnet
    ///
    /// Sends a `POST` request to `/v1/external-subnets`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.external_subnet_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn external_subnet_create(&self) -> builder::ExternalSubnetCreate<'_>;
    /// Fetch external subnet
    ///
    /// Sends a `GET` request to `/v1/external-subnets/{external_subnet}`
    ///
    /// Arguments:
    /// - `external_subnet`: Name or ID of the external subnet
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.external_subnet_view()
    ///    .external_subnet(external_subnet)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn external_subnet_view(&self) -> builder::ExternalSubnetView<'_>;
    /// Update external subnet
    ///
    /// Sends a `PUT` request to `/v1/external-subnets/{external_subnet}`
    ///
    /// Arguments:
    /// - `external_subnet`: Name or ID of the external subnet
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.external_subnet_update()
    ///    .external_subnet(external_subnet)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn external_subnet_update(&self) -> builder::ExternalSubnetUpdate<'_>;
    /// Delete external subnet
    ///
    /// Sends a `DELETE` request to `/v1/external-subnets/{external_subnet}`
    ///
    /// Arguments:
    /// - `external_subnet`: Name or ID of the external subnet
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.external_subnet_delete()
    ///    .external_subnet(external_subnet)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn external_subnet_delete(&self) -> builder::ExternalSubnetDelete<'_>;
    /// Attach external subnet to instance
    ///
    /// Sends a `POST` request to
    /// `/v1/external-subnets/{external_subnet}/attach`
    ///
    /// Arguments:
    /// - `external_subnet`: Name or ID of the external subnet
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.external_subnet_attach()
    ///    .external_subnet(external_subnet)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn external_subnet_attach(&self) -> builder::ExternalSubnetAttach<'_>;
    /// Detach external subnet from instance
    ///
    /// Sends a `POST` request to
    /// `/v1/external-subnets/{external_subnet}/detach`
    ///
    /// Arguments:
    /// - `external_subnet`: Name or ID of the external subnet
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.external_subnet_detach()
    ///    .external_subnet(external_subnet)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn external_subnet_detach(&self) -> builder::ExternalSubnetDetach<'_>;
}

impl ClientExternalSubnetsExt for Client {
    fn external_subnet_list(&self) -> builder::ExternalSubnetList<'_> {
        builder::ExternalSubnetList::new(self)
    }

    fn external_subnet_create(&self) -> builder::ExternalSubnetCreate<'_> {
        builder::ExternalSubnetCreate::new(self)
    }

    fn external_subnet_view(&self) -> builder::ExternalSubnetView<'_> {
        builder::ExternalSubnetView::new(self)
    }

    fn external_subnet_update(&self) -> builder::ExternalSubnetUpdate<'_> {
        builder::ExternalSubnetUpdate::new(self)
    }

    fn external_subnet_delete(&self) -> builder::ExternalSubnetDelete<'_> {
        builder::ExternalSubnetDelete::new(self)
    }

    fn external_subnet_attach(&self) -> builder::ExternalSubnetAttach<'_> {
        builder::ExternalSubnetAttach::new(self)
    }

    fn external_subnet_detach(&self) -> builder::ExternalSubnetDetach<'_> {
        builder::ExternalSubnetDetach::new(self)
    }
}

/// Floating IPs allow a project to allocate well-known IPs to instances.
pub trait ClientFloatingIpsExt {
    /// List floating IPs
    ///
    /// Sends a `GET` request to `/v1/floating-ips`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.floating_ip_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn floating_ip_list(&self) -> builder::FloatingIpList<'_>;
    /// Create floating IP
    ///
    /// A specific IP address can be reserved, or an IP can be auto-allocated
    /// from a specific pool or the silo's default pool.
    ///
    /// Sends a `POST` request to `/v1/floating-ips`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.floating_ip_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn floating_ip_create(&self) -> builder::FloatingIpCreate<'_>;
    /// Fetch floating IP
    ///
    /// Sends a `GET` request to `/v1/floating-ips/{floating_ip}`
    ///
    /// Arguments:
    /// - `floating_ip`: Name or ID of the floating IP
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.floating_ip_view()
    ///    .floating_ip(floating_ip)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn floating_ip_view(&self) -> builder::FloatingIpView<'_>;
    /// Update floating IP
    ///
    /// Sends a `PUT` request to `/v1/floating-ips/{floating_ip}`
    ///
    /// Arguments:
    /// - `floating_ip`: Name or ID of the floating IP
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.floating_ip_update()
    ///    .floating_ip(floating_ip)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn floating_ip_update(&self) -> builder::FloatingIpUpdate<'_>;
    /// Delete floating IP
    ///
    /// Sends a `DELETE` request to `/v1/floating-ips/{floating_ip}`
    ///
    /// Arguments:
    /// - `floating_ip`: Name or ID of the floating IP
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.floating_ip_delete()
    ///    .floating_ip(floating_ip)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn floating_ip_delete(&self) -> builder::FloatingIpDelete<'_>;
    /// Attach floating IP
    ///
    /// Attach floating IP to an instance or other resource.
    ///
    /// Sends a `POST` request to `/v1/floating-ips/{floating_ip}/attach`
    ///
    /// Arguments:
    /// - `floating_ip`: Name or ID of the floating IP
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.floating_ip_attach()
    ///    .floating_ip(floating_ip)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn floating_ip_attach(&self) -> builder::FloatingIpAttach<'_>;
    /// Detach floating IP
    ///
    /// Sends a `POST` request to `/v1/floating-ips/{floating_ip}/detach`
    ///
    /// Arguments:
    /// - `floating_ip`: Name or ID of the floating IP
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.floating_ip_detach()
    ///    .floating_ip(floating_ip)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn floating_ip_detach(&self) -> builder::FloatingIpDetach<'_>;
}

impl ClientFloatingIpsExt for Client {
    fn floating_ip_list(&self) -> builder::FloatingIpList<'_> {
        builder::FloatingIpList::new(self)
    }

    fn floating_ip_create(&self) -> builder::FloatingIpCreate<'_> {
        builder::FloatingIpCreate::new(self)
    }

    fn floating_ip_view(&self) -> builder::FloatingIpView<'_> {
        builder::FloatingIpView::new(self)
    }

    fn floating_ip_update(&self) -> builder::FloatingIpUpdate<'_> {
        builder::FloatingIpUpdate::new(self)
    }

    fn floating_ip_delete(&self) -> builder::FloatingIpDelete<'_> {
        builder::FloatingIpDelete::new(self)
    }

    fn floating_ip_attach(&self) -> builder::FloatingIpAttach<'_> {
        builder::FloatingIpAttach::new(self)
    }

    fn floating_ip_detach(&self) -> builder::FloatingIpDetach<'_> {
        builder::FloatingIpDetach::new(self)
    }
}

/// Images are read-only virtual disks that may be used to boot virtual
/// machines.
pub trait ClientImagesExt {
    /// List images
    ///
    /// List images which are global or scoped to the specified project. The
    /// images are returned sorted by creation date, with the most recent images
    /// appearing first.
    ///
    /// Sends a `GET` request to `/v1/images`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.image_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn image_list(&self) -> builder::ImageList<'_>;
    /// Create image
    ///
    /// Create a new image in a project.
    ///
    /// Sends a `POST` request to `/v1/images`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.image_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn image_create(&self) -> builder::ImageCreate<'_>;
    /// Fetch image
    ///
    /// Fetch the details for a specific image in a project.
    ///
    /// Sends a `GET` request to `/v1/images/{image}`
    ///
    /// Arguments:
    /// - `image`: Name or ID of the image
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.image_view()
    ///    .image(image)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn image_view(&self) -> builder::ImageView<'_>;
    /// Delete image
    ///
    /// Permanently delete an image from a project. This operation cannot be
    /// undone. Any instances in the project using the image will continue to
    /// run, however new instances can not be created with this image.
    ///
    /// Sends a `DELETE` request to `/v1/images/{image}`
    ///
    /// Arguments:
    /// - `image`: Name or ID of the image
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.image_delete()
    ///    .image(image)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn image_delete(&self) -> builder::ImageDelete<'_>;
    /// Demote silo image
    ///
    /// Demote silo image to be visible only to a specified project
    ///
    /// Sends a `POST` request to `/v1/images/{image}/demote`
    ///
    /// Arguments:
    /// - `image`: Name or ID of the image
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.image_demote()
    ///    .image(image)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn image_demote(&self) -> builder::ImageDemote<'_>;
    /// Promote project image
    ///
    /// Promote project image to be visible to all projects in the silo
    ///
    /// Sends a `POST` request to `/v1/images/{image}/promote`
    ///
    /// Arguments:
    /// - `image`: Name or ID of the image
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.image_promote()
    ///    .image(image)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn image_promote(&self) -> builder::ImagePromote<'_>;
}

impl ClientImagesExt for Client {
    fn image_list(&self) -> builder::ImageList<'_> {
        builder::ImageList::new(self)
    }

    fn image_create(&self) -> builder::ImageCreate<'_> {
        builder::ImageCreate::new(self)
    }

    fn image_view(&self) -> builder::ImageView<'_> {
        builder::ImageView::new(self)
    }

    fn image_delete(&self) -> builder::ImageDelete<'_> {
        builder::ImageDelete::new(self)
    }

    fn image_demote(&self) -> builder::ImageDemote<'_> {
        builder::ImageDemote::new(self)
    }

    fn image_promote(&self) -> builder::ImagePromote<'_> {
        builder::ImagePromote::new(self)
    }
}

/// Virtual machine instances are the basic unit of computation. These
/// operations are used for provisioning, controlling, and destroying instances.
pub trait ClientInstancesExt {
    /// List instances
    ///
    /// Sends a `GET` request to `/v1/instances`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.instance_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_list(&self) -> builder::InstanceList<'_>;
    /// Create instance
    ///
    /// Sends a `POST` request to `/v1/instances`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.instance_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_create(&self) -> builder::InstanceCreate<'_>;
    /// Fetch instance
    ///
    /// Sends a `GET` request to `/v1/instances/{instance}`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.instance_view()
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_view(&self) -> builder::InstanceView<'_>;
    /// Update instance
    ///
    /// Sends a `PUT` request to `/v1/instances/{instance}`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.instance_update()
    ///    .instance(instance)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_update(&self) -> builder::InstanceUpdate<'_>;
    /// Delete instance
    ///
    /// Sends a `DELETE` request to `/v1/instances/{instance}`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.instance_delete()
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_delete(&self) -> builder::InstanceDelete<'_>;
    /// List anti-affinity groups containing instance
    ///
    /// Sends a `GET` request to `/v1/instances/{instance}/anti-affinity-groups`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.instance_anti_affinity_group_list()
    ///    .instance(instance)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_anti_affinity_group_list(&self) -> builder::InstanceAntiAffinityGroupList<'_>;
    /// List disks for instance
    ///
    /// Sends a `GET` request to `/v1/instances/{instance}/disks`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.instance_disk_list()
    ///    .instance(instance)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_disk_list(&self) -> builder::InstanceDiskList<'_>;
    /// Attach disk to instance
    ///
    /// Sends a `POST` request to `/v1/instances/{instance}/disks/attach`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.instance_disk_attach()
    ///    .instance(instance)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_disk_attach(&self) -> builder::InstanceDiskAttach<'_>;
    /// Detach disk from instance
    ///
    /// Sends a `POST` request to `/v1/instances/{instance}/disks/detach`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.instance_disk_detach()
    ///    .instance(instance)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_disk_detach(&self) -> builder::InstanceDiskDetach<'_>;
    /// List external IP addresses
    ///
    /// Sends a `GET` request to `/v1/instances/{instance}/external-ips`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.instance_external_ip_list()
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_external_ip_list(&self) -> builder::InstanceExternalIpList<'_>;
    /// Allocate and attach ephemeral IP to instance
    ///
    /// Sends a `POST` request to
    /// `/v1/instances/{instance}/external-ips/ephemeral`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.instance_ephemeral_ip_attach()
    ///    .instance(instance)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_ephemeral_ip_attach(&self) -> builder::InstanceEphemeralIpAttach<'_>;
    /// Detach and deallocate ephemeral IP from instance
    ///
    /// When an instance has both IPv4 and IPv6 ephemeral IPs, the `ip_version`
    /// query parameter must be specified to identify which IP to detach.
    ///
    /// Sends a `DELETE` request to
    /// `/v1/instances/{instance}/external-ips/ephemeral`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `ip_version`: The IP version of the ephemeral IP to detach.
    ///
    /// Required when the instance has both IPv4 and IPv6 ephemeral IPs. If only
    /// one ephemeral IP is attached, this field may be omitted.
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.instance_ephemeral_ip_detach()
    ///    .instance(instance)
    ///    .ip_version(ip_version)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_ephemeral_ip_detach(&self) -> builder::InstanceEphemeralIpDetach<'_>;
    /// List external subnets attached to instance
    ///
    /// Sends a `GET` request to `/v1/instances/{instance}/external-subnets`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.instance_external_subnet_list()
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_external_subnet_list(&self) -> builder::InstanceExternalSubnetList<'_>;
    /// Reboot instance
    ///
    /// Sends a `POST` request to `/v1/instances/{instance}/reboot`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.instance_reboot()
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_reboot(&self) -> builder::InstanceReboot<'_>;
    /// Fetch instance serial console
    ///
    /// Sends a `GET` request to `/v1/instances/{instance}/serial-console`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `from_start`: Character index in the serial buffer from which to read,
    ///   counting the bytes output since instance start. If this is not
    ///   provided, `most_recent` must be provided, and if this *is* provided,
    ///   `most_recent` must *not* be provided.
    /// - `max_bytes`: Maximum number of bytes of buffered serial console
    ///   contents to return. If the requested range runs to the end of the
    ///   available buffer, the data returned will be shorter than `max_bytes`.
    /// - `most_recent`: Character index in the serial buffer from which to
    ///   read, counting *backward* from the most recently buffered data
    ///   retrieved from the instance. (See note on `from_start` about mutual
    ///   exclusivity)
    /// - `project`: Name or ID of the project, only required if `instance` is
    ///   provided as a `Name`
    /// ```ignore
    /// let response = client.instance_serial_console()
    ///    .instance(instance)
    ///    .from_start(from_start)
    ///    .max_bytes(max_bytes)
    ///    .most_recent(most_recent)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_serial_console(&self) -> builder::InstanceSerialConsole<'_>;
    /// Stream instance serial console
    ///
    /// Sends a `GET` request to
    /// `/v1/instances/{instance}/serial-console/stream`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `most_recent`: Character index in the serial buffer from which to
    ///   read, counting *backward* from the most recently buffered data
    ///   retrieved from the instance.
    /// - `project`: Name or ID of the project, only required if `instance` is
    ///   provided as a `Name`
    /// ```ignore
    /// let response = client.instance_serial_console_stream()
    ///    .instance(instance)
    ///    .most_recent(most_recent)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_serial_console_stream(&self) -> builder::InstanceSerialConsoleStream<'_>;
    /// List SSH public keys for instance
    ///
    /// List SSH public keys injected via cloud-init during instance creation.
    /// Note that this list is a snapshot in time and will not reflect updates
    /// made after the instance is created.
    ///
    /// Sends a `GET` request to `/v1/instances/{instance}/ssh-public-keys`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.instance_ssh_public_key_list()
    ///    .instance(instance)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_ssh_public_key_list(&self) -> builder::InstanceSshPublicKeyList<'_>;
    /// Boot instance
    ///
    /// Sends a `POST` request to `/v1/instances/{instance}/start`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.instance_start()
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_start(&self) -> builder::InstanceStart<'_>;
    /// Stop instance
    ///
    /// Sends a `POST` request to `/v1/instances/{instance}/stop`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.instance_stop()
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_stop(&self) -> builder::InstanceStop<'_>;
    /// List network interfaces
    ///
    /// Sends a `GET` request to `/v1/network-interfaces`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project, only required if `instance` is
    ///   provided as a `Name`
    /// - `sort_by`
    /// ```ignore
    /// let response = client.instance_network_interface_list()
    ///    .instance(instance)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_network_interface_list(&self) -> builder::InstanceNetworkInterfaceList<'_>;
    /// Create network interface
    ///
    /// Sends a `POST` request to `/v1/network-interfaces`
    ///
    /// Arguments:
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project, only required if `instance` is
    ///   provided as a `Name`
    /// - `body`
    /// ```ignore
    /// let response = client.instance_network_interface_create()
    ///    .instance(instance)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_network_interface_create(&self) -> builder::InstanceNetworkInterfaceCreate<'_>;
    /// Fetch network interface
    ///
    /// Sends a `GET` request to `/v1/network-interfaces/{interface}`
    ///
    /// Arguments:
    /// - `interface`: Name or ID of the network interface
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project, only required if `instance` is
    ///   provided as a `Name`
    /// ```ignore
    /// let response = client.instance_network_interface_view()
    ///    .interface(interface)
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_network_interface_view(&self) -> builder::InstanceNetworkInterfaceView<'_>;
    /// Update network interface
    ///
    /// Sends a `PUT` request to `/v1/network-interfaces/{interface}`
    ///
    /// Arguments:
    /// - `interface`: Name or ID of the network interface
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project, only required if `instance` is
    ///   provided as a `Name`
    /// - `body`
    /// ```ignore
    /// let response = client.instance_network_interface_update()
    ///    .interface(interface)
    ///    .instance(instance)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_network_interface_update(&self) -> builder::InstanceNetworkInterfaceUpdate<'_>;
    /// Delete network interface
    ///
    /// Note that the primary interface for an instance cannot be deleted if
    /// there are any secondary interfaces. A new primary interface must be
    /// designated first. The primary interface can be deleted if there are no
    /// secondary interfaces.
    ///
    /// Sends a `DELETE` request to `/v1/network-interfaces/{interface}`
    ///
    /// Arguments:
    /// - `interface`: Name or ID of the network interface
    /// - `instance`: Name or ID of the instance
    /// - `project`: Name or ID of the project, only required if `instance` is
    ///   provided as a `Name`
    /// ```ignore
    /// let response = client.instance_network_interface_delete()
    ///    .interface(interface)
    ///    .instance(instance)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn instance_network_interface_delete(&self) -> builder::InstanceNetworkInterfaceDelete<'_>;
}

impl ClientInstancesExt for Client {
    fn instance_list(&self) -> builder::InstanceList<'_> {
        builder::InstanceList::new(self)
    }

    fn instance_create(&self) -> builder::InstanceCreate<'_> {
        builder::InstanceCreate::new(self)
    }

    fn instance_view(&self) -> builder::InstanceView<'_> {
        builder::InstanceView::new(self)
    }

    fn instance_update(&self) -> builder::InstanceUpdate<'_> {
        builder::InstanceUpdate::new(self)
    }

    fn instance_delete(&self) -> builder::InstanceDelete<'_> {
        builder::InstanceDelete::new(self)
    }

    fn instance_anti_affinity_group_list(&self) -> builder::InstanceAntiAffinityGroupList<'_> {
        builder::InstanceAntiAffinityGroupList::new(self)
    }

    fn instance_disk_list(&self) -> builder::InstanceDiskList<'_> {
        builder::InstanceDiskList::new(self)
    }

    fn instance_disk_attach(&self) -> builder::InstanceDiskAttach<'_> {
        builder::InstanceDiskAttach::new(self)
    }

    fn instance_disk_detach(&self) -> builder::InstanceDiskDetach<'_> {
        builder::InstanceDiskDetach::new(self)
    }

    fn instance_external_ip_list(&self) -> builder::InstanceExternalIpList<'_> {
        builder::InstanceExternalIpList::new(self)
    }

    fn instance_ephemeral_ip_attach(&self) -> builder::InstanceEphemeralIpAttach<'_> {
        builder::InstanceEphemeralIpAttach::new(self)
    }

    fn instance_ephemeral_ip_detach(&self) -> builder::InstanceEphemeralIpDetach<'_> {
        builder::InstanceEphemeralIpDetach::new(self)
    }

    fn instance_external_subnet_list(&self) -> builder::InstanceExternalSubnetList<'_> {
        builder::InstanceExternalSubnetList::new(self)
    }

    fn instance_reboot(&self) -> builder::InstanceReboot<'_> {
        builder::InstanceReboot::new(self)
    }

    fn instance_serial_console(&self) -> builder::InstanceSerialConsole<'_> {
        builder::InstanceSerialConsole::new(self)
    }

    fn instance_serial_console_stream(&self) -> builder::InstanceSerialConsoleStream<'_> {
        builder::InstanceSerialConsoleStream::new(self)
    }

    fn instance_ssh_public_key_list(&self) -> builder::InstanceSshPublicKeyList<'_> {
        builder::InstanceSshPublicKeyList::new(self)
    }

    fn instance_start(&self) -> builder::InstanceStart<'_> {
        builder::InstanceStart::new(self)
    }

    fn instance_stop(&self) -> builder::InstanceStop<'_> {
        builder::InstanceStop::new(self)
    }

    fn instance_network_interface_list(&self) -> builder::InstanceNetworkInterfaceList<'_> {
        builder::InstanceNetworkInterfaceList::new(self)
    }

    fn instance_network_interface_create(&self) -> builder::InstanceNetworkInterfaceCreate<'_> {
        builder::InstanceNetworkInterfaceCreate::new(self)
    }

    fn instance_network_interface_view(&self) -> builder::InstanceNetworkInterfaceView<'_> {
        builder::InstanceNetworkInterfaceView::new(self)
    }

    fn instance_network_interface_update(&self) -> builder::InstanceNetworkInterfaceUpdate<'_> {
        builder::InstanceNetworkInterfaceUpdate::new(self)
    }

    fn instance_network_interface_delete(&self) -> builder::InstanceNetworkInterfaceDelete<'_> {
        builder::InstanceNetworkInterfaceDelete::new(self)
    }
}

/// IP pools are collections of external IPs that can be allocated and attached
/// to instances.
pub trait ClientIpPoolsExt {
    /// List IP pools
    ///
    /// Sends a `GET` request to `/v1/ip-pools`
    ///
    /// Arguments:
    /// - `ip_version`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `pool_type`
    /// - `sort_by`
    /// ```ignore
    /// let response = client.ip_pool_list()
    ///    .ip_version(ip_version)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .pool_type(pool_type)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn ip_pool_list(&self) -> builder::IpPoolList<'_>;
    /// Fetch IP pool
    ///
    /// Sends a `GET` request to `/v1/ip-pools/{pool}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// ```ignore
    /// let response = client.ip_pool_view()
    ///    .pool(pool)
    ///    .send()
    ///    .await;
    /// ```
    fn ip_pool_view(&self) -> builder::IpPoolView<'_>;
}

impl ClientIpPoolsExt for Client {
    fn ip_pool_list(&self) -> builder::IpPoolList<'_> {
        builder::IpPoolList::new(self)
    }

    fn ip_pool_view(&self) -> builder::IpPoolView<'_> {
        builder::IpPoolView::new(self)
    }
}

/// Authentication endpoints
pub trait ClientLoginExt {
    /// Authenticate user via SAML
    ///
    /// Sends a `POST` request to `/login/{silo_name}/saml/{provider_name}`
    ///
    /// ```ignore
    /// let response = client.login_saml()
    ///    .silo_name(silo_name)
    ///    .provider_name(provider_name)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn login_saml(&self) -> builder::LoginSaml<'_>;
    /// Authenticate user via username and password
    ///
    /// Sends a `POST` request to `/v1/login/{silo_name}/local`
    ///
    /// ```ignore
    /// let response = client.login_local()
    ///    .silo_name(silo_name)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn login_local(&self) -> builder::LoginLocal<'_>;
}

impl ClientLoginExt for Client {
    fn login_saml(&self) -> builder::LoginSaml<'_> {
        builder::LoginSaml::new(self)
    }

    fn login_local(&self) -> builder::LoginLocal<'_> {
        builder::LoginLocal::new(self)
    }
}

/// Silo-scoped metrics
pub trait ClientMetricsExt {
    /// View metrics
    ///
    /// View CPU, memory, or storage utilization metrics at the silo or project
    /// level.
    ///
    /// Sends a `GET` request to `/v1/metrics/{metric_name}`
    ///
    /// Arguments:
    /// - `metric_name`
    /// - `end_time`: An exclusive end time of metrics.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `order`: Query result order
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `start_time`: An inclusive start time of metrics.
    /// ```ignore
    /// let response = client.silo_metric()
    ///    .metric_name(metric_name)
    ///    .end_time(end_time)
    ///    .limit(limit)
    ///    .order(order)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .start_time(start_time)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_metric(&self) -> builder::SiloMetric<'_>;
}

impl ClientMetricsExt for Client {
    fn silo_metric(&self) -> builder::SiloMetric<'_> {
        builder::SiloMetric::new(self)
    }
}

/// System-wide IAM policy
pub trait ClientPolicyExt {
    /// Fetch top-level IAM policy
    ///
    /// Sends a `GET` request to `/v1/system/policy`
    ///
    /// ```ignore
    /// let response = client.system_policy_view()
    ///    .send()
    ///    .await;
    /// ```
    fn system_policy_view(&self) -> builder::SystemPolicyView<'_>;
    /// Update top-level IAM policy
    ///
    /// Sends a `PUT` request to `/v1/system/policy`
    ///
    /// ```ignore
    /// let response = client.system_policy_update()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_policy_update(&self) -> builder::SystemPolicyUpdate<'_>;
}

impl ClientPolicyExt for Client {
    fn system_policy_view(&self) -> builder::SystemPolicyView<'_> {
        builder::SystemPolicyView::new(self)
    }

    fn system_policy_update(&self) -> builder::SystemPolicyUpdate<'_> {
        builder::SystemPolicyUpdate::new(self)
    }
}

/// Projects are a grouping of associated resources such as instances and disks
/// within a silo for purposes of billing and access control.
pub trait ClientProjectsExt {
    /// List projects
    ///
    /// Sends a `GET` request to `/v1/projects`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.project_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn project_list(&self) -> builder::ProjectList<'_>;
    /// Create project
    ///
    /// Sends a `POST` request to `/v1/projects`
    ///
    /// ```ignore
    /// let response = client.project_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn project_create(&self) -> builder::ProjectCreate<'_>;
    /// Fetch project
    ///
    /// Sends a `GET` request to `/v1/projects/{project}`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.project_view()
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn project_view(&self) -> builder::ProjectView<'_>;
    /// Update project
    ///
    /// Sends a `PUT` request to `/v1/projects/{project}`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.project_update()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn project_update(&self) -> builder::ProjectUpdate<'_>;
    /// Delete project
    ///
    /// Sends a `DELETE` request to `/v1/projects/{project}`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.project_delete()
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn project_delete(&self) -> builder::ProjectDelete<'_>;
    /// Fetch project's IAM policy
    ///
    /// Sends a `GET` request to `/v1/projects/{project}/policy`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.project_policy_view()
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn project_policy_view(&self) -> builder::ProjectPolicyView<'_>;
    /// Update project's IAM policy
    ///
    /// Sends a `PUT` request to `/v1/projects/{project}/policy`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.project_policy_update()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn project_policy_update(&self) -> builder::ProjectPolicyUpdate<'_>;
}

impl ClientProjectsExt for Client {
    fn project_list(&self) -> builder::ProjectList<'_> {
        builder::ProjectList::new(self)
    }

    fn project_create(&self) -> builder::ProjectCreate<'_> {
        builder::ProjectCreate::new(self)
    }

    fn project_view(&self) -> builder::ProjectView<'_> {
        builder::ProjectView::new(self)
    }

    fn project_update(&self) -> builder::ProjectUpdate<'_> {
        builder::ProjectUpdate::new(self)
    }

    fn project_delete(&self) -> builder::ProjectDelete<'_> {
        builder::ProjectDelete::new(self)
    }

    fn project_policy_view(&self) -> builder::ProjectPolicyView<'_> {
        builder::ProjectPolicyView::new(self)
    }

    fn project_policy_update(&self) -> builder::ProjectPolicyUpdate<'_> {
        builder::ProjectPolicyUpdate::new(self)
    }
}

/// Silos represent a logical partition of users and resources.
pub trait ClientSilosExt {
    /// Fetch current silo's auth settings
    ///
    /// Sends a `GET` request to `/v1/auth-settings`
    ///
    /// ```ignore
    /// let response = client.auth_settings_view()
    ///    .send()
    ///    .await;
    /// ```
    fn auth_settings_view(&self) -> builder::AuthSettingsView<'_>;
    /// Update current silo's auth settings
    ///
    /// Sends a `PUT` request to `/v1/auth-settings`
    ///
    /// ```ignore
    /// let response = client.auth_settings_update()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn auth_settings_update(&self) -> builder::AuthSettingsUpdate<'_>;
    /// List TLS certificates in current silo
    ///
    /// Sends a `GET` request to `/v1/certificates`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.certificate_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn certificate_list(&self) -> builder::CertificateList<'_>;
    /// Create TLS certificate in current silo
    ///
    /// This certificate is automatically used by the Oxide control plane to
    /// serve API requests.
    ///
    /// Sends a `POST` request to `/v1/certificates`
    ///
    /// ```ignore
    /// let response = client.certificate_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn certificate_create(&self) -> builder::CertificateCreate<'_>;
    /// Fetch TLS certificate in current silo
    ///
    /// Sends a `GET` request to `/v1/certificates/{certificate}`
    ///
    /// Arguments:
    /// - `certificate`: Name or ID of the certificate
    /// ```ignore
    /// let response = client.certificate_view()
    ///    .certificate(certificate)
    ///    .send()
    ///    .await;
    /// ```
    fn certificate_view(&self) -> builder::CertificateView<'_>;
    /// Delete TLS certificate in current silo
    ///
    /// Sends a `DELETE` request to `/v1/certificates/{certificate}`
    ///
    /// Arguments:
    /// - `certificate`: Name or ID of the certificate
    /// ```ignore
    /// let response = client.certificate_delete()
    ///    .certificate(certificate)
    ///    .send()
    ///    .await;
    /// ```
    fn certificate_delete(&self) -> builder::CertificateDelete<'_>;
    /// List groups
    ///
    /// Sends a `GET` request to `/v1/groups`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.group_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn group_list(&self) -> builder::GroupList<'_>;
    /// Fetch group
    ///
    /// Sends a `GET` request to `/v1/groups/{group_id}`
    ///
    /// Arguments:
    /// - `group_id`: ID of the group
    /// ```ignore
    /// let response = client.group_view()
    ///    .group_id(group_id)
    ///    .send()
    ///    .await;
    /// ```
    fn group_view(&self) -> builder::GroupView<'_>;
    /// Fetch current silo's IAM policy
    ///
    /// Sends a `GET` request to `/v1/policy`
    ///
    /// ```ignore
    /// let response = client.policy_view()
    ///    .send()
    ///    .await;
    /// ```
    fn policy_view(&self) -> builder::PolicyView<'_>;
    /// Update current silo's IAM policy
    ///
    /// Sends a `PUT` request to `/v1/policy`
    ///
    /// ```ignore
    /// let response = client.policy_update()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn policy_update(&self) -> builder::PolicyUpdate<'_>;
    /// List users
    ///
    /// Sends a `GET` request to `/v1/users`
    ///
    /// Arguments:
    /// - `group`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.user_list()
    ///    .group(group)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn user_list(&self) -> builder::UserList<'_>;
    /// Fetch user
    ///
    /// Sends a `GET` request to `/v1/users/{user_id}`
    ///
    /// Arguments:
    /// - `user_id`: ID of the user
    /// ```ignore
    /// let response = client.user_view()
    ///    .user_id(user_id)
    ///    .send()
    ///    .await;
    /// ```
    fn user_view(&self) -> builder::UserView<'_>;
    /// List user's access tokens
    ///
    /// Sends a `GET` request to `/v1/users/{user_id}/access-tokens`
    ///
    /// Arguments:
    /// - `user_id`: ID of the user
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.user_token_list()
    ///    .user_id(user_id)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn user_token_list(&self) -> builder::UserTokenList<'_>;
    /// Log user out
    ///
    /// Silo admins can use this endpoint to log the specified user out by
    /// deleting all of their tokens AND sessions. This cannot be undone.
    ///
    /// Sends a `POST` request to `/v1/users/{user_id}/logout`
    ///
    /// Arguments:
    /// - `user_id`: ID of the user
    /// ```ignore
    /// let response = client.user_logout()
    ///    .user_id(user_id)
    ///    .send()
    ///    .await;
    /// ```
    fn user_logout(&self) -> builder::UserLogout<'_>;
    /// List user's console sessions
    ///
    /// Sends a `GET` request to `/v1/users/{user_id}/sessions`
    ///
    /// Arguments:
    /// - `user_id`: ID of the user
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.user_session_list()
    ///    .user_id(user_id)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn user_session_list(&self) -> builder::UserSessionList<'_>;
    /// Fetch resource utilization for user's current silo
    ///
    /// Sends a `GET` request to `/v1/utilization`
    ///
    /// ```ignore
    /// let response = client.utilization_view()
    ///    .send()
    ///    .await;
    /// ```
    fn utilization_view(&self) -> builder::UtilizationView<'_>;
}

impl ClientSilosExt for Client {
    fn auth_settings_view(&self) -> builder::AuthSettingsView<'_> {
        builder::AuthSettingsView::new(self)
    }

    fn auth_settings_update(&self) -> builder::AuthSettingsUpdate<'_> {
        builder::AuthSettingsUpdate::new(self)
    }

    fn certificate_list(&self) -> builder::CertificateList<'_> {
        builder::CertificateList::new(self)
    }

    fn certificate_create(&self) -> builder::CertificateCreate<'_> {
        builder::CertificateCreate::new(self)
    }

    fn certificate_view(&self) -> builder::CertificateView<'_> {
        builder::CertificateView::new(self)
    }

    fn certificate_delete(&self) -> builder::CertificateDelete<'_> {
        builder::CertificateDelete::new(self)
    }

    fn group_list(&self) -> builder::GroupList<'_> {
        builder::GroupList::new(self)
    }

    fn group_view(&self) -> builder::GroupView<'_> {
        builder::GroupView::new(self)
    }

    fn policy_view(&self) -> builder::PolicyView<'_> {
        builder::PolicyView::new(self)
    }

    fn policy_update(&self) -> builder::PolicyUpdate<'_> {
        builder::PolicyUpdate::new(self)
    }

    fn user_list(&self) -> builder::UserList<'_> {
        builder::UserList::new(self)
    }

    fn user_view(&self) -> builder::UserView<'_> {
        builder::UserView::new(self)
    }

    fn user_token_list(&self) -> builder::UserTokenList<'_> {
        builder::UserTokenList::new(self)
    }

    fn user_logout(&self) -> builder::UserLogout<'_> {
        builder::UserLogout::new(self)
    }

    fn user_session_list(&self) -> builder::UserSessionList<'_> {
        builder::UserSessionList::new(self)
    }

    fn utilization_view(&self) -> builder::UtilizationView<'_> {
        builder::UtilizationView::new(self)
    }
}

/// Snapshots of virtual disks at a particular point in time.
pub trait ClientSnapshotsExt {
    /// List snapshots
    ///
    /// Sends a `GET` request to `/v1/snapshots`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.snapshot_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn snapshot_list(&self) -> builder::SnapshotList<'_>;
    /// Create snapshot
    ///
    /// Creates a point-in-time snapshot from a disk.
    ///
    /// Sends a `POST` request to `/v1/snapshots`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.snapshot_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn snapshot_create(&self) -> builder::SnapshotCreate<'_>;
    /// Fetch snapshot
    ///
    /// Sends a `GET` request to `/v1/snapshots/{snapshot}`
    ///
    /// Arguments:
    /// - `snapshot`: Name or ID of the snapshot
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.snapshot_view()
    ///    .snapshot(snapshot)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn snapshot_view(&self) -> builder::SnapshotView<'_>;
    /// Delete snapshot
    ///
    /// Sends a `DELETE` request to `/v1/snapshots/{snapshot}`
    ///
    /// Arguments:
    /// - `snapshot`: Name or ID of the snapshot
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.snapshot_delete()
    ///    .snapshot(snapshot)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn snapshot_delete(&self) -> builder::SnapshotDelete<'_>;
}

impl ClientSnapshotsExt for Client {
    fn snapshot_list(&self) -> builder::SnapshotList<'_> {
        builder::SnapshotList::new(self)
    }

    fn snapshot_create(&self) -> builder::SnapshotCreate<'_> {
        builder::SnapshotCreate::new(self)
    }

    fn snapshot_view(&self) -> builder::SnapshotView<'_> {
        builder::SnapshotView::new(self)
    }

    fn snapshot_delete(&self) -> builder::SnapshotDelete<'_> {
        builder::SnapshotDelete::new(self)
    }
}

/// Subnet pools are collections of external subnets that can be allocated and
/// attached to instances.
pub trait ClientSubnetPoolsExt {
    /// List subnet pools
    ///
    /// Sends a `GET` request to `/v1/subnet-pools`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.subnet_pool_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn subnet_pool_list(&self) -> builder::SubnetPoolList<'_>;
    /// Fetch subnet pool
    ///
    /// Sends a `GET` request to `/v1/subnet-pools/{pool}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// ```ignore
    /// let response = client.subnet_pool_view()
    ///    .pool(pool)
    ///    .send()
    ///    .await;
    /// ```
    fn subnet_pool_view(&self) -> builder::SubnetPoolView<'_>;
}

impl ClientSubnetPoolsExt for Client {
    fn subnet_pool_list(&self) -> builder::SubnetPoolList<'_> {
        builder::SubnetPoolList::new(self)
    }

    fn subnet_pool_view(&self) -> builder::SubnetPoolView<'_> {
        builder::SubnetPoolView::new(self)
    }
}

/// Alerts deliver notifications for events that occur on the Oxide rack
pub trait ClientSystemAlertsExt {
    /// List alert classes
    ///
    /// Sends a `GET` request to `/v1/alert-classes`
    ///
    /// Arguments:
    /// - `filter`: An optional glob pattern for filtering alert class names.
    ///
    /// If provided, only alert classes which match this glob pattern will be
    /// included in the response.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// ```ignore
    /// let response = client.alert_class_list()
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_class_list(&self) -> builder::AlertClassList<'_>;
    /// List alert receivers
    ///
    /// Sends a `GET` request to `/v1/alert-receivers`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.alert_receiver_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_receiver_list(&self) -> builder::AlertReceiverList<'_>;
    /// Fetch alert receiver
    ///
    /// Sends a `GET` request to `/v1/alert-receivers/{receiver}`
    ///
    /// Arguments:
    /// - `receiver`: The name or ID of the webhook receiver.
    /// ```ignore
    /// let response = client.alert_receiver_view()
    ///    .receiver(receiver)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_receiver_view(&self) -> builder::AlertReceiverView<'_>;
    /// Delete alert receiver
    ///
    /// Sends a `DELETE` request to `/v1/alert-receivers/{receiver}`
    ///
    /// Arguments:
    /// - `receiver`: The name or ID of the webhook receiver.
    /// ```ignore
    /// let response = client.alert_receiver_delete()
    ///    .receiver(receiver)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_receiver_delete(&self) -> builder::AlertReceiverDelete<'_>;
    /// List delivery attempts to alert receiver
    ///
    /// Optional query parameters to this endpoint may be used to filter
    /// deliveries by state. If none of the `failed`, `pending` or `delivered`
    /// query parameters are present, all deliveries are returned. If one or
    /// more of these parameters are provided, only those which are set to
    /// "true" are included in the response.
    ///
    /// Sends a `GET` request to `/v1/alert-receivers/{receiver}/deliveries`
    ///
    /// Arguments:
    /// - `receiver`: The name or ID of the webhook receiver.
    /// - `delivered`: If true, include deliveries which have succeeded.
    ///
    /// If any of the "pending", "failed", or "delivered" query parameters are
    /// set to true, only deliveries matching those state(s) will be included in
    /// the response. If NO state filter parameters are set, then all deliveries
    /// are included.
    /// - `failed`: If true, include deliveries which have failed permanently.
    ///
    /// If any of the "pending", "failed", or "delivered" query parameters are
    /// set to true, only deliveries matching those state(s) will be included in
    /// the response. If NO state filter parameters are set, then all deliveries
    /// are included.
    ///
    /// A delivery fails permanently when the retry limit of three total
    /// attempts is reached without a successful delivery.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `pending`: If true, include deliveries which are currently in
    ///   progress.
    ///
    /// If any of the "pending", "failed", or "delivered" query parameters are
    /// set to true, only deliveries matching those state(s) will be included in
    /// the response. If NO state filter parameters are set, then all deliveries
    /// are included.
    ///
    /// A delivery is considered "pending" if it has not yet been sent at all,
    /// or if a delivery attempt has failed but the delivery has retries
    /// remaining.
    /// - `sort_by`
    /// ```ignore
    /// let response = client.alert_delivery_list()
    ///    .receiver(receiver)
    ///    .delivered(delivered)
    ///    .failed(failed)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .pending(pending)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_delivery_list(&self) -> builder::AlertDeliveryList<'_>;
    /// Send liveness probe to alert receiver
    ///
    /// This endpoint synchronously sends a liveness probe to the selected alert
    /// receiver. The response message describes the outcome of the probe:
    /// either the successful response (as appropriate), or indication of why
    /// the probe failed.
    ///
    /// The result of the probe is represented as an `AlertDelivery` model.
    /// Details relating to the status of the probe depend on the alert delivery
    /// mechanism, and are included in the `AlertDeliveryAttempts` model. For
    /// example, webhook receiver liveness probes include the HTTP status code
    /// returned by the receiver endpoint.
    ///
    /// Note that the response status is `200 OK` as long as a probe request was
    /// able to be sent to the receiver endpoint. If an HTTP-based receiver,
    /// such as a webhook, responds to the another status code, including an
    /// error, this will be indicated by the response body, *not* the status of
    /// the response.
    ///
    /// The `resend` query parameter can be used to request re-delivery of
    /// failed events if the liveness probe succeeds. If it is set to true and
    /// the liveness probe succeeds, any alerts for which delivery to this
    /// receiver has failed will be queued for re-delivery.
    ///
    /// Sends a `POST` request to `/v1/alert-receivers/{receiver}/probe`
    ///
    /// Arguments:
    /// - `receiver`: The name or ID of the webhook receiver.
    /// - `resend`: If true, resend all events that have not been delivered
    ///   successfully if the probe request succeeds.
    /// ```ignore
    /// let response = client.alert_receiver_probe()
    ///    .receiver(receiver)
    ///    .resend(resend)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_receiver_probe(&self) -> builder::AlertReceiverProbe<'_>;
    /// Add alert receiver subscription
    ///
    /// Sends a `POST` request to `/v1/alert-receivers/{receiver}/subscriptions`
    ///
    /// Arguments:
    /// - `receiver`: The name or ID of the webhook receiver.
    /// - `body`
    /// ```ignore
    /// let response = client.alert_receiver_subscription_add()
    ///    .receiver(receiver)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_receiver_subscription_add(&self) -> builder::AlertReceiverSubscriptionAdd<'_>;
    /// Remove alert receiver subscription
    ///
    /// Sends a `DELETE` request to
    /// `/v1/alert-receivers/{receiver}/subscriptions/{subscription}`
    ///
    /// Arguments:
    /// - `receiver`: The name or ID of the webhook receiver.
    /// - `subscription`: The event class subscription itself.
    /// ```ignore
    /// let response = client.alert_receiver_subscription_remove()
    ///    .receiver(receiver)
    ///    .subscription(subscription)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_receiver_subscription_remove(&self) -> builder::AlertReceiverSubscriptionRemove<'_>;
    /// List alerts
    ///
    /// Alerts may be filtered by alert class or alert class glob and by an
    /// inclusive creation time range.
    ///
    /// Sends a `GET` request to `/v1/alerts`
    ///
    /// Arguments:
    /// - `alert_class`: Optional alert class or glob pattern used to filter
    ///   alerts.
    ///
    /// If this is included, only alerts with the specified class or matching
    /// the glob pattern (as appropriate) will be returned. Otherwise, alerts of
    /// all classes will be returned.
    ///
    /// See the guide-level documentation on alerts for details on alert classes
    /// and alert class glob patterns.
    /// - `end_time`: Inclusive upper bound on the alert creation time
    ///
    /// If this is included, only alerts created at or before this time will be
    /// returned.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// - `start_time`: Inclusive lower bound on the alert creation time.
    ///
    /// If this is included, only alerts created at or after this time will be
    /// returned.
    /// ```ignore
    /// let response = client.alert_list()
    ///    .alert_class(alert_class)
    ///    .end_time(end_time)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .start_time(start_time)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_list(&self) -> builder::AlertList<'_>;
    /// Fetch alert
    ///
    /// Sends a `GET` request to `/v1/alerts/{alert_id}`
    ///
    /// Arguments:
    /// - `alert_id`: UUID of the alert
    /// ```ignore
    /// let response = client.alert_view()
    ///    .alert_id(alert_id)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_view(&self) -> builder::AlertView<'_>;
    /// Request re-delivery of alert
    ///
    /// Sends a `POST` request to `/v1/alerts/{alert_id}/resend`
    ///
    /// Arguments:
    /// - `alert_id`: UUID of the alert
    /// - `receiver`: The name or ID of the webhook receiver.
    /// ```ignore
    /// let response = client.alert_delivery_resend()
    ///    .alert_id(alert_id)
    ///    .receiver(receiver)
    ///    .send()
    ///    .await;
    /// ```
    fn alert_delivery_resend(&self) -> builder::AlertDeliveryResend<'_>;
    /// Create webhook receiver
    ///
    /// Sends a `POST` request to `/v1/webhook-receivers`
    ///
    /// ```ignore
    /// let response = client.webhook_receiver_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn webhook_receiver_create(&self) -> builder::WebhookReceiverCreate<'_>;
    /// Update webhook receiver
    ///
    /// Note that receiver secrets are NOT added or removed using this endpoint.
    /// Instead, use the `/v1/webhooks/{secrets}/?receiver={receiver}` endpoint
    /// to add and remove secrets.
    ///
    /// Sends a `PUT` request to `/v1/webhook-receivers/{receiver}`
    ///
    /// Arguments:
    /// - `receiver`: The name or ID of the webhook receiver.
    /// - `body`
    /// ```ignore
    /// let response = client.webhook_receiver_update()
    ///    .receiver(receiver)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn webhook_receiver_update(&self) -> builder::WebhookReceiverUpdate<'_>;
    /// List webhook receiver secret IDs
    ///
    /// Sends a `GET` request to `/v1/webhook-secrets`
    ///
    /// Arguments:
    /// - `receiver`: The name or ID of the webhook receiver.
    /// ```ignore
    /// let response = client.webhook_secrets_list()
    ///    .receiver(receiver)
    ///    .send()
    ///    .await;
    /// ```
    fn webhook_secrets_list(&self) -> builder::WebhookSecretsList<'_>;
    /// Add secret to webhook receiver
    ///
    /// Sends a `POST` request to `/v1/webhook-secrets`
    ///
    /// Arguments:
    /// - `receiver`: The name or ID of the webhook receiver.
    /// - `body`
    /// ```ignore
    /// let response = client.webhook_secrets_add()
    ///    .receiver(receiver)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn webhook_secrets_add(&self) -> builder::WebhookSecretsAdd<'_>;
    /// Remove secret from webhook receiver
    ///
    /// Sends a `DELETE` request to `/v1/webhook-secrets/{secret_id}`
    ///
    /// Arguments:
    /// - `secret_id`: ID of the secret.
    /// ```ignore
    /// let response = client.webhook_secrets_delete()
    ///    .secret_id(secret_id)
    ///    .send()
    ///    .await;
    /// ```
    fn webhook_secrets_delete(&self) -> builder::WebhookSecretsDelete<'_>;
}

impl ClientSystemAlertsExt for Client {
    fn alert_class_list(&self) -> builder::AlertClassList<'_> {
        builder::AlertClassList::new(self)
    }

    fn alert_receiver_list(&self) -> builder::AlertReceiverList<'_> {
        builder::AlertReceiverList::new(self)
    }

    fn alert_receiver_view(&self) -> builder::AlertReceiverView<'_> {
        builder::AlertReceiverView::new(self)
    }

    fn alert_receiver_delete(&self) -> builder::AlertReceiverDelete<'_> {
        builder::AlertReceiverDelete::new(self)
    }

    fn alert_delivery_list(&self) -> builder::AlertDeliveryList<'_> {
        builder::AlertDeliveryList::new(self)
    }

    fn alert_receiver_probe(&self) -> builder::AlertReceiverProbe<'_> {
        builder::AlertReceiverProbe::new(self)
    }

    fn alert_receiver_subscription_add(&self) -> builder::AlertReceiverSubscriptionAdd<'_> {
        builder::AlertReceiverSubscriptionAdd::new(self)
    }

    fn alert_receiver_subscription_remove(&self) -> builder::AlertReceiverSubscriptionRemove<'_> {
        builder::AlertReceiverSubscriptionRemove::new(self)
    }

    fn alert_list(&self) -> builder::AlertList<'_> {
        builder::AlertList::new(self)
    }

    fn alert_view(&self) -> builder::AlertView<'_> {
        builder::AlertView::new(self)
    }

    fn alert_delivery_resend(&self) -> builder::AlertDeliveryResend<'_> {
        builder::AlertDeliveryResend::new(self)
    }

    fn webhook_receiver_create(&self) -> builder::WebhookReceiverCreate<'_> {
        builder::WebhookReceiverCreate::new(self)
    }

    fn webhook_receiver_update(&self) -> builder::WebhookReceiverUpdate<'_> {
        builder::WebhookReceiverUpdate::new(self)
    }

    fn webhook_secrets_list(&self) -> builder::WebhookSecretsList<'_> {
        builder::WebhookSecretsList::new(self)
    }

    fn webhook_secrets_add(&self) -> builder::WebhookSecretsAdd<'_> {
        builder::WebhookSecretsAdd::new(self)
    }

    fn webhook_secrets_delete(&self) -> builder::WebhookSecretsDelete<'_> {
        builder::WebhookSecretsDelete::new(self)
    }
}

/// These endpoints relate to audit logs.
pub trait ClientSystemAuditLogExt {
    /// View audit log
    ///
    /// A single item in the audit log represents both the beginning and end of
    /// the logged operation (represented by `time_started` and
    /// `time_completed`) so that clients do not have to find multiple entries
    /// and match them up by request ID to get the full picture of an operation.
    /// Because timestamps may not be unique, entries have also have a unique
    /// `id` that can be used to deduplicate items fetched from overlapping time
    /// intervals.
    ///
    /// Audit log entries are designed to be immutable: once you see an entry,
    /// fetching it again will never get you a different result. The list is
    /// ordered and filtered by `time_completed`, not `time_started`. If you
    /// fetch the audit log for a time range that is fully in the past, the
    /// resulting list is guaranteed to be complete, i.e., fetching the same
    /// timespan again later will always produce the same set of entries.
    ///
    /// Sends a `GET` request to `/v1/system/audit-log`
    ///
    /// Arguments:
    /// - `end_time`: End of time range (exclusive). Filters on
    ///   `time_completed`. If omitted, the range extends to the present.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// - `start_time`: Start of time range (inclusive). Filters on
    ///   `time_completed`.
    /// ```ignore
    /// let response = client.audit_log_list()
    ///    .end_time(end_time)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .start_time(start_time)
    ///    .send()
    ///    .await;
    /// ```
    fn audit_log_list(&self) -> builder::AuditLogList<'_>;
}

impl ClientSystemAuditLogExt for Client {
    fn audit_log_list(&self) -> builder::AuditLogList<'_> {
        builder::AuditLogList::new(self)
    }
}

/// These operations pertain to hardware inventory and management. Racks are the
/// unit of expansion of an Oxide deployment. Racks are in turn composed of
/// sleds, switches, power supplies, and a cabled backplane.
pub trait ClientSystemHardwareExt {
    /// Enable adoption of a physical disk for general use
    ///
    /// Sends a `PUT` request to `/v1/system/hardware/disk-adoption-request`
    ///
    /// ```ignore
    /// let response = client.physical_disk_enable_adoption()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn physical_disk_enable_adoption(&self) -> builder::PhysicalDiskEnableAdoption<'_>;
    /// Disable adoption of a physical disk for general use
    ///
    /// Sends a `DELETE` request to
    /// `/v1/system/hardware/disk-adoption-request/
    /// {physical_disk_adoption_req_id}`
    ///
    /// Arguments:
    /// - `physical_disk_adoption_req_id`: ID of the physical disk adoption
    ///   request
    /// ```ignore
    /// let response = client.physical_disk_disable_adoption()
    ///    .physical_disk_adoption_req_id(physical_disk_adoption_req_id)
    ///    .send()
    ///    .await;
    /// ```
    fn physical_disk_disable_adoption(&self) -> builder::PhysicalDiskDisableAdoption<'_>;
    /// List physical disk adoption requests
    ///
    /// Sends a `GET` request to `/v1/system/hardware/disk-adoption-requests`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.physical_disk_list_adoption_requests()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn physical_disk_list_adoption_requests(&self)
        -> builder::PhysicalDiskListAdoptionRequests<'_>;
    /// List physical disks
    ///
    /// Sends a `GET` request to `/v1/system/hardware/disks`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.physical_disk_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn physical_disk_list(&self) -> builder::PhysicalDiskList<'_>;
    /// Get physical disk
    ///
    /// Sends a `GET` request to `/v1/system/hardware/disks/{disk_id}`
    ///
    /// Arguments:
    /// - `disk_id`: ID of the physical disk
    /// ```ignore
    /// let response = client.physical_disk_view()
    ///    .disk_id(disk_id)
    ///    .send()
    ///    .await;
    /// ```
    fn physical_disk_view(&self) -> builder::PhysicalDiskView<'_>;
    /// List physical disks that have not yet been adopted for use
    ///
    /// Sends a `GET` request to `/v1/system/hardware/disks-unadopted`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// ```ignore
    /// let response = client.physical_disk_list_unadopted()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .send()
    ///    .await;
    /// ```
    fn physical_disk_list_unadopted(&self) -> builder::PhysicalDiskListUnadopted<'_>;
    /// List racks
    ///
    /// Sends a `GET` request to `/v1/system/hardware/racks`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.rack_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn rack_list(&self) -> builder::RackList<'_>;
    /// Fetch rack
    ///
    /// Sends a `GET` request to `/v1/system/hardware/racks/{rack_id}`
    ///
    /// Arguments:
    /// - `rack_id`: ID of the rack
    /// ```ignore
    /// let response = client.rack_view()
    ///    .rack_id(rack_id)
    ///    .send()
    ///    .await;
    /// ```
    fn rack_view(&self) -> builder::RackView<'_>;
    /// List sleds
    ///
    /// Sends a `GET` request to `/v1/system/hardware/sleds`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.sled_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn sled_list(&self) -> builder::SledList<'_>;
    /// Fetch sled
    ///
    /// Sends a `GET` request to `/v1/system/hardware/sleds/{sled_id}`
    ///
    /// Arguments:
    /// - `sled_id`: ID of the sled
    /// ```ignore
    /// let response = client.sled_view()
    ///    .sled_id(sled_id)
    ///    .send()
    ///    .await;
    /// ```
    fn sled_view(&self) -> builder::SledView<'_>;
    /// List physical disks attached to sleds
    ///
    /// Sends a `GET` request to `/v1/system/hardware/sleds/{sled_id}/disks`
    ///
    /// Arguments:
    /// - `sled_id`: ID of the sled
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.sled_physical_disk_list()
    ///    .sled_id(sled_id)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn sled_physical_disk_list(&self) -> builder::SledPhysicalDiskList<'_>;
    /// List instances running on given sled
    ///
    /// Sends a `GET` request to `/v1/system/hardware/sleds/{sled_id}/instances`
    ///
    /// Arguments:
    /// - `sled_id`: ID of the sled
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.sled_instance_list()
    ///    .sled_id(sled_id)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn sled_instance_list(&self) -> builder::SledInstanceList<'_>;
    /// Set sled provision policy
    ///
    /// Sends a `PUT` request to
    /// `/v1/system/hardware/sleds/{sled_id}/provision-policy`
    ///
    /// Arguments:
    /// - `sled_id`: ID of the sled
    /// - `body`
    /// ```ignore
    /// let response = client.sled_set_provision_policy()
    ///    .sled_id(sled_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn sled_set_provision_policy(&self) -> builder::SledSetProvisionPolicy<'_>;
    /// List uninitialized sleds
    ///
    /// Sends a `GET` request to `/v1/system/hardware/sleds-uninitialized`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// ```ignore
    /// let response = client.sled_list_uninitialized()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .send()
    ///    .await;
    /// ```
    fn sled_list_uninitialized(&self) -> builder::SledListUninitialized<'_>;
    /// List switch ports
    ///
    /// Sends a `GET` request to `/v1/system/hardware/switch-port`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// - `switch_port_id`: An optional switch port id to use when listing
    ///   switch ports.
    /// ```ignore
    /// let response = client.networking_switch_port_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .switch_port_id(switch_port_id)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_list(&self) -> builder::NetworkingSwitchPortList<'_>;
    /// Apply switch port settings
    ///
    /// Sends a `POST` request to
    /// `/v1/system/hardware/switch-port/{port}/settings`
    ///
    /// Arguments:
    /// - `port`: A name to use when selecting switch ports.
    /// - `rack_id`: A rack id to use when selecting switch ports.
    /// - `switch_slot`: The slot of the switch within the rack to use when
    ///   selecting switch ports.
    /// - `body`
    /// ```ignore
    /// let response = client.networking_switch_port_apply_settings()
    ///    .port(port)
    ///    .rack_id(rack_id)
    ///    .switch_slot(switch_slot)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_apply_settings(
        &self,
    ) -> builder::NetworkingSwitchPortApplySettings<'_>;
    /// Clear switch port settings
    ///
    /// Sends a `DELETE` request to
    /// `/v1/system/hardware/switch-port/{port}/settings`
    ///
    /// Arguments:
    /// - `port`: A name to use when selecting switch ports.
    /// - `rack_id`: A rack id to use when selecting switch ports.
    /// - `switch_slot`: The slot of the switch within the rack to use when
    ///   selecting switch ports.
    /// ```ignore
    /// let response = client.networking_switch_port_clear_settings()
    ///    .port(port)
    ///    .rack_id(rack_id)
    ///    .switch_slot(switch_slot)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_clear_settings(
        &self,
    ) -> builder::NetworkingSwitchPortClearSettings<'_>;
    /// Get switch port status
    ///
    /// Sends a `GET` request to `/v1/system/hardware/switch-port/{port}/status`
    ///
    /// Arguments:
    /// - `port`: A name to use when selecting switch ports.
    /// - `rack_id`: A rack id to use when selecting switch ports.
    /// - `switch_slot`: The slot of the switch within the rack to use when
    ///   selecting switch ports.
    /// ```ignore
    /// let response = client.networking_switch_port_status()
    ///    .port(port)
    ///    .rack_id(rack_id)
    ///    .switch_slot(switch_slot)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_status(&self) -> builder::NetworkingSwitchPortStatus<'_>;
    /// List switches
    ///
    /// Sends a `GET` request to `/v1/system/hardware/switches`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.switch_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn switch_list(&self) -> builder::SwitchList<'_>;
    /// Fetch switch
    ///
    /// Sends a `GET` request to `/v1/system/hardware/switches/{switch_id}`
    ///
    /// Arguments:
    /// - `switch_id`: ID of the switch
    /// ```ignore
    /// let response = client.switch_view()
    ///    .switch_id(switch_id)
    ///    .send()
    ///    .await;
    /// ```
    fn switch_view(&self) -> builder::SwitchView<'_>;
}

impl ClientSystemHardwareExt for Client {
    fn physical_disk_enable_adoption(&self) -> builder::PhysicalDiskEnableAdoption<'_> {
        builder::PhysicalDiskEnableAdoption::new(self)
    }

    fn physical_disk_disable_adoption(&self) -> builder::PhysicalDiskDisableAdoption<'_> {
        builder::PhysicalDiskDisableAdoption::new(self)
    }

    fn physical_disk_list_adoption_requests(
        &self,
    ) -> builder::PhysicalDiskListAdoptionRequests<'_> {
        builder::PhysicalDiskListAdoptionRequests::new(self)
    }

    fn physical_disk_list(&self) -> builder::PhysicalDiskList<'_> {
        builder::PhysicalDiskList::new(self)
    }

    fn physical_disk_view(&self) -> builder::PhysicalDiskView<'_> {
        builder::PhysicalDiskView::new(self)
    }

    fn physical_disk_list_unadopted(&self) -> builder::PhysicalDiskListUnadopted<'_> {
        builder::PhysicalDiskListUnadopted::new(self)
    }

    fn rack_list(&self) -> builder::RackList<'_> {
        builder::RackList::new(self)
    }

    fn rack_view(&self) -> builder::RackView<'_> {
        builder::RackView::new(self)
    }

    fn sled_list(&self) -> builder::SledList<'_> {
        builder::SledList::new(self)
    }

    fn sled_view(&self) -> builder::SledView<'_> {
        builder::SledView::new(self)
    }

    fn sled_physical_disk_list(&self) -> builder::SledPhysicalDiskList<'_> {
        builder::SledPhysicalDiskList::new(self)
    }

    fn sled_instance_list(&self) -> builder::SledInstanceList<'_> {
        builder::SledInstanceList::new(self)
    }

    fn sled_set_provision_policy(&self) -> builder::SledSetProvisionPolicy<'_> {
        builder::SledSetProvisionPolicy::new(self)
    }

    fn sled_list_uninitialized(&self) -> builder::SledListUninitialized<'_> {
        builder::SledListUninitialized::new(self)
    }

    fn networking_switch_port_list(&self) -> builder::NetworkingSwitchPortList<'_> {
        builder::NetworkingSwitchPortList::new(self)
    }

    fn networking_switch_port_apply_settings(
        &self,
    ) -> builder::NetworkingSwitchPortApplySettings<'_> {
        builder::NetworkingSwitchPortApplySettings::new(self)
    }

    fn networking_switch_port_clear_settings(
        &self,
    ) -> builder::NetworkingSwitchPortClearSettings<'_> {
        builder::NetworkingSwitchPortClearSettings::new(self)
    }

    fn networking_switch_port_status(&self) -> builder::NetworkingSwitchPortStatus<'_> {
        builder::NetworkingSwitchPortStatus::new(self)
    }

    fn switch_list(&self) -> builder::SwitchList<'_> {
        builder::SwitchList::new(self)
    }

    fn switch_view(&self) -> builder::SwitchView<'_> {
        builder::SwitchView::new(self)
    }
}

/// IP pools are collections of external IPs. Linking a pool to a silo makes it
/// available for allocation by users in that silo.
pub trait ClientSystemIpPoolsExt {
    /// List IP pools
    ///
    /// Sends a `GET` request to `/v1/system/ip-pools`
    ///
    /// Arguments:
    /// - `assignment`
    /// - `ip_version`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `pool_type`
    /// - `sort_by`
    /// ```ignore
    /// let response = client.system_ip_pool_list()
    ///    .assignment(assignment)
    ///    .ip_version(ip_version)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .pool_type(pool_type)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_list(&self) -> builder::SystemIpPoolList<'_>;
    /// Create IP pool
    ///
    /// Sends a `POST` request to `/v1/system/ip-pools`
    ///
    /// ```ignore
    /// let response = client.system_ip_pool_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_create(&self) -> builder::SystemIpPoolCreate<'_>;
    /// Fetch IP pool
    ///
    /// Sends a `GET` request to `/v1/system/ip-pools/{pool}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// ```ignore
    /// let response = client.system_ip_pool_view()
    ///    .pool(pool)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_view(&self) -> builder::SystemIpPoolView<'_>;
    /// Update IP pool
    ///
    /// Sends a `PUT` request to `/v1/system/ip-pools/{pool}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// - `body`
    /// ```ignore
    /// let response = client.system_ip_pool_update()
    ///    .pool(pool)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_update(&self) -> builder::SystemIpPoolUpdate<'_>;
    /// Delete IP pool
    ///
    /// Sends a `DELETE` request to `/v1/system/ip-pools/{pool}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// ```ignore
    /// let response = client.system_ip_pool_delete()
    ///    .pool(pool)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_delete(&self) -> builder::SystemIpPoolDelete<'_>;
    /// Assign IP pool
    ///
    /// Sends a `POST` request to `/v1/system/ip-pools/{pool}/assignment`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// - `body`
    /// ```ignore
    /// let response = client.system_ip_pool_assign()
    ///    .pool(pool)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_assign(&self) -> builder::SystemIpPoolAssign<'_>;
    /// List ranges for IP pool
    ///
    /// Ranges are ordered by their first address.
    ///
    /// Sends a `GET` request to `/v1/system/ip-pools/{pool}/ranges`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// ```ignore
    /// let response = client.system_ip_pool_range_list()
    ///    .pool(pool)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_range_list(&self) -> builder::SystemIpPoolRangeList<'_>;
    /// Add range to IP pool
    ///
    /// For multicast pools, all ranges must be either Any-Source Multicast
    /// (ASM) or Source-Specific Multicast (SSM), but not both. Mixing ASM and
    /// SSM ranges in the same pool is not allowed.
    ///
    /// ASM: IPv4 addresses outside 232.0.0.0/8, IPv6 addresses with flag field
    /// != 3 SSM: IPv4 addresses in 232.0.0.0/8, IPv6 addresses with flag field
    /// = 3
    ///
    /// Sends a `POST` request to `/v1/system/ip-pools/{pool}/ranges/add`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// - `body`
    /// ```ignore
    /// let response = client.system_ip_pool_range_add()
    ///    .pool(pool)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_range_add(&self) -> builder::SystemIpPoolRangeAdd<'_>;
    /// Remove range from IP pool
    ///
    /// Sends a `POST` request to `/v1/system/ip-pools/{pool}/ranges/remove`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// - `body`
    /// ```ignore
    /// let response = client.system_ip_pool_range_remove()
    ///    .pool(pool)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_range_remove(&self) -> builder::SystemIpPoolRangeRemove<'_>;
    /// List IP pool's linked silos
    ///
    /// Sends a `GET` request to `/v1/system/ip-pools/{pool}/silos`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.system_ip_pool_silo_list()
    ///    .pool(pool)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_silo_list(&self) -> builder::SystemIpPoolSiloList<'_>;
    /// Link IP pool to silo
    ///
    /// Users in linked silos can allocate external IPs from this pool for their
    /// instances. A silo can have at most one default pool. IPs are allocated
    /// from the default pool when users ask for one without specifying a pool.
    ///
    /// Sends a `POST` request to `/v1/system/ip-pools/{pool}/silos`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// - `body`
    /// ```ignore
    /// let response = client.system_ip_pool_silo_link()
    ///    .pool(pool)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_silo_link(&self) -> builder::SystemIpPoolSiloLink<'_>;
    /// Make IP pool default for silo
    ///
    /// When a user asks for an IP (e.g., at instance create time) without
    /// specifying a pool, the IP comes from the default pool if a default is
    /// configured. When a pool is made the default for a silo, any existing
    /// default will remain linked to the silo, but will no longer be the
    /// default.
    ///
    /// Sends a `PUT` request to `/v1/system/ip-pools/{pool}/silos/{silo}`
    ///
    /// ```ignore
    /// let response = client.system_ip_pool_silo_update()
    ///    .pool(pool)
    ///    .silo(silo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_silo_update(&self) -> builder::SystemIpPoolSiloUpdate<'_>;
    /// Unlink IP pool from silo
    ///
    /// Will fail if there are any outstanding IPs allocated in the silo.
    ///
    /// Sends a `DELETE` request to `/v1/system/ip-pools/{pool}/silos/{silo}`
    ///
    /// ```ignore
    /// let response = client.system_ip_pool_silo_unlink()
    ///    .pool(pool)
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_silo_unlink(&self) -> builder::SystemIpPoolSiloUnlink<'_>;
    /// Fetch IP pool utilization
    ///
    /// Sends a `GET` request to `/v1/system/ip-pools/{pool}/utilization`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// ```ignore
    /// let response = client.system_ip_pool_utilization_view()
    ///    .pool(pool)
    ///    .send()
    ///    .await;
    /// ```
    fn system_ip_pool_utilization_view(&self) -> builder::SystemIpPoolUtilizationView<'_>;
}

impl ClientSystemIpPoolsExt for Client {
    fn system_ip_pool_list(&self) -> builder::SystemIpPoolList<'_> {
        builder::SystemIpPoolList::new(self)
    }

    fn system_ip_pool_create(&self) -> builder::SystemIpPoolCreate<'_> {
        builder::SystemIpPoolCreate::new(self)
    }

    fn system_ip_pool_view(&self) -> builder::SystemIpPoolView<'_> {
        builder::SystemIpPoolView::new(self)
    }

    fn system_ip_pool_update(&self) -> builder::SystemIpPoolUpdate<'_> {
        builder::SystemIpPoolUpdate::new(self)
    }

    fn system_ip_pool_delete(&self) -> builder::SystemIpPoolDelete<'_> {
        builder::SystemIpPoolDelete::new(self)
    }

    fn system_ip_pool_assign(&self) -> builder::SystemIpPoolAssign<'_> {
        builder::SystemIpPoolAssign::new(self)
    }

    fn system_ip_pool_range_list(&self) -> builder::SystemIpPoolRangeList<'_> {
        builder::SystemIpPoolRangeList::new(self)
    }

    fn system_ip_pool_range_add(&self) -> builder::SystemIpPoolRangeAdd<'_> {
        builder::SystemIpPoolRangeAdd::new(self)
    }

    fn system_ip_pool_range_remove(&self) -> builder::SystemIpPoolRangeRemove<'_> {
        builder::SystemIpPoolRangeRemove::new(self)
    }

    fn system_ip_pool_silo_list(&self) -> builder::SystemIpPoolSiloList<'_> {
        builder::SystemIpPoolSiloList::new(self)
    }

    fn system_ip_pool_silo_link(&self) -> builder::SystemIpPoolSiloLink<'_> {
        builder::SystemIpPoolSiloLink::new(self)
    }

    fn system_ip_pool_silo_update(&self) -> builder::SystemIpPoolSiloUpdate<'_> {
        builder::SystemIpPoolSiloUpdate::new(self)
    }

    fn system_ip_pool_silo_unlink(&self) -> builder::SystemIpPoolSiloUnlink<'_> {
        builder::SystemIpPoolSiloUnlink::new(self)
    }

    fn system_ip_pool_utilization_view(&self) -> builder::SystemIpPoolUtilizationView<'_> {
        builder::SystemIpPoolUtilizationView::new(self)
    }
}

/// Metrics provide insight into the operation of the Oxide deployment. These
/// include telemetry on hardware and software components that can be used to
/// understand the current state as well as to diagnose issues.
pub trait ClientSystemMetricsExt {
    /// View metrics
    ///
    /// View CPU, memory, or storage utilization metrics at the fleet or silo
    /// level.
    ///
    /// Sends a `GET` request to `/v1/system/metrics/{metric_name}`
    ///
    /// Arguments:
    /// - `metric_name`
    /// - `end_time`: An exclusive end time of metrics.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `order`: Query result order
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `silo`: Name or ID of the silo
    /// - `start_time`: An inclusive start time of metrics.
    /// ```ignore
    /// let response = client.system_metric()
    ///    .metric_name(metric_name)
    ///    .end_time(end_time)
    ///    .limit(limit)
    ///    .order(order)
    ///    .page_token(page_token)
    ///    .silo(silo)
    ///    .start_time(start_time)
    ///    .send()
    ///    .await;
    /// ```
    fn system_metric(&self) -> builder::SystemMetric<'_>;
    /// Run timeseries query
    ///
    /// Queries are written in OxQL.
    ///
    /// Sends a `POST` request to `/v1/system/timeseries/query`
    ///
    /// ```ignore
    /// let response = client.system_timeseries_query()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_timeseries_query(&self) -> builder::SystemTimeseriesQuery<'_>;
    /// List timeseries schemas
    ///
    /// Sends a `GET` request to `/v1/system/timeseries/schemas`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// ```ignore
    /// let response = client.system_timeseries_schema_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .send()
    ///    .await;
    /// ```
    fn system_timeseries_schema_list(&self) -> builder::SystemTimeseriesSchemaList<'_>;
}

impl ClientSystemMetricsExt for Client {
    fn system_metric(&self) -> builder::SystemMetric<'_> {
        builder::SystemMetric::new(self)
    }

    fn system_timeseries_query(&self) -> builder::SystemTimeseriesQuery<'_> {
        builder::SystemTimeseriesQuery::new(self)
    }

    fn system_timeseries_schema_list(&self) -> builder::SystemTimeseriesSchemaList<'_> {
        builder::SystemTimeseriesSchemaList::new(self)
    }
}

/// This provides rack-level network configuration.
pub trait ClientSystemNetworkingExt {
    /// Fetch LLDP neighbors for switch port
    ///
    /// Sends a `GET` request to
    /// `/v1/system/hardware/rack-switch-port/{rack_id}/{switch_slot}/{port}/
    /// lldp/neighbors`
    ///
    /// Arguments:
    /// - `rack_id`: A rack id to use when selecting switch ports.
    /// - `switch_slot`: The slot of the switch within the rack to use when
    ///   selecting switch ports.
    /// - `port`: A name to use when selecting switch ports.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.networking_switch_port_lldp_neighbors()
    ///    .rack_id(rack_id)
    ///    .switch_slot(switch_slot)
    ///    .port(port)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_lldp_neighbors(
        &self,
    ) -> builder::NetworkingSwitchPortLldpNeighbors<'_>;
    /// Fetch LLDP configuration for switch port
    ///
    /// Sends a `GET` request to
    /// `/v1/system/hardware/switch-port/{port}/lldp/config`
    ///
    /// Arguments:
    /// - `port`: A name to use when selecting switch ports.
    /// - `rack_id`: A rack id to use when selecting switch ports.
    /// - `switch_slot`: The slot of the switch within the rack to use when
    ///   selecting switch ports.
    /// ```ignore
    /// let response = client.networking_switch_port_lldp_config_view()
    ///    .port(port)
    ///    .rack_id(rack_id)
    ///    .switch_slot(switch_slot)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_lldp_config_view(
        &self,
    ) -> builder::NetworkingSwitchPortLldpConfigView<'_>;
    /// Update LLDP configuration for switch port
    ///
    /// Sends a `POST` request to
    /// `/v1/system/hardware/switch-port/{port}/lldp/config`
    ///
    /// Arguments:
    /// - `port`: A name to use when selecting switch ports.
    /// - `rack_id`: A rack id to use when selecting switch ports.
    /// - `switch_slot`: The slot of the switch within the rack to use when
    ///   selecting switch ports.
    /// - `body`
    /// ```ignore
    /// let response = client.networking_switch_port_lldp_config_update()
    ///    .port(port)
    ///    .rack_id(rack_id)
    ///    .switch_slot(switch_slot)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_lldp_config_update(
        &self,
    ) -> builder::NetworkingSwitchPortLldpConfigUpdate<'_>;
    /// List address lots
    ///
    /// Sends a `GET` request to `/v1/system/networking/address-lot`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.networking_address_lot_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_address_lot_list(&self) -> builder::NetworkingAddressLotList<'_>;
    /// Create address lot
    ///
    /// Sends a `POST` request to `/v1/system/networking/address-lot`
    ///
    /// ```ignore
    /// let response = client.networking_address_lot_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_address_lot_create(&self) -> builder::NetworkingAddressLotCreate<'_>;
    /// Fetch address lot
    ///
    /// Sends a `GET` request to
    /// `/v1/system/networking/address-lot/{address_lot}`
    ///
    /// Arguments:
    /// - `address_lot`: Name or ID of the address lot
    /// ```ignore
    /// let response = client.networking_address_lot_view()
    ///    .address_lot(address_lot)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_address_lot_view(&self) -> builder::NetworkingAddressLotView<'_>;
    /// Delete address lot
    ///
    /// Sends a `DELETE` request to
    /// `/v1/system/networking/address-lot/{address_lot}`
    ///
    /// Arguments:
    /// - `address_lot`: Name or ID of the address lot
    /// ```ignore
    /// let response = client.networking_address_lot_delete()
    ///    .address_lot(address_lot)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_address_lot_delete(&self) -> builder::NetworkingAddressLotDelete<'_>;
    /// List blocks in address lot
    ///
    /// Sends a `GET` request to
    /// `/v1/system/networking/address-lot/{address_lot}/blocks`
    ///
    /// Arguments:
    /// - `address_lot`: Name or ID of the address lot
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.networking_address_lot_block_list()
    ///    .address_lot(address_lot)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_address_lot_block_list(&self) -> builder::NetworkingAddressLotBlockList<'_>;
    /// Get user-facing services IP allowlist
    ///
    /// Sends a `GET` request to `/v1/system/networking/allow-list`
    ///
    /// ```ignore
    /// let response = client.networking_allow_list_view()
    ///    .send()
    ///    .await;
    /// ```
    fn networking_allow_list_view(&self) -> builder::NetworkingAllowListView<'_>;
    /// Update user-facing services IP allowlist
    ///
    /// Sends a `PUT` request to `/v1/system/networking/allow-list`
    ///
    /// ```ignore
    /// let response = client.networking_allow_list_update()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_allow_list_update(&self) -> builder::NetworkingAllowListUpdate<'_>;
    /// Disable BFD session
    ///
    /// Sends a `POST` request to `/v1/system/networking/bfd-disable`
    ///
    /// ```ignore
    /// let response = client.networking_bfd_disable()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bfd_disable(&self) -> builder::NetworkingBfdDisable<'_>;
    /// Enable BFD session
    ///
    /// Sends a `POST` request to `/v1/system/networking/bfd-enable`
    ///
    /// ```ignore
    /// let response = client.networking_bfd_enable()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bfd_enable(&self) -> builder::NetworkingBfdEnable<'_>;
    /// Get BFD status
    ///
    /// Sends a `GET` request to `/v1/system/networking/bfd-status`
    ///
    /// ```ignore
    /// let response = client.networking_bfd_status()
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bfd_status(&self) -> builder::NetworkingBfdStatus<'_>;
    /// List BGP configurations
    ///
    /// Sends a `GET` request to `/v1/system/networking/bgp`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.networking_bgp_config_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_config_list(&self) -> builder::NetworkingBgpConfigList<'_>;
    /// Update the mutable fields of an existing BGP configuration
    ///
    /// The asn field is not updatable; to change the autonomous system number,
    /// create a new BGP configuration object.
    ///
    /// Sends a `PUT` request to `/v1/system/networking/bgp`
    ///
    /// Arguments:
    /// - `name_or_id`: A name or id to use when selecting BGP config.
    /// - `body`
    /// ```ignore
    /// let response = client.networking_bgp_config_update()
    ///    .name_or_id(name_or_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_config_update(&self) -> builder::NetworkingBgpConfigUpdate<'_>;
    /// Create BGP configuration
    ///
    /// Sends a `POST` request to `/v1/system/networking/bgp`
    ///
    /// ```ignore
    /// let response = client.networking_bgp_config_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_config_create(&self) -> builder::NetworkingBgpConfigCreate<'_>;
    /// Delete BGP configuration
    ///
    /// Sends a `DELETE` request to `/v1/system/networking/bgp`
    ///
    /// Arguments:
    /// - `name_or_id`: A name or id to use when selecting BGP config.
    /// ```ignore
    /// let response = client.networking_bgp_config_delete()
    ///    .name_or_id(name_or_id)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_config_delete(&self) -> builder::NetworkingBgpConfigDelete<'_>;
    /// List BGP announce sets
    ///
    /// Sends a `GET` request to `/v1/system/networking/bgp-announce-set`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.networking_bgp_announce_set_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_announce_set_list(&self) -> builder::NetworkingBgpAnnounceSetList<'_>;
    /// Update BGP announce set
    ///
    /// If the announce set exists, this endpoint replaces the existing announce
    /// set with the one specified.
    ///
    /// Sends a `PUT` request to `/v1/system/networking/bgp-announce-set`
    ///
    /// ```ignore
    /// let response = client.networking_bgp_announce_set_update()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_announce_set_update(&self) -> builder::NetworkingBgpAnnounceSetUpdate<'_>;
    /// Delete BGP announce set
    ///
    /// Sends a `DELETE` request to
    /// `/v1/system/networking/bgp-announce-set/{announce_set}`
    ///
    /// Arguments:
    /// - `announce_set`: Name or ID of the announce set
    /// ```ignore
    /// let response = client.networking_bgp_announce_set_delete()
    ///    .announce_set(announce_set)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_announce_set_delete(&self) -> builder::NetworkingBgpAnnounceSetDelete<'_>;
    /// Get originated routes for a specified BGP announce set
    ///
    /// Sends a `GET` request to
    /// `/v1/system/networking/bgp-announce-set/{announce_set}/announcement`
    ///
    /// Arguments:
    /// - `announce_set`: Name or ID of the announce set
    /// ```ignore
    /// let response = client.networking_bgp_announcement_list()
    ///    .announce_set(announce_set)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_announcement_list(&self) -> builder::NetworkingBgpAnnouncementList<'_>;
    /// List BGP exported routes
    ///
    /// Sends a `GET` request to `/v1/system/networking/bgp-exported`
    ///
    /// ```ignore
    /// let response = client.networking_bgp_exported()
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_exported(&self) -> builder::NetworkingBgpExported<'_>;
    /// Get imported IPv4 BGP routes
    ///
    /// Sends a `GET` request to `/v1/system/networking/bgp-imported`
    ///
    /// Arguments:
    /// - `asn`: The ASN to filter on. Required.
    /// ```ignore
    /// let response = client.networking_bgp_imported()
    ///    .asn(asn)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_imported(&self) -> builder::NetworkingBgpImported<'_>;
    /// Get BGP router message history
    ///
    /// Sends a `GET` request to `/v1/system/networking/bgp-message-history`
    ///
    /// Arguments:
    /// - `asn`: The ASN to filter on. Required.
    /// ```ignore
    /// let response = client.networking_bgp_message_history()
    ///    .asn(asn)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_message_history(&self) -> builder::NetworkingBgpMessageHistory<'_>;
    /// Get BGP peer status
    ///
    /// Sends a `GET` request to `/v1/system/networking/bgp-status`
    ///
    /// ```ignore
    /// let response = client.networking_bgp_status()
    ///    .send()
    ///    .await;
    /// ```
    fn networking_bgp_status(&self) -> builder::NetworkingBgpStatus<'_>;
    /// Return whether API services can receive limited ICMP traffic
    ///
    /// Sends a `GET` request to `/v1/system/networking/inbound-icmp`
    ///
    /// ```ignore
    /// let response = client.networking_inbound_icmp_view()
    ///    .send()
    ///    .await;
    /// ```
    fn networking_inbound_icmp_view(&self) -> builder::NetworkingInboundIcmpView<'_>;
    /// Set whether API services can receive limited ICMP traffic
    ///
    /// Sends a `PUT` request to `/v1/system/networking/inbound-icmp`
    ///
    /// ```ignore
    /// let response = client.networking_inbound_icmp_update()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_inbound_icmp_update(&self) -> builder::NetworkingInboundIcmpUpdate<'_>;
    /// List loopback addresses
    ///
    /// Sends a `GET` request to `/v1/system/networking/loopback-address`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.networking_loopback_address_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_loopback_address_list(&self) -> builder::NetworkingLoopbackAddressList<'_>;
    /// Create loopback address
    ///
    /// Sends a `POST` request to `/v1/system/networking/loopback-address`
    ///
    /// ```ignore
    /// let response = client.networking_loopback_address_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_loopback_address_create(&self) -> builder::NetworkingLoopbackAddressCreate<'_>;
    /// Delete loopback address
    ///
    /// Sends a `DELETE` request to
    /// `/v1/system/networking/loopback-address/{rack_id}/{switch_slot}/
    /// {address}/{subnet_mask}`
    ///
    /// Arguments:
    /// - `rack_id`: The rack to use when selecting the loopback address.
    /// - `switch_slot`: The slot of the switch within the rack to use when
    ///   selecting the loopback address.
    /// - `address`: The IP address and subnet mask to use when selecting the
    ///   loopback address.
    /// - `subnet_mask`: The IP address and subnet mask to use when selecting
    ///   the loopback address.
    /// ```ignore
    /// let response = client.networking_loopback_address_delete()
    ///    .rack_id(rack_id)
    ///    .switch_slot(switch_slot)
    ///    .address(address)
    ///    .subnet_mask(subnet_mask)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_loopback_address_delete(&self) -> builder::NetworkingLoopbackAddressDelete<'_>;
    /// Fetch fleet-wide networking settings
    ///
    /// Sends a `GET` request to `/v1/system/networking/settings`
    ///
    /// ```ignore
    /// let response = client.system_networking_settings_view()
    ///    .send()
    ///    .await;
    /// ```
    fn system_networking_settings_view(&self) -> builder::SystemNetworkingSettingsView<'_>;
    /// Update fleet-wide networking settings
    ///
    /// Sends a `PUT` request to `/v1/system/networking/settings`
    ///
    /// ```ignore
    /// let response = client.system_networking_settings_update()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_networking_settings_update(&self) -> builder::SystemNetworkingSettingsUpdate<'_>;
    /// List switch port settings
    ///
    /// Sends a `GET` request to `/v1/system/networking/switch-port-settings`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `port_settings`: An optional name or id to use when selecting port
    ///   settings.
    /// - `sort_by`
    /// ```ignore
    /// let response = client.networking_switch_port_settings_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .port_settings(port_settings)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_settings_list(&self)
        -> builder::NetworkingSwitchPortSettingsList<'_>;
    /// Create switch port settings
    ///
    /// Sends a `POST` request to `/v1/system/networking/switch-port-settings`
    ///
    /// ```ignore
    /// let response = client.networking_switch_port_settings_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_settings_create(
        &self,
    ) -> builder::NetworkingSwitchPortSettingsCreate<'_>;
    /// Delete switch port settings
    ///
    /// Sends a `DELETE` request to `/v1/system/networking/switch-port-settings`
    ///
    /// Arguments:
    /// - `port_settings`: An optional name or id to use when selecting port
    ///   settings.
    /// ```ignore
    /// let response = client.networking_switch_port_settings_delete()
    ///    .port_settings(port_settings)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_settings_delete(
        &self,
    ) -> builder::NetworkingSwitchPortSettingsDelete<'_>;
    /// Get information about switch port
    ///
    /// Sends a `GET` request to
    /// `/v1/system/networking/switch-port-settings/{port}`
    ///
    /// Arguments:
    /// - `port`: A name or id to use when selecting switch port settings info
    ///   objects.
    /// ```ignore
    /// let response = client.networking_switch_port_settings_view()
    ///    .port(port)
    ///    .send()
    ///    .await;
    /// ```
    fn networking_switch_port_settings_view(&self)
        -> builder::NetworkingSwitchPortSettingsView<'_>;
}

impl ClientSystemNetworkingExt for Client {
    fn networking_switch_port_lldp_neighbors(
        &self,
    ) -> builder::NetworkingSwitchPortLldpNeighbors<'_> {
        builder::NetworkingSwitchPortLldpNeighbors::new(self)
    }

    fn networking_switch_port_lldp_config_view(
        &self,
    ) -> builder::NetworkingSwitchPortLldpConfigView<'_> {
        builder::NetworkingSwitchPortLldpConfigView::new(self)
    }

    fn networking_switch_port_lldp_config_update(
        &self,
    ) -> builder::NetworkingSwitchPortLldpConfigUpdate<'_> {
        builder::NetworkingSwitchPortLldpConfigUpdate::new(self)
    }

    fn networking_address_lot_list(&self) -> builder::NetworkingAddressLotList<'_> {
        builder::NetworkingAddressLotList::new(self)
    }

    fn networking_address_lot_create(&self) -> builder::NetworkingAddressLotCreate<'_> {
        builder::NetworkingAddressLotCreate::new(self)
    }

    fn networking_address_lot_view(&self) -> builder::NetworkingAddressLotView<'_> {
        builder::NetworkingAddressLotView::new(self)
    }

    fn networking_address_lot_delete(&self) -> builder::NetworkingAddressLotDelete<'_> {
        builder::NetworkingAddressLotDelete::new(self)
    }

    fn networking_address_lot_block_list(&self) -> builder::NetworkingAddressLotBlockList<'_> {
        builder::NetworkingAddressLotBlockList::new(self)
    }

    fn networking_allow_list_view(&self) -> builder::NetworkingAllowListView<'_> {
        builder::NetworkingAllowListView::new(self)
    }

    fn networking_allow_list_update(&self) -> builder::NetworkingAllowListUpdate<'_> {
        builder::NetworkingAllowListUpdate::new(self)
    }

    fn networking_bfd_disable(&self) -> builder::NetworkingBfdDisable<'_> {
        builder::NetworkingBfdDisable::new(self)
    }

    fn networking_bfd_enable(&self) -> builder::NetworkingBfdEnable<'_> {
        builder::NetworkingBfdEnable::new(self)
    }

    fn networking_bfd_status(&self) -> builder::NetworkingBfdStatus<'_> {
        builder::NetworkingBfdStatus::new(self)
    }

    fn networking_bgp_config_list(&self) -> builder::NetworkingBgpConfigList<'_> {
        builder::NetworkingBgpConfigList::new(self)
    }

    fn networking_bgp_config_update(&self) -> builder::NetworkingBgpConfigUpdate<'_> {
        builder::NetworkingBgpConfigUpdate::new(self)
    }

    fn networking_bgp_config_create(&self) -> builder::NetworkingBgpConfigCreate<'_> {
        builder::NetworkingBgpConfigCreate::new(self)
    }

    fn networking_bgp_config_delete(&self) -> builder::NetworkingBgpConfigDelete<'_> {
        builder::NetworkingBgpConfigDelete::new(self)
    }

    fn networking_bgp_announce_set_list(&self) -> builder::NetworkingBgpAnnounceSetList<'_> {
        builder::NetworkingBgpAnnounceSetList::new(self)
    }

    fn networking_bgp_announce_set_update(&self) -> builder::NetworkingBgpAnnounceSetUpdate<'_> {
        builder::NetworkingBgpAnnounceSetUpdate::new(self)
    }

    fn networking_bgp_announce_set_delete(&self) -> builder::NetworkingBgpAnnounceSetDelete<'_> {
        builder::NetworkingBgpAnnounceSetDelete::new(self)
    }

    fn networking_bgp_announcement_list(&self) -> builder::NetworkingBgpAnnouncementList<'_> {
        builder::NetworkingBgpAnnouncementList::new(self)
    }

    fn networking_bgp_exported(&self) -> builder::NetworkingBgpExported<'_> {
        builder::NetworkingBgpExported::new(self)
    }

    fn networking_bgp_imported(&self) -> builder::NetworkingBgpImported<'_> {
        builder::NetworkingBgpImported::new(self)
    }

    fn networking_bgp_message_history(&self) -> builder::NetworkingBgpMessageHistory<'_> {
        builder::NetworkingBgpMessageHistory::new(self)
    }

    fn networking_bgp_status(&self) -> builder::NetworkingBgpStatus<'_> {
        builder::NetworkingBgpStatus::new(self)
    }

    fn networking_inbound_icmp_view(&self) -> builder::NetworkingInboundIcmpView<'_> {
        builder::NetworkingInboundIcmpView::new(self)
    }

    fn networking_inbound_icmp_update(&self) -> builder::NetworkingInboundIcmpUpdate<'_> {
        builder::NetworkingInboundIcmpUpdate::new(self)
    }

    fn networking_loopback_address_list(&self) -> builder::NetworkingLoopbackAddressList<'_> {
        builder::NetworkingLoopbackAddressList::new(self)
    }

    fn networking_loopback_address_create(&self) -> builder::NetworkingLoopbackAddressCreate<'_> {
        builder::NetworkingLoopbackAddressCreate::new(self)
    }

    fn networking_loopback_address_delete(&self) -> builder::NetworkingLoopbackAddressDelete<'_> {
        builder::NetworkingLoopbackAddressDelete::new(self)
    }

    fn system_networking_settings_view(&self) -> builder::SystemNetworkingSettingsView<'_> {
        builder::SystemNetworkingSettingsView::new(self)
    }

    fn system_networking_settings_update(&self) -> builder::SystemNetworkingSettingsUpdate<'_> {
        builder::SystemNetworkingSettingsUpdate::new(self)
    }

    fn networking_switch_port_settings_list(
        &self,
    ) -> builder::NetworkingSwitchPortSettingsList<'_> {
        builder::NetworkingSwitchPortSettingsList::new(self)
    }

    fn networking_switch_port_settings_create(
        &self,
    ) -> builder::NetworkingSwitchPortSettingsCreate<'_> {
        builder::NetworkingSwitchPortSettingsCreate::new(self)
    }

    fn networking_switch_port_settings_delete(
        &self,
    ) -> builder::NetworkingSwitchPortSettingsDelete<'_> {
        builder::NetworkingSwitchPortSettingsDelete::new(self)
    }

    fn networking_switch_port_settings_view(
        &self,
    ) -> builder::NetworkingSwitchPortSettingsView<'_> {
        builder::NetworkingSwitchPortSettingsView::new(self)
    }
}

/// Silos represent a logical partition of users and resources.
pub trait ClientSystemSilosExt {
    /// List TLS certificates in silo
    ///
    /// Sends a `GET` request to `/v1/system/certificates`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `silo`: Name or ID of the silo
    /// - `sort_by`
    /// ```ignore
    /// let response = client.silo_certificate_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .silo(silo)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_certificate_list(&self) -> builder::SiloCertificateList<'_>;
    /// Create TLS certificate in silo
    ///
    /// This certificate is automatically used by the Oxide control plane to
    /// serve API requests.
    ///
    /// Sends a `POST` request to `/v1/system/certificates`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// - `body`
    /// ```ignore
    /// let response = client.silo_certificate_create()
    ///    .silo(silo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_certificate_create(&self) -> builder::SiloCertificateCreate<'_>;
    /// Fetch TLS certificate in silo
    ///
    /// The silo must be specified if the certificate is specified by name, and
    /// must not be specified if the certificate is specified by ID.
    ///
    /// Sends a `GET` request to `/v1/system/certificates/{certificate}`
    ///
    /// Arguments:
    /// - `certificate`: Name or ID of the certificate
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.silo_certificate_view()
    ///    .certificate(certificate)
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_certificate_view(&self) -> builder::SiloCertificateView<'_>;
    /// Delete TLS certificate in silo
    ///
    /// The silo must be specified if the certificate is specified by name, and
    /// must not be specified if the certificate is specified by ID.
    ///
    /// Sends a `DELETE` request to `/v1/system/certificates/{certificate}`
    ///
    /// Arguments:
    /// - `certificate`: Name or ID of the certificate
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.silo_certificate_delete()
    ///    .certificate(certificate)
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_certificate_delete(&self) -> builder::SiloCertificateDelete<'_>;
    /// List identity providers for silo
    ///
    /// List identity providers for silo by silo name or ID.
    ///
    /// Sends a `GET` request to `/v1/system/identity-providers`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `silo`: Name or ID of the silo
    /// - `sort_by`
    /// ```ignore
    /// let response = client.silo_identity_provider_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .silo(silo)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_identity_provider_list(&self) -> builder::SiloIdentityProviderList<'_>;
    /// Create user
    ///
    /// Users can only be created in Silos with `provision_type` == `Fixed`.
    /// Otherwise, Silo users are just-in-time (JIT) provisioned when a user
    /// first logs in using an external Identity Provider.
    ///
    /// Sends a `POST` request to `/v1/system/identity-providers/local/users`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// - `body`
    /// ```ignore
    /// let response = client.local_idp_user_create()
    ///    .silo(silo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn local_idp_user_create(&self) -> builder::LocalIdpUserCreate<'_>;
    /// Delete user
    ///
    /// Sends a `DELETE` request to
    /// `/v1/system/identity-providers/local/users/{user_id}`
    ///
    /// Arguments:
    /// - `user_id`: The user's internal ID
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.local_idp_user_delete()
    ///    .user_id(user_id)
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn local_idp_user_delete(&self) -> builder::LocalIdpUserDelete<'_>;
    /// Set or invalidate user's password
    ///
    /// Passwords can only be updated for users in Silos with identity mode
    /// `LocalOnly`.
    ///
    /// Sends a `POST` request to
    /// `/v1/system/identity-providers/local/users/{user_id}/set-password`
    ///
    /// Arguments:
    /// - `user_id`: The user's internal ID
    /// - `silo`: Name or ID of the silo
    /// - `body`
    /// ```ignore
    /// let response = client.local_idp_user_set_password()
    ///    .user_id(user_id)
    ///    .silo(silo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn local_idp_user_set_password(&self) -> builder::LocalIdpUserSetPassword<'_>;
    /// Create SAML identity provider
    ///
    /// Sends a `POST` request to `/v1/system/identity-providers/saml`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// - `body`
    /// ```ignore
    /// let response = client.saml_identity_provider_create()
    ///    .silo(silo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn saml_identity_provider_create(&self) -> builder::SamlIdentityProviderCreate<'_>;
    /// Fetch SAML identity provider
    ///
    /// Sends a `GET` request to `/v1/system/identity-providers/saml/{provider}`
    ///
    /// Arguments:
    /// - `provider`: Name or ID of the SAML identity provider
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.saml_identity_provider_view()
    ///    .provider(provider)
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn saml_identity_provider_view(&self) -> builder::SamlIdentityProviderView<'_>;
    /// List SCIM tokens
    ///
    /// Specify the silo by name or ID using the `silo` query parameter.
    ///
    /// Sends a `GET` request to `/v1/system/scim/tokens`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.scim_token_list()
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn scim_token_list(&self) -> builder::ScimTokenList<'_>;
    /// Create SCIM token
    ///
    /// Specify the silo by name or ID using the `silo` query parameter. Be sure
    /// to save the bearer token in the response. It will not be retrievable
    /// later through the token view and list endpoints.
    ///
    /// Sends a `POST` request to `/v1/system/scim/tokens`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.scim_token_create()
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn scim_token_create(&self) -> builder::ScimTokenCreate<'_>;
    /// Fetch SCIM token
    ///
    /// Specify the silo by name or ID using the `silo` query parameter.
    ///
    /// Sends a `GET` request to `/v1/system/scim/tokens/{token_id}`
    ///
    /// Arguments:
    /// - `token_id`
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.scim_token_view()
    ///    .token_id(token_id)
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn scim_token_view(&self) -> builder::ScimTokenView<'_>;
    /// Delete SCIM token
    ///
    /// Specify the silo by name or ID using the `silo` query parameter.
    ///
    /// Sends a `DELETE` request to `/v1/system/scim/tokens/{token_id}`
    ///
    /// Arguments:
    /// - `token_id`
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.scim_token_delete()
    ///    .token_id(token_id)
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn scim_token_delete(&self) -> builder::ScimTokenDelete<'_>;
    /// List resource quotas for all silos
    ///
    /// Sends a `GET` request to `/v1/system/silo-quotas`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.system_quotas_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn system_quotas_list(&self) -> builder::SystemQuotasList<'_>;
    /// List silos
    ///
    /// Lists silos that are discoverable based on the current permissions.
    ///
    /// Sends a `GET` request to `/v1/system/silos`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.silo_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_list(&self) -> builder::SiloList<'_>;
    /// Create silo
    ///
    /// Sends a `POST` request to `/v1/system/silos`
    ///
    /// ```ignore
    /// let response = client.silo_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_create(&self) -> builder::SiloCreate<'_>;
    /// Fetch silo
    ///
    /// Fetch silo by name or ID.
    ///
    /// Sends a `GET` request to `/v1/system/silos/{silo}`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.silo_view()
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_view(&self) -> builder::SiloView<'_>;
    /// Delete silo
    ///
    /// Delete a silo by name or ID.
    ///
    /// Sends a `DELETE` request to `/v1/system/silos/{silo}`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.silo_delete()
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_delete(&self) -> builder::SiloDelete<'_>;
    /// List IP pools linked to silo
    ///
    /// Linked IP pools are available to users in the specified silo. A silo can
    /// have at most one default pool. IPs are allocated from the default pool
    /// when users ask for one without specifying a pool.
    ///
    /// Sends a `GET` request to `/v1/system/silos/{silo}/ip-pools`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.silo_ip_pool_list()
    ///    .silo(silo)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_ip_pool_list(&self) -> builder::SiloIpPoolList<'_>;
    /// Fetch silo IAM policy
    ///
    /// Sends a `GET` request to `/v1/system/silos/{silo}/policy`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.silo_policy_view()
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_policy_view(&self) -> builder::SiloPolicyView<'_>;
    /// Update silo IAM policy
    ///
    /// Sends a `PUT` request to `/v1/system/silos/{silo}/policy`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// - `body`
    /// ```ignore
    /// let response = client.silo_policy_update()
    ///    .silo(silo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_policy_update(&self) -> builder::SiloPolicyUpdate<'_>;
    /// Fetch resource quotas for silo
    ///
    /// Sends a `GET` request to `/v1/system/silos/{silo}/quotas`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.silo_quotas_view()
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_quotas_view(&self) -> builder::SiloQuotasView<'_>;
    /// Update resource quotas for silo
    ///
    /// If a quota value is not specified, it will remain unchanged.
    ///
    /// Sends a `PUT` request to `/v1/system/silos/{silo}/quotas`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// - `body`
    /// ```ignore
    /// let response = client.silo_quotas_update()
    ///    .silo(silo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_quotas_update(&self) -> builder::SiloQuotasUpdate<'_>;
    /// List subnet pools linked to a silo
    ///
    /// Sends a `GET` request to `/v1/system/silos/{silo}/subnet-pools`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.silo_subnet_pool_list()
    ///    .silo(silo)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_subnet_pool_list(&self) -> builder::SiloSubnetPoolList<'_>;
    /// List users in silo
    ///
    /// Sends a `GET` request to `/v1/system/users`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `silo`: Name or ID of the silo
    /// - `sort_by`
    /// ```ignore
    /// let response = client.silo_user_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .silo(silo)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_user_list(&self) -> builder::SiloUserList<'_>;
    /// Fetch user in silo
    ///
    /// Sends a `GET` request to `/v1/system/users/{user_id}`
    ///
    /// Arguments:
    /// - `user_id`: The user's internal ID
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.silo_user_view()
    ///    .user_id(user_id)
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_user_view(&self) -> builder::SiloUserView<'_>;
    /// List built-in users
    ///
    /// Sends a `GET` request to `/v1/system/users-builtin`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.user_builtin_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn user_builtin_list(&self) -> builder::UserBuiltinList<'_>;
    /// Fetch built-in user
    ///
    /// Sends a `GET` request to `/v1/system/users-builtin/{user}`
    ///
    /// ```ignore
    /// let response = client.user_builtin_view()
    ///    .user(user)
    ///    .send()
    ///    .await;
    /// ```
    fn user_builtin_view(&self) -> builder::UserBuiltinView<'_>;
    /// List current utilization state for all silos
    ///
    /// Sends a `GET` request to `/v1/system/utilization/silos`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.silo_utilization_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_utilization_list(&self) -> builder::SiloUtilizationList<'_>;
    /// Fetch current utilization for given silo
    ///
    /// Sends a `GET` request to `/v1/system/utilization/silos/{silo}`
    ///
    /// Arguments:
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.silo_utilization_view()
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn silo_utilization_view(&self) -> builder::SiloUtilizationView<'_>;
}

impl ClientSystemSilosExt for Client {
    fn silo_certificate_list(&self) -> builder::SiloCertificateList<'_> {
        builder::SiloCertificateList::new(self)
    }

    fn silo_certificate_create(&self) -> builder::SiloCertificateCreate<'_> {
        builder::SiloCertificateCreate::new(self)
    }

    fn silo_certificate_view(&self) -> builder::SiloCertificateView<'_> {
        builder::SiloCertificateView::new(self)
    }

    fn silo_certificate_delete(&self) -> builder::SiloCertificateDelete<'_> {
        builder::SiloCertificateDelete::new(self)
    }

    fn silo_identity_provider_list(&self) -> builder::SiloIdentityProviderList<'_> {
        builder::SiloIdentityProviderList::new(self)
    }

    fn local_idp_user_create(&self) -> builder::LocalIdpUserCreate<'_> {
        builder::LocalIdpUserCreate::new(self)
    }

    fn local_idp_user_delete(&self) -> builder::LocalIdpUserDelete<'_> {
        builder::LocalIdpUserDelete::new(self)
    }

    fn local_idp_user_set_password(&self) -> builder::LocalIdpUserSetPassword<'_> {
        builder::LocalIdpUserSetPassword::new(self)
    }

    fn saml_identity_provider_create(&self) -> builder::SamlIdentityProviderCreate<'_> {
        builder::SamlIdentityProviderCreate::new(self)
    }

    fn saml_identity_provider_view(&self) -> builder::SamlIdentityProviderView<'_> {
        builder::SamlIdentityProviderView::new(self)
    }

    fn scim_token_list(&self) -> builder::ScimTokenList<'_> {
        builder::ScimTokenList::new(self)
    }

    fn scim_token_create(&self) -> builder::ScimTokenCreate<'_> {
        builder::ScimTokenCreate::new(self)
    }

    fn scim_token_view(&self) -> builder::ScimTokenView<'_> {
        builder::ScimTokenView::new(self)
    }

    fn scim_token_delete(&self) -> builder::ScimTokenDelete<'_> {
        builder::ScimTokenDelete::new(self)
    }

    fn system_quotas_list(&self) -> builder::SystemQuotasList<'_> {
        builder::SystemQuotasList::new(self)
    }

    fn silo_list(&self) -> builder::SiloList<'_> {
        builder::SiloList::new(self)
    }

    fn silo_create(&self) -> builder::SiloCreate<'_> {
        builder::SiloCreate::new(self)
    }

    fn silo_view(&self) -> builder::SiloView<'_> {
        builder::SiloView::new(self)
    }

    fn silo_delete(&self) -> builder::SiloDelete<'_> {
        builder::SiloDelete::new(self)
    }

    fn silo_ip_pool_list(&self) -> builder::SiloIpPoolList<'_> {
        builder::SiloIpPoolList::new(self)
    }

    fn silo_policy_view(&self) -> builder::SiloPolicyView<'_> {
        builder::SiloPolicyView::new(self)
    }

    fn silo_policy_update(&self) -> builder::SiloPolicyUpdate<'_> {
        builder::SiloPolicyUpdate::new(self)
    }

    fn silo_quotas_view(&self) -> builder::SiloQuotasView<'_> {
        builder::SiloQuotasView::new(self)
    }

    fn silo_quotas_update(&self) -> builder::SiloQuotasUpdate<'_> {
        builder::SiloQuotasUpdate::new(self)
    }

    fn silo_subnet_pool_list(&self) -> builder::SiloSubnetPoolList<'_> {
        builder::SiloSubnetPoolList::new(self)
    }

    fn silo_user_list(&self) -> builder::SiloUserList<'_> {
        builder::SiloUserList::new(self)
    }

    fn silo_user_view(&self) -> builder::SiloUserView<'_> {
        builder::SiloUserView::new(self)
    }

    fn user_builtin_list(&self) -> builder::UserBuiltinList<'_> {
        builder::UserBuiltinList::new(self)
    }

    fn user_builtin_view(&self) -> builder::UserBuiltinView<'_> {
        builder::UserBuiltinView::new(self)
    }

    fn silo_utilization_list(&self) -> builder::SiloUtilizationList<'_> {
        builder::SiloUtilizationList::new(self)
    }

    fn silo_utilization_view(&self) -> builder::SiloUtilizationView<'_> {
        builder::SiloUtilizationView::new(self)
    }
}

/// Endpoints related to system health
pub trait ClientSystemStatusExt {
    /// Ping API
    ///
    /// Always responds with Ok if it responds at all.
    ///
    /// Sends a `GET` request to `/v1/ping`
    ///
    /// ```ignore
    /// let response = client.ping()
    ///    .send()
    ///    .await;
    /// ```
    fn ping(&self) -> builder::Ping<'_>;
}

impl ClientSystemStatusExt for Client {
    fn ping(&self) -> builder::Ping<'_> {
        builder::Ping::new(self)
    }
}

/// Subnet pools are collections of external subnets. Linking a pool to a silo
/// makes it available for allocation by users in that silo.
pub trait ClientSystemSubnetPoolsExt {
    /// List subnet pools
    ///
    /// Sends a `GET` request to `/v1/system/subnet-pools`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.system_subnet_pool_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_list(&self) -> builder::SystemSubnetPoolList<'_>;
    /// Create subnet pool
    ///
    /// Sends a `POST` request to `/v1/system/subnet-pools`
    ///
    /// ```ignore
    /// let response = client.system_subnet_pool_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_create(&self) -> builder::SystemSubnetPoolCreate<'_>;
    /// Fetch subnet pool
    ///
    /// Sends a `GET` request to `/v1/system/subnet-pools/{pool}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// ```ignore
    /// let response = client.system_subnet_pool_view()
    ///    .pool(pool)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_view(&self) -> builder::SystemSubnetPoolView<'_>;
    /// Update subnet pool
    ///
    /// Sends a `PUT` request to `/v1/system/subnet-pools/{pool}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// - `body`
    /// ```ignore
    /// let response = client.system_subnet_pool_update()
    ///    .pool(pool)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_update(&self) -> builder::SystemSubnetPoolUpdate<'_>;
    /// Delete subnet pool
    ///
    /// Sends a `DELETE` request to `/v1/system/subnet-pools/{pool}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// ```ignore
    /// let response = client.system_subnet_pool_delete()
    ///    .pool(pool)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_delete(&self) -> builder::SystemSubnetPoolDelete<'_>;
    /// List members in subnet pool
    ///
    /// Sends a `GET` request to `/v1/system/subnet-pools/{pool}/members`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// ```ignore
    /// let response = client.system_subnet_pool_member_list()
    ///    .pool(pool)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_member_list(&self) -> builder::SystemSubnetPoolMemberList<'_>;
    /// Add member to subnet pool
    ///
    /// Sends a `POST` request to `/v1/system/subnet-pools/{pool}/members/add`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// - `body`
    /// ```ignore
    /// let response = client.system_subnet_pool_member_add()
    ///    .pool(pool)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_member_add(&self) -> builder::SystemSubnetPoolMemberAdd<'_>;
    /// Remove member from subnet pool
    ///
    /// Sends a `POST` request to
    /// `/v1/system/subnet-pools/{pool}/members/remove`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// - `body`
    /// ```ignore
    /// let response = client.system_subnet_pool_member_remove()
    ///    .pool(pool)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_member_remove(&self) -> builder::SystemSubnetPoolMemberRemove<'_>;
    /// List silos linked to subnet pool
    ///
    /// Sends a `GET` request to `/v1/system/subnet-pools/{pool}/silos`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.system_subnet_pool_silo_list()
    ///    .pool(pool)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_silo_list(&self) -> builder::SystemSubnetPoolSiloList<'_>;
    /// Link subnet pool to silo
    ///
    /// Sends a `POST` request to `/v1/system/subnet-pools/{pool}/silos`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// - `body`
    /// ```ignore
    /// let response = client.system_subnet_pool_silo_link()
    ///    .pool(pool)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_silo_link(&self) -> builder::SystemSubnetPoolSiloLink<'_>;
    /// Update subnet pool's link to silo
    ///
    /// Sends a `PUT` request to `/v1/system/subnet-pools/{pool}/silos/{silo}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// - `silo`: Name or ID of the silo
    /// - `body`
    /// ```ignore
    /// let response = client.system_subnet_pool_silo_update()
    ///    .pool(pool)
    ///    .silo(silo)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_silo_update(&self) -> builder::SystemSubnetPoolSiloUpdate<'_>;
    /// Unlink subnet pool from silo
    ///
    /// Sends a `DELETE` request to
    /// `/v1/system/subnet-pools/{pool}/silos/{silo}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// - `silo`: Name or ID of the silo
    /// ```ignore
    /// let response = client.system_subnet_pool_silo_unlink()
    ///    .pool(pool)
    ///    .silo(silo)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_silo_unlink(&self) -> builder::SystemSubnetPoolSiloUnlink<'_>;
    /// Fetch subnet pool utilization
    ///
    /// Sends a `GET` request to `/v1/system/subnet-pools/{pool}/utilization`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the subnet pool
    /// ```ignore
    /// let response = client.system_subnet_pool_utilization_view()
    ///    .pool(pool)
    ///    .send()
    ///    .await;
    /// ```
    fn system_subnet_pool_utilization_view(&self) -> builder::SystemSubnetPoolUtilizationView<'_>;
}

impl ClientSystemSubnetPoolsExt for Client {
    fn system_subnet_pool_list(&self) -> builder::SystemSubnetPoolList<'_> {
        builder::SystemSubnetPoolList::new(self)
    }

    fn system_subnet_pool_create(&self) -> builder::SystemSubnetPoolCreate<'_> {
        builder::SystemSubnetPoolCreate::new(self)
    }

    fn system_subnet_pool_view(&self) -> builder::SystemSubnetPoolView<'_> {
        builder::SystemSubnetPoolView::new(self)
    }

    fn system_subnet_pool_update(&self) -> builder::SystemSubnetPoolUpdate<'_> {
        builder::SystemSubnetPoolUpdate::new(self)
    }

    fn system_subnet_pool_delete(&self) -> builder::SystemSubnetPoolDelete<'_> {
        builder::SystemSubnetPoolDelete::new(self)
    }

    fn system_subnet_pool_member_list(&self) -> builder::SystemSubnetPoolMemberList<'_> {
        builder::SystemSubnetPoolMemberList::new(self)
    }

    fn system_subnet_pool_member_add(&self) -> builder::SystemSubnetPoolMemberAdd<'_> {
        builder::SystemSubnetPoolMemberAdd::new(self)
    }

    fn system_subnet_pool_member_remove(&self) -> builder::SystemSubnetPoolMemberRemove<'_> {
        builder::SystemSubnetPoolMemberRemove::new(self)
    }

    fn system_subnet_pool_silo_list(&self) -> builder::SystemSubnetPoolSiloList<'_> {
        builder::SystemSubnetPoolSiloList::new(self)
    }

    fn system_subnet_pool_silo_link(&self) -> builder::SystemSubnetPoolSiloLink<'_> {
        builder::SystemSubnetPoolSiloLink::new(self)
    }

    fn system_subnet_pool_silo_update(&self) -> builder::SystemSubnetPoolSiloUpdate<'_> {
        builder::SystemSubnetPoolSiloUpdate::new(self)
    }

    fn system_subnet_pool_silo_unlink(&self) -> builder::SystemSubnetPoolSiloUnlink<'_> {
        builder::SystemSubnetPoolSiloUnlink::new(self)
    }

    fn system_subnet_pool_utilization_view(&self) -> builder::SystemSubnetPoolUtilizationView<'_> {
        builder::SystemSubnetPoolUtilizationView::new(self)
    }
}

/// Support bundles collect debugging information from the rack for use by Oxide
/// support.
pub trait ClientSystemSupportBundlesExt {
    /// List all support bundles
    ///
    /// Sends a `GET` request to `/v1/system/support-bundles`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.support_bundle_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_list(&self) -> builder::SupportBundleList<'_>;
    /// Create support bundle
    ///
    /// Sends a `POST` request to `/v1/system/support-bundles`
    ///
    /// ```ignore
    /// let response = client.support_bundle_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_create(&self) -> builder::SupportBundleCreate<'_>;
    /// View support bundle
    ///
    /// Sends a `GET` request to `/v1/system/support-bundles/{bundle_id}`
    ///
    /// Arguments:
    /// - `bundle_id`: ID of the support bundle
    /// ```ignore
    /// let response = client.support_bundle_view()
    ///    .bundle_id(bundle_id)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_view(&self) -> builder::SupportBundleView<'_>;
    /// Update support bundle
    ///
    /// Sends a `PUT` request to `/v1/system/support-bundles/{bundle_id}`
    ///
    /// Arguments:
    /// - `bundle_id`: ID of the support bundle
    /// - `body`
    /// ```ignore
    /// let response = client.support_bundle_update()
    ///    .bundle_id(bundle_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_update(&self) -> builder::SupportBundleUpdate<'_>;
    /// Delete support bundle
    ///
    /// May also be used to cancel a support bundle which is currently being
    /// collected, or to remove metadata for a support bundle that has failed.
    ///
    /// Sends a `DELETE` request to `/v1/system/support-bundles/{bundle_id}`
    ///
    /// Arguments:
    /// - `bundle_id`: ID of the support bundle
    /// ```ignore
    /// let response = client.support_bundle_delete()
    ///    .bundle_id(bundle_id)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_delete(&self) -> builder::SupportBundleDelete<'_>;
    /// Download support bundle contents
    ///
    /// Sends a `GET` request to
    /// `/v1/system/support-bundles/{bundle_id}/download`
    ///
    /// Arguments:
    /// - `bundle_id`: ID of the support bundle
    /// - `range`: A request to access a portion of the resource, such as
    ///   `bytes=0-499`
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Range>
    /// ```ignore
    /// let response = client.support_bundle_download()
    ///    .bundle_id(bundle_id)
    ///    .range(range)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_download(&self) -> builder::SupportBundleDownload<'_>;
    /// Download support bundle metadata
    ///
    /// Sends a `HEAD` request to
    /// `/v1/system/support-bundles/{bundle_id}/download`
    ///
    /// Arguments:
    /// - `bundle_id`: ID of the support bundle
    /// - `range`: A request to access a portion of the resource, such as
    ///   `bytes=0-499`
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Range>
    /// ```ignore
    /// let response = client.support_bundle_head()
    ///    .bundle_id(bundle_id)
    ///    .range(range)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_head(&self) -> builder::SupportBundleHead<'_>;
    /// Download file from support bundle
    ///
    /// Sends a `GET` request to
    /// `/v1/system/support-bundles/{bundle_id}/download/{file}`
    ///
    /// Arguments:
    /// - `bundle_id`: ID of the support bundle
    /// - `file`: The file within the bundle to download
    /// - `range`: A request to access a portion of the resource, such as
    ///   `bytes=0-499`
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Range>
    /// ```ignore
    /// let response = client.support_bundle_download_file()
    ///    .bundle_id(bundle_id)
    ///    .file(file)
    ///    .range(range)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_download_file(&self) -> builder::SupportBundleDownloadFile<'_>;
    /// Download metadata of file in support bundle
    ///
    /// Sends a `HEAD` request to
    /// `/v1/system/support-bundles/{bundle_id}/download/{file}`
    ///
    /// Arguments:
    /// - `bundle_id`: ID of the support bundle
    /// - `file`: The file within the bundle to download
    /// - `range`: A request to access a portion of the resource, such as
    ///   `bytes=0-499`
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Range>
    /// ```ignore
    /// let response = client.support_bundle_head_file()
    ///    .bundle_id(bundle_id)
    ///    .file(file)
    ///    .range(range)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_head_file(&self) -> builder::SupportBundleHeadFile<'_>;
    /// Download support bundle index
    ///
    /// Sends a `GET` request to `/v1/system/support-bundles/{bundle_id}/index`
    ///
    /// Arguments:
    /// - `bundle_id`: ID of the support bundle
    /// - `range`: A request to access a portion of the resource, such as
    ///   `bytes=0-499`
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Range>
    /// ```ignore
    /// let response = client.support_bundle_index()
    ///    .bundle_id(bundle_id)
    ///    .range(range)
    ///    .send()
    ///    .await;
    /// ```
    fn support_bundle_index(&self) -> builder::SupportBundleIndex<'_>;
}

impl ClientSystemSupportBundlesExt for Client {
    fn support_bundle_list(&self) -> builder::SupportBundleList<'_> {
        builder::SupportBundleList::new(self)
    }

    fn support_bundle_create(&self) -> builder::SupportBundleCreate<'_> {
        builder::SupportBundleCreate::new(self)
    }

    fn support_bundle_view(&self) -> builder::SupportBundleView<'_> {
        builder::SupportBundleView::new(self)
    }

    fn support_bundle_update(&self) -> builder::SupportBundleUpdate<'_> {
        builder::SupportBundleUpdate::new(self)
    }

    fn support_bundle_delete(&self) -> builder::SupportBundleDelete<'_> {
        builder::SupportBundleDelete::new(self)
    }

    fn support_bundle_download(&self) -> builder::SupportBundleDownload<'_> {
        builder::SupportBundleDownload::new(self)
    }

    fn support_bundle_head(&self) -> builder::SupportBundleHead<'_> {
        builder::SupportBundleHead::new(self)
    }

    fn support_bundle_download_file(&self) -> builder::SupportBundleDownloadFile<'_> {
        builder::SupportBundleDownloadFile::new(self)
    }

    fn support_bundle_head_file(&self) -> builder::SupportBundleHeadFile<'_> {
        builder::SupportBundleHeadFile::new(self)
    }

    fn support_bundle_index(&self) -> builder::SupportBundleIndex<'_> {
        builder::SupportBundleIndex::new(self)
    }
}

/// Upload and manage system updates
pub trait ClientSystemUpdateExt {
    /// Clear system recovery status
    ///
    /// Instructs the system that a system recovery operation ("mupdate") was
    /// completed using the software in the specified release.
    ///
    /// The system recovery operation is used to bypass the control plane to
    /// deploy known-working software when the control plane itself is not
    /// functioning or otherwise unable to update itself.  When the control
    /// plane detects this, it stops making any changes to deployed software to
    /// avoid reverting the recovery itself.  This operation puts the control
    /// plane back in charge of determining what software should be deployed,
    /// instructing it that the specified software (which is also what's
    /// currently running) is what's supposed to be deployed.
    ///
    /// If the control plane knows the version of all running software (e.g., a
    /// single sled was recovered to the same version as the rest of the rack),
    /// requests where the provided version does not match what's currently
    /// running will fail. If the control plane does not know the version of all
    /// running software (e.g., the entire rack was mupdated to a new release),
    /// requests with an incorrect provided version will succeed, but the
    /// control plane will continue to avoid changing deployed software until
    /// this operation is invoked with the correct version.
    ///
    /// This endpoint should only be called at the direction of Oxide support.
    ///
    /// Sends a `PUT` request to `/v1/system/update/recovery-finish`
    ///
    /// ```ignore
    /// let response = client.system_update_recovery_finish()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_update_recovery_finish(&self) -> builder::SystemUpdateRecoveryFinish<'_>;
    /// List all TUF repositories
    ///
    /// Returns a paginated list of all TUF repositories ordered by system
    /// version (newest first by default).
    ///
    /// Sends a `GET` request to `/v1/system/update/repositories`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.system_update_repository_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn system_update_repository_list(&self) -> builder::SystemUpdateRepositoryList<'_>;
    /// Upload system release repository
    ///
    /// System release repositories are verified by the updates trust store.
    ///
    /// Sends a `PUT` request to `/v1/system/update/repositories`
    ///
    /// Arguments:
    /// - `file_name`: The name of the uploaded file.
    /// - `body`
    /// ```ignore
    /// let response = client.system_update_repository_upload()
    ///    .file_name(file_name)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_update_repository_upload(&self) -> builder::SystemUpdateRepositoryUpload<'_>;
    /// Fetch system release repository by version
    ///
    /// Sends a `GET` request to
    /// `/v1/system/update/repositories/{system_version}`
    ///
    /// Arguments:
    /// - `system_version`: The version to get.
    /// ```ignore
    /// let response = client.system_update_repository_view()
    ///    .system_version(system_version)
    ///    .send()
    ///    .await;
    /// ```
    fn system_update_repository_view(&self) -> builder::SystemUpdateRepositoryView<'_>;
    /// Fetch system update status
    ///
    /// Returns information about the current target release and the progress of
    /// system software updates.
    ///
    /// Sends a `GET` request to `/v1/system/update/status`
    ///
    /// ```ignore
    /// let response = client.system_update_status()
    ///    .send()
    ///    .await;
    /// ```
    fn system_update_status(&self) -> builder::SystemUpdateStatus<'_>;
    /// Set target release
    ///
    /// Set the current target release of the rack's system software. The rack
    /// reconfigurator will treat the software specified here as a goal state
    /// for the rack's software, and attempt to asynchronously update to that
    /// release. Use the update status endpoint to view the current target
    /// release.
    ///
    /// Sends a `PUT` request to `/v1/system/update/target-release`
    ///
    /// ```ignore
    /// let response = client.target_release_update()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn target_release_update(&self) -> builder::TargetReleaseUpdate<'_>;
    /// List root roles in the updates trust store
    ///
    /// A root role is a JSON document describing the cryptographic keys that
    /// are trusted to sign system release repositories, as described by The
    /// Update Framework. Uploading a repository requires its metadata to be
    /// signed by keys trusted by the trust store.
    ///
    /// Sends a `GET` request to `/v1/system/update/trust-roots`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.system_update_trust_root_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn system_update_trust_root_list(&self) -> builder::SystemUpdateTrustRootList<'_>;
    /// Add trusted root role to updates trust store
    ///
    /// Sends a `POST` request to `/v1/system/update/trust-roots`
    ///
    /// ```ignore
    /// let response = client.system_update_trust_root_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn system_update_trust_root_create(&self) -> builder::SystemUpdateTrustRootCreate<'_>;
    /// Fetch trusted root role
    ///
    /// Sends a `GET` request to `/v1/system/update/trust-roots/{trust_root_id}`
    ///
    /// Arguments:
    /// - `trust_root_id`: ID of the trust root
    /// ```ignore
    /// let response = client.system_update_trust_root_view()
    ///    .trust_root_id(trust_root_id)
    ///    .send()
    ///    .await;
    /// ```
    fn system_update_trust_root_view(&self) -> builder::SystemUpdateTrustRootView<'_>;
    /// Delete trusted root role
    ///
    /// Note that this method does not currently check for any uploaded system
    /// release repositories that would become untrusted after deleting the root
    /// role.
    ///
    /// Sends a `DELETE` request to
    /// `/v1/system/update/trust-roots/{trust_root_id}`
    ///
    /// Arguments:
    /// - `trust_root_id`: ID of the trust root
    /// ```ignore
    /// let response = client.system_update_trust_root_delete()
    ///    .trust_root_id(trust_root_id)
    ///    .send()
    ///    .await;
    /// ```
    fn system_update_trust_root_delete(&self) -> builder::SystemUpdateTrustRootDelete<'_>;
}

impl ClientSystemUpdateExt for Client {
    fn system_update_recovery_finish(&self) -> builder::SystemUpdateRecoveryFinish<'_> {
        builder::SystemUpdateRecoveryFinish::new(self)
    }

    fn system_update_repository_list(&self) -> builder::SystemUpdateRepositoryList<'_> {
        builder::SystemUpdateRepositoryList::new(self)
    }

    fn system_update_repository_upload(&self) -> builder::SystemUpdateRepositoryUpload<'_> {
        builder::SystemUpdateRepositoryUpload::new(self)
    }

    fn system_update_repository_view(&self) -> builder::SystemUpdateRepositoryView<'_> {
        builder::SystemUpdateRepositoryView::new(self)
    }

    fn system_update_status(&self) -> builder::SystemUpdateStatus<'_> {
        builder::SystemUpdateStatus::new(self)
    }

    fn target_release_update(&self) -> builder::TargetReleaseUpdate<'_> {
        builder::TargetReleaseUpdate::new(self)
    }

    fn system_update_trust_root_list(&self) -> builder::SystemUpdateTrustRootList<'_> {
        builder::SystemUpdateTrustRootList::new(self)
    }

    fn system_update_trust_root_create(&self) -> builder::SystemUpdateTrustRootCreate<'_> {
        builder::SystemUpdateTrustRootCreate::new(self)
    }

    fn system_update_trust_root_view(&self) -> builder::SystemUpdateTrustRootView<'_> {
        builder::SystemUpdateTrustRootView::new(self)
    }

    fn system_update_trust_root_delete(&self) -> builder::SystemUpdateTrustRootDelete<'_> {
        builder::SystemUpdateTrustRootDelete::new(self)
    }
}

/// API clients use device access tokens for authentication.
pub trait ClientTokensExt {
    /// List access tokens
    ///
    /// List device access tokens for the currently authenticated user.
    ///
    /// Sends a `GET` request to `/v1/me/access-tokens`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    /// ```ignore
    /// let response = client.current_user_access_token_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn current_user_access_token_list(&self) -> builder::CurrentUserAccessTokenList<'_>;
    /// Delete access token
    ///
    /// Delete a device access token for the currently authenticated user.
    ///
    /// Sends a `DELETE` request to `/v1/me/access-tokens/{token_id}`
    ///
    /// Arguments:
    /// - `token_id`: ID of the token
    /// ```ignore
    /// let response = client.current_user_access_token_delete()
    ///    .token_id(token_id)
    ///    .send()
    ///    .await;
    /// ```
    fn current_user_access_token_delete(&self) -> builder::CurrentUserAccessTokenDelete<'_>;
}

impl ClientTokensExt for Client {
    fn current_user_access_token_list(&self) -> builder::CurrentUserAccessTokenList<'_> {
        builder::CurrentUserAccessTokenList::new(self)
    }

    fn current_user_access_token_delete(&self) -> builder::CurrentUserAccessTokenDelete<'_> {
        builder::CurrentUserAccessTokenDelete::new(self)
    }
}

/// Virtual Private Clouds (VPCs) provide isolated network environments for
/// managing and deploying services.
pub trait ClientVpcsExt {
    /// List IP addresses attached to internet gateway
    ///
    /// Sends a `GET` request to `/v1/internet-gateway-ip-addresses`
    ///
    /// Arguments:
    /// - `gateway`: Name or ID of the internet gateway
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `sort_by`
    /// - `vpc`: Name or ID of the VPC, only required if `gateway` is provided
    ///   as a `Name`
    /// ```ignore
    /// let response = client.internet_gateway_ip_address_list()
    ///    .gateway(gateway)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_ip_address_list(&self) -> builder::InternetGatewayIpAddressList<'_>;
    /// Attach IP address to internet gateway
    ///
    /// Sends a `POST` request to `/v1/internet-gateway-ip-addresses`
    ///
    /// Arguments:
    /// - `gateway`: Name or ID of the internet gateway
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC, only required if `gateway` is provided
    ///   as a `Name`
    /// - `body`
    /// ```ignore
    /// let response = client.internet_gateway_ip_address_create()
    ///    .gateway(gateway)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_ip_address_create(&self) -> builder::InternetGatewayIpAddressCreate<'_>;
    /// Detach IP address from internet gateway
    ///
    /// Sends a `DELETE` request to
    /// `/v1/internet-gateway-ip-addresses/{address}`
    ///
    /// Arguments:
    /// - `address`: Name or ID of the IP address
    /// - `cascade`: Detach attached IP pools/addresses and delete any routes
    ///   targeting this gateway. Without `cascade`, delete fails if the gateway
    ///   has any IP pools or IP addresses attached.
    /// - `gateway`: Name or ID of the internet gateway
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC, only required if `gateway` is provided
    ///   as a `Name`
    /// ```ignore
    /// let response = client.internet_gateway_ip_address_delete()
    ///    .address(address)
    ///    .cascade(cascade)
    ///    .gateway(gateway)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_ip_address_delete(&self) -> builder::InternetGatewayIpAddressDelete<'_>;
    /// List IP pools attached to internet gateway
    ///
    /// Sends a `GET` request to `/v1/internet-gateway-ip-pools`
    ///
    /// Arguments:
    /// - `gateway`: Name or ID of the internet gateway
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `sort_by`
    /// - `vpc`: Name or ID of the VPC, only required if `gateway` is provided
    ///   as a `Name`
    /// ```ignore
    /// let response = client.internet_gateway_ip_pool_list()
    ///    .gateway(gateway)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_ip_pool_list(&self) -> builder::InternetGatewayIpPoolList<'_>;
    /// Attach IP pool to internet gateway
    ///
    /// Sends a `POST` request to `/v1/internet-gateway-ip-pools`
    ///
    /// Arguments:
    /// - `gateway`: Name or ID of the internet gateway
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC, only required if `gateway` is provided
    ///   as a `Name`
    /// - `body`
    /// ```ignore
    /// let response = client.internet_gateway_ip_pool_create()
    ///    .gateway(gateway)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_ip_pool_create(&self) -> builder::InternetGatewayIpPoolCreate<'_>;
    /// Detach IP pool from internet gateway
    ///
    /// Sends a `DELETE` request to `/v1/internet-gateway-ip-pools/{pool}`
    ///
    /// Arguments:
    /// - `pool`: Name or ID of the IP pool
    /// - `cascade`: Detach attached IP pools/addresses and delete any routes
    ///   targeting this gateway. Without `cascade`, delete fails if the gateway
    ///   has any IP pools or IP addresses attached.
    /// - `gateway`: Name or ID of the internet gateway
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC, only required if `gateway` is provided
    ///   as a `Name`
    /// ```ignore
    /// let response = client.internet_gateway_ip_pool_delete()
    ///    .pool(pool)
    ///    .cascade(cascade)
    ///    .gateway(gateway)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_ip_pool_delete(&self) -> builder::InternetGatewayIpPoolDelete<'_>;
    /// List internet gateways
    ///
    /// Sends a `GET` request to `/v1/internet-gateways`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `sort_by`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.internet_gateway_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_list(&self) -> builder::InternetGatewayList<'_>;
    /// Create VPC internet gateway
    ///
    /// Sends a `POST` request to `/v1/internet-gateways`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// - `body`
    /// ```ignore
    /// let response = client.internet_gateway_create()
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_create(&self) -> builder::InternetGatewayCreate<'_>;
    /// Fetch internet gateway
    ///
    /// Sends a `GET` request to `/v1/internet-gateways/{gateway}`
    ///
    /// Arguments:
    /// - `gateway`: Name or ID of the gateway
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.internet_gateway_view()
    ///    .gateway(gateway)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_view(&self) -> builder::InternetGatewayView<'_>;
    /// Delete internet gateway
    ///
    /// Sends a `DELETE` request to `/v1/internet-gateways/{gateway}`
    ///
    /// Arguments:
    /// - `gateway`: Name or ID of the gateway
    /// - `cascade`: Detach attached IP pools/addresses and delete any routes
    ///   targeting this gateway. Without `cascade`, delete fails if the gateway
    ///   has any IP pools or IP addresses attached.
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.internet_gateway_delete()
    ///    .gateway(gateway)
    ///    .cascade(cascade)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn internet_gateway_delete(&self) -> builder::InternetGatewayDelete<'_>;
    /// List firewall rules
    ///
    /// Sends a `GET` request to `/v1/vpc-firewall-rules`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.vpc_firewall_rules_view()
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_firewall_rules_view(&self) -> builder::VpcFirewallRulesView<'_>;
    /// Replace firewall rules
    ///
    /// The maximum number of rules per VPC is 1024.
    ///
    /// Targets are used to specify the set of instances to which a firewall
    /// rule applies. You can target instances directly by name, or specify a
    /// VPC, VPC subnet, IP, or IP subnet, which will apply the rule to traffic
    /// going to all matching instances. Targets are additive: the rule applies
    /// to instances matching ANY target. The maximum number of targets is 256.
    ///
    /// Filters reduce the scope of a firewall rule. Without filters, the rule
    /// applies to all packets to the targets (or from the targets, if it's an
    /// outbound rule). With multiple filters, the rule applies only to packets
    /// matching ALL filters. The maximum number of each type of filter is 256.
    ///
    /// Sends a `PUT` request to `/v1/vpc-firewall-rules`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// - `body`
    /// ```ignore
    /// let response = client.vpc_firewall_rules_update()
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_firewall_rules_update(&self) -> builder::VpcFirewallRulesUpdate<'_>;
    /// List routes
    ///
    /// List the routes associated with a router in a particular VPC.
    ///
    /// Sends a `GET` request to `/v1/vpc-router-routes`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `router`: Name or ID of the router
    /// - `sort_by`
    /// - `vpc`: Name or ID of the VPC, only required if `router` is provided as
    ///   a `Name`
    /// ```ignore
    /// let response = client.vpc_router_route_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .router(router)
    ///    .sort_by(sort_by)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_route_list(&self) -> builder::VpcRouterRouteList<'_>;
    /// Create route
    ///
    /// Sends a `POST` request to `/v1/vpc-router-routes`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `router`: Name or ID of the router
    /// - `vpc`: Name or ID of the VPC, only required if `router` is provided as
    ///   a `Name`
    /// - `body`
    /// ```ignore
    /// let response = client.vpc_router_route_create()
    ///    .project(project)
    ///    .router(router)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_route_create(&self) -> builder::VpcRouterRouteCreate<'_>;
    /// Fetch route
    ///
    /// Sends a `GET` request to `/v1/vpc-router-routes/{route}`
    ///
    /// Arguments:
    /// - `route`: Name or ID of the route
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `router`: Name or ID of the router
    /// - `vpc`: Name or ID of the VPC, only required if `router` is provided as
    ///   a `Name`
    /// ```ignore
    /// let response = client.vpc_router_route_view()
    ///    .route(route)
    ///    .project(project)
    ///    .router(router)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_route_view(&self) -> builder::VpcRouterRouteView<'_>;
    /// Update route
    ///
    /// Sends a `PUT` request to `/v1/vpc-router-routes/{route}`
    ///
    /// Arguments:
    /// - `route`: Name or ID of the route
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `router`: Name or ID of the router
    /// - `vpc`: Name or ID of the VPC, only required if `router` is provided as
    ///   a `Name`
    /// - `body`
    /// ```ignore
    /// let response = client.vpc_router_route_update()
    ///    .route(route)
    ///    .project(project)
    ///    .router(router)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_route_update(&self) -> builder::VpcRouterRouteUpdate<'_>;
    /// Delete route
    ///
    /// Sends a `DELETE` request to `/v1/vpc-router-routes/{route}`
    ///
    /// Arguments:
    /// - `route`: Name or ID of the route
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `router`: Name or ID of the router
    /// - `vpc`: Name or ID of the VPC, only required if `router` is provided as
    ///   a `Name`
    /// ```ignore
    /// let response = client.vpc_router_route_delete()
    ///    .route(route)
    ///    .project(project)
    ///    .router(router)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_route_delete(&self) -> builder::VpcRouterRouteDelete<'_>;
    /// List routers
    ///
    /// Sends a `GET` request to `/v1/vpc-routers`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `sort_by`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.vpc_router_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_list(&self) -> builder::VpcRouterList<'_>;
    /// Create VPC router
    ///
    /// Sends a `POST` request to `/v1/vpc-routers`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// - `body`
    /// ```ignore
    /// let response = client.vpc_router_create()
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_create(&self) -> builder::VpcRouterCreate<'_>;
    /// Fetch router
    ///
    /// Sends a `GET` request to `/v1/vpc-routers/{router}`
    ///
    /// Arguments:
    /// - `router`: Name or ID of the router
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.vpc_router_view()
    ///    .router(router)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_view(&self) -> builder::VpcRouterView<'_>;
    /// Update router
    ///
    /// Sends a `PUT` request to `/v1/vpc-routers/{router}`
    ///
    /// Arguments:
    /// - `router`: Name or ID of the router
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// - `body`
    /// ```ignore
    /// let response = client.vpc_router_update()
    ///    .router(router)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_update(&self) -> builder::VpcRouterUpdate<'_>;
    /// Delete router
    ///
    /// Sends a `DELETE` request to `/v1/vpc-routers/{router}`
    ///
    /// Arguments:
    /// - `router`: Name or ID of the router
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.vpc_router_delete()
    ///    .router(router)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_router_delete(&self) -> builder::VpcRouterDelete<'_>;
    /// List subnets
    ///
    /// Sends a `GET` request to `/v1/vpc-subnets`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `sort_by`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.vpc_subnet_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_subnet_list(&self) -> builder::VpcSubnetList<'_>;
    /// Create subnet
    ///
    /// Sends a `POST` request to `/v1/vpc-subnets`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// - `body`
    /// ```ignore
    /// let response = client.vpc_subnet_create()
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_subnet_create(&self) -> builder::VpcSubnetCreate<'_>;
    /// Fetch subnet
    ///
    /// Sends a `GET` request to `/v1/vpc-subnets/{subnet}`
    ///
    /// Arguments:
    /// - `subnet`: Name or ID of the subnet
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.vpc_subnet_view()
    ///    .subnet(subnet)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_subnet_view(&self) -> builder::VpcSubnetView<'_>;
    /// Update subnet
    ///
    /// Sends a `PUT` request to `/v1/vpc-subnets/{subnet}`
    ///
    /// Arguments:
    /// - `subnet`: Name or ID of the subnet
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// - `body`
    /// ```ignore
    /// let response = client.vpc_subnet_update()
    ///    .subnet(subnet)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_subnet_update(&self) -> builder::VpcSubnetUpdate<'_>;
    /// Delete subnet
    ///
    /// Sends a `DELETE` request to `/v1/vpc-subnets/{subnet}`
    ///
    /// Arguments:
    /// - `subnet`: Name or ID of the subnet
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.vpc_subnet_delete()
    ///    .subnet(subnet)
    ///    .project(project)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_subnet_delete(&self) -> builder::VpcSubnetDelete<'_>;
    /// List network interfaces
    ///
    /// Sends a `GET` request to `/v1/vpc-subnets/{subnet}/network-interfaces`
    ///
    /// Arguments:
    /// - `subnet`: Name or ID of the subnet
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project, only required if `vpc` is
    ///   provided as a `Name`
    /// - `sort_by`
    /// - `vpc`: Name or ID of the VPC
    /// ```ignore
    /// let response = client.vpc_subnet_list_network_interfaces()
    ///    .subnet(subnet)
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .vpc(vpc)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_subnet_list_network_interfaces(&self) -> builder::VpcSubnetListNetworkInterfaces<'_>;
    /// List VPCs
    ///
    /// Sends a `GET` request to `/v1/vpcs`
    ///
    /// Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`: Name or ID of the project
    /// - `sort_by`
    /// ```ignore
    /// let response = client.vpc_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .project(project)
    ///    .sort_by(sort_by)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_list(&self) -> builder::VpcList<'_>;
    /// Create VPC
    ///
    /// Sends a `POST` request to `/v1/vpcs`
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.vpc_create()
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_create(&self) -> builder::VpcCreate<'_>;
    /// Fetch VPC
    ///
    /// Sends a `GET` request to `/v1/vpcs/{vpc}`
    ///
    /// Arguments:
    /// - `vpc`: Name or ID of the VPC
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.vpc_view()
    ///    .vpc(vpc)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_view(&self) -> builder::VpcView<'_>;
    /// Update VPC
    ///
    /// Sends a `PUT` request to `/v1/vpcs/{vpc}`
    ///
    /// Arguments:
    /// - `vpc`: Name or ID of the VPC
    /// - `project`: Name or ID of the project
    /// - `body`
    /// ```ignore
    /// let response = client.vpc_update()
    ///    .vpc(vpc)
    ///    .project(project)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_update(&self) -> builder::VpcUpdate<'_>;
    /// Delete VPC
    ///
    /// Sends a `DELETE` request to `/v1/vpcs/{vpc}`
    ///
    /// Arguments:
    /// - `vpc`: Name or ID of the VPC
    /// - `project`: Name or ID of the project
    /// ```ignore
    /// let response = client.vpc_delete()
    ///    .vpc(vpc)
    ///    .project(project)
    ///    .send()
    ///    .await;
    /// ```
    fn vpc_delete(&self) -> builder::VpcDelete<'_>;
}

impl ClientVpcsExt for Client {
    fn internet_gateway_ip_address_list(&self) -> builder::InternetGatewayIpAddressList<'_> {
        builder::InternetGatewayIpAddressList::new(self)
    }

    fn internet_gateway_ip_address_create(&self) -> builder::InternetGatewayIpAddressCreate<'_> {
        builder::InternetGatewayIpAddressCreate::new(self)
    }

    fn internet_gateway_ip_address_delete(&self) -> builder::InternetGatewayIpAddressDelete<'_> {
        builder::InternetGatewayIpAddressDelete::new(self)
    }

    fn internet_gateway_ip_pool_list(&self) -> builder::InternetGatewayIpPoolList<'_> {
        builder::InternetGatewayIpPoolList::new(self)
    }

    fn internet_gateway_ip_pool_create(&self) -> builder::InternetGatewayIpPoolCreate<'_> {
        builder::InternetGatewayIpPoolCreate::new(self)
    }

    fn internet_gateway_ip_pool_delete(&self) -> builder::InternetGatewayIpPoolDelete<'_> {
        builder::InternetGatewayIpPoolDelete::new(self)
    }

    fn internet_gateway_list(&self) -> builder::InternetGatewayList<'_> {
        builder::InternetGatewayList::new(self)
    }

    fn internet_gateway_create(&self) -> builder::InternetGatewayCreate<'_> {
        builder::InternetGatewayCreate::new(self)
    }

    fn internet_gateway_view(&self) -> builder::InternetGatewayView<'_> {
        builder::InternetGatewayView::new(self)
    }

    fn internet_gateway_delete(&self) -> builder::InternetGatewayDelete<'_> {
        builder::InternetGatewayDelete::new(self)
    }

    fn vpc_firewall_rules_view(&self) -> builder::VpcFirewallRulesView<'_> {
        builder::VpcFirewallRulesView::new(self)
    }

    fn vpc_firewall_rules_update(&self) -> builder::VpcFirewallRulesUpdate<'_> {
        builder::VpcFirewallRulesUpdate::new(self)
    }

    fn vpc_router_route_list(&self) -> builder::VpcRouterRouteList<'_> {
        builder::VpcRouterRouteList::new(self)
    }

    fn vpc_router_route_create(&self) -> builder::VpcRouterRouteCreate<'_> {
        builder::VpcRouterRouteCreate::new(self)
    }

    fn vpc_router_route_view(&self) -> builder::VpcRouterRouteView<'_> {
        builder::VpcRouterRouteView::new(self)
    }

    fn vpc_router_route_update(&self) -> builder::VpcRouterRouteUpdate<'_> {
        builder::VpcRouterRouteUpdate::new(self)
    }

    fn vpc_router_route_delete(&self) -> builder::VpcRouterRouteDelete<'_> {
        builder::VpcRouterRouteDelete::new(self)
    }

    fn vpc_router_list(&self) -> builder::VpcRouterList<'_> {
        builder::VpcRouterList::new(self)
    }

    fn vpc_router_create(&self) -> builder::VpcRouterCreate<'_> {
        builder::VpcRouterCreate::new(self)
    }

    fn vpc_router_view(&self) -> builder::VpcRouterView<'_> {
        builder::VpcRouterView::new(self)
    }

    fn vpc_router_update(&self) -> builder::VpcRouterUpdate<'_> {
        builder::VpcRouterUpdate::new(self)
    }

    fn vpc_router_delete(&self) -> builder::VpcRouterDelete<'_> {
        builder::VpcRouterDelete::new(self)
    }

    fn vpc_subnet_list(&self) -> builder::VpcSubnetList<'_> {
        builder::VpcSubnetList::new(self)
    }

    fn vpc_subnet_create(&self) -> builder::VpcSubnetCreate<'_> {
        builder::VpcSubnetCreate::new(self)
    }

    fn vpc_subnet_view(&self) -> builder::VpcSubnetView<'_> {
        builder::VpcSubnetView::new(self)
    }

    fn vpc_subnet_update(&self) -> builder::VpcSubnetUpdate<'_> {
        builder::VpcSubnetUpdate::new(self)
    }

    fn vpc_subnet_delete(&self) -> builder::VpcSubnetDelete<'_> {
        builder::VpcSubnetDelete::new(self)
    }

    fn vpc_subnet_list_network_interfaces(&self) -> builder::VpcSubnetListNetworkInterfaces<'_> {
        builder::VpcSubnetListNetworkInterfaces::new(self)
    }

    fn vpc_list(&self) -> builder::VpcList<'_> {
        builder::VpcList::new(self)
    }

    fn vpc_create(&self) -> builder::VpcCreate<'_> {
        builder::VpcCreate::new(self)
    }

    fn vpc_view(&self) -> builder::VpcView<'_> {
        builder::VpcView::new(self)
    }

    fn vpc_update(&self) -> builder::VpcUpdate<'_> {
        builder::VpcUpdate::new(self)
    }

    fn vpc_delete(&self) -> builder::VpcDelete<'_> {
        builder::VpcDelete::new(self)
    }
}
