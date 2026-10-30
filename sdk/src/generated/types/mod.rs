// The contents of this file are generated; do not modify them.

/// Types for composing complex structures.
pub mod builder;
/// Generation of default values for serde.
pub mod defaults;
/// Error types.
pub mod error;
/// An address tied to an address lot.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Address {
    /// The address and prefix length of this address.
    pub address: IpNet,
    /// The address lot this address is drawn from.
    pub address_lot: NameOrId,
    /// Optional VLAN ID for this address
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vlan_id: ::std::option::Option<u16>,
}

impl Address {
    pub fn builder() -> builder::Address {
        Default::default()
    }
}

/// Specify how to allocate a floating IP address.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum AddressAllocator {
    /// Reserve a specific IP address. The pool is inferred from the address
    /// since IP pools cannot have overlapping ranges.
    #[serde(rename = "explicit")]
    Explicit {
        /// The IP address to reserve.
        ip: ::std::net::IpAddr,
    },
    /// Automatically allocate an IP address from a pool.
    #[serde(rename = "auto")]
    Auto {
        /// Pool selection.
        ///
        /// If omitted, the silo's default pool is used. If the silo has default
        /// pools for both IPv4 and IPv6, the request will fail unless
        /// `ip_version` is specified.
        #[serde(default = "defaults::address_allocator_auto_pool_selector")]
        pool_selector: PoolSelector,
    },
}

/// A set of addresses associated with a port configuration.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AddressConfig {
    /// The set of addresses assigned to the port configuration.
    pub addresses: ::std::vec::Vec<Address>,
    /// Link to assign the addresses to. On ports that are not broken out, this
    /// is always phy0. On a 2x breakout the options are phy0 and phy1, on 4x
    /// phy0-phy3, etc.
    pub link_name: Name,
}

impl AddressConfig {
    pub fn builder() -> builder::AddressConfig {
        Default::default()
    }
}

/// Represents an address lot object, containing the id of the lot that can be
/// used in other API calls.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AddressLot {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Desired use of `AddressLot`
    pub kind: AddressLotKind,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl AddressLot {
    pub fn builder() -> builder::AddressLot {
        Default::default()
    }
}

/// An address lot block is a part of an address lot and contains a range of
/// addresses. The range is inclusive.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AddressLotBlock {
    /// The first address of the block (inclusive).
    pub first_address: ::std::net::IpAddr,
    /// The id of the address lot block.
    pub id: ::uuid::Uuid,
    /// The last address of the block (inclusive).
    pub last_address: ::std::net::IpAddr,
}

impl AddressLotBlock {
    pub fn builder() -> builder::AddressLotBlock {
        Default::default()
    }
}

/// Parameters for creating an address lot block. First and last addresses are
/// inclusive.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AddressLotBlockCreate {
    /// The first address in the lot (inclusive).
    pub first_address: ::std::net::IpAddr,
    /// The last address in the lot (inclusive).
    pub last_address: ::std::net::IpAddr,
}

impl AddressLotBlockCreate {
    pub fn builder() -> builder::AddressLotBlockCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AddressLotBlockResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AddressLotBlock>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AddressLotBlockResultsPage {
    pub fn builder() -> builder::AddressLotBlockResultsPage {
        Default::default()
    }
}

/// Parameters for creating an address lot.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AddressLotCreate {
    /// The blocks to add along with the new address lot.
    pub blocks: ::std::vec::Vec<AddressLotBlockCreate>,
    pub description: ::std::string::String,
    /// The kind of address lot to create.
    pub kind: AddressLotKind,
    pub name: Name,
}

impl AddressLotCreate {
    pub fn builder() -> builder::AddressLotCreate {
        Default::default()
    }
}

/// An address lot and associated blocks resulting from creating an address lot.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AddressLotCreateResponse {
    /// The address lot blocks that were created.
    pub blocks: ::std::vec::Vec<AddressLotBlock>,
    /// The address lot that was created.
    pub lot: AddressLot,
}

impl AddressLotCreateResponse {
    pub fn builder() -> builder::AddressLotCreateResponse {
        Default::default()
    }
}

/// The kind associated with an address lot.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum AddressLotKind {
    /// Infrastructure address lots are used for network infrastructure like
    /// addresses assigned to rack switches.
    #[serde(rename = "infra")]
    Infra,
    /// Pool address lots are used by IP pools.
    #[serde(rename = "pool")]
    Pool,
}

impl ::std::fmt::Display for AddressLotKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Infra => f.write_str("infra"),
            Self::Pool => f.write_str("pool"),
        }
    }
}

impl ::std::str::FromStr for AddressLotKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "infra" => Ok(Self::Infra),
            "pool" => Ok(Self::Pool),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for AddressLotKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for AddressLotKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AddressLotResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AddressLot>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AddressLotResultsPage {
    pub fn builder() -> builder::AddressLotResultsPage {
        Default::default()
    }
}

/// An address lot and associated blocks resulting from viewing an address lot.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AddressLotViewResponse {
    /// The address lot blocks.
    pub blocks: ::std::vec::Vec<AddressLotBlock>,
    /// The address lot.
    pub lot: AddressLot,
}

impl AddressLotViewResponse {
    pub fn builder() -> builder::AddressLotViewResponse {
        Default::default()
    }
}

/// View of an Affinity Group
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AffinityGroup {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    pub failure_domain: FailureDomain,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    pub policy: AffinityPolicy,
    pub project_id: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl AffinityGroup {
    pub fn builder() -> builder::AffinityGroup {
        Default::default()
    }
}

/// Create-time parameters for an `AffinityGroup`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AffinityGroupCreate {
    pub description: ::std::string::String,
    pub failure_domain: FailureDomain,
    pub name: Name,
    pub policy: AffinityPolicy,
}

impl AffinityGroupCreate {
    pub fn builder() -> builder::AffinityGroupCreate {
        Default::default()
    }
}

/// A member of an Affinity Group
///
/// Membership in a group is not exclusive - members may belong to multiple
/// affinity / anti-affinity groups.
///
/// Affinity Groups can contain up to 32 members.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum AffinityGroupMember {
    /// An instance belonging to this group
    ///
    /// Instances can belong to up to 16 affinity groups.
    #[serde(rename = "instance")]
    Instance {
        id: ::uuid::Uuid,
        name: Name,
        run_state: InstanceState,
    },
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AffinityGroupMemberResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AffinityGroupMember>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AffinityGroupMemberResultsPage {
    pub fn builder() -> builder::AffinityGroupMemberResultsPage {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AffinityGroupResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AffinityGroup>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AffinityGroupResultsPage {
    pub fn builder() -> builder::AffinityGroupResultsPage {
        Default::default()
    }
}

/// Updateable properties of an `AffinityGroup`
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct AffinityGroupUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl AffinityGroupUpdate {
    pub fn builder() -> builder::AffinityGroupUpdate {
        Default::default()
    }
}

/// Affinity policy used to describe "what to do when a request cannot be
/// satisfied"
///
/// Used for both Affinity and Anti-Affinity Groups
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum AffinityPolicy {
    /// Best-effort: instances can start even when the group's constraints
    /// cannot be satisfied.
    #[serde(rename = "allow")]
    Allow,
    /// When the group's constraints cannot be satisfied, an instance start
    /// request will fail and the instance will remain stopped. Starting the
    /// instance may succeed later if conditions change, e.g., other instances
    /// stop.
    #[serde(rename = "fail")]
    Fail,
}

impl ::std::fmt::Display for AffinityPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allow => f.write_str("allow"),
            Self::Fail => f.write_str("fail"),
        }
    }
}

impl ::std::str::FromStr for AffinityPolicy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow" => Ok(Self::Allow),
            "fail" => Ok(Self::Fail),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for AffinityPolicy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for AffinityPolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// BGP message history for rack switches.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AggregateBgpMessageHistory {
    /// BGP history organized by switch.
    pub switch_histories: ::std::vec::Vec<SwitchBgpHistory>,
}

impl AggregateBgpMessageHistory {
    pub fn builder() -> builder::AggregateBgpMessageHistory {
        Default::default()
    }
}

/// An alert.
///
/// Alerts provide notifications about events that occurred in the system at a
/// point in time. See the guide-level documentation on alerts for details.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Alert {
    /// The alert's class.
    ///
    /// See the guide-level documentation on alerts for details on alert
    /// classes.
    pub class: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The alert's data payload.
    ///
    /// The schema for this object depends on the alert class and version.
    pub payload: ::serde_json::Value,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The schema version of this alert's data payload.
    ///
    /// Alert schemas are versioned on a per-alert-class basis. The schema
    /// version for a particular alert class does not correspond to an Oxide API
    /// version. Clients should expect to encounter earlier schema versions when
    /// retrieving alerts recorded by an earlier version of the system software.
    ///
    /// See the guide-level documentation on alerts for details.
    pub version: u32,
}

impl Alert {
    pub fn builder() -> builder::Alert {
        Default::default()
    }
}

/// An alert class.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertClass {
    /// A description of what this alert class represents.
    pub description: ::std::string::String,
    /// The name of the alert class.
    pub name: ::std::string::String,
}

impl AlertClass {
    pub fn builder() -> builder::AlertClass {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertClassResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AlertClass>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AlertClassResultsPage {
    pub fn builder() -> builder::AlertClassResultsPage {
        Default::default()
    }
}

/// A delivery of a webhook event.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertDelivery {
    /// The event class.
    pub alert_class: ::std::string::String,
    /// The UUID of the event.
    pub alert_id: ::uuid::Uuid,
    /// Individual attempts to deliver this webhook event, and their outcomes.
    pub attempts: AlertDeliveryAttempts,
    /// The UUID of this delivery attempt.
    pub id: ::uuid::Uuid,
    /// The UUID of the alert receiver that this event was delivered to.
    pub receiver_id: ::uuid::Uuid,
    /// The state of this delivery.
    pub state: AlertDeliveryState,
    /// The time at which this delivery began (i.e. the event was dispatched to
    /// the receiver).
    pub time_started: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Why this delivery was performed.
    pub trigger: AlertDeliveryTrigger,
}

impl AlertDelivery {
    pub fn builder() -> builder::AlertDelivery {
        Default::default()
    }
}

/// A list of attempts to deliver an alert to a receiver.
///
/// The type of the delivery attempt model depends on the receiver type, as it
/// may contain information specific to that delivery mechanism. For example,
/// webhook delivery attempts contain the HTTP status code of the webhook
/// request.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub enum AlertDeliveryAttempts {
    /// A list of attempts to deliver an alert to a webhook receiver.
    #[serde(rename = "webhook")]
    Webhook(::std::vec::Vec<WebhookDeliveryAttempt>),
}

impl ::std::convert::From<::std::vec::Vec<WebhookDeliveryAttempt>> for AlertDeliveryAttempts {
    fn from(value: ::std::vec::Vec<WebhookDeliveryAttempt>) -> Self {
        Self::Webhook(value)
    }
}

/// `AlertDeliveryId`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertDeliveryId {
    pub delivery_id: ::uuid::Uuid,
}

impl AlertDeliveryId {
    pub fn builder() -> builder::AlertDeliveryId {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertDeliveryResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AlertDelivery>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AlertDeliveryResultsPage {
    pub fn builder() -> builder::AlertDeliveryResultsPage {
        Default::default()
    }
}

/// The state of a webhook delivery attempt.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum AlertDeliveryState {
    /// The webhook event has not yet been delivered successfully.
    ///
    /// Either no delivery attempts have yet been performed, or the delivery has
    /// failed at least once but has retries remaining.
    #[serde(rename = "pending")]
    Pending,
    /// The webhook event has been delivered successfully.
    #[serde(rename = "delivered")]
    Delivered,
    /// The webhook delivery attempt has failed permanently and will not be
    /// retried again.
    #[serde(rename = "failed")]
    Failed,
}

impl ::std::fmt::Display for AlertDeliveryState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
            Self::Delivered => f.write_str("delivered"),
            Self::Failed => f.write_str("failed"),
        }
    }
}

impl ::std::str::FromStr for AlertDeliveryState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            "delivered" => Ok(Self::Delivered),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for AlertDeliveryState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for AlertDeliveryState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// The reason an alert was delivered
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum AlertDeliveryTrigger {
    /// Delivery was triggered by the alert itself.
    #[serde(rename = "alert")]
    Alert,
    /// Delivery was triggered by a request to resend the alert.
    #[serde(rename = "resend")]
    Resend,
    /// This delivery is a liveness probe.
    #[serde(rename = "probe")]
    Probe,
}

impl ::std::fmt::Display for AlertDeliveryTrigger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Alert => f.write_str("alert"),
            Self::Resend => f.write_str("resend"),
            Self::Probe => f.write_str("probe"),
        }
    }
}

impl ::std::str::FromStr for AlertDeliveryTrigger {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "alert" => Ok(Self::Alert),
            "resend" => Ok(Self::Resend),
            "probe" => Ok(Self::Probe),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for AlertDeliveryTrigger {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for AlertDeliveryTrigger {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Data describing the result of an alert receiver liveness probe attempt.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertProbeResult {
    /// The outcome of the probe delivery.
    pub probe: AlertDelivery,
    /// If the probe request succeeded, and resending failed deliveries on
    /// success was requested, the number of new delivery attempts started.
    /// Otherwise, if the probe did not succeed, or resending failed deliveries
    /// was not requested, this is null.
    ///
    /// Note that this may be 0, if there were no events found which had not
    /// been delivered successfully to this receiver.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resends_started: ::std::option::Option<u32>,
}

impl AlertProbeResult {
    pub fn builder() -> builder::AlertProbeResult {
        Default::default()
    }
}

/// The configuration for an alert receiver.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertReceiver {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Configuration specific to the kind of alert receiver that this is.
    pub kind: AlertReceiverKind,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// The list of alert classes to which this receiver is subscribed.
    pub subscriptions: ::std::vec::Vec<AlertSubscription>,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl AlertReceiver {
    pub fn builder() -> builder::AlertReceiver {
        Default::default()
    }
}

/// The possible alert delivery mechanisms for an alert receiver.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "kind")]
pub enum AlertReceiverKind {
    /// Webhook-specific alert receiver configuration.
    #[serde(rename = "webhook")]
    Webhook {
        /// The URL that webhook notification requests are sent to.
        endpoint: ::std::string::String,
        /// A list containing the IDs of the secret keys used to sign payloads
        /// sent to this receiver.
        secrets: ::std::vec::Vec<WebhookSecret>,
    },
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertReceiverResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AlertReceiver>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AlertReceiverResultsPage {
    pub fn builder() -> builder::AlertReceiverResultsPage {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Alert>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AlertResultsPage {
    pub fn builder() -> builder::AlertResultsPage {
        Default::default()
    }
}

/// A webhook event class subscription matches either a single event class
/// exactly, or a glob pattern including wildcards that may match multiple event
/// classes
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct AlertSubscription(::std::string::String);
impl ::std::ops::Deref for AlertSubscription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<AlertSubscription> for ::std::string::String {
    fn from(value: AlertSubscription) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for AlertSubscription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for AlertSubscription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^([a-zA-Z0-9_]+|\\*|\\*\\*)(\\.([a-zA-Z0-9_]+|\\*|\\*\\*))*$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \
                        \"^([a-zA-Z0-9_]+|\\*|\\*\\*)(\\.([a-zA-Z0-9_]+|\\*|\\*\\*))*$\""
                .into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for AlertSubscription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AlertSubscription {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// `AlertSubscriptionCreate`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertSubscriptionCreate {
    /// The event class pattern to subscribe to.
    pub subscription: AlertSubscription,
}

impl AlertSubscriptionCreate {
    pub fn builder() -> builder::AlertSubscriptionCreate {
        Default::default()
    }
}

/// `AlertSubscriptionCreated`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AlertSubscriptionCreated {
    /// The new subscription added to the receiver.
    pub subscription: AlertSubscription,
}

impl AlertSubscriptionCreated {
    pub fn builder() -> builder::AlertSubscriptionCreated {
        Default::default()
    }
}

/// Allowlist of IPs or subnets that can make requests to user-facing services.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AllowList {
    /// The allowlist of IPs or subnets.
    pub allowed_ips: AllowedSourceIps,
    /// Time the list was created.
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Time the list was last modified.
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl AllowList {
    pub fn builder() -> builder::AllowList {
        Default::default()
    }
}

/// Parameters for updating allowed source IPs
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AllowListUpdate {
    /// The new list of allowed source IPs.
    pub allowed_ips: AllowedSourceIps,
}

impl AllowListUpdate {
    pub fn builder() -> builder::AllowListUpdate {
        Default::default()
    }
}

/// Description of source IPs allowed to reach rack services.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "allow", content = "ips")]
pub enum AllowedSourceIps {
    #[serde(rename = "any")]
    Any,
    /// Restrict access to a specific set of source IP addresses or subnets.
    ///
    /// All others are prevented from reaching rack services.
    #[serde(rename = "list")]
    List(::std::vec::Vec<IpNet>),
}

impl ::std::convert::From<::std::vec::Vec<IpNet>> for AllowedSourceIps {
    fn from(value: ::std::vec::Vec<IpNet>) -> Self {
        Self::List(value)
    }
}

/// View of an Anti-Affinity Group
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AntiAffinityGroup {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    pub failure_domain: FailureDomain,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    pub policy: AffinityPolicy,
    pub project_id: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl AntiAffinityGroup {
    pub fn builder() -> builder::AntiAffinityGroup {
        Default::default()
    }
}

/// Create-time parameters for an `AntiAffinityGroup`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AntiAffinityGroupCreate {
    pub description: ::std::string::String,
    pub failure_domain: FailureDomain,
    pub name: Name,
    pub policy: AffinityPolicy,
}

impl AntiAffinityGroupCreate {
    pub fn builder() -> builder::AntiAffinityGroupCreate {
        Default::default()
    }
}

/// A member of an Anti-Affinity Group
///
/// Membership in a group is not exclusive - members may belong to multiple
/// affinity / anti-affinity groups.
///
/// Anti-Affinity Groups can contain up to 32 members.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum AntiAffinityGroupMember {
    /// An instance belonging to this group
    ///
    /// Instances can belong to up to 16 anti-affinity groups.
    #[serde(rename = "instance")]
    Instance {
        id: ::uuid::Uuid,
        name: Name,
        run_state: InstanceState,
    },
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AntiAffinityGroupMemberResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AntiAffinityGroupMember>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AntiAffinityGroupMemberResultsPage {
    pub fn builder() -> builder::AntiAffinityGroupMemberResultsPage {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AntiAffinityGroupResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AntiAffinityGroup>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AntiAffinityGroupResultsPage {
    pub fn builder() -> builder::AntiAffinityGroupResultsPage {
        Default::default()
    }
}

/// Updateable properties of an `AntiAffinityGroup`
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct AntiAffinityGroupUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl AntiAffinityGroupUpdate {
    pub fn builder() -> builder::AntiAffinityGroupUpdate {
        Default::default()
    }
}

/// Audit log entry
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AuditLogEntry {
    pub actor: AuditLogEntryActor,
    /// How the user authenticated the request (access token, session, or SCIM
    /// token). Null for unauthenticated requests like login attempts.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub auth_method: ::std::option::Option<AuthMethod>,
    /// ID of the credential used for authentication. Null for unauthenticated
    /// requests. The value of `auth_method` indicates what kind of credential
    /// it is (access token, session, or SCIM token).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub credential_id: ::std::option::Option<::uuid::Uuid>,
    /// Unique identifier for the audit log entry
    pub id: ::uuid::Uuid,
    /// API endpoint ID, e.g., `project_create`
    pub operation_id: ::std::string::String,
    /// Request ID for tracing requests through the system
    pub request_id: ::std::string::String,
    /// URI of the request, truncated to 512 characters. Will only include host
    /// and scheme for HTTP/2 requests. For HTTP/1.1, the URI will consist of
    /// only the path and query.
    pub request_uri: ::std::string::String,
    /// Result of the operation
    pub result: AuditLogEntryResult,
    /// IP address that made the request
    pub source_ip: ::std::net::IpAddr,
    /// Time operation completed
    pub time_completed: ::chrono::DateTime<::chrono::offset::Utc>,
    /// When the request was received
    pub time_started: ::chrono::DateTime<::chrono::offset::Utc>,
    /// User agent string from the request, truncated to 256 characters.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_agent: ::std::option::Option<::std::string::String>,
}

impl AuditLogEntry {
    pub fn builder() -> builder::AuditLogEntry {
        Default::default()
    }
}

/// `AuditLogEntryActor`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "kind")]
pub enum AuditLogEntryActor {
    #[serde(rename = "user_builtin")]
    UserBuiltin { user_builtin_id: ::uuid::Uuid },
    #[serde(rename = "silo_user")]
    SiloUser {
        silo_id: ::uuid::Uuid,
        silo_user_id: ::uuid::Uuid,
    },
    #[serde(rename = "scim")]
    Scim { silo_id: ::uuid::Uuid },
    #[serde(rename = "unauthenticated")]
    Unauthenticated,
}

/// Result of an audit log entry
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "kind")]
pub enum AuditLogEntryResult {
    /// The operation completed successfully
    #[serde(rename = "success")]
    Success {
        /// HTTP status code
        http_status_code: u16,
    },
    /// The operation failed
    #[serde(rename = "error")]
    Error {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        error_code: ::std::option::Option<::std::string::String>,
        error_message: ::std::string::String,
        /// HTTP status code
        http_status_code: u16,
    },
    #[serde(rename = "unknown")]
    Unknown,
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct AuditLogEntryResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<AuditLogEntry>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl AuditLogEntryResultsPage {
    pub fn builder() -> builder::AuditLogEntryResultsPage {
        Default::default()
    }
}

/// Authentication method used for a request
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum AuthMethod {
    /// Console session cookie
    #[serde(rename = "session_cookie")]
    SessionCookie,
    /// Device access token (OAuth 2.0 device authorization flow)
    #[serde(rename = "access_token")]
    AccessToken,
    /// SCIM client bearer token
    #[serde(rename = "scim_token")]
    ScimToken,
}

impl ::std::fmt::Display for AuthMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCookie => f.write_str("session_cookie"),
            Self::AccessToken => f.write_str("access_token"),
            Self::ScimToken => f.write_str("scim_token"),
        }
    }
}

impl ::std::str::FromStr for AuthMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session_cookie" => Ok(Self::SessionCookie),
            "access_token" => Ok(Self::AccessToken),
            "scim_token" => Ok(Self::ScimToken),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for AuthMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for AuthMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Authorization scope for a timeseries.
///
/// This describes the level at which a user must be authorized to read data
/// from a timeseries. For example, fleet-scoping means the data is only visible
/// to an operator or fleet reader. Project-scoped, on the other hand, indicates
/// that a user will see data limited to the projects on which they have read
/// permissions.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum AuthzScope {
    /// Timeseries data is limited to fleet readers.
    #[serde(rename = "fleet")]
    Fleet,
    /// Timeseries data is limited to the authorized silo for a user.
    #[serde(rename = "silo")]
    Silo,
    /// Timeseries data is limited to the authorized projects for a user.
    #[serde(rename = "project")]
    Project,
    /// The timeseries is viewable to all without limitation.
    #[serde(rename = "viewable_to_all")]
    ViewableToAll,
}

impl ::std::fmt::Display for AuthzScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Fleet => f.write_str("fleet"),
            Self::Silo => f.write_str("silo"),
            Self::Project => f.write_str("project"),
            Self::ViewableToAll => f.write_str("viewable_to_all"),
        }
    }
}

impl ::std::str::FromStr for AuthzScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fleet" => Ok(Self::Fleet),
            "silo" => Ok(Self::Silo),
            "project" => Ok(Self::Project),
            "viewable_to_all" => Ok(Self::ViewableToAll),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for AuthzScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for AuthzScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Properties that uniquely identify an Oxide hardware component
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Baseboard {
    pub part: ::std::string::String,
    pub revision: u32,
    pub serial: ::std::string::String,
}

impl Baseboard {
    pub fn builder() -> builder::Baseboard {
        Default::default()
    }
}

/// A representation of a Baseboard ID as used in the inventory subsystem.
///
/// This type is essentially the same as a `Baseboard` except it doesn't have a
/// revision or HW type (Gimlet, PC, Unknown).
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BaseboardId {
    /// Oxide Part Number
    pub part_number: ::std::string::String,
    /// Serial number (unique for a given part number)
    pub serial_number: ::std::string::String,
}

impl BaseboardId {
    pub fn builder() -> builder::BaseboardId {
        Default::default()
    }
}

/// BFD connection mode.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum BfdMode {
    #[serde(rename = "single_hop")]
    SingleHop,
    #[serde(rename = "multi_hop")]
    MultiHop,
}

impl ::std::fmt::Display for BfdMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SingleHop => f.write_str("single_hop"),
            Self::MultiHop => f.write_str("multi_hop"),
        }
    }
}

impl ::std::str::FromStr for BfdMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "single_hop" => Ok(Self::SingleHop),
            "multi_hop" => Ok(Self::MultiHop),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BfdMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BfdMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Information needed to disable a BFD session
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BfdSessionDisable {
    /// Address of the remote peer to disable a BFD session for.
    pub remote: ::std::net::IpAddr,
    /// The slot of the switch within the rack to disable this session on.
    pub switch_slot: SwitchSlot,
}

impl BfdSessionDisable {
    pub fn builder() -> builder::BfdSessionDisable {
        Default::default()
    }
}

/// Information about a bidirectional forwarding detection (BFD) session.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BfdSessionEnable {
    /// The negotiated Control packet transmission interval, multiplied by this
    /// variable, will be the Detection Time for this session (as seen by the
    /// remote system)
    pub detection_threshold: u8,
    /// Address the Oxide switch will listen on for BFD traffic. If `None` then
    /// the unspecified address (0.0.0.0 or ::) is used.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub local: ::std::option::Option<::std::net::IpAddr>,
    /// Select either single-hop (RFC 5881) or multi-hop (RFC 5883)
    pub mode: BfdMode,
    /// Address of the remote peer to establish a BFD session with.
    pub remote: ::std::net::IpAddr,
    /// The minimum interval, in microseconds, between received BFD Control
    /// packets that this system requires
    pub required_rx: u64,
    /// The slot of the switch within the rack to enable this session on.
    pub switch_slot: SwitchSlot,
}

impl BfdSessionEnable {
    pub fn builder() -> builder::BfdSessionEnable {
        Default::default()
    }
}

/// `BfdState`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum BfdState {
    /// A stable down state. Non-responsive to incoming messages.
    #[serde(rename = "admin_down")]
    AdminDown,
    /// The initial state.
    #[serde(rename = "down")]
    Down,
    /// The peer has detected a remote peer in the down state.
    #[serde(rename = "init")]
    Init,
    /// The peer has detected a remote peer in the up or init state while in the
    /// init state.
    #[serde(rename = "up")]
    Up,
}

impl ::std::fmt::Display for BfdState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AdminDown => f.write_str("admin_down"),
            Self::Down => f.write_str("down"),
            Self::Init => f.write_str("init"),
            Self::Up => f.write_str("up"),
        }
    }
}

impl ::std::str::FromStr for BfdState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "admin_down" => Ok(Self::AdminDown),
            "down" => Ok(Self::Down),
            "init" => Ok(Self::Init),
            "up" => Ok(Self::Up),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BfdState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BfdState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `BfdStatus`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BfdStatus {
    pub detection_threshold: u8,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub local: ::std::option::Option<::std::net::IpAddr>,
    pub mode: BfdMode,
    pub peer: ::std::net::IpAddr,
    pub required_rx: u64,
    pub state: BfdState,
    pub switch_slot: SwitchSlot,
}

impl BfdStatus {
    pub fn builder() -> builder::BfdStatus {
        Default::default()
    }
}

/// Represents a BGP announce set by id. The id can be used with other API calls
/// to view and manage the announce set.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpAnnounceSet {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl BgpAnnounceSet {
    pub fn builder() -> builder::BgpAnnounceSet {
        Default::default()
    }
}

/// Parameters for creating a named set of BGP announcements.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpAnnounceSetCreate {
    /// The announcements in this set.
    pub announcement: ::std::vec::Vec<BgpAnnouncementCreate>,
    pub description: ::std::string::String,
    pub name: Name,
}

impl BgpAnnounceSetCreate {
    pub fn builder() -> builder::BgpAnnounceSetCreate {
        Default::default()
    }
}

/// A BGP announcement tied to an address lot block.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpAnnouncement {
    /// The address block the IP network being announced is drawn from.
    pub address_lot_block_id: ::uuid::Uuid,
    /// The id of the set this announcement is a part of.
    pub announce_set_id: ::uuid::Uuid,
    /// The IP network being announced.
    pub network: IpNet,
}

impl BgpAnnouncement {
    pub fn builder() -> builder::BgpAnnouncement {
        Default::default()
    }
}

/// A BGP announcement tied to a particular address lot block.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpAnnouncementCreate {
    /// Address lot this announcement is drawn from.
    pub address_lot_block: NameOrId,
    /// The network being announced.
    pub network: IpNet,
}

impl BgpAnnouncementCreate {
    pub fn builder() -> builder::BgpAnnouncementCreate {
        Default::default()
    }
}

/// A base BGP configuration.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpConfig {
    /// The autonomous system number of this BGP configuration.
    pub asn: u32,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Maximum number of paths to use when multiple "best paths" exist
    pub max_paths: MaxPathConfig,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Optional virtual routing and forwarding identifier for this BGP
    /// configuration.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vrf: ::std::option::Option<::std::string::String>,
}

impl BgpConfig {
    pub fn builder() -> builder::BgpConfig {
        Default::default()
    }
}

/// Parameters for creating a BGP configuration. This includes an autonomous
/// system number (ASN) and a virtual routing and forwarding (VRF) identifier.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpConfigCreate {
    /// The autonomous system number of this BGP configuration.
    pub asn: u32,
    pub bgp_announce_set_id: NameOrId,
    pub description: ::std::string::String,
    /// Maximum number of paths to use when multiple "best paths" exist
    #[serde(default = "defaults::bgp_config_create_max_paths")]
    pub max_paths: MaxPathConfig,
    pub name: Name,
    /// Optional virtual routing and forwarding identifier for this BGP
    /// configuration.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vrf: ::std::option::Option<Name>,
}

impl BgpConfigCreate {
    pub fn builder() -> builder::BgpConfigCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpConfigResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<BgpConfig>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl BgpConfigResultsPage {
    pub fn builder() -> builder::BgpConfigResultsPage {
        Default::default()
    }
}

/// Parameters for updating a BGP configuration
///
/// If a value is not specified, it will remain unchanged.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct BgpConfigUpdate {
    /// Update the BGP announce set associated with this configuration.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub bgp_announce_set_id: ::std::option::Option<NameOrId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    /// Update the maximum number of equal-cost paths.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_paths: ::std::option::Option<MaxPathConfig>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl BgpConfigUpdate {
    pub fn builder() -> builder::BgpConfigUpdate {
        Default::default()
    }
}

/// Route exported to a peer.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpExported {
    /// Identifier for the BGP peer.
    pub peer_id: ::std::string::String,
    /// The destination network prefix.
    pub prefix: IpNet,
    /// Switch the route is exported from.
    pub switch: SwitchSlot,
}

impl BgpExported {
    pub fn builder() -> builder::BgpExported {
        Default::default()
    }
}

/// A route imported from a BGP peer.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpImported {
    /// BGP identifier of the originating router.
    pub id: u32,
    /// The nexthop the prefix is reachable through.
    pub nexthop: ::std::net::IpAddr,
    /// The destination network prefix.
    pub prefix: IpNet,
    /// Switch the route is imported into.
    pub switch: SwitchSlot,
}

impl BgpImported {
    pub fn builder() -> builder::BgpImported {
        Default::default()
    }
}

/// `BgpMessageHistory`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct BgpMessageHistory(pub ::serde_json::Value);
impl ::std::ops::Deref for BgpMessageHistory {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}

impl ::std::convert::From<BgpMessageHistory> for ::serde_json::Value {
    fn from(value: BgpMessageHistory) -> Self {
        value.0
    }
}

impl ::std::convert::From<::serde_json::Value> for BgpMessageHistory {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}

/// A BGP peer configuration for an interface. Includes the set of announcements
/// that will be advertised to the peer. The `bgp_config` parameter is a
/// reference to global BGP parameters.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpPeer {
    /// The address of the host to peer with, or specifying the configuration of
    /// an unnumbered BGP session.
    pub addr: RouterPeerType,
    /// Define export policy for a peer.
    pub allowed_export: ImportExportPolicy,
    /// Define import policy for a peer.
    pub allowed_import: ImportExportPolicy,
    /// The global BGP configuration used for establishing a session with this
    /// peer.
    pub bgp_config: NameOrId,
    /// Include the provided communities in updates sent to the peer.
    pub communities: ::std::vec::Vec<u32>,
    /// How long to wait between TCP connection retries (seconds).
    pub connect_retry: u32,
    /// How long to delay sending an open request after establishing a TCP
    /// session (seconds).
    pub delay_open: u32,
    /// Enforce that the first AS in paths received from this peer is the peer's
    /// AS.
    pub enforce_first_as: bool,
    /// How long to hold peer connections between keepalives (seconds).
    pub hold_time: u32,
    /// How long to hold a peer in idle before attempting a new session
    /// (seconds).
    pub idle_hold_time: u32,
    /// How often to send keepalive requests (seconds).
    pub keepalive: u32,
    /// Apply a local preference to routes received from this peer.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub local_pref: ::std::option::Option<u32>,
    /// Use the given key for TCP-MD5 authentication with the peer.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub md5_auth_key: ::std::option::Option<::std::string::String>,
    /// Require messages from a peer have a minimum IP time to live field.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub min_ttl: ::std::option::Option<u8>,
    /// Apply the provided multi-exit discriminator (MED) updates sent to the
    /// peer.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub multi_exit_discriminator: ::std::option::Option<u32>,
    /// Require that a peer has a specified ASN.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub remote_asn: ::std::option::Option<u32>,
    /// Associate a VLAN ID with a peer.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vlan_id: ::std::option::Option<u16>,
}

impl BgpPeer {
    pub fn builder() -> builder::BgpPeer {
        Default::default()
    }
}

/// `BgpPeerConfig`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpPeerConfig {
    /// Link that the peer is reachable on. On ports that are not broken out,
    /// this is always phy0. On a 2x breakout the options are phy0 and phy1, on
    /// 4x phy0-phy3, etc.
    pub link_name: Name,
    pub peers: ::std::vec::Vec<BgpPeer>,
}

impl BgpPeerConfig {
    pub fn builder() -> builder::BgpPeerConfig {
        Default::default()
    }
}

/// The current state of a BGP peer.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum BgpPeerState {
    /// Initial state. Refuse all incoming BGP connections. No resources
    /// allocated to peer.
    #[serde(rename = "idle")]
    Idle,
    /// Waiting for the TCP connection to be completed.
    #[serde(rename = "connect")]
    Connect,
    /// Trying to acquire peer by listening for and accepting a TCP connection.
    #[serde(rename = "active")]
    Active,
    /// Waiting for open message from peer.
    #[serde(rename = "open_sent")]
    OpenSent,
    /// Waiting for keepaliave or notification from peer.
    #[serde(rename = "open_confirm")]
    OpenConfirm,
    /// There is an ongoing Connection Collision that hasn't yet been resolved.
    /// Two connections are maintained until one connection receives an Open or
    /// is able to progress into Established.
    #[serde(rename = "connection_collision")]
    ConnectionCollision,
    /// Synchronizing with peer.
    #[serde(rename = "session_setup")]
    SessionSetup,
    /// Session established. Able to exchange update, notification and keepalive
    /// messages with peers.
    #[serde(rename = "established")]
    Established,
}

impl ::std::fmt::Display for BgpPeerState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Idle => f.write_str("idle"),
            Self::Connect => f.write_str("connect"),
            Self::Active => f.write_str("active"),
            Self::OpenSent => f.write_str("open_sent"),
            Self::OpenConfirm => f.write_str("open_confirm"),
            Self::ConnectionCollision => f.write_str("connection_collision"),
            Self::SessionSetup => f.write_str("session_setup"),
            Self::Established => f.write_str("established"),
        }
    }
}

impl ::std::str::FromStr for BgpPeerState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "idle" => Ok(Self::Idle),
            "connect" => Ok(Self::Connect),
            "active" => Ok(Self::Active),
            "open_sent" => Ok(Self::OpenSent),
            "open_confirm" => Ok(Self::OpenConfirm),
            "connection_collision" => Ok(Self::ConnectionCollision),
            "session_setup" => Ok(Self::SessionSetup),
            "established" => Ok(Self::Established),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for BgpPeerState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for BgpPeerState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// The current status of a BGP peer.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct BgpPeerStatus {
    /// IP address of the peer.
    pub addr: ::std::net::IpAddr,
    /// Local autonomous system number.
    pub local_asn: u32,
    /// Interface name
    pub peer_id: ::std::string::String,
    /// Remote autonomous system number.
    pub remote_asn: u32,
    /// State of the peer.
    pub state: BgpPeerState,
    /// Time of last state change.
    pub state_duration_millis: u64,
    /// Switch with the peer session.
    pub switch: SwitchSlot,
}

impl BgpPeerStatus {
    pub fn builder() -> builder::BgpPeerStatus {
        Default::default()
    }
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangedouble {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: f64 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: f64, start: f64 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: f64 },
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangefloat {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: f32 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: f32, start: f32 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: f32 },
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangeint16 {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: i16 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: i16, start: i16 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: i16 },
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangeint32 {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: i32 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: i32, start: i32 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: i32 },
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangeint64 {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: i64 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: i64, start: i64 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: i64 },
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangeint8 {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: i8 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: i8, start: i8 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: i8 },
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangeuint16 {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: u16 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: u16, start: u16 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: u16 },
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangeuint32 {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: u32 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: u32, start: u32 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: u32 },
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangeuint64 {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: u64 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: u64, start: u64 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: u64 },
}

/// A type storing a range over `T`.
///
/// This type supports ranges similar to the `RangeTo`, `Range` and `RangeFrom`
/// types in the standard library. Those cover `(..end)`, `(start..end)`, and
/// `(start..)` respectively.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum BinRangeuint8 {
    /// A range unbounded below and exclusively above, `..end`.
    #[serde(rename = "range_to")]
    RangeTo { end: u8 },
    /// A range bounded inclusively below and exclusively above, `start..end`.
    #[serde(rename = "range")]
    Range { end: u8, start: u8 },
    /// A range bounded inclusively below and unbounded above, `start..`.
    #[serde(rename = "range_from")]
    RangeFrom { start: u8 },
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Bindouble {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangedouble,
}

impl Bindouble {
    pub fn builder() -> builder::Bindouble {
        Default::default()
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Binfloat {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangefloat,
}

impl Binfloat {
    pub fn builder() -> builder::Binfloat {
        Default::default()
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Binint16 {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangeint16,
}

impl Binint16 {
    pub fn builder() -> builder::Binint16 {
        Default::default()
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Binint32 {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangeint32,
}

impl Binint32 {
    pub fn builder() -> builder::Binint32 {
        Default::default()
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Binint64 {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangeint64,
}

impl Binint64 {
    pub fn builder() -> builder::Binint64 {
        Default::default()
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Binint8 {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangeint8,
}

impl Binint8 {
    pub fn builder() -> builder::Binint8 {
        Default::default()
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Binuint16 {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangeuint16,
}

impl Binuint16 {
    pub fn builder() -> builder::Binuint16 {
        Default::default()
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Binuint32 {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangeuint32,
}

impl Binuint32 {
    pub fn builder() -> builder::Binuint32 {
        Default::default()
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Binuint64 {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangeuint64,
}

impl Binuint64 {
    pub fn builder() -> builder::Binuint64 {
        Default::default()
    }
}

/// Type storing bin edges and a count of samples within it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Binuint8 {
    /// The total count of samples in this bin.
    pub count: u64,
    /// The range of the support covered by this bin.
    pub range: BinRangeuint8,
}

impl Binuint8 {
    pub fn builder() -> builder::Binuint8 {
        Default::default()
    }
}

/// Valid values are: 512, 2048, or 4096.
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct BlockSize(i64);
impl ::std::ops::Deref for BlockSize {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl ::std::convert::From<BlockSize> for i64 {
    fn from(value: BlockSize) -> Self {
        value.0
    }
}

impl ::std::convert::TryFrom<i64> for BlockSize {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![512_i64, 2048_i64, 4096_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}

impl<'de> ::serde::Deserialize<'de> for BlockSize {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

impl ::schemars::JsonSchema for BlockSize {
    fn schema_name() -> ::std::string::String {
        "BlockSize".to_string()
    }

    fn json_schema(g: &mut ::schemars::r#gen::SchemaGenerator) -> ::schemars::schema::Schema {
        let mut schema = <i64 as ::schemars::JsonSchema>::json_schema(g).into_object();
        schema.enum_values = ::std::option::Option::Some(
            [
                ::serde_json::from_str("512").unwrap(),
                ::serde_json::from_str("2048").unwrap(),
                ::serde_json::from_str("4096").unwrap(),
            ]
            .into_iter()
            .collect(),
        );
        schema.into()
    }
}

/// Byte count to express memory or storage capacity.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct ByteCount(pub u64);
impl ::std::ops::Deref for ByteCount {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl ::std::convert::From<ByteCount> for u64 {
    fn from(value: ByteCount) -> Self {
        value.0
    }
}

impl ::std::convert::From<u64> for ByteCount {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl ::std::fmt::Display for ByteCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for ByteCount {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

/// View of a Certificate
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Certificate {
    /// PEM-formatted string containing public certificate chain
    pub cert: ::std::string::String,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// The service using this certificate
    pub service: ServiceUsingCertificate,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Certificate {
    pub fn builder() -> builder::Certificate {
        Default::default()
    }
}

/// Create-time parameters for a `Certificate`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct CertificateCreate {
    /// PEM-formatted string containing public certificate chain
    pub cert: ::std::string::String,
    pub description: ::std::string::String,
    /// PEM-formatted string containing private key
    pub key: ::std::string::String,
    pub name: Name,
    /// The service using this certificate
    pub service: ServiceUsingCertificate,
}

impl CertificateCreate {
    pub fn builder() -> builder::CertificateCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct CertificateResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Certificate>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl CertificateResultsPage {
    pub fn builder() -> builder::CertificateResultsPage {
        Default::default()
    }
}

/// View of a console session
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ConsoleSession {
    /// A unique, immutable, system-controlled identifier for the session
    pub id: ::uuid::Uuid,
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub time_last_used: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl ConsoleSession {
    pub fn builder() -> builder::ConsoleSession {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ConsoleSessionResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<ConsoleSession>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl ConsoleSessionResultsPage {
    pub fn builder() -> builder::ConsoleSessionResultsPage {
        Default::default()
    }
}

/// A cumulative or counter data type.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Cumulativedouble {
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    pub value: f64,
}

impl Cumulativedouble {
    pub fn builder() -> builder::Cumulativedouble {
        Default::default()
    }
}

/// A cumulative or counter data type.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Cumulativefloat {
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    pub value: f32,
}

impl Cumulativefloat {
    pub fn builder() -> builder::Cumulativefloat {
        Default::default()
    }
}

/// A cumulative or counter data type.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Cumulativeint64 {
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    pub value: i64,
}

impl Cumulativeint64 {
    pub fn builder() -> builder::Cumulativeint64 {
        Default::default()
    }
}

/// A cumulative or counter data type.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Cumulativeuint64 {
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    pub value: u64,
}

impl Cumulativeuint64 {
    pub fn builder() -> builder::Cumulativeuint64 {
        Default::default()
    }
}

/// Info about the current user
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct CurrentUser {
    /// Human-readable name that can identify the user
    pub display_name: ::std::string::String,
    /// Whether this user has the viewer role on the fleet. Used by the web
    /// console to determine whether to show system-level UI.
    pub fleet_viewer: bool,
    pub id: ::uuid::Uuid,
    /// Whether this user has the admin role on their silo. Used by the web
    /// console to determine whether to show admin-only UI elements.
    pub silo_admin: bool,
    /// Uuid of the silo to which this user belongs
    pub silo_id: ::uuid::Uuid,
    /// Name of the silo to which this user belongs.
    pub silo_name: Name,
    /// Timestamp when this user was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this user was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl CurrentUser {
    pub fn builder() -> builder::CurrentUser {
        Default::default()
    }
}

/// A `Datum` is a single sampled data point from a metric.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "datum")]
pub enum Datum {
    #[serde(rename = "bool")]
    Bool(bool),
    #[serde(rename = "i8")]
    I8(i8),
    #[serde(rename = "u8")]
    U8(u8),
    #[serde(rename = "i16")]
    I16(i16),
    #[serde(rename = "u16")]
    U16(u16),
    #[serde(rename = "i32")]
    I32(i32),
    #[serde(rename = "u32")]
    U32(u32),
    #[serde(rename = "i64")]
    I64(i64),
    #[serde(rename = "u64")]
    U64(u64),
    #[serde(rename = "f32")]
    F32(f32),
    #[serde(rename = "f64")]
    F64(f64),
    #[serde(rename = "string")]
    String(::std::string::String),
    #[serde(rename = "bytes")]
    Bytes(::std::vec::Vec<u8>),
    #[serde(rename = "cumulative_i64")]
    CumulativeI64(Cumulativeint64),
    #[serde(rename = "cumulative_u64")]
    CumulativeU64(Cumulativeuint64),
    #[serde(rename = "cumulative_f32")]
    CumulativeF32(Cumulativefloat),
    #[serde(rename = "cumulative_f64")]
    CumulativeF64(Cumulativedouble),
    #[serde(rename = "histogram_i8")]
    HistogramI8(Histogramint8),
    #[serde(rename = "histogram_u8")]
    HistogramU8(Histogramuint8),
    #[serde(rename = "histogram_i16")]
    HistogramI16(Histogramint16),
    #[serde(rename = "histogram_u16")]
    HistogramU16(Histogramuint16),
    #[serde(rename = "histogram_i32")]
    HistogramI32(Histogramint32),
    #[serde(rename = "histogram_u32")]
    HistogramU32(Histogramuint32),
    #[serde(rename = "histogram_i64")]
    HistogramI64(Histogramint64),
    #[serde(rename = "histogram_u64")]
    HistogramU64(Histogramuint64),
    #[serde(rename = "histogram_f32")]
    HistogramF32(Histogramfloat),
    #[serde(rename = "histogram_f64")]
    HistogramF64(Histogramdouble),
    #[serde(rename = "missing")]
    Missing(MissingDatum),
}

impl ::std::convert::From<bool> for Datum {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl ::std::convert::From<i8> for Datum {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl ::std::convert::From<u8> for Datum {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl ::std::convert::From<i16> for Datum {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl ::std::convert::From<u16> for Datum {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl ::std::convert::From<i32> for Datum {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl ::std::convert::From<u32> for Datum {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl ::std::convert::From<i64> for Datum {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl ::std::convert::From<u64> for Datum {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl ::std::convert::From<f32> for Datum {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl ::std::convert::From<f64> for Datum {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl ::std::convert::From<::std::vec::Vec<u8>> for Datum {
    fn from(value: ::std::vec::Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl ::std::convert::From<Cumulativeint64> for Datum {
    fn from(value: Cumulativeint64) -> Self {
        Self::CumulativeI64(value)
    }
}

impl ::std::convert::From<Cumulativeuint64> for Datum {
    fn from(value: Cumulativeuint64) -> Self {
        Self::CumulativeU64(value)
    }
}

impl ::std::convert::From<Cumulativefloat> for Datum {
    fn from(value: Cumulativefloat) -> Self {
        Self::CumulativeF32(value)
    }
}

impl ::std::convert::From<Cumulativedouble> for Datum {
    fn from(value: Cumulativedouble) -> Self {
        Self::CumulativeF64(value)
    }
}

impl ::std::convert::From<Histogramint8> for Datum {
    fn from(value: Histogramint8) -> Self {
        Self::HistogramI8(value)
    }
}

impl ::std::convert::From<Histogramuint8> for Datum {
    fn from(value: Histogramuint8) -> Self {
        Self::HistogramU8(value)
    }
}

impl ::std::convert::From<Histogramint16> for Datum {
    fn from(value: Histogramint16) -> Self {
        Self::HistogramI16(value)
    }
}

impl ::std::convert::From<Histogramuint16> for Datum {
    fn from(value: Histogramuint16) -> Self {
        Self::HistogramU16(value)
    }
}

impl ::std::convert::From<Histogramint32> for Datum {
    fn from(value: Histogramint32) -> Self {
        Self::HistogramI32(value)
    }
}

impl ::std::convert::From<Histogramuint32> for Datum {
    fn from(value: Histogramuint32) -> Self {
        Self::HistogramU32(value)
    }
}

impl ::std::convert::From<Histogramint64> for Datum {
    fn from(value: Histogramint64) -> Self {
        Self::HistogramI64(value)
    }
}

impl ::std::convert::From<Histogramuint64> for Datum {
    fn from(value: Histogramuint64) -> Self {
        Self::HistogramU64(value)
    }
}

impl ::std::convert::From<Histogramfloat> for Datum {
    fn from(value: Histogramfloat) -> Self {
        Self::HistogramF32(value)
    }
}

impl ::std::convert::From<Histogramdouble> for Datum {
    fn from(value: Histogramdouble) -> Self {
        Self::HistogramF64(value)
    }
}

impl ::std::convert::From<MissingDatum> for Datum {
    fn from(value: MissingDatum) -> Self {
        Self::Missing(value)
    }
}

/// The type of an individual datum of a metric.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum DatumType {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "i8")]
    I8,
    #[serde(rename = "u8")]
    U8,
    #[serde(rename = "i16")]
    I16,
    #[serde(rename = "u16")]
    U16,
    #[serde(rename = "i32")]
    I32,
    #[serde(rename = "u32")]
    U32,
    #[serde(rename = "i64")]
    I64,
    #[serde(rename = "u64")]
    U64,
    #[serde(rename = "f32")]
    F32,
    #[serde(rename = "f64")]
    F64,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "bytes")]
    Bytes,
    #[serde(rename = "cumulative_i64")]
    CumulativeI64,
    #[serde(rename = "cumulative_u64")]
    CumulativeU64,
    #[serde(rename = "cumulative_f32")]
    CumulativeF32,
    #[serde(rename = "cumulative_f64")]
    CumulativeF64,
    #[serde(rename = "histogram_i8")]
    HistogramI8,
    #[serde(rename = "histogram_u8")]
    HistogramU8,
    #[serde(rename = "histogram_i16")]
    HistogramI16,
    #[serde(rename = "histogram_u16")]
    HistogramU16,
    #[serde(rename = "histogram_i32")]
    HistogramI32,
    #[serde(rename = "histogram_u32")]
    HistogramU32,
    #[serde(rename = "histogram_i64")]
    HistogramI64,
    #[serde(rename = "histogram_u64")]
    HistogramU64,
    #[serde(rename = "histogram_f32")]
    HistogramF32,
    #[serde(rename = "histogram_f64")]
    HistogramF64,
}

impl ::std::fmt::Display for DatumType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bool => f.write_str("bool"),
            Self::I8 => f.write_str("i8"),
            Self::U8 => f.write_str("u8"),
            Self::I16 => f.write_str("i16"),
            Self::U16 => f.write_str("u16"),
            Self::I32 => f.write_str("i32"),
            Self::U32 => f.write_str("u32"),
            Self::I64 => f.write_str("i64"),
            Self::U64 => f.write_str("u64"),
            Self::F32 => f.write_str("f32"),
            Self::F64 => f.write_str("f64"),
            Self::String => f.write_str("string"),
            Self::Bytes => f.write_str("bytes"),
            Self::CumulativeI64 => f.write_str("cumulative_i64"),
            Self::CumulativeU64 => f.write_str("cumulative_u64"),
            Self::CumulativeF32 => f.write_str("cumulative_f32"),
            Self::CumulativeF64 => f.write_str("cumulative_f64"),
            Self::HistogramI8 => f.write_str("histogram_i8"),
            Self::HistogramU8 => f.write_str("histogram_u8"),
            Self::HistogramI16 => f.write_str("histogram_i16"),
            Self::HistogramU16 => f.write_str("histogram_u16"),
            Self::HistogramI32 => f.write_str("histogram_i32"),
            Self::HistogramU32 => f.write_str("histogram_u32"),
            Self::HistogramI64 => f.write_str("histogram_i64"),
            Self::HistogramU64 => f.write_str("histogram_u64"),
            Self::HistogramF32 => f.write_str("histogram_f32"),
            Self::HistogramF64 => f.write_str("histogram_f64"),
        }
    }
}

impl ::std::str::FromStr for DatumType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bool" => Ok(Self::Bool),
            "i8" => Ok(Self::I8),
            "u8" => Ok(Self::U8),
            "i16" => Ok(Self::I16),
            "u16" => Ok(Self::U16),
            "i32" => Ok(Self::I32),
            "u32" => Ok(Self::U32),
            "i64" => Ok(Self::I64),
            "u64" => Ok(Self::U64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "string" => Ok(Self::String),
            "bytes" => Ok(Self::Bytes),
            "cumulative_i64" => Ok(Self::CumulativeI64),
            "cumulative_u64" => Ok(Self::CumulativeU64),
            "cumulative_f32" => Ok(Self::CumulativeF32),
            "cumulative_f64" => Ok(Self::CumulativeF64),
            "histogram_i8" => Ok(Self::HistogramI8),
            "histogram_u8" => Ok(Self::HistogramU8),
            "histogram_i16" => Ok(Self::HistogramI16),
            "histogram_u16" => Ok(Self::HistogramU16),
            "histogram_i32" => Ok(Self::HistogramI32),
            "histogram_u32" => Ok(Self::HistogramU32),
            "histogram_i64" => Ok(Self::HistogramI64),
            "histogram_u64" => Ok(Self::HistogramU64),
            "histogram_f32" => Ok(Self::HistogramF32),
            "histogram_f64" => Ok(Self::HistogramF64),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for DatumType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for DatumType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `DerEncodedKeyPair`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct DerEncodedKeyPair {
    /// Request signing RSA private key in PKCS#1 format (base64 encoded DER
    /// file)
    pub private_key: ::std::string::String,
    /// Request signing public certificate (base64 encoded DER file)
    pub public_cert: ::std::string::String,
}

impl DerEncodedKeyPair {
    pub fn builder() -> builder::DerEncodedKeyPair {
        Default::default()
    }
}

/// View of a device access token
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct DeviceAccessToken {
    /// A unique, immutable, system-controlled identifier for the token.
    ///
    /// Note that this ID is not the bearer token itself, which starts with
    /// "oxide-token-".
    pub id: ::uuid::Uuid,
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Expiration timestamp. A null value means the token does not
    /// automatically expire.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time_expires: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl DeviceAccessToken {
    pub fn builder() -> builder::DeviceAccessToken {
        Default::default()
    }
}

/// `DeviceAccessTokenRequest`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct DeviceAccessTokenRequest {
    pub client_id: ::uuid::Uuid,
    pub device_code: ::std::string::String,
    pub grant_type: ::std::string::String,
}

impl DeviceAccessTokenRequest {
    pub fn builder() -> builder::DeviceAccessTokenRequest {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct DeviceAccessTokenResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<DeviceAccessToken>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl DeviceAccessTokenResultsPage {
    pub fn builder() -> builder::DeviceAccessTokenResultsPage {
        Default::default()
    }
}

/// `DeviceAuthRequest`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct DeviceAuthRequest {
    pub client_id: ::uuid::Uuid,
    /// Optional lifetime for the access token in seconds.
    ///
    /// This value will be validated during the confirmation step. If not
    /// specified, it defaults to the silo's max TTL, which can be seen at
    /// `/v1/auth-settings`.  If specified, must not exceed the silo's max TTL.
    ///
    /// Some special logic applies when authenticating the confirmation request
    /// with an existing device token: the requested TTL must not produce an
    /// expiration time later than the authenticating token's expiration. If no
    /// TTL is specified, the expiration will be the lesser of the silo max and
    /// the authenticating token's expiration time. To get the longest allowed
    /// lifetime, omit the TTL and authenticate with a web console session.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl_seconds: ::std::option::Option<::std::num::NonZeroU32>,
}

impl DeviceAuthRequest {
    pub fn builder() -> builder::DeviceAuthRequest {
        Default::default()
    }
}

/// `DeviceAuthVerify`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct DeviceAuthVerify {
    pub user_code: ::std::string::String,
}

impl DeviceAuthVerify {
    pub fn builder() -> builder::DeviceAuthVerify {
        Default::default()
    }
}

/// `Digest`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Digest {
    #[serde(rename = "sha256")]
    Sha256(::std::string::String),
}

/// View of a Disk
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Disk {
    pub block_size: BlockSize,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    pub device_path: ::std::string::String,
    pub disk_type: DiskType,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// ID of image from which disk was created, if any
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_id: ::std::option::Option<::uuid::Uuid>,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    pub project_id: ::uuid::Uuid,
    /// Whether or not this disk is read-only.
    pub read_only: bool,
    pub size: ByteCount,
    /// ID of snapshot from which disk was created, if any
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub snapshot_id: ::std::option::Option<::uuid::Uuid>,
    pub state: DiskState,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Disk {
    pub fn builder() -> builder::Disk {
        Default::default()
    }
}

/// The source of a `Disk`'s blocks
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "disk_source")]
pub enum DiskBackend {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "distributed")]
    Distributed(DiskSource),
}

impl ::std::convert::From<DiskSource> for DiskBackend {
    fn from(value: DiskSource) -> Self {
        Self::Distributed(value)
    }
}

/// Create-time parameters for a `Disk`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct DiskCreate {
    pub description: ::std::string::String,
    /// The source for this `Disk`'s blocks
    pub disk_backend: DiskBackend,
    pub name: Name,
    /// The total size of the Disk (in bytes)
    pub size: ByteCount,
}

impl DiskCreate {
    pub fn builder() -> builder::DiskCreate {
        Default::default()
    }
}

/// `DiskPath`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct DiskPath {
    /// Name or ID of the disk
    pub disk: NameOrId,
}

impl DiskPath {
    pub fn builder() -> builder::DiskPath {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct DiskResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Disk>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl DiskResultsPage {
    pub fn builder() -> builder::DiskResultsPage {
        Default::default()
    }
}

/// Different sources for a Distributed Disk
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum DiskSource {
    /// Create a blank disk
    #[serde(rename = "blank")]
    Blank {
        /// Size of blocks for this disk. Valid values are: 512, 2048, or 4096.
        block_size: BlockSize,
    },
    /// Create a disk from a disk snapshot
    #[serde(rename = "snapshot")]
    Snapshot {
        /// If `true`, the disk created from this snapshot will be read-only.
        #[serde(default)]
        read_only: bool,
        snapshot_id: ::uuid::Uuid,
    },
    /// Create a disk from an image
    #[serde(rename = "image")]
    Image {
        image_id: ::uuid::Uuid,
        /// If `true`, the disk created from this image will be read-only.
        #[serde(default)]
        read_only: bool,
    },
    /// Create a blank disk that will accept bulk writes or pull blocks from an
    /// external source.
    #[serde(rename = "importing_blocks")]
    ImportingBlocks { block_size: BlockSize },
}

/// State of a Disk
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "state", content = "instance")]
pub enum DiskState {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "detached")]
    Detached,
    #[serde(rename = "import_ready")]
    ImportReady,
    #[serde(rename = "importing_from_url")]
    ImportingFromUrl,
    #[serde(rename = "importing_from_bulk_writes")]
    ImportingFromBulkWrites,
    #[serde(rename = "finalizing")]
    Finalizing,
    #[serde(rename = "maintenance")]
    Maintenance,
    /// Disk is being attached to the given Instance
    #[serde(rename = "attaching")]
    Attaching(::uuid::Uuid),
    /// Disk is attached to the given Instance
    #[serde(rename = "attached")]
    Attached(::uuid::Uuid),
    /// Disk is being detached from the given Instance
    #[serde(rename = "detaching")]
    Detaching(::uuid::Uuid),
    #[serde(rename = "destroyed")]
    Destroyed,
    #[serde(rename = "faulted")]
    Faulted,
}

/// `DiskType`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum DiskType {
    #[serde(rename = "distributed")]
    Distributed,
    #[serde(rename = "local")]
    Local,
}

impl ::std::fmt::Display for DiskType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Distributed => f.write_str("distributed"),
            Self::Local => f.write_str("local"),
        }
    }
}

impl ::std::str::FromStr for DiskType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "distributed" => Ok(Self::Distributed),
            "local" => Ok(Self::Local),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for DiskType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for DiskType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A distribution is a sequence of bins and counts in those bins, and some
/// statistical information tracked to compute the mean, standard deviation, and
/// quantile estimates.
///
/// Min, max, and the p-* quantiles are treated as optional due to the
/// possibility of distribution operations, like subtraction.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Distributiondouble {
    pub bins: ::std::vec::Vec<f64>,
    pub counts: ::std::vec::Vec<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub min: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub p50: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub p90: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub p99: ::std::option::Option<f64>,
    pub squared_mean: f64,
    pub sum_of_samples: f64,
}

impl Distributiondouble {
    pub fn builder() -> builder::Distributiondouble {
        Default::default()
    }
}

/// A distribution is a sequence of bins and counts in those bins, and some
/// statistical information tracked to compute the mean, standard deviation, and
/// quantile estimates.
///
/// Min, max, and the p-* quantiles are treated as optional due to the
/// possibility of distribution operations, like subtraction.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Distributionint64 {
    pub bins: ::std::vec::Vec<i64>,
    pub counts: ::std::vec::Vec<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub min: ::std::option::Option<i64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub p50: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub p90: ::std::option::Option<f64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub p99: ::std::option::Option<f64>,
    pub squared_mean: f64,
    pub sum_of_samples: i64,
}

impl Distributionint64 {
    pub fn builder() -> builder::Distributionint64 {
        Default::default()
    }
}

/// Parameters for creating an ephemeral IP address for an instance.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct EphemeralIpCreate {
    /// Pool to allocate from.
    #[serde(default = "defaults::ephemeral_ip_create_pool_selector")]
    pub pool_selector: PoolSelector,
}

impl ::std::default::Default for EphemeralIpCreate {
    fn default() -> Self {
        Self {
            pool_selector: defaults::ephemeral_ip_create_pool_selector(),
        }
    }
}

impl EphemeralIpCreate {
    pub fn builder() -> builder::EphemeralIpCreate {
        Default::default()
    }
}

/// Error information from a response.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Error {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_code: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    pub request_id: ::std::string::String,
}

impl Error {
    pub fn builder() -> builder::Error {
        Default::default()
    }
}

/// `ExternalIp`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "kind")]
pub enum ExternalIp {
    /// A source NAT IP address.
    ///
    /// SNAT addresses are ephemeral addresses used only for outbound
    /// connectivity.
    #[serde(rename = "snat")]
    Snat {
        /// The first usable port within the IP address.
        first_port: u16,
        /// The IP address.
        ip: ::std::net::IpAddr,
        /// ID of the IP Pool from which the address is taken.
        ip_pool_id: ::uuid::Uuid,
        /// The last usable port within the IP address.
        last_port: u16,
    },
    #[serde(rename = "ephemeral")]
    Ephemeral {
        ip: ::std::net::IpAddr,
        ip_pool_id: ::uuid::Uuid,
    },
    /// A Floating IP is a well-known IP address which can be attached and
    /// detached from instances.
    #[serde(rename = "floating")]
    Floating {
        /// Human-readable free-form text about a resource
        description: ::std::string::String,
        /// Unique, immutable, system-controlled identifier for each resource
        id: ::uuid::Uuid,
        /// The ID of the instance that this Floating IP is attached to, if it
        /// is presently in use.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        instance_id: ::std::option::Option<::uuid::Uuid>,
        /// The IP address held by this resource.
        ip: ::std::net::IpAddr,
        /// The ID of the IP pool this resource belongs to.
        ip_pool_id: ::uuid::Uuid,
        /// Unique, mutable, user-controlled identifier for each resource
        name: Name,
        /// The project this resource exists within.
        project_id: ::uuid::Uuid,
        /// Timestamp when this resource was created
        time_created: ::chrono::DateTime<::chrono::offset::Utc>,
        /// Timestamp when this resource was last modified
        time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    },
}

/// Parameters for creating an external IP address for instances.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum ExternalIpCreate {
    /// An IP address providing both inbound and outbound access. The address is
    /// automatically assigned from a pool.
    #[serde(rename = "ephemeral")]
    Ephemeral {
        /// Pool to allocate from.
        #[serde(default = "defaults::external_ip_create_ephemeral_pool_selector")]
        pool_selector: PoolSelector,
    },
    /// An IP address providing both inbound and outbound access. The address is
    /// an existing floating IP object assigned to the current project.
    ///
    /// The floating IP must not be in use by another instance or service.
    #[serde(rename = "floating")]
    Floating { floating_ip: NameOrId },
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ExternalIpResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<ExternalIp>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl ExternalIpResultsPage {
    pub fn builder() -> builder::ExternalIpResultsPage {
        Default::default()
    }
}

/// An external subnet allocated from a subnet pool
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ExternalSubnet {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The instance this subnet is attached to, if any
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instance_id: ::std::option::Option<::uuid::Uuid>,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// The project this subnet belongs to
    pub project_id: ::uuid::Uuid,
    /// The allocated subnet CIDR
    pub subnet: IpNet,
    /// The subnet pool this was allocated from
    pub subnet_pool_id: ::uuid::Uuid,
    /// The subnet pool member this subnet corresponds to
    pub subnet_pool_member_id: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl ExternalSubnet {
    pub fn builder() -> builder::ExternalSubnet {
        Default::default()
    }
}

/// Specify how to allocate an external subnet.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum ExternalSubnetAllocator {
    /// Reserve a specific subnet.
    #[serde(rename = "explicit")]
    Explicit {
        /// The subnet CIDR to reserve. Must be available in the pool.
        subnet: IpNet,
    },
    /// Automatically allocate a subnet with the specified prefix length.
    #[serde(rename = "auto")]
    Auto {
        /// Pool selection.
        ///
        /// If omitted, this field uses the silo's default pool. If the silo has
        /// default pools for both IPv4 and IPv6, the request will fail unless
        /// `ip_version` is specified in the pool selector.
        #[serde(default = "defaults::external_subnet_allocator_auto_pool_selector")]
        pool_selector: PoolSelector,
        /// The prefix length for the allocated subnet (e.g., 24 for a /24).
        prefix_length: u8,
    },
}

/// Attach an external subnet to an instance
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ExternalSubnetAttach {
    /// Name or ID of the instance to attach to
    pub instance: NameOrId,
}

impl ExternalSubnetAttach {
    pub fn builder() -> builder::ExternalSubnetAttach {
        Default::default()
    }
}

/// Create an external subnet
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ExternalSubnetCreate {
    /// Subnet allocation method.
    pub allocator: ExternalSubnetAllocator,
    pub description: ::std::string::String,
    pub name: Name,
}

impl ExternalSubnetCreate {
    pub fn builder() -> builder::ExternalSubnetCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ExternalSubnetResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<ExternalSubnet>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl ExternalSubnetResultsPage {
    pub fn builder() -> builder::ExternalSubnetResultsPage {
        Default::default()
    }
}

/// Update an external subnet
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct ExternalSubnetUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl ExternalSubnetUpdate {
    pub fn builder() -> builder::ExternalSubnetUpdate {
        Default::default()
    }
}

/// Describes the scope of affinity for the purposes of co-location.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum FailureDomain {
    /// Instances are considered co-located if they are on the same sled
    #[serde(rename = "sled")]
    Sled,
}

impl ::std::fmt::Display for FailureDomain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sled => f.write_str("sled"),
        }
    }
}

impl ::std::str::FromStr for FailureDomain {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "sled" => Ok(Self::Sled),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for FailureDomain {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for FailureDomain {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// The name and type information for a field of a timeseries schema.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct FieldSchema {
    pub description: ::std::string::String,
    pub field_type: FieldType,
    pub name: ::std::string::String,
    pub source: FieldSource,
}

impl FieldSchema {
    pub fn builder() -> builder::FieldSchema {
        Default::default()
    }
}

/// The source from which a field is derived, the target or metric.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum FieldSource {
    #[serde(rename = "target")]
    Target,
    #[serde(rename = "metric")]
    Metric,
}

impl ::std::fmt::Display for FieldSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Target => f.write_str("target"),
            Self::Metric => f.write_str("metric"),
        }
    }
}

impl ::std::str::FromStr for FieldSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "target" => Ok(Self::Target),
            "metric" => Ok(Self::Metric),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for FieldSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for FieldSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// The `FieldType` identifies the data type of a target or metric field.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum FieldType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "i8")]
    I8,
    #[serde(rename = "u8")]
    U8,
    #[serde(rename = "i16")]
    I16,
    #[serde(rename = "u16")]
    U16,
    #[serde(rename = "i32")]
    I32,
    #[serde(rename = "u32")]
    U32,
    #[serde(rename = "i64")]
    I64,
    #[serde(rename = "u64")]
    U64,
    #[serde(rename = "ip_addr")]
    IpAddr,
    #[serde(rename = "uuid")]
    Uuid,
    #[serde(rename = "bool")]
    Bool,
}

impl ::std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::String => f.write_str("string"),
            Self::I8 => f.write_str("i8"),
            Self::U8 => f.write_str("u8"),
            Self::I16 => f.write_str("i16"),
            Self::U16 => f.write_str("u16"),
            Self::I32 => f.write_str("i32"),
            Self::U32 => f.write_str("u32"),
            Self::I64 => f.write_str("i64"),
            Self::U64 => f.write_str("u64"),
            Self::IpAddr => f.write_str("ip_addr"),
            Self::Uuid => f.write_str("uuid"),
            Self::Bool => f.write_str("bool"),
        }
    }
}

impl ::std::str::FromStr for FieldType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "string" => Ok(Self::String),
            "i8" => Ok(Self::I8),
            "u8" => Ok(Self::U8),
            "i16" => Ok(Self::I16),
            "u16" => Ok(Self::U16),
            "i32" => Ok(Self::I32),
            "u32" => Ok(Self::U32),
            "i64" => Ok(Self::I64),
            "u64" => Ok(Self::U64),
            "ip_addr" => Ok(Self::IpAddr),
            "uuid" => Ok(Self::Uuid),
            "bool" => Ok(Self::Bool),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for FieldType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for FieldType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// The `FieldValue` contains the value of a target or metric field.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum FieldValue {
    #[serde(rename = "string")]
    String(::std::string::String),
    #[serde(rename = "i8")]
    I8(i8),
    #[serde(rename = "u8")]
    U8(u8),
    #[serde(rename = "i16")]
    I16(i16),
    #[serde(rename = "u16")]
    U16(u16),
    #[serde(rename = "i32")]
    I32(i32),
    #[serde(rename = "u32")]
    U32(u32),
    #[serde(rename = "i64")]
    I64(i64),
    #[serde(rename = "u64")]
    U64(u64),
    #[serde(rename = "ip_addr")]
    IpAddr(::std::net::IpAddr),
    #[serde(rename = "uuid")]
    Uuid(::uuid::Uuid),
    #[serde(rename = "bool")]
    Bool(bool),
}

impl ::std::convert::From<i8> for FieldValue {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl ::std::convert::From<u8> for FieldValue {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl ::std::convert::From<i16> for FieldValue {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl ::std::convert::From<u16> for FieldValue {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl ::std::convert::From<i32> for FieldValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl ::std::convert::From<u32> for FieldValue {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl ::std::convert::From<i64> for FieldValue {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl ::std::convert::From<u64> for FieldValue {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl ::std::convert::From<::std::net::IpAddr> for FieldValue {
    fn from(value: ::std::net::IpAddr) -> Self {
        Self::IpAddr(value)
    }
}

impl ::std::convert::From<::uuid::Uuid> for FieldValue {
    fn from(value: ::uuid::Uuid) -> Self {
        Self::Uuid(value)
    }
}

impl ::std::convert::From<bool> for FieldValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

/// Parameters for finalizing a disk
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct FinalizeDisk {
    /// If specified a snapshot of the disk will be created with the given name
    /// during finalization. If not specified, a snapshot for the disk will
    /// _not_ be created. A snapshot can be manually created once the disk
    /// transitions into the `Detached` state.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub snapshot_name: ::std::option::Option<Name>,
}

impl FinalizeDisk {
    pub fn builder() -> builder::FinalizeDisk {
        Default::default()
    }
}

/// `FleetRole`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum FleetRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "collaborator")]
    Collaborator,
    #[serde(rename = "viewer")]
    Viewer,
}

impl ::std::fmt::Display for FleetRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Admin => f.write_str("admin"),
            Self::Collaborator => f.write_str("collaborator"),
            Self::Viewer => f.write_str("viewer"),
        }
    }
}

impl ::std::str::FromStr for FleetRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "admin" => Ok(Self::Admin),
            "collaborator" => Ok(Self::Collaborator),
            "viewer" => Ok(Self::Viewer),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for FleetRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for FleetRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Policy for a particular resource
///
/// Note that the Policy only describes access granted explicitly for this
/// resource.  The policies of parent resources can also cause a user to have
/// access to this resource.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct FleetRolePolicy {
    /// Roles directly assigned on this resource
    pub role_assignments: ::std::vec::Vec<FleetRoleRoleAssignment>,
}

impl FleetRolePolicy {
    pub fn builder() -> builder::FleetRolePolicy {
        Default::default()
    }
}

/// Describes the assignment of a particular role on a particular resource to a
/// particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, `RoleAssignment`s are
/// put into a `Policy` and that Policy is applied to a particular resource.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct FleetRoleRoleAssignment {
    pub identity_id: ::uuid::Uuid,
    pub identity_type: IdentityType,
    pub role_name: FleetRole,
}

impl FleetRoleRoleAssignment {
    pub fn builder() -> builder::FleetRoleRoleAssignment {
        Default::default()
    }
}

/// A Floating IP is a well-known IP address which can be attached and detached
/// from instances.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct FloatingIp {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The ID of the instance that this Floating IP is attached to, if it is
    /// presently in use.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instance_id: ::std::option::Option<::uuid::Uuid>,
    /// The IP address held by this resource.
    pub ip: ::std::net::IpAddr,
    /// The ID of the IP pool this resource belongs to.
    pub ip_pool_id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// The project this resource exists within.
    pub project_id: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl FloatingIp {
    pub fn builder() -> builder::FloatingIp {
        Default::default()
    }
}

/// Parameters for attaching a floating IP address to another resource
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct FloatingIpAttach {
    /// The type of `parent`'s resource
    pub kind: FloatingIpParentKind,
    /// Name or ID of the resource that this IP address should be attached to
    pub parent: NameOrId,
}

impl FloatingIpAttach {
    pub fn builder() -> builder::FloatingIpAttach {
        Default::default()
    }
}

/// Parameters for creating a new floating IP address for instances.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct FloatingIpCreate {
    /// IP address allocation method.
    #[serde(default = "defaults::floating_ip_create_address_allocator")]
    pub address_allocator: AddressAllocator,
    pub description: ::std::string::String,
    pub name: Name,
}

impl FloatingIpCreate {
    pub fn builder() -> builder::FloatingIpCreate {
        Default::default()
    }
}

/// The type of resource that a floating IP is attached to
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum FloatingIpParentKind {
    #[serde(rename = "instance")]
    Instance,
}

impl ::std::fmt::Display for FloatingIpParentKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Instance => f.write_str("instance"),
        }
    }
}

impl ::std::str::FromStr for FloatingIpParentKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "instance" => Ok(Self::Instance),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for FloatingIpParentKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for FloatingIpParentKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct FloatingIpResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<FloatingIp>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl FloatingIpResultsPage {
    pub fn builder() -> builder::FloatingIpResultsPage {
        Default::default()
    }
}

/// Updateable identity-related parameters
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct FloatingIpUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl FloatingIpUpdate {
    pub fn builder() -> builder::FloatingIpUpdate {
        Default::default()
    }
}

/// View of a Group
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Group {
    /// Human-readable name that can identify the group
    pub display_name: ::std::string::String,
    pub id: ::uuid::Uuid,
    /// Uuid of the silo to which this group belongs
    pub silo_id: ::uuid::Uuid,
    /// Timestamp when this group was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this group was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Group {
    pub fn builder() -> builder::Group {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct GroupResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Group>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl GroupResultsPage {
    pub fn builder() -> builder::GroupResultsPage {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramdouble {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Bindouble>,
    /// The maximum value of all samples in the histogram.
    pub max: f64,
    /// The minimum value of all samples in the histogram.
    pub min: f64,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: f64,
}

impl Histogramdouble {
    pub fn builder() -> builder::Histogramdouble {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramfloat {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Binfloat>,
    /// The maximum value of all samples in the histogram.
    pub max: f32,
    /// The minimum value of all samples in the histogram.
    pub min: f32,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: f64,
}

impl Histogramfloat {
    pub fn builder() -> builder::Histogramfloat {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramint16 {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Binint16>,
    /// The maximum value of all samples in the histogram.
    pub max: i16,
    /// The minimum value of all samples in the histogram.
    pub min: i16,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: i64,
}

impl Histogramint16 {
    pub fn builder() -> builder::Histogramint16 {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramint32 {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Binint32>,
    /// The maximum value of all samples in the histogram.
    pub max: i32,
    /// The minimum value of all samples in the histogram.
    pub min: i32,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: i64,
}

impl Histogramint32 {
    pub fn builder() -> builder::Histogramint32 {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramint64 {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Binint64>,
    /// The maximum value of all samples in the histogram.
    pub max: i64,
    /// The minimum value of all samples in the histogram.
    pub min: i64,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: i64,
}

impl Histogramint64 {
    pub fn builder() -> builder::Histogramint64 {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramint8 {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Binint8>,
    /// The maximum value of all samples in the histogram.
    pub max: i8,
    /// The minimum value of all samples in the histogram.
    pub min: i8,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: i64,
}

impl Histogramint8 {
    pub fn builder() -> builder::Histogramint8 {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramuint16 {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Binuint16>,
    /// The maximum value of all samples in the histogram.
    pub max: u16,
    /// The minimum value of all samples in the histogram.
    pub min: u16,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: i64,
}

impl Histogramuint16 {
    pub fn builder() -> builder::Histogramuint16 {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramuint32 {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Binuint32>,
    /// The maximum value of all samples in the histogram.
    pub max: u32,
    /// The minimum value of all samples in the histogram.
    pub min: u32,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: i64,
}

impl Histogramuint32 {
    pub fn builder() -> builder::Histogramuint32 {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramuint64 {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Binuint64>,
    /// The maximum value of all samples in the histogram.
    pub max: u64,
    /// The minimum value of all samples in the histogram.
    pub min: u64,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: i64,
}

impl Histogramuint64 {
    pub fn builder() -> builder::Histogramuint64 {
        Default::default()
    }
}

/// Histogram metric
///
/// A histogram maintains the count of any number of samples, over a set of
/// bins. Bins are specified on construction via their _left_ edges, inclusive.
/// There can't be any "gaps" in the bins, and an additional bin may be added to
/// the left, right, or both so that the bins extend to the entire range of the
/// support.
///
/// Note that any gaps, unsorted bins, or non-finite values will result in an
/// error.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Histogramuint8 {
    /// The bins of the histogram.
    pub bins: ::std::vec::Vec<Binuint8>,
    /// The maximum value of all samples in the histogram.
    pub max: u8,
    /// The minimum value of all samples in the histogram.
    pub min: u8,
    /// The total number of samples in the histogram.
    pub n_samples: u64,
    /// p50 Quantile
    pub p50: Quantile,
    /// p95 Quantile
    pub p90: Quantile,
    /// p99 Quantile
    pub p99: Quantile,
    /// M2 for Welford's algorithm for variance calculation.
    ///
    /// Read about [Welford's algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm) for more information on the algorithm.
    pub squared_mean: f64,
    /// The start time of the histogram.
    pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The sum of all samples in the histogram.
    pub sum_of_samples: i64,
}

impl Histogramuint8 {
    pub fn builder() -> builder::Histogramuint8 {
        Default::default()
    }
}

/// A hostname identifies a host on a network, and is usually a dot-delimited
/// sequence of labels, where each label contains only letters, digits, or the
/// hyphen. See RFCs 1035 and 952 for more details.
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct Hostname(::std::string::String);
impl ::std::ops::Deref for Hostname {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<Hostname> for ::std::string::String {
    fn from(value: Hostname) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for Hostname {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for Hostname {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }

        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }

        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^([a-zA-Z0-9]+[a-zA-Z0-9\\-]*(?<!-))(\\.[a-zA-Z0-9]+[a-zA-Z0-9\\-]*(?<!-))*$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \
                        \"^([a-zA-Z0-9]+[a-zA-Z0-9\\-]*(?<!-))(\\.[a-zA-Z0-9]+[a-zA-Z0-9\\-]*(?\
                        <!-))*$\""
                .into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for Hostname {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for Hostname {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// An inclusive-inclusive range of ICMP(v6) types or codes. The second value
/// may be omitted to represent a single parameter.
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct IcmpParamRange(::std::string::String);
impl ::std::ops::Deref for IcmpParamRange {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<IcmpParamRange> for ::std::string::String {
    fn from(value: IcmpParamRange) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for IcmpParamRange {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for IcmpParamRange {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 7usize {
            return Err("longer than 7 characters".into());
        }

        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }

        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[0-9]{1,3}(-[0-9]{1,3})?$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]{1,3}(-[0-9]{1,3})?$\"".into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for IcmpParamRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for IcmpParamRange {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// Supported set of sort modes for scanning by id only.
///
/// Currently, we only support scanning in ascending order.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum IdSortMode {
    /// Sort in increasing order of "id"
    #[serde(rename = "id_ascending")]
    IdAscending,
}

impl ::std::fmt::Display for IdSortMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::IdAscending => f.write_str("id_ascending"),
        }
    }
}

impl ::std::str::FromStr for IdSortMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "id_ascending" => Ok(Self::IdAscending),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for IdSortMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for IdSortMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// View of an Identity Provider
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IdentityProvider {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Identity provider type
    pub provider_type: IdentityProviderType,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl IdentityProvider {
    pub fn builder() -> builder::IdentityProvider {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IdentityProviderResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<IdentityProvider>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl IdentityProviderResultsPage {
    pub fn builder() -> builder::IdentityProviderResultsPage {
        Default::default()
    }
}

/// `IdentityProviderType`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum IdentityProviderType {
    /// SAML identity provider
    #[serde(rename = "saml")]
    Saml,
}

impl ::std::fmt::Display for IdentityProviderType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Saml => f.write_str("saml"),
        }
    }
}

impl ::std::str::FromStr for IdentityProviderType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "saml" => Ok(Self::Saml),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for IdentityProviderType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for IdentityProviderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Describes what kind of identity is described by an id
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum IdentityType {
    #[serde(rename = "silo_user")]
    SiloUser,
    #[serde(rename = "silo_group")]
    SiloGroup,
}

impl ::std::fmt::Display for IdentityType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SiloUser => f.write_str("silo_user"),
            Self::SiloGroup => f.write_str("silo_group"),
        }
    }
}

impl ::std::str::FromStr for IdentityType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "silo_user" => Ok(Self::SiloUser),
            "silo_group" => Ok(Self::SiloGroup),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for IdentityType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for IdentityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `IdpMetadataSource`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum IdpMetadataSource {
    #[serde(rename = "url")]
    Url { url: ::std::string::String },
    #[serde(rename = "base64_encoded_xml")]
    Base64EncodedXml { data: ::std::string::String },
}

/// View of an image
///
/// If `project_id` is present then the image is only visible inside that
/// project. If it's not present then the image is visible to all projects in
/// the silo.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Image {
    /// Size of blocks in bytes
    pub block_size: BlockSize,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Hash of the image contents, if applicable
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub digest: ::std::option::Option<Digest>,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// The family of the operating system like Debian, Ubuntu, etc.
    pub os: ::std::string::String,
    /// ID of the parent project if the image is a project image
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub project_id: ::std::option::Option<::uuid::Uuid>,
    /// Total size in bytes
    pub size: ByteCount,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Version of the operating system
    pub version: ::std::string::String,
}

impl Image {
    pub fn builder() -> builder::Image {
        Default::default()
    }
}

/// Create-time parameters for an `Image`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ImageCreate {
    pub description: ::std::string::String,
    pub name: Name,
    /// The family of the operating system (e.g. Debian, Ubuntu, etc.)
    pub os: ::std::string::String,
    /// The source of the image's contents.
    pub source: ImageSource,
    /// The version of the operating system (e.g. 18.04, 20.04, etc.)
    pub version: ::std::string::String,
}

impl ImageCreate {
    pub fn builder() -> builder::ImageCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ImageResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Image>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl ImageResultsPage {
    pub fn builder() -> builder::ImageResultsPage {
        Default::default()
    }
}

/// The source of the underlying image.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "id")]
pub enum ImageSource {
    #[serde(rename = "snapshot")]
    Snapshot(::uuid::Uuid),
}

impl ::std::convert::From<::uuid::Uuid> for ImageSource {
    fn from(value: ::uuid::Uuid) -> Self {
        Self::Snapshot(value)
    }
}

/// Parameters for importing blocks with a bulk write
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ImportBlocksBulkWrite {
    pub base64_encoded_data: ::std::string::String,
    pub offset: u64,
}

impl ImportBlocksBulkWrite {
    pub fn builder() -> builder::ImportBlocksBulkWrite {
        Default::default()
    }
}

/// Define policy relating to the import and export of prefixes from a BGP peer.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum ImportExportPolicy {
    #[serde(rename = "no_filtering")]
    NoFiltering,
    #[serde(rename = "allow")]
    Allow(::std::vec::Vec<IpNet>),
}

impl ::std::convert::From<::std::vec::Vec<IpNet>> for ImportExportPolicy {
    fn from(value: ::std::vec::Vec<IpNet>) -> Self {
        Self::Allow(value)
    }
}

/// View of an Instance
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Instance {
    /// The time at which the auto-restart cooldown period for this instance
    /// completes, permitting it to be automatically restarted again. If the
    /// instance enters the `Failed` state, it will not be restarted until after
    /// this time.
    ///
    /// If this is not present, then either the instance has never been
    /// automatically restarted, or the cooldown period has already expired,
    /// allowing the instance to be restarted immediately if it fails.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub auto_restart_cooldown_expiration:
        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// `true` if this instance's auto-restart policy will permit the control
    /// plane to automatically restart it if it enters the `Failed` state.
    pub auto_restart_enabled: bool,
    /// The auto-restart policy configured for this instance, or `null` if no
    /// explicit policy has been configured.
    ///
    /// This policy determines whether the instance should be automatically
    /// restarted by the control plane on failure. If this is `null`, the
    /// control plane will use the default policy when determining whether or
    /// not to automatically restart this instance, which may or may not allow
    /// it to be restarted. The value of the `auto_restart_enabled` field
    /// indicates whether the instance will be auto-restarted, based on its
    /// current policy or the default if it has no configured policy.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub auto_restart_policy: ::std::option::Option<InstanceAutoRestartPolicy>,
    /// The ID of the disk used to boot this instance, if a specific one is
    /// assigned
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub boot_disk_id: ::std::option::Option<::uuid::Uuid>,
    /// The CPU platform for this instance. If this is `null`, the instance
    /// requires no particular CPU platform.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cpu_platform: ::std::option::Option<InstanceCpuPlatform>,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// When true, this instance has opted in to jumbo frames (8500 byte MTU) on
    /// its primary network interface. The effective MTU also depends on the
    /// fleet-wide jumbo-frames opt-in; if that is disabled, the primary
    /// interface uses the default MTU regardless of this value. Changes only
    /// take effect on the next instance restart.
    pub enable_jumbo_frames: bool,
    /// RFC1035-compliant hostname for the instance
    pub hostname: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Memory allocated for this instance
    pub memory: ByteCount,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Number of CPUs allocated for this instance
    pub ncpus: InstanceCpuCount,
    /// ID for the project containing this instance
    pub project_id: ::uuid::Uuid,
    pub run_state: InstanceState,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The timestamp of the most recent time this instance was automatically
    /// restarted by the control plane.
    ///
    /// If this is not present, then this instance has not been automatically
    /// restarted.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time_last_auto_restarted: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub time_run_state_updated: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Instance {
    pub fn builder() -> builder::Instance {
        Default::default()
    }
}

/// A policy determining when an instance should be automatically restarted by
/// the control plane.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum InstanceAutoRestartPolicy {
    /// The instance should not be automatically restarted by the control plane
    /// if it fails.
    #[serde(rename = "never")]
    Never,
    /// If this instance is running and unexpectedly fails (e.g. due to a host
    /// software crash or unexpected host reboot), the control plane will make a
    /// best-effort attempt to restart it. The control plane may choose not to
    /// restart the instance to preserve the overall availability of the system.
    #[serde(rename = "best_effort")]
    BestEffort,
}

impl ::std::fmt::Display for InstanceAutoRestartPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Never => f.write_str("never"),
            Self::BestEffort => f.write_str("best_effort"),
        }
    }
}

impl ::std::str::FromStr for InstanceAutoRestartPolicy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "never" => Ok(Self::Never),
            "best_effort" => Ok(Self::BestEffort),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for InstanceAutoRestartPolicy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for InstanceAutoRestartPolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// The number of CPUs in an Instance
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct InstanceCpuCount(pub u16);
impl ::std::ops::Deref for InstanceCpuCount {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl ::std::convert::From<InstanceCpuCount> for u16 {
    fn from(value: InstanceCpuCount) -> Self {
        value.0
    }
}

impl ::std::convert::From<u16> for InstanceCpuCount {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl ::std::fmt::Display for InstanceCpuCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for InstanceCpuCount {
    type Err = <u16 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

/// A required CPU platform for an instance.
///
/// When an instance specifies a required CPU platform:
///
/// - The system may expose (to the VM) new CPU features that are only present
///   on that platform (or on newer platforms of the same lineage that also
///   support those features). - The instance must run on hosts that have CPUs
///   that support all the features of the supplied platform.
///
/// That is, the instance is restricted to hosts that have the CPUs which
/// support all features of the required platform, but in exchange the CPU
/// features exposed by the platform are available for the guest to use. Note
/// that this may prevent an instance from starting (if the hosts that could run
/// it are full but there is capacity on other incompatible hosts).
///
/// If an instance does not specify a required CPU platform, then when it
/// starts, the control plane selects a host for the instance and then supplies
/// the guest with the "minimum" CPU platform supported by that host. This
/// maximizes the number of hosts that can run the VM if it later needs to
/// migrate to another host.
///
/// In all cases, the CPU features presented by a given CPU platform are a
/// subset of what the corresponding hardware may actually support; features
/// which cannot be used from a virtual environment or do not have full
/// hypervisor support may be masked off.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum InstanceCpuPlatform {
    /// An AMD Milan-like CPU platform.
    #[serde(rename = "amd_milan")]
    AmdMilan,
    /// An AMD Turin-like CPU platform. Prefer `amd_turin_v2` over this; this
    /// CPU platform is retained for instances that specifically requested it
    /// before `amd_turin_v2` was added.
    ///
    /// This initial version of the Turin CPU platform includes no cache or TLB information in CPUID leaf `8000_0006`. While this was intentional, Oxide later discovered some guest software interprets the zeroed leaves as reporting cache sizes of 0 bytes and behaves incorrectly and unpredictably as a result (see [Propolis#1152](https://github.com/oxidecomputer/propolis/issues/1152)).
    #[serde(rename = "amd_turin")]
    AmdTurin,
    /// An AMD Turin-like CPU platform.
    ///
    /// This version of the Turin CPU platform includes cache and TLB
    /// information in CPUID leaf `8000_0006`, similar to the cache information
    /// included in the initial Milan-like CPU platform.
    #[serde(rename = "amd_turin_v2")]
    AmdTurinV2,
}

impl ::std::fmt::Display for InstanceCpuPlatform {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AmdMilan => f.write_str("amd_milan"),
            Self::AmdTurin => f.write_str("amd_turin"),
            Self::AmdTurinV2 => f.write_str("amd_turin_v2"),
        }
    }
}

impl ::std::str::FromStr for InstanceCpuPlatform {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "amd_milan" => Ok(Self::AmdMilan),
            "amd_turin" => Ok(Self::AmdTurin),
            "amd_turin_v2" => Ok(Self::AmdTurinV2),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for InstanceCpuPlatform {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for InstanceCpuPlatform {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Create-time parameters for an `Instance`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InstanceCreate {
    /// Anti-affinity groups to which this instance should be added.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub anti_affinity_groups: ::std::vec::Vec<NameOrId>,
    /// The auto-restart policy for this instance.
    ///
    /// This policy determines whether the instance should be automatically
    /// restarted by the control plane on failure. If this is `null`, no
    /// auto-restart policy will be explicitly configured for this instance, and
    /// the control plane will select the default policy when determining
    /// whether the instance can be automatically restarted.
    ///
    /// Currently, the global default auto-restart policy is "best-effort", so
    /// instances with `null` auto-restart policies will be automatically
    /// restarted. However, in the future, the default policy may be
    /// configurable through other mechanisms, such as on a per-project basis.
    /// In that case, any configured default policy will be used if this is
    /// `null`.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub auto_restart_policy: ::std::option::Option<InstanceAutoRestartPolicy>,
    /// The disk the instance is configured to boot from.
    ///
    /// This disk can either be attached if it already exists or created along
    /// with the instance.
    ///
    /// Specifying a boot disk is optional but recommended to ensure predictable
    /// boot behavior. The boot disk can be set during instance creation or
    /// later if the instance is stopped. The boot disk counts against the disk
    /// attachment limit.
    ///
    /// An instance that does not have a boot disk set will use the boot options
    /// specified in its UEFI settings, which are controlled by both the
    /// instance's UEFI firmware and the guest operating system. Boot options
    /// can change as disks are attached and detached, which may result in an
    /// instance that only boots to the EFI shell until a boot disk is set.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub boot_disk: ::std::option::Option<InstanceDiskAttachment>,
    /// The CPU platform to be used for this instance. If this is `null`, the
    /// instance requires no particular CPU platform; when it is started the
    /// instance will have the most general CPU platform supported by the sled
    /// it is initially placed on.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cpu_platform: ::std::option::Option<InstanceCpuPlatform>,
    pub description: ::std::string::String,
    /// A list of disks to be attached to the instance.
    ///
    /// Disk attachments of type "create" will be created, while those of type
    /// "attach" must already exist.
    ///
    /// The order of this list does not guarantee a boot order for the instance.
    /// Use the boot_disk attribute to specify a boot disk. When boot_disk is
    /// specified it will count against the disk attachment limit.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub disks: ::std::vec::Vec<InstanceDiskAttachment>,
    /// Enable jumbo frames (8500 byte MTU) on the instance's primary network
    /// interface. Requires the fleet-wide jumbo-frames opt-in to be enabled by
    /// an operator; otherwise this field must be `false`. Changes only take
    /// effect on the next instance restart.
    #[serde(default)]
    pub enable_jumbo_frames: bool,
    /// The external IP addresses provided to this instance.
    ///
    /// By default, all instances have outbound connectivity, but no inbound
    /// connectivity. These external addresses can be used to provide a fixed,
    /// known IP address for making inbound connections to the instance.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub external_ips: ::std::vec::Vec<ExternalIpCreate>,
    /// The hostname to be assigned to the instance
    pub hostname: Hostname,
    /// The amount of RAM (in bytes) to be allocated to the instance
    pub memory: ByteCount,
    /// Multicast groups this instance should join at creation.
    ///
    /// Groups can be specified by name, UUID, or IP address. Non-existent
    /// groups are created automatically.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub multicast_groups: ::std::vec::Vec<MulticastGroupJoinSpec>,
    pub name: Name,
    /// The number of vCPUs to be allocated to the instance
    pub ncpus: InstanceCpuCount,
    /// The network interfaces to be created for this instance.
    #[serde(default = "defaults::instance_create_network_interfaces")]
    pub network_interfaces: InstanceNetworkInterfaceAttachment,
    /// An allowlist of SSH public keys to be transferred to the instance via
    /// cloud-init during instance creation.
    ///
    /// If not provided, all SSH public keys from the user's profile will be
    /// sent. If an empty list is provided, no public keys will be transmitted
    /// to the instance.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ssh_public_keys: ::std::option::Option<::std::vec::Vec<NameOrId>>,
    /// Should this instance be started upon creation; true by default.
    #[serde(default = "defaults::default_bool::<true>")]
    pub start: bool,
    /// User data for instance initialization systems (such as cloud-init). Must
    /// be a Base64-encoded string, as specified in RFC 4648 § 4 (+ and /
    /// characters with padding). Maximum 32 KiB unencoded data.
    #[serde(default)]
    pub user_data: ::std::string::String,
}

impl InstanceCreate {
    pub fn builder() -> builder::InstanceCreate {
        Default::default()
    }
}

/// Describe the instance's disks at creation time
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum InstanceDiskAttachment {
    /// During instance creation, create and attach disks
    #[serde(rename = "create")]
    Create {
        description: ::std::string::String,
        /// The source for this `Disk`'s blocks
        disk_backend: DiskBackend,
        name: Name,
        /// The total size of the Disk (in bytes)
        size: ByteCount,
    },
    /// During instance creation, attach this disk
    #[serde(rename = "attach")]
    Attach {
        /// A disk name to attach
        name: Name,
    },
}

/// Parameters for joining an instance to a multicast group.
///
/// When joining by IP address, the pool containing the multicast IP is
/// auto-discovered from all linked multicast pools.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct InstanceMulticastGroupJoin {
    /// IP version for pool selection when creating a group by name. Required if
    /// both IPv4 and IPv6 default multicast pools are linked.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ip_version: ::std::option::Option<IpVersion>,
    /// Source IPs for source-filtered multicast (SSM). Optional for ASM groups,
    /// required for SSM groups (232.0.0.0/8, ff3x::/32).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source_ips: ::std::option::Option<::std::vec::Vec<::std::net::IpAddr>>,
}

impl InstanceMulticastGroupJoin {
    pub fn builder() -> builder::InstanceMulticastGroupJoin {
        Default::default()
    }
}

/// An `InstanceNetworkInterface` represents a virtual network interface device
/// attached to an instance.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InstanceNetworkInterface {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The Instance to which the interface belongs.
    pub instance_id: ::uuid::Uuid,
    /// The VPC-private IP stack for this interface.
    pub ip_stack: PrivateIpStack,
    /// The MAC address assigned to this interface.
    pub mac: MacAddr,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// True if this interface is the primary for the instance to which it's
    /// attached.
    pub primary: bool,
    /// The subnet to which the interface belongs.
    pub subnet_id: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The VPC to which the interface belongs.
    pub vpc_id: ::uuid::Uuid,
}

impl InstanceNetworkInterface {
    pub fn builder() -> builder::InstanceNetworkInterface {
        Default::default()
    }
}

/// Describes an attachment of an `InstanceNetworkInterface` to an `Instance`,
/// at the time the instance is created.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "params")]
pub enum InstanceNetworkInterfaceAttachment {
    /// Create one or more `InstanceNetworkInterface`s for the `Instance`.
    ///
    /// If more than one interface is provided, then the first will be
    /// designated the primary interface for the instance.
    #[serde(rename = "create")]
    Create(::std::vec::Vec<InstanceNetworkInterfaceCreate>),
    #[serde(rename = "default_ipv4")]
    DefaultIpv4,
    #[serde(rename = "default_ipv6")]
    DefaultIpv6,
    #[serde(rename = "default_dual_stack")]
    DefaultDualStack,
    #[serde(rename = "none")]
    None,
}

impl ::std::convert::From<::std::vec::Vec<InstanceNetworkInterfaceCreate>>
    for InstanceNetworkInterfaceAttachment
{
    fn from(value: ::std::vec::Vec<InstanceNetworkInterfaceCreate>) -> Self {
        Self::Create(value)
    }
}

/// Create-time parameters for an `InstanceNetworkInterface`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InstanceNetworkInterfaceCreate {
    pub description: ::std::string::String,
    /// The IP stack configuration for this interface.
    ///
    /// If not provided, a default configuration will be used, which creates a
    /// dual-stack IPv4 / IPv6 interface.
    #[serde(default = "defaults::instance_network_interface_create_ip_config")]
    pub ip_config: PrivateIpStackCreate,
    pub name: Name,
    /// The VPC Subnet in which to create the interface.
    pub subnet_name: Name,
    /// The VPC in which to create the interface.
    pub vpc_name: Name,
}

impl InstanceNetworkInterfaceCreate {
    pub fn builder() -> builder::InstanceNetworkInterfaceCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InstanceNetworkInterfaceResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<InstanceNetworkInterface>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl InstanceNetworkInterfaceResultsPage {
    pub fn builder() -> builder::InstanceNetworkInterfaceResultsPage {
        Default::default()
    }
}

/// Parameters for updating an `InstanceNetworkInterface`
///
/// Note that modifying IP addresses for an interface is not yet supported, a
/// new interface must be created instead.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct InstanceNetworkInterfaceUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
    /// Make a secondary interface the instance's primary interface.
    ///
    /// If applied to a secondary interface, that interface will become the
    /// primary on the next reboot of the instance. Note that this may have
    /// implications for routing between instances, as the new primary interface
    /// will be on a distinct subnet from the previous primary interface.
    ///
    /// Note that this can only be used to select a new primary interface for an
    /// instance. Requests to change the primary interface into a secondary will
    /// return an error.
    #[serde(default)]
    pub primary: bool,
    /// A set of additional networks that this interface may send and receive
    /// traffic on
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub transit_ips: ::std::vec::Vec<IpNet>,
}

impl InstanceNetworkInterfaceUpdate {
    pub fn builder() -> builder::InstanceNetworkInterfaceUpdate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InstanceResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Instance>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl InstanceResultsPage {
    pub fn builder() -> builder::InstanceResultsPage {
        Default::default()
    }
}

/// Contents of an Instance's serial console buffer.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InstanceSerialConsoleData {
    /// The bytes starting from the requested offset up to either the end of the
    /// buffer or the request's `max_bytes`. Provided as a u8 array rather than
    /// a string, as it may not be UTF-8.
    pub data: ::std::vec::Vec<u8>,
    /// The absolute offset since boot (suitable for use as `byte_offset` in a
    /// subsequent request) of the last byte returned in `data`.
    pub last_byte_offset: u64,
}

impl InstanceSerialConsoleData {
    pub fn builder() -> builder::InstanceSerialConsoleData {
        Default::default()
    }
}

/// Running state of an Instance (primarily: booted or stopped)
///
/// This typically reflects whether it's starting, running, stopping, or
/// stopped, but also includes states related to the Instance's lifecycle
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum InstanceState {
    /// The instance is being created.
    #[serde(rename = "creating")]
    Creating,
    /// The instance is currently starting up.
    #[serde(rename = "starting")]
    Starting,
    /// The instance is currently running.
    #[serde(rename = "running")]
    Running,
    /// The instance has been requested to stop and a transition to "Stopped" is
    /// imminent.
    #[serde(rename = "stopping")]
    Stopping,
    /// The instance is currently stopped.
    #[serde(rename = "stopped")]
    Stopped,
    /// The instance is in the process of rebooting - it will remain in the
    /// "rebooting" state until the VM is starting once more.
    #[serde(rename = "rebooting")]
    Rebooting,
    /// The instance is in the process of migrating - it will remain in the
    /// "migrating" state until the migration process is complete and the
    /// destination propolis is ready to continue execution.
    #[serde(rename = "migrating")]
    Migrating,
    /// The instance is attempting to recover from a failure.
    #[serde(rename = "repairing")]
    Repairing,
    /// The instance has encountered a failure.
    #[serde(rename = "failed")]
    Failed,
    /// The instance has been deleted.
    #[serde(rename = "destroyed")]
    Destroyed,
}

impl ::std::fmt::Display for InstanceState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Creating => f.write_str("creating"),
            Self::Starting => f.write_str("starting"),
            Self::Running => f.write_str("running"),
            Self::Stopping => f.write_str("stopping"),
            Self::Stopped => f.write_str("stopped"),
            Self::Rebooting => f.write_str("rebooting"),
            Self::Migrating => f.write_str("migrating"),
            Self::Repairing => f.write_str("repairing"),
            Self::Failed => f.write_str("failed"),
            Self::Destroyed => f.write_str("destroyed"),
        }
    }
}

impl ::std::str::FromStr for InstanceState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "creating" => Ok(Self::Creating),
            "starting" => Ok(Self::Starting),
            "running" => Ok(Self::Running),
            "stopping" => Ok(Self::Stopping),
            "stopped" => Ok(Self::Stopped),
            "rebooting" => Ok(Self::Rebooting),
            "migrating" => Ok(Self::Migrating),
            "repairing" => Ok(Self::Repairing),
            "failed" => Ok(Self::Failed),
            "destroyed" => Ok(Self::Destroyed),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for InstanceState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for InstanceState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Parameters of an `Instance` that can be reconfigured after creation.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InstanceUpdate {
    /// The auto-restart policy for this instance.
    ///
    /// This policy determines whether the instance should be automatically
    /// restarted by the control plane on failure. If this is `null`, any
    /// explicitly configured auto-restart policy will be unset, and the control
    /// plane will select the default policy when determining whether the
    /// instance can be automatically restarted.
    ///
    /// Currently, the global default auto-restart policy is "best-effort", so
    /// instances with `null` auto-restart policies will be automatically
    /// restarted. However, in the future, the default policy may be
    /// configurable through other mechanisms, such as on a per-project basis.
    /// In that case, any configured default policy will be used if this is
    /// `null`.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub auto_restart_policy: ::std::option::Option<InstanceAutoRestartPolicy>,
    /// The disk the instance is configured to boot from.
    ///
    /// Setting a boot disk is optional but recommended to ensure predictable
    /// boot behavior. The boot disk can be set during instance creation or
    /// later if the instance is stopped. The boot disk counts against the disk
    /// attachment limit.
    ///
    /// An instance that does not have a boot disk set will use the boot options
    /// specified in its UEFI settings, which are controlled by both the
    /// instance's UEFI firmware and the guest operating system. Boot options
    /// can change as disks are attached and detached, which may result in an
    /// instance that only boots to the EFI shell until a boot disk is set.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub boot_disk: ::std::option::Option<NameOrId>,
    /// The CPU platform to be used for this instance. If this is `null`, the
    /// instance requires no particular CPU platform; when it is started the
    /// instance will have the most general CPU platform supported by the sled
    /// it is initially placed on.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub cpu_platform: ::std::option::Option<InstanceCpuPlatform>,
    /// Update the per-instance jumbo-frames opt-in. Setting this to `true`
    /// requires the fleet-wide jumbo-frames opt-in to be enabled. Changes only
    /// take effect on the next instance restart.
    pub enable_jumbo_frames: bool,
    /// The amount of RAM (in bytes) to be allocated to the instance
    pub memory: ByteCount,
    /// Multicast groups this instance should join.
    ///
    /// When specified, this replaces the instance's current multicast group
    /// membership with the new set of groups. The instance will leave any
    /// groups not listed here and join any new groups that are specified.
    ///
    /// Each entry can specify the group by name, UUID, or IP address, along
    /// with optional source IP filtering for SSM (Source-Specific Multicast).
    /// When a group doesn't exist, it will be implicitly created using the
    /// default multicast pool (or you can specify `ip_version` to disambiguate
    /// if needed).
    ///
    /// If not provided, the instance's multicast group membership will not be
    /// changed.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub multicast_groups: ::std::option::Option<::std::vec::Vec<MulticastGroupJoinSpec>>,
    /// The number of vCPUs to be allocated to the instance
    pub ncpus: InstanceCpuCount,
}

impl InstanceUpdate {
    pub fn builder() -> builder::InstanceUpdate {
        Default::default()
    }
}

/// `InterfaceNum`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub enum InterfaceNum {
    #[serde(rename = "unknown")]
    Unknown(u32),
    #[serde(rename = "if_index")]
    IfIndex(u32),
    #[serde(rename = "port_number")]
    PortNumber(u32),
}

/// An internet gateway provides a path between VPC networks and external
/// networks.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InternetGateway {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The VPC to which the gateway belongs.
    pub vpc_id: ::uuid::Uuid,
}

impl InternetGateway {
    pub fn builder() -> builder::InternetGateway {
        Default::default()
    }
}

/// Create-time parameters for an `InternetGateway`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InternetGatewayCreate {
    pub description: ::std::string::String,
    pub name: Name,
}

impl InternetGatewayCreate {
    pub fn builder() -> builder::InternetGatewayCreate {
        Default::default()
    }
}

/// An IP address that is attached to an internet gateway
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InternetGatewayIpAddress {
    /// The associated IP address
    pub address: ::std::net::IpAddr,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The associated internet gateway
    pub internet_gateway_id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl InternetGatewayIpAddress {
    pub fn builder() -> builder::InternetGatewayIpAddress {
        Default::default()
    }
}

/// Create-time identity-related parameters
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InternetGatewayIpAddressCreate {
    pub address: ::std::net::IpAddr,
    pub description: ::std::string::String,
    pub name: Name,
}

impl InternetGatewayIpAddressCreate {
    pub fn builder() -> builder::InternetGatewayIpAddressCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InternetGatewayIpAddressResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<InternetGatewayIpAddress>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl InternetGatewayIpAddressResultsPage {
    pub fn builder() -> builder::InternetGatewayIpAddressResultsPage {
        Default::default()
    }
}

/// An IP pool that is attached to an internet gateway
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InternetGatewayIpPool {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The associated internet gateway.
    pub internet_gateway_id: ::uuid::Uuid,
    /// The associated IP pool.
    pub ip_pool_id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl InternetGatewayIpPool {
    pub fn builder() -> builder::InternetGatewayIpPool {
        Default::default()
    }
}

/// Create-time identity-related parameters
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InternetGatewayIpPoolCreate {
    pub description: ::std::string::String,
    pub ip_pool: NameOrId,
    pub name: Name,
}

impl InternetGatewayIpPoolCreate {
    pub fn builder() -> builder::InternetGatewayIpPoolCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InternetGatewayIpPoolResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<InternetGatewayIpPool>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl InternetGatewayIpPoolResultsPage {
    pub fn builder() -> builder::InternetGatewayIpPoolResultsPage {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct InternetGatewayResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<InternetGateway>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl InternetGatewayResultsPage {
    pub fn builder() -> builder::InternetGatewayResultsPage {
        Default::default()
    }
}

/// `IpNet`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(untagged)]
pub enum IpNet {
    V4(Ipv4Net),
    V6(Ipv6Net),
}

impl ::std::fmt::Display for IpNet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::V4(x) => x.fmt(f),
            Self::V6(x) => x.fmt(f),
        }
    }
}

impl ::std::str::FromStr for IpNet {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::V4(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::V6(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}

impl ::std::convert::TryFrom<&str> for IpNet {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for IpNet {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::From<Ipv4Net> for IpNet {
    fn from(value: Ipv4Net) -> Self {
        Self::V4(value)
    }
}

impl ::std::convert::From<Ipv6Net> for IpNet {
    fn from(value: Ipv6Net) -> Self {
        Self::V6(value)
    }
}

/// A collection of IP ranges.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPool {
    /// What this pool is currently assigned to.
    pub assignment: IpPoolAssignment,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The IP version for the pool.
    pub ip_version: IpVersion,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Type of IP pool (unicast or multicast).
    pub pool_type: IpPoolType,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl IpPool {
    pub fn builder() -> builder::IpPool {
        Default::default()
    }
}

/// Body parameters for reassigning an IP pool.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolAssignParam {
    pub assignment: IpPoolAssignment,
}

impl IpPoolAssignParam {
    pub fn builder() -> builder::IpPoolAssignParam {
        Default::default()
    }
}

/// Assignment of an IP pool to resources and services.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum IpPoolAssignment {
    /// Pool is available to be linked to customer silos.
    #[serde(rename = "silos")]
    Silos,
    /// Pool is reserved for Oxide-operated rack services (NTP, DNS, etc.).
    #[serde(rename = "system_services")]
    SystemServices,
}

impl ::std::fmt::Display for IpPoolAssignment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Silos => f.write_str("silos"),
            Self::SystemServices => f.write_str("system_services"),
        }
    }
}

impl ::std::str::FromStr for IpPoolAssignment {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "silos" => Ok(Self::Silos),
            "system_services" => Ok(Self::SystemServices),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for IpPoolAssignment {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for IpPoolAssignment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Create-time parameters for an `IpPool`.
///
/// For multicast pools, all ranges must be either Any-Source Multicast (ASM) or
/// Source-Specific Multicast (SSM), but not both. Mixing ASM and SSM ranges in
/// the same pool is not allowed.
///
/// ASM: IPv4 addresses outside 232.0.0.0/8, IPv6 addresses with flag field != 3
/// SSM: IPv4 addresses in 232.0.0.0/8, IPv6 addresses with flag field = 3
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolCreate {
    /// What this pool is assigned to (defaults to Silos).
    #[serde(default = "defaults::ip_pool_create_assignment")]
    pub assignment: IpPoolAssignment,
    pub description: ::std::string::String,
    /// The IP version of the pool.
    ///
    /// The default is IPv4.
    #[serde(default = "defaults::ip_pool_create_ip_version")]
    pub ip_version: IpVersion,
    pub name: Name,
    /// Type of IP pool (defaults to Unicast)
    #[serde(default = "defaults::ip_pool_create_pool_type")]
    pub pool_type: IpPoolType,
}

impl IpPoolCreate {
    pub fn builder() -> builder::IpPoolCreate {
        Default::default()
    }
}

/// `IpPoolLinkSilo`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolLinkSilo {
    /// When a pool is the default for a silo, floating IPs and instance
    /// ephemeral IPs will come from that pool when no other pool is specified.
    ///
    /// A silo can have at most one default pool per combination of pool type
    /// (unicast or multicast) and IP version (IPv4 or IPv6), allowing up to 4
    /// default pools total.
    pub is_default: bool,
    pub silo: NameOrId,
}

impl IpPoolLinkSilo {
    pub fn builder() -> builder::IpPoolLinkSilo {
        Default::default()
    }
}

/// `IpPoolRange`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolRange {
    pub id: ::uuid::Uuid,
    pub ip_pool_id: ::uuid::Uuid,
    pub range: IpRange,
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl IpPoolRange {
    pub fn builder() -> builder::IpPoolRange {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolRangeResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<IpPoolRange>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl IpPoolRangeResultsPage {
    pub fn builder() -> builder::IpPoolRangeResultsPage {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<IpPool>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl IpPoolResultsPage {
    pub fn builder() -> builder::IpPoolResultsPage {
        Default::default()
    }
}

/// A link between an IP pool and a silo that allows one to allocate IPs from
/// the pool within the silo
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolSiloLink {
    pub ip_pool_id: ::uuid::Uuid,
    /// When a pool is the default for a silo, floating IPs and instance
    /// ephemeral IPs will come from that pool when no other pool is specified.
    ///
    /// A silo can have at most one default pool per combination of pool type
    /// (unicast or multicast) and IP version (IPv4 or IPv6), allowing up to 4
    /// default pools total.
    pub is_default: bool,
    pub silo_id: ::uuid::Uuid,
}

impl IpPoolSiloLink {
    pub fn builder() -> builder::IpPoolSiloLink {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolSiloLinkResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<IpPoolSiloLink>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl IpPoolSiloLinkResultsPage {
    pub fn builder() -> builder::IpPoolSiloLinkResultsPage {
        Default::default()
    }
}

/// `IpPoolSiloUpdate`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolSiloUpdate {
    /// When a pool is the default for a silo, floating IPs and instance
    /// ephemeral IPs will come from that pool when no other pool is specified.
    ///
    /// A silo can have at most one default pool per combination of pool type
    /// (unicast or multicast) and IP version (IPv4 or IPv6), allowing up to 4
    /// default pools total. When a pool is made default, an existing default of
    /// the same type and version will remain linked but will no longer be the
    /// default.
    pub is_default: bool,
}

impl IpPoolSiloUpdate {
    pub fn builder() -> builder::IpPoolSiloUpdate {
        Default::default()
    }
}

/// Type of IP pool.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum IpPoolType {
    /// Unicast IP pool for standard IP allocations.
    #[serde(rename = "unicast")]
    Unicast,
    /// Multicast IP pool for multicast group allocations.
    ///
    /// All ranges in a multicast pool must be either ASM or SSM (not mixed).
    #[serde(rename = "multicast")]
    Multicast,
}

impl ::std::fmt::Display for IpPoolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unicast => f.write_str("unicast"),
            Self::Multicast => f.write_str("multicast"),
        }
    }
}

impl ::std::str::FromStr for IpPoolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unicast" => Ok(Self::Unicast),
            "multicast" => Ok(Self::Multicast),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for IpPoolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for IpPoolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Parameters for updating an IP Pool
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct IpPoolUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl IpPoolUpdate {
    pub fn builder() -> builder::IpPoolUpdate {
        Default::default()
    }
}

/// The utilization of IP addresses in a pool.
///
/// Note that both the count of remaining addresses and the total capacity are
/// integers, reported as floating point numbers. This accommodates allocations
/// larger than a 64-bit integer, which is common with IPv6 address spaces. With
/// very large IP Pools (> 2**53 addresses), integer precision will be lost, in
/// exchange for representing the entire range. In such a case the pool still
/// has many available addresses.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct IpPoolUtilization {
    /// The total number of addresses in the pool.
    pub capacity: f64,
    /// The number of remaining addresses in the pool.
    pub remaining: f64,
}

impl IpPoolUtilization {
    pub fn builder() -> builder::IpPoolUtilization {
        Default::default()
    }
}

/// `IpRange`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(untagged)]
pub enum IpRange {
    V4(Ipv4Range),
    V6(Ipv6Range),
}

impl ::std::convert::From<Ipv4Range> for IpRange {
    fn from(value: Ipv4Range) -> Self {
        Self::V4(value)
    }
}

impl ::std::convert::From<Ipv6Range> for IpRange {
    fn from(value: Ipv6Range) -> Self {
        Self::V6(value)
    }
}

/// The IP address version.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum IpVersion {
    #[serde(rename = "v4")]
    V4,
    #[serde(rename = "v6")]
    V6,
}

impl ::std::fmt::Display for IpVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::V4 => f.write_str("v4"),
            Self::V6 => f.write_str("v6"),
        }
    }
}

impl ::std::str::FromStr for IpVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "v4" => Ok(Self::V4),
            "v6" => Ok(Self::V6),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for IpVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for IpVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// How a VPC-private IP address is assigned to a network interface.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Ipv4Assignment {
    #[serde(rename = "auto")]
    Auto,
    /// Explicitly assign a specific address, if available.
    #[serde(rename = "explicit")]
    Explicit(::std::net::Ipv4Addr),
}

impl ::std::convert::From<::std::net::Ipv4Addr> for Ipv4Assignment {
    fn from(value: ::std::net::Ipv4Addr) -> Self {
        Self::Explicit(value)
    }
}

/// An IPv4 subnet, including prefix and prefix length
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct Ipv4Net(::std::string::String);
impl ::std::ops::Deref for Ipv4Net {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<Ipv4Net> for ::std::string::String {
    fn from(value: Ipv4Net) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for Ipv4Net {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for Ipv4Net {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.\
                     ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])/\
                     ([0-9]|1[0-9]|2[0-9]|3[0-2])$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \
                        \"^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.\
                        ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])/\
                        ([0-9]|1[0-9]|2[0-9]|3[0-2])$\""
                .into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for Ipv4Net {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for Ipv4Net {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// A non-decreasing IPv4 address range, inclusive of both ends.
///
/// The first address must be less than or equal to the last address.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Ipv4Range {
    pub first: ::std::net::Ipv4Addr,
    pub last: ::std::net::Ipv4Addr,
}

impl Ipv4Range {
    pub fn builder() -> builder::Ipv4Range {
        Default::default()
    }
}

/// How a VPC-private IP address is assigned to a network interface.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Ipv6Assignment {
    #[serde(rename = "auto")]
    Auto,
    /// Explicitly assign a specific address, if available.
    #[serde(rename = "explicit")]
    Explicit(::std::net::Ipv6Addr),
}

impl ::std::convert::From<::std::net::Ipv6Addr> for Ipv6Assignment {
    fn from(value: ::std::net::Ipv6Addr) -> Self {
        Self::Explicit(value)
    }
}

/// An IPv6 subnet, including prefix and subnet mask
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct Ipv6Net(::std::string::String);
impl ::std::ops::Deref for Ipv6Net {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<Ipv6Net> for ::std::string::String {
    fn from(value: Ipv6Net) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for Ipv6Net {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for Ipv6Net {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:\
                     |([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:\
                     [0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,\
                     3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,\
                     2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:\
                     [0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,\
                     }|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\\.\
                     ){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:\
                     ((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\\.){3,3}(25[0-5]|(2[0-4]|1{0,\
                     1}[0-9]){0,1}[0-9]))\\/([0-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8])$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \
                        \"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:\
                        |([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:\
                        [0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,\
                        3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:\
                        ){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,\
                        6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%\
                        [0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,\
                        1}[0-9]){0,1}[0-9])\\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,\
                        1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,\
                        1}[0-9])\\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))\\/\
                        ([0-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8])$\""
                .into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for Ipv6Net {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for Ipv6Net {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// A non-decreasing IPv6 address range, inclusive of both ends.
///
/// The first address must be less than or equal to the last address.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Ipv6Range {
    pub first: ::std::net::Ipv6Addr,
    pub last: ::std::net::Ipv6Addr,
}

impl Ipv6Range {
    pub fn builder() -> builder::Ipv6Range {
        Default::default()
    }
}

/// An inclusive-inclusive range of IP ports. The second port may be omitted to
/// represent a single port.
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct L4PortRange(::std::string::String);
impl ::std::ops::Deref for L4PortRange {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<L4PortRange> for ::std::string::String {
    fn from(value: L4PortRange) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for L4PortRange {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for L4PortRange {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 11usize {
            return Err("longer than 11 characters".into());
        }

        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }

        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[0-9]{1,5}(-[0-9]{1,5})?$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9]{1,5}(-[0-9]{1,5})?$\"".into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for L4PortRange {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for L4PortRange {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// Switch link configuration.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct LinkConfigCreate {
    /// Whether or not to set autonegotiation.
    pub autoneg: bool,
    /// The requested forward-error correction method.  If this is not
    /// specified, the standard FEC for the underlying media will be applied if
    /// it can be determined.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fec: ::std::option::Option<LinkFec>,
    /// Link name. On ports that are not broken out, this is always phy0. On a
    /// 2x breakout the options are phy0 and phy1, on 4x phy0-phy3, etc.
    pub link_name: Name,
    /// The link-layer discovery protocol (LLDP) configuration for the link.
    pub lldp: LldpLinkConfigCreate,
    /// Maximum transmission unit for the link.
    pub mtu: u16,
    /// The speed of the link.
    pub speed: LinkSpeed,
    /// Optional tx_eq settings.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_eq: ::std::option::Option<TxEqConfig>,
}

impl LinkConfigCreate {
    pub fn builder() -> builder::LinkConfigCreate {
        Default::default()
    }
}

/// The forward error correction mode of a link.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum LinkFec {
    /// Firecode forward error correction.
    #[serde(rename = "firecode")]
    Firecode,
    /// No forward error correction.
    #[serde(rename = "none")]
    None,
    /// Reed-Solomon forward error correction.
    #[serde(rename = "rs")]
    Rs,
}

impl ::std::fmt::Display for LinkFec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Firecode => f.write_str("firecode"),
            Self::None => f.write_str("none"),
            Self::Rs => f.write_str("rs"),
        }
    }
}

impl ::std::str::FromStr for LinkFec {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "firecode" => Ok(Self::Firecode),
            "none" => Ok(Self::None),
            "rs" => Ok(Self::Rs),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for LinkFec {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for LinkFec {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// The speed of a link.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum LinkSpeed {
    /// Zero gigabits per second.
    #[serde(rename = "speed0_g")]
    Speed0G,
    /// 1 gigabit per second.
    #[serde(rename = "speed1_g")]
    Speed1G,
    /// 10 gigabits per second.
    #[serde(rename = "speed10_g")]
    Speed10G,
    /// 25 gigabits per second.
    #[serde(rename = "speed25_g")]
    Speed25G,
    /// 40 gigabits per second.
    #[serde(rename = "speed40_g")]
    Speed40G,
    /// 50 gigabits per second.
    #[serde(rename = "speed50_g")]
    Speed50G,
    /// 100 gigabits per second.
    #[serde(rename = "speed100_g")]
    Speed100G,
    /// 200 gigabits per second.
    #[serde(rename = "speed200_g")]
    Speed200G,
    /// 400 gigabits per second.
    #[serde(rename = "speed400_g")]
    Speed400G,
}

impl ::std::fmt::Display for LinkSpeed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Speed0G => f.write_str("speed0_g"),
            Self::Speed1G => f.write_str("speed1_g"),
            Self::Speed10G => f.write_str("speed10_g"),
            Self::Speed25G => f.write_str("speed25_g"),
            Self::Speed40G => f.write_str("speed40_g"),
            Self::Speed50G => f.write_str("speed50_g"),
            Self::Speed100G => f.write_str("speed100_g"),
            Self::Speed200G => f.write_str("speed200_g"),
            Self::Speed400G => f.write_str("speed400_g"),
        }
    }
}

impl ::std::str::FromStr for LinkSpeed {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "speed0_g" => Ok(Self::Speed0G),
            "speed1_g" => Ok(Self::Speed1G),
            "speed10_g" => Ok(Self::Speed10G),
            "speed25_g" => Ok(Self::Speed25G),
            "speed40_g" => Ok(Self::Speed40G),
            "speed50_g" => Ok(Self::Speed50G),
            "speed100_g" => Ok(Self::Speed100G),
            "speed200_g" => Ok(Self::Speed200G),
            "speed400_g" => Ok(Self::Speed400G),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for LinkSpeed {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for LinkSpeed {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A link layer discovery protocol (LLDP) service configuration.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct LldpLinkConfig {
    /// The LLDP chassis identifier TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub chassis_id: ::std::option::Option<::std::string::String>,
    /// Whether or not the LLDP service is enabled.
    pub enabled: bool,
    /// The id of this LLDP service instance.
    pub id: ::uuid::Uuid,
    /// The LLDP link description TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub link_description: ::std::option::Option<::std::string::String>,
    /// The LLDP link name TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub link_name: ::std::option::Option<::std::string::String>,
    /// The LLDP management IP TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub management_ip: ::std::option::Option<::std::net::IpAddr>,
    /// The LLDP system description TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system_description: ::std::option::Option<::std::string::String>,
    /// The LLDP system name TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system_name: ::std::option::Option<::std::string::String>,
}

impl LldpLinkConfig {
    pub fn builder() -> builder::LldpLinkConfig {
        Default::default()
    }
}

/// The LLDP configuration associated with a port.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct LldpLinkConfigCreate {
    /// The LLDP chassis identifier TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub chassis_id: ::std::option::Option<::std::string::String>,
    /// Whether or not LLDP is enabled.
    pub enabled: bool,
    /// The LLDP link description TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub link_description: ::std::option::Option<::std::string::String>,
    /// The LLDP link name TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub link_name: ::std::option::Option<::std::string::String>,
    /// The LLDP management IP TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub management_ip: ::std::option::Option<::std::net::IpAddr>,
    /// The LLDP system description TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system_description: ::std::option::Option<::std::string::String>,
    /// The LLDP system name TLV.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system_name: ::std::option::Option<::std::string::String>,
}

impl LldpLinkConfigCreate {
    pub fn builder() -> builder::LldpLinkConfigCreate {
        Default::default()
    }
}

/// Information about LLDP advertisements from other network entities directly
/// connected to a switch port.  This structure contains both metadata about
/// when and where the neighbor was seen, as well as the specific information
/// the neighbor was advertising.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct LldpNeighbor {
    /// The LLDP chassis identifier advertised by the neighbor
    pub chassis_id: ::std::string::String,
    /// Initial sighting of this LldpNeighbor
    pub first_seen: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Most recent sighting of this LldpNeighbor
    pub last_seen: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The LLDP link description advertised by the neighbor
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub link_description: ::std::option::Option<::std::string::String>,
    /// The LLDP link name advertised by the neighbor
    pub link_name: ::std::string::String,
    /// The port on which the neighbor was seen
    pub local_port: ::std::string::String,
    /// The LLDP management IP(s) advertised by the neighbor
    pub management_ip: ::std::vec::Vec<ManagementAddress>,
    /// The LLDP system description advertised by the neighbor
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system_description: ::std::option::Option<::std::string::String>,
    /// The LLDP system name advertised by the neighbor
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system_name: ::std::option::Option<::std::string::String>,
}

impl LldpNeighbor {
    pub fn builder() -> builder::LldpNeighbor {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct LldpNeighborResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<LldpNeighbor>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl LldpNeighborResultsPage {
    pub fn builder() -> builder::LldpNeighborResultsPage {
        Default::default()
    }
}

/// A loopback address is an address that is assigned to a rack switch but is
/// not associated with any particular port.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct LoopbackAddress {
    /// The loopback IP address and prefix length.
    pub address: IpNet,
    /// The address lot block this address came from.
    pub address_lot_block_id: ::uuid::Uuid,
    /// The id of the loopback address.
    pub id: ::uuid::Uuid,
    /// The id of the rack where this loopback address is assigned.
    pub rack_id: ::uuid::Uuid,
    /// The slot of the switch within the rack where this loopback address is
    /// assigned.
    pub switch_slot: SwitchSlot,
}

impl LoopbackAddress {
    pub fn builder() -> builder::LoopbackAddress {
        Default::default()
    }
}

/// Parameters for creating a loopback address on a particular rack switch.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct LoopbackAddressCreate {
    /// The address to create.
    pub address: ::std::net::IpAddr,
    /// The name or id of the address lot this loopback address will pull an
    /// address from.
    pub address_lot: NameOrId,
    /// Address is an anycast address.
    ///
    /// This allows the address to be assigned to multiple locations
    /// simultaneously.
    pub anycast: bool,
    /// The subnet mask to use for the address.
    pub mask: u8,
    /// The rack containing the switch this loopback address will be configured
    /// on.
    pub rack_id: ::uuid::Uuid,
    /// The slot of the switch within the rack this loopback address will be
    /// configured on.
    pub switch_slot: SwitchSlot,
}

impl LoopbackAddressCreate {
    pub fn builder() -> builder::LoopbackAddressCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct LoopbackAddressResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<LoopbackAddress>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl LoopbackAddressResultsPage {
    pub fn builder() -> builder::LoopbackAddressResultsPage {
        Default::default()
    }
}

/// A Media Access Control address, in EUI-48 format
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct MacAddr(::std::string::String);
impl ::std::ops::Deref for MacAddr {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<MacAddr> for ::std::string::String {
    fn from(value: MacAddr) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for MacAddr {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for MacAddr {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 17usize {
            return Err("longer than 17 characters".into());
        }

        if value.chars().count() < 5usize {
            return Err("shorter than 5 characters".into());
        }

        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^([0-9a-fA-F]{0,2}:){5}[0-9a-fA-F]{0,2}$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^([0-9a-fA-F]{0,2}:){5}[0-9a-fA-F]{0,2}$\"".into(),
            );
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for MacAddr {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for MacAddr {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// `ManagementAddress`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ManagementAddress {
    pub addr: NetworkAddress,
    pub interface_num: InterfaceNum,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub oid: ::std::option::Option<::std::vec::Vec<u8>>,
}

impl ManagementAddress {
    pub fn builder() -> builder::ManagementAddress {
        Default::default()
    }
}

/// `MaxPathConfig`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct MaxPathConfig(pub ::std::num::NonZeroU8);
impl ::std::ops::Deref for MaxPathConfig {
    type Target = ::std::num::NonZeroU8;
    fn deref(&self) -> &::std::num::NonZeroU8 {
        &self.0
    }
}

impl ::std::convert::From<MaxPathConfig> for ::std::num::NonZeroU8 {
    fn from(value: MaxPathConfig) -> Self {
        value.0
    }
}

impl ::std::convert::From<::std::num::NonZeroU8> for MaxPathConfig {
    fn from(value: ::std::num::NonZeroU8) -> Self {
        Self(value)
    }
}

impl ::std::fmt::Display for MaxPathConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for MaxPathConfig {
    type Err = <::std::num::NonZeroU8 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

/// A `Measurement` is a timestamped datum from a single metric
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Measurement {
    pub datum: Datum,
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Measurement {
    pub fn builder() -> builder::Measurement {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct MeasurementResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Measurement>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl MeasurementResultsPage {
    pub fn builder() -> builder::MeasurementResultsPage {
        Default::default()
    }
}

/// The type of the metric itself, indicating what its values represent.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum MetricType {
    /// The value represents an instantaneous measurement in time.
    #[serde(rename = "gauge")]
    Gauge,
    /// The value represents a difference between two points in time.
    #[serde(rename = "delta")]
    Delta,
    /// The value represents an accumulation between two points in time.
    #[serde(rename = "cumulative")]
    Cumulative,
}

impl ::std::fmt::Display for MetricType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gauge => f.write_str("gauge"),
            Self::Delta => f.write_str("delta"),
            Self::Cumulative => f.write_str("cumulative"),
        }
    }
}

impl ::std::str::FromStr for MetricType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gauge" => Ok(Self::Gauge),
            "delta" => Ok(Self::Delta),
            "cumulative" => Ok(Self::Cumulative),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for MetricType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for MetricType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `MissingDatum`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct MissingDatum {
    pub datum_type: DatumType,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl MissingDatum {
    pub fn builder() -> builder::MissingDatum {
        Default::default()
    }
}

/// View of a Multicast Group
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct MulticastGroup {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// True if any member joined without specifying source IPs (any-source).
    ///
    /// When true, at least one member receives traffic from any source rather
    /// than filtering to specific sources.
    pub has_any_source_member: bool,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The ID of the IP pool this resource belongs to.
    pub ip_pool_id: ::uuid::Uuid,
    /// The multicast IP address held by this resource.
    pub multicast_ip: ::std::net::IpAddr,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Deduplicated union of source IPs specified by members.
    ///
    /// Contains only sources from members that joined with explicit
    /// `source_ips`. Members using any-source multicast (empty `source_ips`) do
    /// not contribute, so a non-empty value does not imply all members use
    /// source filtering. For SSM addresses (232/8, ff3x::/32), this is always
    /// non-empty.
    pub source_ips: ::std::vec::Vec<::std::net::IpAddr>,
    /// Current state of the multicast group.
    pub state: ::std::string::String,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl MulticastGroup {
    pub fn builder() -> builder::MulticastGroup {
        Default::default()
    }
}

/// Can be a UUID, a name, or an IP address
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct MulticastGroupIdentifier(pub ::std::string::String);
impl ::std::ops::Deref for MulticastGroupIdentifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<MulticastGroupIdentifier> for ::std::string::String {
    fn from(value: MulticastGroupIdentifier) -> Self {
        value.0
    }
}

impl ::std::convert::From<::std::string::String> for MulticastGroupIdentifier {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}

impl ::std::fmt::Display for MulticastGroupIdentifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for MulticastGroupIdentifier {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}

/// Specification for joining a multicast group with optional source filtering.
///
/// Used in `InstanceCreate` and `InstanceUpdate` to specify multicast group
/// membership along with per-member source IP configuration.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct MulticastGroupJoinSpec {
    /// The multicast group to join, specified by name, UUID, or IP address.
    pub group: MulticastGroupIdentifier,
    /// IP version for pool selection when creating a group by name. Required if
    /// both IPv4 and IPv6 default multicast pools are linked.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ip_version: ::std::option::Option<IpVersion>,
    /// Source IPs for source-filtered multicast (SSM). Optional for ASM groups,
    /// required for SSM groups (232.0.0.0/8, ff3x::/32).
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source_ips: ::std::option::Option<::std::vec::Vec<::std::net::IpAddr>>,
}

impl MulticastGroupJoinSpec {
    pub fn builder() -> builder::MulticastGroupJoinSpec {
        Default::default()
    }
}

/// View of a Multicast Group Member (instance belonging to a multicast group)
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct MulticastGroupMember {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The ID of the instance that is a member of this group.
    pub instance_id: ::uuid::Uuid,
    /// The ID of the multicast group this member belongs to.
    pub multicast_group_id: ::uuid::Uuid,
    /// The multicast IP address of the group this member belongs to.
    pub multicast_ip: ::std::net::IpAddr,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Source IP addresses for this member's multicast subscription.
    ///
    /// - **ASM**: Sources are optional. Empty array means any source is
    ///   allowed. Non-empty array enables source filtering (IGMPv3/MLDv2). -
    ///   **SSM**: Sources are required for SSM addresses (232/8, ff3x::/32).
    pub source_ips: ::std::vec::Vec<::std::net::IpAddr>,
    /// Current state of the multicast group membership.
    pub state: ::std::string::String,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl MulticastGroupMember {
    pub fn builder() -> builder::MulticastGroupMember {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct MulticastGroupMemberResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<MulticastGroupMember>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl MulticastGroupMemberResultsPage {
    pub fn builder() -> builder::MulticastGroupMemberResultsPage {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct MulticastGroupResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<MulticastGroup>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl MulticastGroupResultsPage {
    pub fn builder() -> builder::MulticastGroupResultsPage {
        Default::default()
    }
}

/// Names must begin with a lower case ASCII letter, be composed exclusively of
/// lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a
/// '-'. Names cannot be a UUID, but they may contain a UUID. They can be at
/// most 63 characters long.
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct Name(::std::string::String);
impl ::std::ops::Deref for Name {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<Name> for ::std::string::String {
    fn from(value: Name) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for Name {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for Name {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 63usize {
            return Err("longer than 63 characters".into());
        }

        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }

        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z]([a-zA-Z0-9-]*[a-zA-Z0-9]+)?$") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z]([a-zA-Z0-9-]*[a-zA-Z0-9]+)?$\"" . into ()) ;
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for Name {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// `NameOrId`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(untagged)]
pub enum NameOrId {
    Id(::uuid::Uuid),
    Name(Name),
}

impl ::std::fmt::Display for NameOrId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Id(x) => x.fmt(f),
            Self::Name(x) => x.fmt(f),
        }
    }
}

impl ::std::str::FromStr for NameOrId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Id(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Name(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}

impl ::std::convert::TryFrom<&str> for NameOrId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for NameOrId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::From<::uuid::Uuid> for NameOrId {
    fn from(value: ::uuid::Uuid) -> Self {
        Self::Id(value)
    }
}

impl ::std::convert::From<Name> for NameOrId {
    fn from(value: Name) -> Self {
        Self::Name(value)
    }
}

/// Supported set of sort modes for scanning by name or id
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum NameOrIdSortMode {
    /// Sort in increasing order of "name"
    #[serde(rename = "name_ascending")]
    NameAscending,
    /// Sort in decreasing order of "name"
    #[serde(rename = "name_descending")]
    NameDescending,
    /// Sort in increasing order of "id"
    #[serde(rename = "id_ascending")]
    IdAscending,
}

impl ::std::fmt::Display for NameOrIdSortMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NameAscending => f.write_str("name_ascending"),
            Self::NameDescending => f.write_str("name_descending"),
            Self::IdAscending => f.write_str("id_ascending"),
        }
    }
}

impl ::std::str::FromStr for NameOrIdSortMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "name_ascending" => Ok(Self::NameAscending),
            "name_descending" => Ok(Self::NameDescending),
            "id_ascending" => Ok(Self::IdAscending),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for NameOrIdSortMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for NameOrIdSortMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Supported set of sort modes for scanning by name only
///
/// Currently, we only support scanning in ascending order.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum NameSortMode {
    /// Sort in increasing order of "name"
    #[serde(rename = "name_ascending")]
    NameAscending,
}

impl ::std::fmt::Display for NameSortMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NameAscending => f.write_str("name_ascending"),
        }
    }
}

impl ::std::str::FromStr for NameSortMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "name_ascending" => Ok(Self::NameAscending),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for NameSortMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for NameSortMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `NetworkAddress`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub enum NetworkAddress {
    #[serde(rename = "ip_addr")]
    IpAddr(::std::net::IpAddr),
    #[serde(rename = "i_e_e_e802")]
    IEEE802(::std::vec::Vec<u8>),
}

impl ::std::convert::From<::std::net::IpAddr> for NetworkAddress {
    fn from(value: ::std::net::IpAddr) -> Self {
        Self::IpAddr(value)
    }
}

impl ::std::convert::From<::std::vec::Vec<u8>> for NetworkAddress {
    fn from(value: ::std::vec::Vec<u8>) -> Self {
        Self::IEEE802(value)
    }
}

/// Information required to construct a virtual network interface
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct NetworkInterface {
    pub id: ::uuid::Uuid,
    pub ip_config: PrivateIpConfig,
    pub kind: NetworkInterfaceKind,
    pub mac: MacAddr,
    pub name: Name,
    pub primary: bool,
    pub slot: u8,
    pub vni: Vni,
}

impl NetworkInterface {
    pub fn builder() -> builder::NetworkInterface {
        Default::default()
    }
}

/// The type of network interface
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "id")]
pub enum NetworkInterfaceKind {
    /// A vNIC attached to a guest instance
    #[serde(rename = "instance")]
    Instance(::uuid::Uuid),
    /// A vNIC associated with an internal service
    #[serde(rename = "service")]
    Service(::uuid::Uuid),
    /// A vNIC associated with a probe
    #[serde(rename = "probe")]
    Probe(::uuid::Uuid),
}

/// The result of a successful OxQL query.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct OxqlQueryResult {
    /// Tables resulting from the query, each containing timeseries.
    pub tables: ::std::vec::Vec<OxqlTable>,
}

impl OxqlQueryResult {
    pub fn builder() -> builder::OxqlQueryResult {
        Default::default()
    }
}

/// A table represents one or more timeseries with the same schema.
///
/// A table is the result of an OxQL query. It contains a name, usually the name
/// of the timeseries schema from which the data is derived, and any number of
/// timeseries, which contain the actual data.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct OxqlTable {
    /// The name of the table.
    pub name: ::std::string::String,
    /// The set of timeseries in the table, ordered by key.
    pub timeseries: ::std::vec::Vec<Timeseries>,
}

impl OxqlTable {
    pub fn builder() -> builder::OxqlTable {
        Default::default()
    }
}

/// The order in which the client wants to page through the requested collection
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum PaginationOrder {
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
}

impl ::std::fmt::Display for PaginationOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ascending => f.write_str("ascending"),
            Self::Descending => f.write_str("descending"),
        }
    }
}

impl ::std::str::FromStr for PaginationOrder {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ascending" => Ok(Self::Ascending),
            "descending" => Ok(Self::Descending),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for PaginationOrder {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for PaginationOrder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Passwords may be subject to additional constraints.
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct Password(::std::string::String);
impl ::std::ops::Deref for Password {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<Password> for ::std::string::String {
    fn from(value: Password) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for Password {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for Password {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for Password {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// View of a Physical Disk
///
/// Physical disks reside in a particular sled and are used to store both
/// Instance Disk data as well as internal metadata.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PhysicalDisk {
    pub form_factor: PhysicalDiskKind,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    pub model: ::std::string::String,
    /// The operator-defined policy for a physical disk.
    pub policy: PhysicalDiskPolicy,
    pub serial: ::std::string::String,
    /// The sled to which this disk is attached, if any.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub sled_id: ::std::option::Option<::uuid::Uuid>,
    /// The current state Nexus believes the disk to be in.
    pub state: PhysicalDiskState,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    pub vendor: ::std::string::String,
}

impl PhysicalDisk {
    pub fn builder() -> builder::PhysicalDisk {
        Default::default()
    }
}

/// A request to adopt a physical disk into the control plane
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PhysicalDiskAdoptionRequest {
    pub disk_id: PhysicalDiskManufacturerIdentity,
    pub id: PhysicalDiskAdoptionRequestUuid,
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl PhysicalDiskAdoptionRequest {
    pub fn builder() -> builder::PhysicalDiskAdoptionRequest {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PhysicalDiskAdoptionRequestResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<PhysicalDiskAdoptionRequest>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl PhysicalDiskAdoptionRequestResultsPage {
    pub fn builder() -> builder::PhysicalDiskAdoptionRequestResultsPage {
        Default::default()
    }
}

/// `PhysicalDiskAdoptionRequestUuid`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct PhysicalDiskAdoptionRequestUuid(pub ::uuid::Uuid);
impl ::std::ops::Deref for PhysicalDiskAdoptionRequestUuid {
    type Target = ::uuid::Uuid;
    fn deref(&self) -> &::uuid::Uuid {
        &self.0
    }
}

impl ::std::convert::From<PhysicalDiskAdoptionRequestUuid> for ::uuid::Uuid {
    fn from(value: PhysicalDiskAdoptionRequestUuid) -> Self {
        value.0
    }
}

impl ::std::convert::From<::uuid::Uuid> for PhysicalDiskAdoptionRequestUuid {
    fn from(value: ::uuid::Uuid) -> Self {
        Self(value)
    }
}

impl ::std::fmt::Display for PhysicalDiskAdoptionRequestUuid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for PhysicalDiskAdoptionRequestUuid {
    type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

/// Describes the form factor of physical disks.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum PhysicalDiskKind {
    #[serde(rename = "m2")]
    M2,
    #[serde(rename = "u2")]
    U2,
}

impl ::std::fmt::Display for PhysicalDiskKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::M2 => f.write_str("m2"),
            Self::U2 => f.write_str("u2"),
        }
    }
}

impl ::std::str::FromStr for PhysicalDiskKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "m2" => Ok(Self::M2),
            "u2" => Ok(Self::U2),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for PhysicalDiskKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for PhysicalDiskKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// The unique identity of a physical disk provided by the manufacturer
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PhysicalDiskManufacturerIdentity {
    pub model: ::std::string::String,
    pub serial: ::std::string::String,
    pub vendor: ::std::string::String,
}

impl PhysicalDiskManufacturerIdentity {
    pub fn builder() -> builder::PhysicalDiskManufacturerIdentity {
        Default::default()
    }
}

/// The operator-defined policy of a physical disk.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(tag = "kind")]
pub enum PhysicalDiskPolicy {
    #[serde(rename = "in_service")]
    InService,
    #[serde(rename = "expunged")]
    Expunged,
}

impl ::std::fmt::Display for PhysicalDiskPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InService => f.write_str("in_service"),
            Self::Expunged => f.write_str("expunged"),
        }
    }
}

impl ::std::str::FromStr for PhysicalDiskPolicy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_service" => Ok(Self::InService),
            "expunged" => Ok(Self::Expunged),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for PhysicalDiskPolicy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for PhysicalDiskPolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PhysicalDiskResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<PhysicalDisk>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl PhysicalDiskResultsPage {
    pub fn builder() -> builder::PhysicalDiskResultsPage {
        Default::default()
    }
}

/// The current state of the disk, as determined by Nexus.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum PhysicalDiskState {
    /// The disk is currently active, and has resources allocated on it.
    #[serde(rename = "active")]
    Active,
    /// The disk has been permanently removed from service.
    ///
    /// This is a terminal state: once a particular disk ID is decommissioned,
    /// it will never return to service. (The actual hardware may be reused, but
    /// it will be treated as a brand-new disk.)
    #[serde(rename = "decommissioned")]
    Decommissioned,
}

impl ::std::fmt::Display for PhysicalDiskState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Decommissioned => f.write_str("decommissioned"),
        }
    }
}

impl ::std::str::FromStr for PhysicalDiskState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "decommissioned" => Ok(Self::Decommissioned),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for PhysicalDiskState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for PhysicalDiskState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `Ping`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Ping {
    /// Whether the external API is reachable. Will always be Ok if the endpoint
    /// returns anything at all.
    pub status: PingStatus,
}

impl Ping {
    pub fn builder() -> builder::Ping {
        Default::default()
    }
}

/// `PingStatus`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum PingStatus {
    #[serde(rename = "ok")]
    Ok,
}

impl ::std::fmt::Display for PingStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}

impl ::std::str::FromStr for PingStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for PingStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for PingStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Timepoints and values for one timeseries.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Points {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_times:
        ::std::option::Option<::std::vec::Vec<::chrono::DateTime<::chrono::offset::Utc>>>,
    pub timestamps: ::std::vec::Vec<::chrono::DateTime<::chrono::offset::Utc>>,
    pub values: ::std::vec::Vec<Values>,
}

impl Points {
    pub fn builder() -> builder::Points {
        Default::default()
    }
}

/// Specify which IP or external subnet pool to allocate from.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum PoolSelector {
    /// Use the specified pool by name or ID.
    #[serde(rename = "explicit")]
    Explicit {
        /// The pool to allocate from.
        pool: NameOrId,
    },
    /// Use the default pool for the silo.
    #[serde(rename = "auto")]
    Auto {
        /// IP version to use when multiple default pools exist. Required if
        /// both IPv4 and IPv6 default pools are configured.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        ip_version: ::std::option::Option<IpVersion>,
    },
}

/// VPC-private IP address configuration for a network interface.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum PrivateIpConfig {
    /// The interface has only an IPv4 configuration.
    #[serde(rename = "v4")]
    V4(PrivateIpv4Config),
    /// The interface has only an IPv6 configuration.
    #[serde(rename = "v6")]
    V6(PrivateIpv6Config),
    /// The interface is dual-stack.
    #[serde(rename = "dual_stack")]
    DualStack {
        /// The interface's IPv4 configuration.
        v4: PrivateIpv4Config,
        /// The interface's IPv6 configuration.
        v6: PrivateIpv6Config,
    },
}

impl ::std::convert::From<PrivateIpv4Config> for PrivateIpConfig {
    fn from(value: PrivateIpv4Config) -> Self {
        Self::V4(value)
    }
}

impl ::std::convert::From<PrivateIpv6Config> for PrivateIpConfig {
    fn from(value: PrivateIpv6Config) -> Self {
        Self::V6(value)
    }
}

/// The VPC-private IP stack for a network interface.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum PrivateIpStack {
    /// The interface has only an IPv4 stack.
    #[serde(rename = "v4")]
    V4(PrivateIpv4Stack),
    /// The interface has only an IPv6 stack.
    #[serde(rename = "v6")]
    V6(PrivateIpv6Stack),
    /// The interface is dual-stack IPv4 and IPv6.
    #[serde(rename = "dual_stack")]
    DualStack {
        v4: PrivateIpv4Stack,
        v6: PrivateIpv6Stack,
    },
}

impl ::std::convert::From<PrivateIpv4Stack> for PrivateIpStack {
    fn from(value: PrivateIpv4Stack) -> Self {
        Self::V4(value)
    }
}

impl ::std::convert::From<PrivateIpv6Stack> for PrivateIpStack {
    fn from(value: PrivateIpv6Stack) -> Self {
        Self::V6(value)
    }
}

/// Create parameters for a network interface's IP stack.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum PrivateIpStackCreate {
    /// The interface has only an IPv4 stack.
    #[serde(rename = "v4")]
    V4(PrivateIpv4StackCreate),
    /// The interface has only an IPv6 stack.
    #[serde(rename = "v6")]
    V6(PrivateIpv6StackCreate),
    /// The interface has both an IPv4 and IPv6 stack.
    #[serde(rename = "dual_stack")]
    DualStack {
        v4: PrivateIpv4StackCreate,
        v6: PrivateIpv6StackCreate,
    },
}

impl ::std::convert::From<PrivateIpv4StackCreate> for PrivateIpStackCreate {
    fn from(value: PrivateIpv4StackCreate) -> Self {
        Self::V4(value)
    }
}

impl ::std::convert::From<PrivateIpv6StackCreate> for PrivateIpStackCreate {
    fn from(value: PrivateIpv6StackCreate) -> Self {
        Self::V6(value)
    }
}

/// VPC-private IPv4 configuration for a network interface.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PrivateIpv4Config {
    /// VPC-private IP address.
    pub ip: ::std::net::Ipv4Addr,
    /// The IP subnet.
    pub subnet: Ipv4Net,
    /// Additional networks on which the interface can send / receive traffic.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub transit_ips: ::std::vec::Vec<Ipv4Net>,
}

impl PrivateIpv4Config {
    pub fn builder() -> builder::PrivateIpv4Config {
        Default::default()
    }
}

/// The VPC-private IPv4 stack for a network interface
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PrivateIpv4Stack {
    /// The VPC-private IPv4 address for the interface.
    pub ip: ::std::net::Ipv4Addr,
    /// A set of additional IPv4 networks that this interface may send and
    /// receive traffic on.
    pub transit_ips: ::std::vec::Vec<Ipv4Net>,
}

impl PrivateIpv4Stack {
    pub fn builder() -> builder::PrivateIpv4Stack {
        Default::default()
    }
}

/// Configuration for a network interface's IPv4 addressing.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PrivateIpv4StackCreate {
    /// The VPC-private address to assign to the interface.
    pub ip: Ipv4Assignment,
    /// Additional IP networks the interface can send / receive on.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub transit_ips: ::std::vec::Vec<Ipv4Net>,
}

impl PrivateIpv4StackCreate {
    pub fn builder() -> builder::PrivateIpv4StackCreate {
        Default::default()
    }
}

/// VPC-private IPv6 configuration for a network interface.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PrivateIpv6Config {
    /// VPC-private IP address.
    pub ip: ::std::net::Ipv6Addr,
    /// The IP subnet.
    pub subnet: Ipv6Net,
    /// Additional networks on which the interface can send / receive traffic.
    pub transit_ips: ::std::vec::Vec<Ipv6Net>,
}

impl PrivateIpv6Config {
    pub fn builder() -> builder::PrivateIpv6Config {
        Default::default()
    }
}

/// The VPC-private IPv6 stack for a network interface
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PrivateIpv6Stack {
    /// The VPC-private IPv6 address for the interface.
    pub ip: ::std::net::Ipv6Addr,
    /// A set of additional IPv6 networks that this interface may send and
    /// receive traffic on.
    pub transit_ips: ::std::vec::Vec<Ipv6Net>,
}

impl PrivateIpv6Stack {
    pub fn builder() -> builder::PrivateIpv6Stack {
        Default::default()
    }
}

/// Configuration for a network interface's IPv6 addressing.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct PrivateIpv6StackCreate {
    /// The VPC-private address to assign to the interface.
    pub ip: Ipv6Assignment,
    /// Additional IP networks the interface can send / receive on.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub transit_ips: ::std::vec::Vec<Ipv6Net>,
}

impl PrivateIpv6StackCreate {
    pub fn builder() -> builder::PrivateIpv6StackCreate {
        Default::default()
    }
}

/// A networking probe
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Probe {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    pub sled: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Probe {
    pub fn builder() -> builder::Probe {
        Default::default()
    }
}

/// Create time parameters for probes.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ProbeCreate {
    pub description: ::std::string::String,
    pub name: Name,
    /// Pool to allocate from.
    #[serde(default = "defaults::probe_create_pool_selector")]
    pub pool_selector: PoolSelector,
    pub sled: ::uuid::Uuid,
}

impl ProbeCreate {
    pub fn builder() -> builder::ProbeCreate {
        Default::default()
    }
}

/// `ProbeExternalIp`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ProbeExternalIp {
    pub first_port: u16,
    pub ip: ::std::net::IpAddr,
    pub kind: ProbeExternalIpKind,
    pub last_port: u16,
}

impl ProbeExternalIp {
    pub fn builder() -> builder::ProbeExternalIp {
        Default::default()
    }
}

/// `ProbeExternalIpKind`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum ProbeExternalIpKind {
    #[serde(rename = "snat")]
    Snat,
    #[serde(rename = "floating")]
    Floating,
    #[serde(rename = "ephemeral")]
    Ephemeral,
}

impl ::std::fmt::Display for ProbeExternalIpKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Snat => f.write_str("snat"),
            Self::Floating => f.write_str("floating"),
            Self::Ephemeral => f.write_str("ephemeral"),
        }
    }
}

impl ::std::str::FromStr for ProbeExternalIpKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "snat" => Ok(Self::Snat),
            "floating" => Ok(Self::Floating),
            "ephemeral" => Ok(Self::Ephemeral),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ProbeExternalIpKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ProbeExternalIpKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `ProbeInfo`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ProbeInfo {
    pub external_ips: ::std::vec::Vec<ProbeExternalIp>,
    pub id: ::uuid::Uuid,
    pub interface: NetworkInterface,
    pub name: Name,
    pub sled: ::uuid::Uuid,
}

impl ProbeInfo {
    pub fn builder() -> builder::ProbeInfo {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ProbeInfoResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<ProbeInfo>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl ProbeInfoResultsPage {
    pub fn builder() -> builder::ProbeInfoResultsPage {
        Default::default()
    }
}

/// View of a Project
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Project {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Project {
    pub fn builder() -> builder::Project {
        Default::default()
    }
}

/// Create-time parameters for a `Project`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ProjectCreate {
    /// Default resources to create in the project
    ///
    /// Omit this field or pass `null` to create all defaults: currently, a
    /// default VPC with its own defaults. Pass an object to specify which
    /// resources to create. `{}` creates none.
    ///
    /// For example, to create the default VPC but not its default subnet, pass
    /// `{"vpc": {"type": "explicit", "defaults": {}}}`.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub defaults: ::std::option::Option<ProjectCreateDefaults>,
    pub description: ::std::string::String,
    pub name: Name,
}

impl ProjectCreate {
    pub fn builder() -> builder::ProjectCreate {
        Default::default()
    }
}

/// Default resources to create in a project
///
/// Each field corresponds to one resource. Set a field to an object to create
/// that resource. Omit it or pass `null` to skip it.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct ProjectCreateDefaults {
    /// Create the default VPC. Omit this field or pass `null` to skip it.
    ///
    /// When present, the value also determines which of the VPC's own defaults
    /// to create: `{"type": "all"}` creates all of them, and `{"type":
    /// "explicit", "defaults": {...}}` creates only those specified.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vpc: ::std::option::Option<VpcCreateDefaultsSelection>,
}

impl ProjectCreateDefaults {
    pub fn builder() -> builder::ProjectCreateDefaults {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ProjectResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Project>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl ProjectResultsPage {
    pub fn builder() -> builder::ProjectResultsPage {
        Default::default()
    }
}

/// `ProjectRole`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum ProjectRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "collaborator")]
    Collaborator,
    #[serde(rename = "limited_collaborator")]
    LimitedCollaborator,
    #[serde(rename = "viewer")]
    Viewer,
}

impl ::std::fmt::Display for ProjectRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Admin => f.write_str("admin"),
            Self::Collaborator => f.write_str("collaborator"),
            Self::LimitedCollaborator => f.write_str("limited_collaborator"),
            Self::Viewer => f.write_str("viewer"),
        }
    }
}

impl ::std::str::FromStr for ProjectRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "admin" => Ok(Self::Admin),
            "collaborator" => Ok(Self::Collaborator),
            "limited_collaborator" => Ok(Self::LimitedCollaborator),
            "viewer" => Ok(Self::Viewer),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ProjectRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ProjectRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Policy for a particular resource
///
/// Note that the Policy only describes access granted explicitly for this
/// resource.  The policies of parent resources can also cause a user to have
/// access to this resource.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ProjectRolePolicy {
    /// Roles directly assigned on this resource
    pub role_assignments: ::std::vec::Vec<ProjectRoleRoleAssignment>,
}

impl ProjectRolePolicy {
    pub fn builder() -> builder::ProjectRolePolicy {
        Default::default()
    }
}

/// Describes the assignment of a particular role on a particular resource to a
/// particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, `RoleAssignment`s are
/// put into a `Policy` and that Policy is applied to a particular resource.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ProjectRoleRoleAssignment {
    pub identity_id: ::uuid::Uuid,
    pub identity_type: IdentityType,
    pub role_name: ProjectRole,
}

impl ProjectRoleRoleAssignment {
    pub fn builder() -> builder::ProjectRoleRoleAssignment {
        Default::default()
    }
}

/// Updateable properties of a `Project`
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct ProjectUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl ProjectUpdate {
    pub fn builder() -> builder::ProjectUpdate {
        Default::default()
    }
}

/// Structure for estimating the p-quantile of a population.
///
/// This is based on the P² algorithm for estimating quantiles using constant
/// space.
///
/// The algorithm consists of maintaining five markers: the minimum, the p/2-,
/// p-, and (1 + p)/2 quantiles, and the maximum.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Quantile {
    /// The desired marker positions.
    pub desired_marker_positions: [f64; 5usize],
    /// The heights of the markers.
    pub marker_heights: [f64; 5usize],
    /// The positions of the markers.
    ///
    /// We track sample size in the 5th position, as useful observations won't
    /// start until we've filled the heights at the 6th sample anyway This does
    /// deviate from the paper, but it's a more useful representation that works
    /// according to the paper's algorithm.
    pub marker_positions: [u64; 5usize],
    /// The p value for the quantile.
    pub p: f64,
}

impl Quantile {
    pub fn builder() -> builder::Quantile {
        Default::default()
    }
}

/// View of a Rack
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Rack {
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Rack {
    pub fn builder() -> builder::Rack {
        Default::default()
    }
}

/// `RackMembershipAddSledsRequest`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct RackMembershipAddSledsRequest {
    pub sled_ids: ::std::vec::Vec<BaseboardId>,
}

impl RackMembershipAddSledsRequest {
    pub fn builder() -> builder::RackMembershipAddSledsRequest {
        Default::default()
    }
}

/// `RackMembershipChangeState`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum RackMembershipChangeState {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "committed")]
    Committed,
    #[serde(rename = "aborted")]
    Aborted,
}

impl ::std::fmt::Display for RackMembershipChangeState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InProgress => f.write_str("in_progress"),
            Self::Committed => f.write_str("committed"),
            Self::Aborted => f.write_str("aborted"),
        }
    }
}

impl ::std::str::FromStr for RackMembershipChangeState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "in_progress" => Ok(Self::InProgress),
            "committed" => Ok(Self::Committed),
            "aborted" => Ok(Self::Aborted),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for RackMembershipChangeState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for RackMembershipChangeState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Status of the rack membership uniquely identified by the (rack_id, version)
/// pair
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct RackMembershipStatus {
    /// All members of the rack for this version
    pub members: ::std::vec::Vec<BaseboardId>,
    pub rack_id: ::uuid::Uuid,
    pub state: RackMembershipChangeState,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time_aborted: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time_committed: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// All members that have not yet confirmed this membership version
    pub unacknowledged_members: ::std::vec::Vec<BaseboardId>,
    /// Version that uniquely identifies the rack membership at a given point in
    /// time
    pub version: RackMembershipVersion,
}

impl RackMembershipStatus {
    pub fn builder() -> builder::RackMembershipStatus {
        Default::default()
    }
}

/// A unique, monotonically increasing number representing the set of active
/// sleds in a rack at a given point in time.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct RackMembershipVersion(pub u64);
impl ::std::ops::Deref for RackMembershipVersion {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl ::std::convert::From<RackMembershipVersion> for u64 {
    fn from(value: RackMembershipVersion) -> Self {
        value.0
    }
}

impl ::std::convert::From<u64> for RackMembershipVersion {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl ::std::fmt::Display for RackMembershipVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for RackMembershipVersion {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct RackResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Rack>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl RackResultsPage {
    pub fn builder() -> builder::RackResultsPage {
        Default::default()
    }
}

/// A route to a destination network through a gateway address.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Route {
    /// The route destination.
    pub dst: IpNet,
    /// The route gateway.
    pub gw: ::std::net::IpAddr,
    /// Route RIB priority. Higher priority indicates precedence within and
    /// across protocols.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rib_priority: ::std::option::Option<u8>,
    /// VLAN id the gateway is reachable over.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vid: ::std::option::Option<u16>,
}

impl Route {
    pub fn builder() -> builder::Route {
        Default::default()
    }
}

/// Route configuration data associated with a switch port configuration.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct RouteConfig {
    /// Link name. On ports that are not broken out, this is always phy0. On a
    /// 2x breakout the options are phy0 and phy1, on 4x phy0-phy3, etc.
    pub link_name: Name,
    /// The set of routes assigned to a switch port.
    pub routes: ::std::vec::Vec<Route>,
}

impl RouteConfig {
    pub fn builder() -> builder::RouteConfig {
        Default::default()
    }
}

/// A `RouteDestination` is used to match traffic with a routing rule based on
/// the destination of that traffic.
///
/// When traffic is to be sent to a destination that is within a given
/// `RouteDestination`, the corresponding `RouterRoute` applies, and traffic
/// will be forward to the `RouteTarget` for that rule.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum RouteDestination {
    /// Route applies to traffic destined for the specified IP address
    #[serde(rename = "ip")]
    Ip(::std::net::IpAddr),
    /// Route applies to traffic destined for the specified IP subnet
    #[serde(rename = "ip_net")]
    IpNet(IpNet),
    /// Route applies to traffic destined for the specified VPC
    #[serde(rename = "vpc")]
    Vpc(Name),
    /// Route applies to traffic destined for the specified VPC subnet
    #[serde(rename = "subnet")]
    Subnet(Name),
}

impl ::std::convert::From<::std::net::IpAddr> for RouteDestination {
    fn from(value: ::std::net::IpAddr) -> Self {
        Self::Ip(value)
    }
}

impl ::std::convert::From<IpNet> for RouteDestination {
    fn from(value: IpNet) -> Self {
        Self::IpNet(value)
    }
}

/// A `RouteTarget` describes the possible locations that traffic matching a
/// route destination can be sent.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum RouteTarget {
    /// Forward traffic to a particular IP address.
    #[serde(rename = "ip")]
    Ip(::std::net::IpAddr),
    /// Forward traffic to a VPC
    #[serde(rename = "vpc")]
    Vpc(Name),
    /// Forward traffic to a VPC Subnet
    #[serde(rename = "subnet")]
    Subnet(Name),
    /// Forward traffic to a specific instance
    #[serde(rename = "instance")]
    Instance(Name),
    /// Forward traffic to an internet gateway
    #[serde(rename = "internet_gateway")]
    InternetGateway(Name),
    #[serde(rename = "drop")]
    Drop,
}

impl ::std::convert::From<::std::net::IpAddr> for RouteTarget {
    fn from(value: ::std::net::IpAddr) -> Self {
        Self::Ip(value)
    }
}

/// Router lifetime in seconds for unnumbered BGP peers
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct RouterLifetimeConfig(pub u16);
impl ::std::ops::Deref for RouterLifetimeConfig {
    type Target = u16;
    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl ::std::convert::From<RouterLifetimeConfig> for u16 {
    fn from(value: RouterLifetimeConfig) -> Self {
        value.0
    }
}

impl ::std::convert::From<u16> for RouterLifetimeConfig {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl ::std::fmt::Display for RouterLifetimeConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for RouterLifetimeConfig {
    type Err = <u16 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

/// `RouterPeerType`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type")]
pub enum RouterPeerType {
    #[serde(rename = "unnumbered")]
    Unnumbered {
        /// Router lifetime in seconds for unnumbered BGP peers.
        router_lifetime: RouterLifetimeConfig,
    },
    #[serde(rename = "numbered")]
    Numbered {
        /// Optional local IP address to bind when establishing outbound TCP
        /// connections to this peer. If `None`, the OS selects the source
        /// address.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        src_addr: ::std::option::Option<::std::net::IpAddr>,
        /// Target IP address for numbered BGP peers.
        target_addr: ::std::net::IpAddr,
    },
}

/// A route defines a rule that governs where traffic should be sent based on
/// its destination.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct RouterRoute {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Selects which traffic this routing rule will apply to
    pub destination: RouteDestination,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Describes the kind of router. Set at creation. `read-only`
    pub kind: RouterRouteKind,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// The location that matched packets should be forwarded to
    pub target: RouteTarget,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The ID of the VPC Router to which the route belongs
    pub vpc_router_id: ::uuid::Uuid,
}

impl RouterRoute {
    pub fn builder() -> builder::RouterRoute {
        Default::default()
    }
}

/// Create-time parameters for a `RouterRoute`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct RouterRouteCreate {
    pub description: ::std::string::String,
    /// Selects which traffic this routing rule will apply to.
    pub destination: RouteDestination,
    pub name: Name,
    /// The location that matched packets should be forwarded to.
    pub target: RouteTarget,
}

impl RouterRouteCreate {
    pub fn builder() -> builder::RouterRouteCreate {
        Default::default()
    }
}

/// The kind of a `RouterRoute`
///
/// The kind determines certain attributes such as if the route is modifiable
/// and describes how or where the route was created.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum RouterRouteKind {
    /// Determines the default destination of traffic, such as whether it goes
    /// to the internet or not.
    ///
    /// `Destination: An Internet Gateway` `Modifiable: true`
    #[serde(rename = "default")]
    Default,
    /// Automatically added for each VPC Subnet in the VPC
    ///
    /// `Destination: A VPC Subnet` `Modifiable: false`
    #[serde(rename = "vpc_subnet")]
    VpcSubnet,
    /// Automatically added when VPC peering is established
    ///
    /// `Destination: A different VPC` `Modifiable: false`
    #[serde(rename = "vpc_peering")]
    VpcPeering,
    /// Created by a user; see `RouteTarget`
    ///
    /// `Destination: User defined` `Modifiable: true`
    #[serde(rename = "custom")]
    Custom,
}

impl ::std::fmt::Display for RouterRouteKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Default => f.write_str("default"),
            Self::VpcSubnet => f.write_str("vpc_subnet"),
            Self::VpcPeering => f.write_str("vpc_peering"),
            Self::Custom => f.write_str("custom"),
        }
    }
}

impl ::std::str::FromStr for RouterRouteKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "vpc_subnet" => Ok(Self::VpcSubnet),
            "vpc_peering" => Ok(Self::VpcPeering),
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for RouterRouteKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for RouterRouteKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct RouterRouteResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<RouterRoute>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl RouterRouteResultsPage {
    pub fn builder() -> builder::RouterRouteResultsPage {
        Default::default()
    }
}

/// Updateable properties of a `RouterRoute`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct RouterRouteUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    /// Selects which traffic this routing rule will apply to.
    pub destination: RouteDestination,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
    /// The location that matched packets should be forwarded to.
    pub target: RouteTarget,
}

impl RouterRouteUpdate {
    pub fn builder() -> builder::RouterRouteUpdate {
        Default::default()
    }
}

/// A SAML identity provider
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SamlIdentityProvider {
    /// Service provider endpoint where the response will be sent
    pub acs_url: ::std::string::String,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// If set, attributes with this name will be considered to denote a user's
    /// group membership, where the values will be the group names.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub group_attribute_name: ::std::option::Option<::std::string::String>,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// IdP's entity id
    pub idp_entity_id: ::std::string::String,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Optional request signing public certificate (base64 encoded der file)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub public_cert: ::std::option::Option<::std::string::String>,
    /// Service provider endpoint where the idp should send log out requests
    pub slo_url: ::std::string::String,
    /// SP's client id
    pub sp_client_id: ::std::string::String,
    /// Customer's technical contact for saml configuration
    pub technical_contact_email: ::std::string::String,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SamlIdentityProvider {
    pub fn builder() -> builder::SamlIdentityProvider {
        Default::default()
    }
}

/// Create-time identity-related parameters
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SamlIdentityProviderCreate {
    /// Service provider endpoint where the response will be sent
    pub acs_url: ::std::string::String,
    pub description: ::std::string::String,
    /// If set, SAML attributes with this name will be considered to denote a
    /// user's group membership, where the attribute value(s) should be a
    /// comma-separated list of group names.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub group_attribute_name: ::std::option::Option<::std::string::String>,
    /// IdP's entity ID
    pub idp_entity_id: ::std::string::String,
    /// The source of an identity provider metadata descriptor
    pub idp_metadata_source: IdpMetadataSource,
    pub name: Name,
    /// Request signing key pair
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub signing_keypair: ::std::option::Option<DerEncodedKeyPair>,
    /// Service provider endpoint where the IdP should send log out requests
    pub slo_url: ::std::string::String,
    /// SP's client ID
    pub sp_client_id: ::std::string::String,
    /// Customer's technical contact for SAML configuration
    pub technical_contact_email: ::std::string::String,
}

impl SamlIdentityProviderCreate {
    pub fn builder() -> builder::SamlIdentityProviderCreate {
        Default::default()
    }
}

/// `ScimClientBearerToken`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ScimClientBearerToken {
    pub id: ::uuid::Uuid,
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time_expires: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl ScimClientBearerToken {
    pub fn builder() -> builder::ScimClientBearerToken {
        Default::default()
    }
}

/// The POST response is the only time the generated bearer token is returned to
/// the client.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ScimClientBearerTokenValue {
    pub bearer_token: ::std::string::String,
    pub id: ::uuid::Uuid,
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time_expires: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl ScimClientBearerTokenValue {
    pub fn builder() -> builder::ScimClientBearerTokenValue {
        Default::default()
    }
}

/// Configuration of inbound ICMP allowed by API services.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct ServiceIcmpConfig {
    /// When enabled, Nexus is able to receive ICMP Destination Unreachable type
    /// 3 (port unreachable) and type 4 (fragmentation needed), Redirect, and
    /// Time Exceeded messages. These enable Nexus to perform Path MTU discovery
    /// and better cope with fragmentation issues. Otherwise all inbound ICMP
    /// traffic will be dropped.
    pub enabled: bool,
}

impl ServiceIcmpConfig {
    pub fn builder() -> builder::ServiceIcmpConfig {
        Default::default()
    }
}

/// The service intended to use this certificate.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum ServiceUsingCertificate {
    /// This certificate is intended for access to the external API.
    #[serde(rename = "external_api")]
    ExternalApi,
}

impl ::std::fmt::Display for ServiceUsingCertificate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExternalApi => f.write_str("external_api"),
        }
    }
}

impl ::std::str::FromStr for ServiceUsingCertificate {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "external_api" => Ok(Self::ExternalApi),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for ServiceUsingCertificate {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for ServiceUsingCertificate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Parameters for PUT requests to `/v1/system/update/target-release`.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SetTargetReleaseParams {
    /// Version of the system software to make the target release.
    pub system_version: SetTargetReleaseParamsSystemVersion,
}

impl SetTargetReleaseParams {
    pub fn builder() -> builder::SetTargetReleaseParams {
        Default::default()
    }
}

/// Version of the system software to make the target release.
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct SetTargetReleaseParamsSystemVersion(::std::string::String);
impl ::std::ops::Deref for SetTargetReleaseParamsSystemVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<SetTargetReleaseParamsSystemVersion> for ::std::string::String {
    fn from(value: SetTargetReleaseParamsSystemVersion) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for SetTargetReleaseParamsSystemVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for SetTargetReleaseParamsSystemVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*\
                     [a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?\
                     (?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \
                        \"^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*\
                        [a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*\
                        ))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$\""
                .into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SetTargetReleaseParamsSystemVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for SetTargetReleaseParamsSystemVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// View of a Silo
///
/// A Silo is the highest level unit of isolation.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Silo {
    /// Optionally, silos can have a group name that is automatically granted
    /// the silo admin role.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub admin_group_name: ::std::option::Option<::std::string::String>,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// A non-discoverable silo can only be retrieved by ID - it will not be
    /// part of the "list all silos" output.
    pub discoverable: bool,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// How users and groups are managed in this Silo
    pub identity_mode: SiloIdentityMode,
    /// Mapping of which Fleet roles are conferred by each Silo role
    ///
    /// The default is that no Fleet roles are conferred by any Silo roles
    /// unless there's a corresponding entry in this map.
    pub mapped_fleet_roles:
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<FleetRole>>,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Silo {
    pub fn builder() -> builder::Silo {
        Default::default()
    }
}

/// View of silo authentication settings
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloAuthSettings {
    /// Maximum lifetime of a device token in seconds. If set to null, users
    /// will be able to create tokens that do not expire.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub device_token_max_ttl_seconds: ::std::option::Option<u32>,
    pub silo_id: ::uuid::Uuid,
}

impl SiloAuthSettings {
    pub fn builder() -> builder::SiloAuthSettings {
        Default::default()
    }
}

/// Updateable properties of a silo's settings.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloAuthSettingsUpdate {
    /// Maximum lifetime of a device token in seconds. If set to null, users
    /// will be able to create tokens that do not expire.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub device_token_max_ttl_seconds: ::std::option::Option<::std::num::NonZeroU32>,
}

impl SiloAuthSettingsUpdate {
    pub fn builder() -> builder::SiloAuthSettingsUpdate {
        Default::default()
    }
}

/// Create-time parameters for a `Silo`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloCreate {
    /// If set, this group will be created during Silo creation and granted the
    /// "Silo Admin" role. Identity providers can assert that users belong to
    /// this group and those users can log in and further initialize the Silo.
    ///
    /// Note that if configuring a SAML based identity provider,
    /// group_attribute_name must be set for users to be considered part of a
    /// group. See `SamlIdentityProviderCreate` for more information.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub admin_group_name: ::std::option::Option<::std::string::String>,
    pub description: ::std::string::String,
    pub identity_mode: SiloIdentityMode,
    /// Mapping of which Fleet roles are conferred by each Silo role
    ///
    /// The default is that no Fleet roles are conferred by any Silo roles
    /// unless there's a corresponding entry in this map.
    #[serde(default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
    pub mapped_fleet_roles:
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<FleetRole>>,
    pub name: Name,
    /// Limits the amount of provisionable CPU, memory, and storage in the Silo.
    /// CPU and memory are only consumed by running instances, while storage is
    /// consumed by any disk or snapshot. A value of 0 means that resource is
    /// *not* provisionable.
    pub quotas: SiloQuotasCreate,
    /// Initial TLS certificates to be used for the new Silo's console and API
    /// endpoints.  These should be valid for the Silo's DNS name(s).
    pub tls_certificates: ::std::vec::Vec<CertificateCreate>,
}

impl SiloCreate {
    pub fn builder() -> builder::SiloCreate {
        Default::default()
    }
}

/// Describes how identities are managed and users are authenticated in this
/// Silo
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum SiloIdentityMode {
    /// Users are authenticated with SAML using an external authentication
    /// provider.  The system updates information about users and groups only
    /// during successful authentication (i.e,. "JIT provisioning" of users and
    /// groups).
    #[serde(rename = "saml_jit")]
    SamlJit,
    /// The system is the source of truth about users.  There is no linkage to
    /// an external authentication provider or identity provider.
    #[serde(rename = "local_only")]
    LocalOnly,
    /// Users are authenticated with SAML using an external authentication
    /// provider. Users and groups are managed with SCIM API calls, likely from
    /// the same authentication provider.
    #[serde(rename = "saml_scim")]
    SamlScim,
}

impl ::std::fmt::Display for SiloIdentityMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SamlJit => f.write_str("saml_jit"),
            Self::LocalOnly => f.write_str("local_only"),
            Self::SamlScim => f.write_str("saml_scim"),
        }
    }
}

impl ::std::str::FromStr for SiloIdentityMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "saml_jit" => Ok(Self::SamlJit),
            "local_only" => Ok(Self::LocalOnly),
            "saml_scim" => Ok(Self::SamlScim),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SiloIdentityMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SiloIdentityMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// An IP pool in the context of a silo
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloIpPool {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The IP version for the pool.
    pub ip_version: IpVersion,
    /// When a pool is the default for a silo, floating IPs and instance
    /// ephemeral IPs will come from that pool when no other pool is specified.
    ///
    /// A silo can have at most one default pool per combination of pool type
    /// (unicast or multicast) and IP version (IPv4 or IPv6), allowing up to 4
    /// default pools total.
    pub is_default: bool,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Type of IP pool (unicast or multicast).
    pub pool_type: IpPoolType,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SiloIpPool {
    pub fn builder() -> builder::SiloIpPool {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloIpPoolResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SiloIpPool>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SiloIpPoolResultsPage {
    pub fn builder() -> builder::SiloIpPoolResultsPage {
        Default::default()
    }
}

/// A collection of resource counts used to set the virtual capacity of a silo
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloQuotas {
    /// Number of virtual CPUs
    pub cpus: i64,
    /// Amount of memory in bytes
    pub memory: ByteCount,
    pub silo_id: ::uuid::Uuid,
    /// Amount of disk storage in bytes
    pub storage: ByteCount,
}

impl SiloQuotas {
    pub fn builder() -> builder::SiloQuotas {
        Default::default()
    }
}

/// The amount of provisionable resources for a Silo
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloQuotasCreate {
    /// The amount of virtual CPUs available for running instances in the Silo
    pub cpus: i64,
    /// The amount of RAM (in bytes) available for running instances in the Silo
    pub memory: ByteCount,
    /// The amount of storage (in bytes) available for disks or snapshots
    pub storage: ByteCount,
}

impl SiloQuotasCreate {
    pub fn builder() -> builder::SiloQuotasCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloQuotasResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SiloQuotas>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SiloQuotasResultsPage {
    pub fn builder() -> builder::SiloQuotasResultsPage {
        Default::default()
    }
}

/// Updateable properties of a Silo's resource limits. If a value is omitted it
/// will not be updated.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct SiloQuotasUpdate {
    /// The amount of virtual CPUs available for running instances in the Silo
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cpus: ::std::option::Option<i64>,
    /// The amount of RAM (in bytes) available for running instances in the Silo
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub memory: ::std::option::Option<ByteCount>,
    /// The amount of storage (in bytes) available for disks or snapshots
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub storage: ::std::option::Option<ByteCount>,
}

impl SiloQuotasUpdate {
    pub fn builder() -> builder::SiloQuotasUpdate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Silo>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SiloResultsPage {
    pub fn builder() -> builder::SiloResultsPage {
        Default::default()
    }
}

/// `SiloRole`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum SiloRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "collaborator")]
    Collaborator,
    #[serde(rename = "limited_collaborator")]
    LimitedCollaborator,
    #[serde(rename = "viewer")]
    Viewer,
}

impl ::std::fmt::Display for SiloRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Admin => f.write_str("admin"),
            Self::Collaborator => f.write_str("collaborator"),
            Self::LimitedCollaborator => f.write_str("limited_collaborator"),
            Self::Viewer => f.write_str("viewer"),
        }
    }
}

impl ::std::str::FromStr for SiloRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "admin" => Ok(Self::Admin),
            "collaborator" => Ok(Self::Collaborator),
            "limited_collaborator" => Ok(Self::LimitedCollaborator),
            "viewer" => Ok(Self::Viewer),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SiloRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SiloRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Policy for a particular resource
///
/// Note that the Policy only describes access granted explicitly for this
/// resource.  The policies of parent resources can also cause a user to have
/// access to this resource.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloRolePolicy {
    /// Roles directly assigned on this resource
    pub role_assignments: ::std::vec::Vec<SiloRoleRoleAssignment>,
}

impl SiloRolePolicy {
    pub fn builder() -> builder::SiloRolePolicy {
        Default::default()
    }
}

/// Describes the assignment of a particular role on a particular resource to a
/// particular identity (user, group, etc.)
///
/// The resource is not part of this structure.  Rather, `RoleAssignment`s are
/// put into a `Policy` and that Policy is applied to a particular resource.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloRoleRoleAssignment {
    pub identity_id: ::uuid::Uuid,
    pub identity_type: IdentityType,
    pub role_name: SiloRole,
}

impl SiloRoleRoleAssignment {
    pub fn builder() -> builder::SiloRoleRoleAssignment {
        Default::default()
    }
}

/// A subnet pool in the context of a silo
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloSubnetPool {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The IP version for the pool.
    pub ip_version: IpVersion,
    /// When a pool is the default for a silo, external subnet allocations will
    /// come from that pool when no other pool is specified.
    ///
    /// A silo can have at most one default pool per IP version (IPv4 or IPv6),
    /// allowing up to 2 default pools total.
    pub is_default: bool,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SiloSubnetPool {
    pub fn builder() -> builder::SiloSubnetPool {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloSubnetPoolResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SiloSubnetPool>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SiloSubnetPoolResultsPage {
    pub fn builder() -> builder::SiloSubnetPoolResultsPage {
        Default::default()
    }
}

/// View of a silo's resource utilization and capacity
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloUtilization {
    /// Accounts for the total amount of resources reserved for silos via their
    /// quotas.
    pub allocated: VirtualResourceCounts,
    /// Accounts for the total resources allocated by the silo, including CPU
    /// and memory for running instances and storage for disks and snapshots.
    ///
    /// Note that CPU and memory resources associated with stopped instances are
    /// not counted here.
    pub provisioned: VirtualResourceCounts,
    pub silo_id: ::uuid::Uuid,
    pub silo_name: Name,
}

impl SiloUtilization {
    pub fn builder() -> builder::SiloUtilization {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SiloUtilizationResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SiloUtilization>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SiloUtilizationResultsPage {
    pub fn builder() -> builder::SiloUtilizationResultsPage {
        Default::default()
    }
}

/// An operator's view of a Sled.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Sled {
    pub baseboard: Baseboard,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The operator-defined policy of a sled.
    pub policy: SledPolicy,
    /// The rack to which this Sled is currently attached
    pub rack_id: ::uuid::Uuid,
    /// The physical slot in the rack where this sled was last observed to be
    /// located, or null if its location is not known at this time.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub slot: ::std::option::Option<u16>,
    /// The current state of the sled.
    pub state: SledState,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The number of hardware threads which can execute on this sled
    pub usable_hardware_threads: u32,
    /// Amount of RAM which may be used by the Sled's OS
    pub usable_physical_ram: ByteCount,
}

impl Sled {
    pub fn builder() -> builder::Sled {
        Default::default()
    }
}

/// An operator's view of an instance running on a given sled
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SledInstance {
    pub active_sled_id: ::uuid::Uuid,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    pub memory: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub migration_id: ::std::option::Option<::uuid::Uuid>,
    pub name: Name,
    pub ncpus: i64,
    pub project_name: Name,
    pub silo_name: Name,
    pub state: InstanceState,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SledInstance {
    pub fn builder() -> builder::SledInstance {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SledInstanceResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SledInstance>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SledInstanceResultsPage {
    pub fn builder() -> builder::SledInstanceResultsPage {
        Default::default()
    }
}

/// The operator-defined policy of a sled.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "kind", content = "provision_policy")]
pub enum SledPolicy {
    /// The operator has indicated that the sled is in-service.
    #[serde(rename = "in_service")]
    InService(SledProvisionPolicy),
    #[serde(rename = "expunged")]
    Expunged,
}

impl ::std::convert::From<SledProvisionPolicy> for SledPolicy {
    fn from(value: SledProvisionPolicy) -> Self {
        Self::InService(value)
    }
}

/// The operator-defined provision policy of a sled.
///
/// This controls whether new resources are going to be provisioned on this
/// sled.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum SledProvisionPolicy {
    /// New resources will be provisioned on this sled.
    #[serde(rename = "provisionable")]
    Provisionable,
    /// New resources will not be provisioned on this sled. However, if the sled
    /// is currently in service, existing resources will continue to be on this
    /// sled unless manually migrated off.
    #[serde(rename = "non_provisionable")]
    NonProvisionable,
}

impl ::std::fmt::Display for SledProvisionPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Provisionable => f.write_str("provisionable"),
            Self::NonProvisionable => f.write_str("non_provisionable"),
        }
    }
}

impl ::std::str::FromStr for SledProvisionPolicy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "provisionable" => Ok(Self::Provisionable),
            "non_provisionable" => Ok(Self::NonProvisionable),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SledProvisionPolicy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SledProvisionPolicy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Parameters for `sled_set_provision_policy`.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SledProvisionPolicyParams {
    /// The provision state.
    pub state: SledProvisionPolicy,
}

impl SledProvisionPolicyParams {
    pub fn builder() -> builder::SledProvisionPolicyParams {
        Default::default()
    }
}

/// Response to `sled_set_provision_policy`.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SledProvisionPolicyResponse {
    /// The new provision state.
    pub new_state: SledProvisionPolicy,
    /// The old provision state.
    pub old_state: SledProvisionPolicy,
}

impl SledProvisionPolicyResponse {
    pub fn builder() -> builder::SledProvisionPolicyResponse {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SledResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Sled>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SledResultsPage {
    pub fn builder() -> builder::SledResultsPage {
        Default::default()
    }
}

/// The current state of the sled.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum SledState {
    /// The sled is currently active, and has resources allocated on it.
    #[serde(rename = "active")]
    Active,
    /// The sled has been permanently removed from service.
    ///
    /// This is a terminal state: once a particular sled ID is decommissioned,
    /// it will never return to service. (The actual hardware may be reused, but
    /// it will be treated as a brand-new sled.)
    #[serde(rename = "decommissioned")]
    Decommissioned,
}

impl ::std::fmt::Display for SledState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Decommissioned => f.write_str("decommissioned"),
        }
    }
}

impl ::std::str::FromStr for SledState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "decommissioned" => Ok(Self::Decommissioned),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SledState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SledState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `SledUuid`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct SledUuid(pub ::uuid::Uuid);
impl ::std::ops::Deref for SledUuid {
    type Target = ::uuid::Uuid;
    fn deref(&self) -> &::uuid::Uuid {
        &self.0
    }
}

impl ::std::convert::From<SledUuid> for ::uuid::Uuid {
    fn from(value: SledUuid) -> Self {
        value.0
    }
}

impl ::std::convert::From<::uuid::Uuid> for SledUuid {
    fn from(value: ::uuid::Uuid) -> Self {
        Self(value)
    }
}

impl ::std::fmt::Display for SledUuid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for SledUuid {
    type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

/// View of a Snapshot
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Snapshot {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    pub disk_id: ::uuid::Uuid,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    pub project_id: ::uuid::Uuid,
    pub size: ByteCount,
    pub state: SnapshotState,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Snapshot {
    pub fn builder() -> builder::Snapshot {
        Default::default()
    }
}

/// Create-time parameters for a `Snapshot`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SnapshotCreate {
    pub description: ::std::string::String,
    /// The disk to be snapshotted
    pub disk: NameOrId,
    pub name: Name,
}

impl SnapshotCreate {
    pub fn builder() -> builder::SnapshotCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SnapshotResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Snapshot>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SnapshotResultsPage {
    pub fn builder() -> builder::SnapshotResultsPage {
        Default::default()
    }
}

/// `SnapshotState`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum SnapshotState {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "faulted")]
    Faulted,
    #[serde(rename = "destroyed")]
    Destroyed,
}

impl ::std::fmt::Display for SnapshotState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Creating => f.write_str("creating"),
            Self::Ready => f.write_str("ready"),
            Self::Faulted => f.write_str("faulted"),
            Self::Destroyed => f.write_str("destroyed"),
        }
    }
}

impl ::std::str::FromStr for SnapshotState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "creating" => Ok(Self::Creating),
            "ready" => Ok(Self::Ready),
            "faulted" => Ok(Self::Faulted),
            "destroyed" => Ok(Self::Destroyed),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SnapshotState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SnapshotState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// View of an SSH Key
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SshKey {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// SSH public key, e.g., `"ssh-ed25519 AAAAC3NzaC..."`
    pub public_key: ::std::string::String,
    /// The user to whom this key belongs
    pub silo_user_id: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SshKey {
    pub fn builder() -> builder::SshKey {
        Default::default()
    }
}

/// Create-time parameters for an `SshKey`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SshKeyCreate {
    pub description: ::std::string::String,
    pub name: Name,
    /// SSH public key, e.g., `"ssh-ed25519 AAAAC3NzaC..."`
    pub public_key: ::std::string::String,
}

impl SshKeyCreate {
    pub fn builder() -> builder::SshKeyCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SshKeyResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SshKey>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SshKeyResultsPage {
    pub fn builder() -> builder::SshKeyResultsPage {
        Default::default()
    }
}

/// Default resources to create in the default subnet
///
/// Including this object in the request creates the default subnet. A subnet
/// has no default resources yet, so the object is always empty.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct SubnetCreateDefaults {}
impl SubnetCreateDefaults {
    pub fn builder() -> builder::SubnetCreateDefaults {
        Default::default()
    }
}

/// A pool of subnets for external subnet allocation
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPool {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The IP version for this pool
    pub ip_version: IpVersion,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SubnetPool {
    pub fn builder() -> builder::SubnetPool {
        Default::default()
    }
}

/// Create a subnet pool
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolCreate {
    pub description: ::std::string::String,
    /// The IP version for this pool (IPv4 or IPv6). All subnets in the pool
    /// must match this version.
    pub ip_version: IpVersion,
    pub name: Name,
}

impl SubnetPoolCreate {
    pub fn builder() -> builder::SubnetPoolCreate {
        Default::default()
    }
}

/// Link a subnet pool to a silo
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolLinkSilo {
    /// Whether this is the default subnet pool for the silo. When true,
    /// external subnet allocations that don't specify a pool use this one.
    pub is_default: bool,
    /// The silo to link
    pub silo: NameOrId,
}

impl SubnetPoolLinkSilo {
    pub fn builder() -> builder::SubnetPoolLinkSilo {
        Default::default()
    }
}

/// A member (subnet) within a subnet pool
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolMember {
    /// ID of the pool member
    pub id: ::uuid::Uuid,
    /// Maximum prefix length for allocations from this subnet; a larger prefix
    /// means smaller allocations are allowed (e.g. a /24 prefix yields smaller
    /// subnet allocations than a /16 prefix).
    pub max_prefix_length: u8,
    /// Minimum prefix length for allocations from this subnet; a smaller prefix
    /// means larger allocations are allowed (e.g. a /16 prefix yields larger
    /// subnet allocations than a /24 prefix).
    pub min_prefix_length: u8,
    /// The subnet CIDR
    pub subnet: IpNet,
    /// ID of the parent subnet pool
    pub subnet_pool_id: ::uuid::Uuid,
    /// Time the pool member was created.
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SubnetPoolMember {
    pub fn builder() -> builder::SubnetPoolMember {
        Default::default()
    }
}

/// Add a member (subnet) to a subnet pool
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolMemberAdd {
    /// Maximum prefix length for allocations from this subnet; a larger prefix
    /// means smaller allocations are allowed (e.g. a /24 prefix yields smaller
    /// subnet allocations than a /16 prefix).
    ///
    /// Valid values: 0-32 for IPv4, 0-128 for IPv6. Default if not specified is
    /// 32 for IPv4 and 128 for IPv6.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_prefix_length: ::std::option::Option<u8>,
    /// Minimum prefix length for allocations from this subnet; a smaller prefix
    /// means larger allocations are allowed (e.g. a /16 prefix yields larger
    /// subnet allocations than a /24 prefix).
    ///
    /// Valid values: 0-32 for IPv4, 0-128 for IPv6. Default if not specified is
    /// equal to the subnet's prefix length.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub min_prefix_length: ::std::option::Option<u8>,
    /// The subnet to add to the pool
    pub subnet: IpNet,
}

impl SubnetPoolMemberAdd {
    pub fn builder() -> builder::SubnetPoolMemberAdd {
        Default::default()
    }
}

/// Remove a subnet from a pool
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolMemberRemove {
    /// The subnet to remove from the pool. Must match an existing entry
    /// exactly.
    pub subnet: IpNet,
}

impl SubnetPoolMemberRemove {
    pub fn builder() -> builder::SubnetPoolMemberRemove {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolMemberResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SubnetPoolMember>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SubnetPoolMemberResultsPage {
    pub fn builder() -> builder::SubnetPoolMemberResultsPage {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SubnetPool>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SubnetPoolResultsPage {
    pub fn builder() -> builder::SubnetPoolResultsPage {
        Default::default()
    }
}

/// A link between a subnet pool and a silo
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolSiloLink {
    pub is_default: bool,
    pub silo_id: ::uuid::Uuid,
    pub subnet_pool_id: ::uuid::Uuid,
}

impl SubnetPoolSiloLink {
    pub fn builder() -> builder::SubnetPoolSiloLink {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolSiloLinkResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SubnetPoolSiloLink>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SubnetPoolSiloLinkResultsPage {
    pub fn builder() -> builder::SubnetPoolSiloLinkResultsPage {
        Default::default()
    }
}

/// Update a subnet pool's silo link
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolSiloUpdate {
    /// Whether this is the default subnet pool for the silo
    pub is_default: bool,
}

impl SubnetPoolSiloUpdate {
    pub fn builder() -> builder::SubnetPoolSiloUpdate {
        Default::default()
    }
}

/// Update a subnet pool
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct SubnetPoolUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl SubnetPoolUpdate {
    pub fn builder() -> builder::SubnetPoolUpdate {
        Default::default()
    }
}

/// Utilization of addresses in a subnet pool.
///
/// Note that both the count of remaining addresses and the total capacity are
/// integers, reported as floating point numbers. This accommodates allocations
/// larger than a 64-bit integer, which is common with IPv6 address spaces. With
/// very large subnet pools (> 2**53 addresses), integer precision will be lost,
/// in exchange for representing the entire range. In such a case the pool still
/// has many available addresses.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SubnetPoolUtilization {
    /// The total number of addresses in the pool.
    pub capacity: f64,
    /// The number of remaining addresses in the pool.
    pub remaining: f64,
}

impl SubnetPoolUtilization {
    pub fn builder() -> builder::SubnetPoolUtilization {
        Default::default()
    }
}

/// `SupportBundleCreate`
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct SupportBundleCreate {
    /// User comment for the support bundle
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_comment: ::std::option::Option<::std::string::String>,
}

impl SupportBundleCreate {
    pub fn builder() -> builder::SupportBundleCreate {
        Default::default()
    }
}

/// `SupportBundleInfo`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SupportBundleInfo {
    pub id: ::uuid::Uuid,
    pub reason_for_creation: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reason_for_failure: ::std::option::Option<::std::string::String>,
    pub state: SupportBundleState,
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_comment: ::std::option::Option<::std::string::String>,
}

impl SupportBundleInfo {
    pub fn builder() -> builder::SupportBundleInfo {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SupportBundleInfoResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SupportBundleInfo>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SupportBundleInfoResultsPage {
    pub fn builder() -> builder::SupportBundleInfoResultsPage {
        Default::default()
    }
}

/// `SupportBundleState`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum SupportBundleState {
    /// Support Bundle still actively being collected.
    ///
    /// This is the initial state for a Support Bundle, and it will
    /// automatically transition to either "Failing" or "Active".
    ///
    /// If a user no longer wants to access a Support Bundle, they can request
    /// cancellation, which will transition to the "Destroying" state.
    #[serde(rename = "collecting")]
    Collecting,
    /// Support Bundle is being destroyed.
    ///
    /// Once backing storage has been freed, this bundle is destroyed.
    #[serde(rename = "destroying")]
    Destroying,
    /// Support Bundle was not created successfully, or was created and has lost
    /// backing storage.
    ///
    /// The record of the bundle still exists for readability, but the only
    /// valid operation on these bundles is to destroy them.
    #[serde(rename = "failed")]
    Failed,
    /// Support Bundle has been processed, and is ready for usage.
    #[serde(rename = "active")]
    Active,
}

impl ::std::fmt::Display for SupportBundleState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Collecting => f.write_str("collecting"),
            Self::Destroying => f.write_str("destroying"),
            Self::Failed => f.write_str("failed"),
            Self::Active => f.write_str("active"),
        }
    }
}

impl ::std::str::FromStr for SupportBundleState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "collecting" => Ok(Self::Collecting),
            "destroying" => Ok(Self::Destroying),
            "failed" => Ok(Self::Failed),
            "active" => Ok(Self::Active),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SupportBundleState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SupportBundleState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `SupportBundleUpdate`
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct SupportBundleUpdate {
    /// User comment for the support bundle
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub user_comment: ::std::option::Option<::std::string::String>,
}

impl SupportBundleUpdate {
    pub fn builder() -> builder::SupportBundleUpdate {
        Default::default()
    }
}

/// An operator's view of a Switch.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Switch {
    pub baseboard: Baseboard,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The rack to which this Switch is currently attached
    pub rack_id: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Switch {
    pub fn builder() -> builder::Switch {
        Default::default()
    }
}

/// BGP message history for a particular switch.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchBgpHistory {
    /// Message history indexed by peer address.
    pub history: ::std::collections::HashMap<::std::string::String, BgpMessageHistory>,
    /// Switch this message history is associated with.
    pub switch: SwitchSlot,
}

impl SwitchBgpHistory {
    pub fn builder() -> builder::SwitchBgpHistory {
        Default::default()
    }
}

/// A switch port interface configuration for a port settings object.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchInterfaceConfig {
    /// A unique identifier for this switch interface.
    pub id: ::uuid::Uuid,
    /// The name of this switch interface.
    pub interface_name: Name,
    /// The switch interface kind.
    pub kind: SwitchInterfaceKind,
    /// The port settings object this switch interface configuration belongs to.
    pub port_settings_id: ::uuid::Uuid,
    /// Whether or not IPv6 is enabled on this interface.
    pub v6_enabled: bool,
}

impl SwitchInterfaceConfig {
    pub fn builder() -> builder::SwitchInterfaceConfig {
        Default::default()
    }
}

/// A layer-3 switch interface configuration. When IPv6 is enabled, a link local
/// address will be created for the interface.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchInterfaceConfigCreate {
    /// What kind of switch interface this configuration represents.
    pub kind: SwitchInterfaceKind,
    /// Link name. On ports that are not broken out, this is always phy0. On a
    /// 2x breakout the options are phy0 and phy1, on 4x phy0-phy3, etc.
    pub link_name: Name,
    /// Whether or not IPv6 is enabled.
    pub v6_enabled: bool,
}

impl SwitchInterfaceConfigCreate {
    pub fn builder() -> builder::SwitchInterfaceConfigCreate {
        Default::default()
    }
}

/// Indicates the kind for a switch interface.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "vid")]
pub enum SwitchInterfaceKind {
    #[serde(rename = "primary")]
    Primary,
    /// VLAN interfaces allow physical interfaces to be multiplexed onto
    /// multiple logical links, each distinguished by a 12-bit 802.1Q Ethernet
    /// tag.
    #[serde(rename = "vlan")]
    Vlan(u16),
    #[serde(rename = "loopback")]
    Loopback,
}

impl ::std::convert::From<u16> for SwitchInterfaceKind {
    fn from(value: u16) -> Self {
        Self::Vlan(value)
    }
}

/// `SwitchLinkState`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct SwitchLinkState(pub ::serde_json::Value);
impl ::std::ops::Deref for SwitchLinkState {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}

impl ::std::convert::From<SwitchLinkState> for ::serde_json::Value {
    fn from(value: SwitchLinkState) -> Self {
        value.0
    }
}

impl ::std::convert::From<::serde_json::Value> for SwitchLinkState {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}

/// A switch port represents a physical external port on a rack switch.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPort {
    /// The id of the switch port.
    pub id: ::uuid::Uuid,
    /// The name of this switch port.
    pub port_name: Name,
    /// The primary settings group of this switch port. Will be `None` until
    /// this switch port is configured.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub port_settings_id: ::std::option::Option<::uuid::Uuid>,
    /// The rack this switch port belongs to.
    pub rack_id: ::uuid::Uuid,
    /// The slot of the switch within the rack of this switch port.
    pub switch_slot: SwitchSlot,
}

impl SwitchPort {
    pub fn builder() -> builder::SwitchPort {
        Default::default()
    }
}

/// An IP address configuration for a port settings object.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortAddressView {
    /// The IP address and prefix.
    pub address: IpNet,
    /// The id of the address lot block this address is drawn from.
    pub address_lot_block_id: ::uuid::Uuid,
    /// The id of the address lot this address is drawn from.
    pub address_lot_id: ::uuid::Uuid,
    /// The name of the address lot this address is drawn from.
    pub address_lot_name: Name,
    /// The interface name this address belongs to.
    pub interface_name: Name,
    /// The port settings object this address configuration belongs to.
    pub port_settings_id: ::uuid::Uuid,
    /// An optional VLAN ID
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vlan_id: ::std::option::Option<u16>,
}

impl SwitchPortAddressView {
    pub fn builder() -> builder::SwitchPortAddressView {
        Default::default()
    }
}

/// Parameters for applying settings to switch ports.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortApplySettings {
    /// A name or id to use when applying switch port settings.
    pub port_settings: NameOrId,
}

impl SwitchPortApplySettings {
    pub fn builder() -> builder::SwitchPortApplySettings {
        Default::default()
    }
}

/// A physical port configuration for a port settings object.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortConfig {
    /// The physical link geometry of the port.
    pub geometry: SwitchPortGeometry,
    /// The id of the port settings object this configuration belongs to.
    pub port_settings_id: ::uuid::Uuid,
}

impl SwitchPortConfig {
    pub fn builder() -> builder::SwitchPortConfig {
        Default::default()
    }
}

/// Physical switch port configuration.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortConfigCreate {
    /// Link geometry for the switch port.
    pub geometry: SwitchPortGeometry,
}

impl SwitchPortConfigCreate {
    pub fn builder() -> builder::SwitchPortConfigCreate {
        Default::default()
    }
}

/// The link geometry associated with a switch port.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum SwitchPortGeometry {
    /// The port contains a single QSFP28 link with four lanes.
    #[serde(rename = "qsfp28x1")]
    Qsfp28x1,
    /// The port contains two QSFP28 links each with two lanes.
    #[serde(rename = "qsfp28x2")]
    Qsfp28x2,
    /// The port contains four SFP28 links each with one lane.
    #[serde(rename = "sfp28x4")]
    Sfp28x4,
}

impl ::std::fmt::Display for SwitchPortGeometry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Qsfp28x1 => f.write_str("qsfp28x1"),
            Self::Qsfp28x2 => f.write_str("qsfp28x2"),
            Self::Sfp28x4 => f.write_str("sfp28x4"),
        }
    }
}

impl ::std::str::FromStr for SwitchPortGeometry {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "qsfp28x1" => Ok(Self::Qsfp28x1),
            "qsfp28x2" => Ok(Self::Qsfp28x2),
            "sfp28x4" => Ok(Self::Sfp28x4),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SwitchPortGeometry {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SwitchPortGeometry {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A link configuration for a port settings object.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortLinkConfig {
    /// Whether or not the link has autonegotiation enabled.
    pub autoneg: bool,
    /// The requested forward-error correction method.  If this is not
    /// specified, the standard FEC for the underlying media will be applied if
    /// it can be determined.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fec: ::std::option::Option<LinkFec>,
    /// The name of this link.
    pub link_name: Name,
    /// The link-layer discovery protocol service configuration for this link.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lldp_link_config: ::std::option::Option<LldpLinkConfig>,
    /// The maximum transmission unit for this link.
    pub mtu: u16,
    /// The port settings this link configuration belongs to.
    pub port_settings_id: ::uuid::Uuid,
    /// The configured speed of the link.
    pub speed: LinkSpeed,
    /// The tx_eq configuration for this link.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_eq_config: ::std::option::Option<TxEqConfig>,
}

impl SwitchPortLinkConfig {
    pub fn builder() -> builder::SwitchPortLinkConfig {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SwitchPort>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SwitchPortResultsPage {
    pub fn builder() -> builder::SwitchPortResultsPage {
        Default::default()
    }
}

/// A route configuration for a port settings object.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortRouteConfig {
    /// The route's destination network.
    pub dst: IpNet,
    /// The route's gateway address.
    pub gw: ::std::net::IpAddr,
    /// The interface name this route configuration is assigned to.
    pub interface_name: Name,
    /// The port settings object this route configuration belongs to.
    pub port_settings_id: ::uuid::Uuid,
    /// Route RIB priority. Higher priority indicates precedence within and
    /// across protocols.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rib_priority: ::std::option::Option<u8>,
    /// The VLAN identifier for the route. Use this if the gateway is reachable
    /// over an 802.1Q tagged L2 segment.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vlan_id: ::std::option::Option<u16>,
}

impl SwitchPortRouteConfig {
    pub fn builder() -> builder::SwitchPortRouteConfig {
        Default::default()
    }
}

/// This structure contains all port settings information in one place. It's a
/// convenience data structure for getting a complete view of a particular
/// port's settings.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortSettings {
    /// Layer 3 IP address settings.
    pub addresses: ::std::vec::Vec<SwitchPortAddressView>,
    /// BGP peer settings.
    pub bgp_peers: ::std::vec::Vec<BgpPeer>,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Switch port settings included from other switch port settings groups.
    pub groups: ::std::vec::Vec<SwitchPortSettingsGroups>,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Layer 3 interface settings.
    pub interfaces: ::std::vec::Vec<SwitchInterfaceConfig>,
    /// Layer 2 link settings.
    pub links: ::std::vec::Vec<SwitchPortLinkConfig>,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Layer 1 physical port settings.
    pub port: SwitchPortConfig,
    /// IP route settings.
    pub routes: ::std::vec::Vec<SwitchPortRouteConfig>,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SwitchPortSettings {
    pub fn builder() -> builder::SwitchPortSettings {
        Default::default()
    }
}

/// Parameters for creating switch port settings. Switch port settings are the
/// central data structure for setting up external networking. Switch port
/// settings include link, interface, route, address and dynamic network
/// protocol configuration.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortSettingsCreate {
    /// Address configurations.
    pub addresses: ::std::vec::Vec<AddressConfig>,
    /// BGP peer configurations.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub bgp_peers: ::std::vec::Vec<BgpPeerConfig>,
    pub description: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub groups: ::std::vec::Vec<NameOrId>,
    /// Interface configurations.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub interfaces: ::std::vec::Vec<SwitchInterfaceConfigCreate>,
    /// Link configurations.
    pub links: ::std::vec::Vec<LinkConfigCreate>,
    pub name: Name,
    pub port_config: SwitchPortConfigCreate,
    /// Route configurations.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub routes: ::std::vec::Vec<RouteConfig>,
}

impl SwitchPortSettingsCreate {
    pub fn builder() -> builder::SwitchPortSettingsCreate {
        Default::default()
    }
}

/// This structure maps a port settings object to a port settings groups. Port
/// settings objects may inherit settings from groups. This mapping defines the
/// relationship between settings objects and the groups they reference.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortSettingsGroups {
    /// The id of a port settings group being referenced by a port settings
    /// object.
    pub port_settings_group_id: ::uuid::Uuid,
    /// The id of a port settings object referencing a port settings group.
    pub port_settings_id: ::uuid::Uuid,
}

impl SwitchPortSettingsGroups {
    pub fn builder() -> builder::SwitchPortSettingsGroups {
        Default::default()
    }
}

/// A switch port settings identity whose id may be used to view additional
/// details.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortSettingsIdentity {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SwitchPortSettingsIdentity {
    pub fn builder() -> builder::SwitchPortSettingsIdentity {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchPortSettingsIdentityResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<SwitchPortSettingsIdentity>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SwitchPortSettingsIdentityResultsPage {
    pub fn builder() -> builder::SwitchPortSettingsIdentityResultsPage {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SwitchResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Switch>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl SwitchResultsPage {
    pub fn builder() -> builder::SwitchResultsPage {
        Default::default()
    }
}

/// Identifies switch physical location
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum SwitchSlot {
    /// Switch in upper slot
    #[serde(rename = "switch0")]
    Switch0,
    /// Switch in lower slot
    #[serde(rename = "switch1")]
    Switch1,
}

impl ::std::fmt::Display for SwitchSlot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Switch0 => f.write_str("switch0"),
            Self::Switch1 => f.write_str("switch1"),
        }
    }
}

impl ::std::str::FromStr for SwitchSlot {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "switch0" => Ok(Self::Switch0),
            "switch1" => Ok(Self::Switch1),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SwitchSlot {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SwitchSlot {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `SystemMetricName`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum SystemMetricName {
    #[serde(rename = "virtual_disk_space_provisioned")]
    VirtualDiskSpaceProvisioned,
    #[serde(rename = "cpus_provisioned")]
    CpusProvisioned,
    #[serde(rename = "ram_provisioned")]
    RamProvisioned,
}

impl ::std::fmt::Display for SystemMetricName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VirtualDiskSpaceProvisioned => f.write_str("virtual_disk_space_provisioned"),
            Self::CpusProvisioned => f.write_str("cpus_provisioned"),
            Self::RamProvisioned => f.write_str("ram_provisioned"),
        }
    }
}

impl ::std::str::FromStr for SystemMetricName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "virtual_disk_space_provisioned" => Ok(Self::VirtualDiskSpaceProvisioned),
            "cpus_provisioned" => Ok(Self::CpusProvisioned),
            "ram_provisioned" => Ok(Self::RamProvisioned),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for SystemMetricName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SystemMetricName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Fleet-wide networking settings. Only fleet viewers may view these settings.
/// Only fleet admins can modify them.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct SystemNetworkingSettings {
    /// When true, end users may opt in to jumbo frames (8500 byte MTU) on the
    /// primary interface of an instance. When false, instance-level opt-in is
    /// ignored and the primary interface uses the default MTU.
    pub external_jumbo_frames_opt_in_enabled: bool,
}

impl SystemNetworkingSettings {
    pub fn builder() -> builder::SystemNetworkingSettings {
        Default::default()
    }
}

/// Parameters for updating the fleet-wide networking settings.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct SystemNetworkingSettingsUpdate {
    /// Toggle the fleet-wide external jumbo-frames opt-in.
    #[serde(default)]
    pub external_jumbo_frames_opt_in_enabled: bool,
}

impl SystemNetworkingSettingsUpdate {
    pub fn builder() -> builder::SystemNetworkingSettingsUpdate {
        Default::default()
    }
}

/// `SystemUpdateRepositoryViewSystemVersion`
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct SystemUpdateRepositoryViewSystemVersion(::std::string::String);
impl ::std::ops::Deref for SystemUpdateRepositoryViewSystemVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<SystemUpdateRepositoryViewSystemVersion> for ::std::string::String {
    fn from(value: SystemUpdateRepositoryViewSystemVersion) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for SystemUpdateRepositoryViewSystemVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for SystemUpdateRepositoryViewSystemVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*\
                     [a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?\
                     (?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \
                        \"^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*\
                        [a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*\
                        ))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$\""
                .into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for SystemUpdateRepositoryViewSystemVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for SystemUpdateRepositoryViewSystemVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// View of a system software target release
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct TargetRelease {
    /// Time this was set as the target release
    pub time_requested: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The specified release of the rack's system software
    pub version: TargetReleaseVersion,
}

impl TargetRelease {
    pub fn builder() -> builder::TargetRelease {
        Default::default()
    }
}

/// The specified release of the rack's system software
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct TargetReleaseVersion(::std::string::String);
impl ::std::ops::Deref for TargetReleaseVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<TargetReleaseVersion> for ::std::string::String {
    fn from(value: TargetReleaseVersion) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for TargetReleaseVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for TargetReleaseVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*\
                     [a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?\
                     (?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \
                        \"^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*\
                        [a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*\
                        ))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$\""
                .into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for TargetReleaseVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for TargetReleaseVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// Supported set of sort modes for scanning by timestamp and ID
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum TimeAndIdSortMode {
    /// Sort in increasing order of timestamp and ID, i.e., earliest first
    #[serde(rename = "time_and_id_ascending")]
    TimeAndIdAscending,
    /// Sort in increasing order of timestamp and ID, i.e., most recent first
    #[serde(rename = "time_and_id_descending")]
    TimeAndIdDescending,
}

impl ::std::fmt::Display for TimeAndIdSortMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TimeAndIdAscending => f.write_str("time_and_id_ascending"),
            Self::TimeAndIdDescending => f.write_str("time_and_id_descending"),
        }
    }
}

impl ::std::str::FromStr for TimeAndIdSortMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "time_and_id_ascending" => Ok(Self::TimeAndIdAscending),
            "time_and_id_descending" => Ok(Self::TimeAndIdDescending),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for TimeAndIdSortMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for TimeAndIdSortMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A timeseries contains a timestamped set of values from one source.
///
/// This includes the typed key-value pairs that uniquely identify it, and the
/// set of timestamps and data values from it.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Timeseries {
    pub fields: ::std::collections::HashMap<::std::string::String, FieldValue>,
    pub points: Points,
}

impl Timeseries {
    pub fn builder() -> builder::Timeseries {
        Default::default()
    }
}

/// Text descriptions for the target and metric of a timeseries.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct TimeseriesDescription {
    pub metric: ::std::string::String,
    pub target: ::std::string::String,
}

impl TimeseriesDescription {
    pub fn builder() -> builder::TimeseriesDescription {
        Default::default()
    }
}

/// Names are constructed by concatenating the target and metric names with ':'.
/// Target and metric names must be lowercase alphanumeric characters with '_'
/// separating words.
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct TimeseriesName(::std::string::String);
impl ::std::ops::Deref for TimeseriesName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<TimeseriesName> for ::std::string::String {
    fn from(value: TimeseriesName) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for TimeseriesName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for TimeseriesName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*):(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*)$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \
                        \"^(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*):(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*\
                        )$\""
                .into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for TimeseriesName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for TimeseriesName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// A timeseries query string, written in the Oximeter query language.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct TimeseriesQuery {
    /// A timeseries query string, written in the Oximeter query language.
    pub query: ::std::string::String,
}

impl TimeseriesQuery {
    pub fn builder() -> builder::TimeseriesQuery {
        Default::default()
    }
}

/// The schema for a timeseries.
///
/// This includes the name of the timeseries, as well as the datum type of its
/// metric and the schema for each field.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct TimeseriesSchema {
    pub authz_scope: AuthzScope,
    pub created: ::chrono::DateTime<::chrono::offset::Utc>,
    pub datum_type: DatumType,
    pub description: TimeseriesDescription,
    pub field_schema: ::std::vec::Vec<FieldSchema>,
    pub timeseries_name: TimeseriesName,
    pub units: Units,
    pub version: ::std::num::NonZeroU8,
}

impl TimeseriesSchema {
    pub fn builder() -> builder::TimeseriesSchema {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct TimeseriesSchemaResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<TimeseriesSchema>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl TimeseriesSchemaResultsPage {
    pub fn builder() -> builder::TimeseriesSchemaResultsPage {
        Default::default()
    }
}

/// Metadata about a TUF repository
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct TufRepo {
    /// The file name of the repository, as reported by the client that uploaded
    /// it
    ///
    /// This is intended for debugging. The file name may not match any
    /// particular pattern, and even if it does, it may not be accurate since
    /// it's just what the client reported.
    pub file_name: ::std::string::String,
    /// The hash of the repository
    pub hash: TufRepoHash,
    /// The system version for this repository
    ///
    /// The system version is a top-level version number applied to all the
    /// software in the repository.
    pub system_version: TufRepoSystemVersion,
    /// Time the repository was uploaded
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl TufRepo {
    pub fn builder() -> builder::TufRepo {
        Default::default()
    }
}

/// The hash of the repository
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct TufRepoHash(::std::string::String);
impl ::std::ops::Deref for TufRepoHash {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<TufRepoHash> for ::std::string::String {
    fn from(value: TufRepoHash) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for TufRepoHash {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for TufRepoHash {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }

        if value.chars().count() < 64usize {
            return Err("shorter than 64 characters".into());
        }

        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9a-fA-F]{64}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-fA-F]{64}$\"".into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for TufRepoHash {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for TufRepoHash {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct TufRepoResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<TufRepo>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl TufRepoResultsPage {
    pub fn builder() -> builder::TufRepoResultsPage {
        Default::default()
    }
}

/// The system version for this repository
///
/// The system version is a top-level version number applied to all the software
/// in the repository.
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct TufRepoSystemVersion(::std::string::String);
impl ::std::ops::Deref for TufRepoSystemVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<TufRepoSystemVersion> for ::std::string::String {
    fn from(value: TufRepoSystemVersion) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for TufRepoSystemVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for TufRepoSystemVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*\
                     [a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?\
                     (?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \
                        \"^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*\
                        [a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*\
                        ))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$\""
                .into());
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for TufRepoSystemVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for TufRepoSystemVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// `TufRepoUpload`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct TufRepoUpload {
    pub repo: TufRepo,
    pub status: TufRepoUploadStatus,
}

impl TufRepoUpload {
    pub fn builder() -> builder::TufRepoUpload {
        Default::default()
    }
}

/// Whether the uploaded TUF repo already existed or was new and had to be
/// inserted. Part of `TufRepoUpload`.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum TufRepoUploadStatus {
    /// The repository already existed in the database
    #[serde(rename = "already_exists")]
    AlreadyExists,
    /// The repository did not exist, and was inserted into the database
    #[serde(rename = "inserted")]
    Inserted,
}

impl ::std::fmt::Display for TufRepoUploadStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AlreadyExists => f.write_str("already_exists"),
            Self::Inserted => f.write_str("inserted"),
        }
    }
}

impl ::std::str::FromStr for TufRepoUploadStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "already_exists" => Ok(Self::AlreadyExists),
            "inserted" => Ok(Self::Inserted),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for TufRepoUploadStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for TufRepoUploadStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Per-port tx-eq overrides.  This can be used to fine-tune the transceiver
/// equalization settings to improve signal integrity.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct TxEqConfig {
    /// Main tap
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub main: ::std::option::Option<i32>,
    /// Post-cursor tap1
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub post1: ::std::option::Option<i32>,
    /// Post-cursor tap2
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub post2: ::std::option::Option<i32>,
    /// Pre-cursor tap1
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pre1: ::std::option::Option<i32>,
    /// Pre-cursor tap2
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pre2: ::std::option::Option<i32>,
}

impl TxEqConfig {
    pub fn builder() -> builder::TxEqConfig {
        Default::default()
    }
}

/// A physical disk that has not yet been adopted by the control plane
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UnadoptedPhysicalDisk {
    pub disk_id: PhysicalDiskManufacturerIdentity,
    pub sled_id: SledUuid,
    pub slot: i64,
    pub variant: PhysicalDiskKind,
}

impl UnadoptedPhysicalDisk {
    pub fn builder() -> builder::UnadoptedPhysicalDisk {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UnadoptedPhysicalDiskResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<UnadoptedPhysicalDisk>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl UnadoptedPhysicalDiskResultsPage {
    pub fn builder() -> builder::UnadoptedPhysicalDiskResultsPage {
        Default::default()
    }
}

/// A sled that has not been added to an initialized rack yet
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UninitializedSled {
    pub baseboard: Baseboard,
    pub cubby: u16,
    pub rack_id: ::uuid::Uuid,
}

impl UninitializedSled {
    pub fn builder() -> builder::UninitializedSled {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UninitializedSledResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<UninitializedSled>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl UninitializedSledResultsPage {
    pub fn builder() -> builder::UninitializedSledResultsPage {
        Default::default()
    }
}

/// Measurement units for timeseries samples.
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum Units {
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "bytes")]
    Bytes,
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "nanoseconds")]
    Nanoseconds,
    #[serde(rename = "volts")]
    Volts,
    #[serde(rename = "amps")]
    Amps,
    #[serde(rename = "watts")]
    Watts,
    #[serde(rename = "degrees_celsius")]
    DegreesCelsius,
    #[serde(rename = "joules")]
    Joules,
    /// No meaningful units, e.g. a dimensionless quanity.
    #[serde(rename = "none")]
    None,
    /// Rotations per minute.
    #[serde(rename = "rpm")]
    Rpm,
}

impl ::std::fmt::Display for Units {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Count => f.write_str("count"),
            Self::Bytes => f.write_str("bytes"),
            Self::Seconds => f.write_str("seconds"),
            Self::Nanoseconds => f.write_str("nanoseconds"),
            Self::Volts => f.write_str("volts"),
            Self::Amps => f.write_str("amps"),
            Self::Watts => f.write_str("watts"),
            Self::DegreesCelsius => f.write_str("degrees_celsius"),
            Self::Joules => f.write_str("joules"),
            Self::None => f.write_str("none"),
            Self::Rpm => f.write_str("rpm"),
        }
    }
}

impl ::std::str::FromStr for Units {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "count" => Ok(Self::Count),
            "bytes" => Ok(Self::Bytes),
            "seconds" => Ok(Self::Seconds),
            "nanoseconds" => Ok(Self::Nanoseconds),
            "volts" => Ok(Self::Volts),
            "amps" => Ok(Self::Amps),
            "watts" => Ok(Self::Watts),
            "degrees_celsius" => Ok(Self::DegreesCelsius),
            "joules" => Ok(Self::Joules),
            "none" => Ok(Self::None),
            "rpm" => Ok(Self::Rpm),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for Units {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for Units {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `UpdateStatus`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UpdateStatus {
    /// Count of components running each release version
    ///
    /// Keys will be either:
    ///
    /// * Semver-like release version strings * "install dataset", representing
    ///   the initial rack software before any updates * "unknown", which means
    ///   there is no TUF repo uploaded that matches the software running on the
    ///   component)
    pub components_by_release_version: ::std::collections::HashMap<::std::string::String, u32>,
    /// If true, the system has detected one or more known conditions that
    /// require Oxide support to resolve
    ///
    /// You should contact support to resolve these issues before proceeding
    /// with an update, or after one has completed. The checks underlying this
    /// field are not exhaustive, so this being `false` does not mean the entire
    /// system is completely healthy.
    pub contact_support: bool,
    /// Whether automatic update is suspended due to manual update activity
    ///
    /// After a manual support procedure that changes the system software,
    /// automatic update activity is suspended to avoid undoing the change. To
    /// resume automatic update, first upload the TUF repository matching the
    /// manually applied update, then set that as the target release.
    pub suspended: bool,
    /// Current target release of the system software
    ///
    /// This may not correspond to the actual system software running at the
    /// time of request; it is instead the release that the system should be
    /// moving towards as a goal state. The system asynchronously updates
    /// software to match this target release.
    ///
    /// Will only be null if a target release has never been set. In that case,
    /// the system is not automatically attempting to manage software versions.
    #[serde(deserialize_with = "::std::option::Option::deserialize")]
    pub target_release: ::std::option::Option<TargetRelease>,
    /// Time of most recent update planning activity
    ///
    /// This is intended as a rough indicator of the last time something
    /// happened in the update planner.
    pub time_last_step_planned: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl UpdateStatus {
    pub fn builder() -> builder::UpdateStatus {
        Default::default()
    }
}

/// Trusted root role used by the update system to verify update repositories.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UpdatesTrustRoot {
    /// The UUID of this trusted root role.
    pub id: ::uuid::Uuid,
    /// The trusted root role itself, a JSON document as described by The Update
    /// Framework.
    pub root_role: ::serde_json::Value,
    /// Time the trusted root role was added.
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl UpdatesTrustRoot {
    pub fn builder() -> builder::UpdatesTrustRoot {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UpdatesTrustRootResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<UpdatesTrustRoot>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl UpdatesTrustRootResultsPage {
    pub fn builder() -> builder::UpdatesTrustRootResultsPage {
        Default::default()
    }
}

/// View of a User
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct User {
    /// Human-readable name that can identify the user
    pub display_name: ::std::string::String,
    pub id: ::uuid::Uuid,
    /// Uuid of the silo to which this user belongs
    pub silo_id: ::uuid::Uuid,
    /// Timestamp when this user was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this user was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl User {
    pub fn builder() -> builder::User {
        Default::default()
    }
}

/// View of a Built-in User
///
/// Built-in users are identities internal to the system, used when the control
/// plane performs actions autonomously
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UserBuiltin {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl UserBuiltin {
    pub fn builder() -> builder::UserBuiltin {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UserBuiltinResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<UserBuiltin>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl UserBuiltinResultsPage {
    pub fn builder() -> builder::UserBuiltinResultsPage {
        Default::default()
    }
}

/// Create-time parameters for a `User`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UserCreate {
    /// Username used to log in
    pub external_id: UserId,
    /// How to set the user's login password
    pub password: UserPassword,
}

impl UserCreate {
    pub fn builder() -> builder::UserCreate {
        Default::default()
    }
}

/// Usernames must begin with a lower case ASCII letter, be composed exclusively
/// of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with
/// a '-'. Usernames cannot be a UUID, but they may contain a UUID. They can be
/// at most 63 characters long.
#[derive(
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
#[serde(transparent)]
pub struct UserId(::std::string::String);
impl ::std::ops::Deref for UserId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}

impl ::std::convert::From<UserId> for ::std::string::String {
    fn from(value: UserId) -> Self {
        value.0
    }
}

impl ::std::str::FromStr for UserId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}

impl ::std::convert::TryFrom<&str> for UserId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 63usize {
            return Err("longer than 63 characters".into());
        }

        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }

        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z]([a-zA-Z0-9-]*[a-zA-Z0-9]+)?$") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z]([a-zA-Z0-9-]*[a-zA-Z0-9]+)?$\"" . into ()) ;
        }

        Ok(Self(value.to_string()))
    }
}

impl ::std::convert::TryFrom<::std::string::String> for UserId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}

/// Parameters for setting a user's password
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "mode", content = "value")]
pub enum UserPassword {
    /// Sets the user's password to the provided value
    #[serde(rename = "password")]
    Password(Password),
    #[serde(rename = "login_disallowed")]
    LoginDisallowed,
}

impl ::std::convert::From<Password> for UserPassword {
    fn from(value: Password) -> Self {
        Self::Password(value)
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UserResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<User>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl UserResultsPage {
    pub fn builder() -> builder::UserResultsPage {
        Default::default()
    }
}

/// Credentials for local user login
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct UsernamePasswordCredentials {
    pub password: Password,
    pub username: UserId,
}

impl UsernamePasswordCredentials {
    pub fn builder() -> builder::UsernamePasswordCredentials {
        Default::default()
    }
}

/// View of the current silo's resource utilization and capacity
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Utilization {
    /// The total amount of resources that can be provisioned in this silo.
    /// Actions that would exceed this limit will fail.
    pub capacity: VirtualResourceCounts,
    /// Accounts for resources allocated to running instances or storage
    /// allocated via disks or snapshots.
    ///
    /// Note that CPU and memory resources associated with stopped instances are
    /// not counted here, whereas associated disks will still be counted.
    pub provisioned: VirtualResourceCounts,
}

impl Utilization {
    pub fn builder() -> builder::Utilization {
        Default::default()
    }
}

/// List of data values for one timeseries.
///
/// Each element is an option, where `None` represents a missing sample.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "values")]
pub enum ValueArray {
    #[serde(rename = "integer")]
    Integer(::std::vec::Vec<::std::option::Option<i64>>),
    #[serde(rename = "double")]
    Double(::std::vec::Vec<::std::option::Option<f64>>),
    #[serde(rename = "boolean")]
    Boolean(::std::vec::Vec<::std::option::Option<bool>>),
    #[serde(rename = "string")]
    String(::std::vec::Vec<::std::option::Option<::std::string::String>>),
    #[serde(rename = "integer_distribution")]
    IntegerDistribution(::std::vec::Vec<::std::option::Option<Distributionint64>>),
    #[serde(rename = "double_distribution")]
    DoubleDistribution(::std::vec::Vec<::std::option::Option<Distributiondouble>>),
}

impl ::std::convert::From<::std::vec::Vec<::std::option::Option<i64>>> for ValueArray {
    fn from(value: ::std::vec::Vec<::std::option::Option<i64>>) -> Self {
        Self::Integer(value)
    }
}

impl ::std::convert::From<::std::vec::Vec<::std::option::Option<f64>>> for ValueArray {
    fn from(value: ::std::vec::Vec<::std::option::Option<f64>>) -> Self {
        Self::Double(value)
    }
}

impl ::std::convert::From<::std::vec::Vec<::std::option::Option<bool>>> for ValueArray {
    fn from(value: ::std::vec::Vec<::std::option::Option<bool>>) -> Self {
        Self::Boolean(value)
    }
}

impl ::std::convert::From<::std::vec::Vec<::std::option::Option<::std::string::String>>>
    for ValueArray
{
    fn from(value: ::std::vec::Vec<::std::option::Option<::std::string::String>>) -> Self {
        Self::String(value)
    }
}

impl ::std::convert::From<::std::vec::Vec<::std::option::Option<Distributionint64>>>
    for ValueArray
{
    fn from(value: ::std::vec::Vec<::std::option::Option<Distributionint64>>) -> Self {
        Self::IntegerDistribution(value)
    }
}

impl ::std::convert::From<::std::vec::Vec<::std::option::Option<Distributiondouble>>>
    for ValueArray
{
    fn from(value: ::std::vec::Vec<::std::option::Option<Distributiondouble>>) -> Self {
        Self::DoubleDistribution(value)
    }
}

/// A single list of values, for one dimension of a timeseries.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Values {
    /// The type of this metric.
    pub metric_type: MetricType,
    /// The data values.
    pub values: ValueArray,
}

impl Values {
    pub fn builder() -> builder::Values {
        Default::default()
    }
}

/// Supported sort modes when scanning by semantic version
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum VersionSortMode {
    /// Sort in increasing semantic version order (oldest first)
    #[serde(rename = "version_ascending")]
    VersionAscending,
    /// Sort in decreasing semantic version order (newest first)
    #[serde(rename = "version_descending")]
    VersionDescending,
}

impl ::std::fmt::Display for VersionSortMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VersionAscending => f.write_str("version_ascending"),
            Self::VersionDescending => f.write_str("version_descending"),
        }
    }
}

impl ::std::str::FromStr for VersionSortMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "version_ascending" => Ok(Self::VersionAscending),
            "version_descending" => Ok(Self::VersionDescending),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for VersionSortMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for VersionSortMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A collection of resource counts used to describe capacity and utilization
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VirtualResourceCounts {
    /// Number of virtual CPUs
    pub cpus: i64,
    /// Amount of memory in bytes
    pub memory: ByteCount,
    /// Amount of disk storage in bytes
    pub storage: ByteCount,
}

impl VirtualResourceCounts {
    pub fn builder() -> builder::VirtualResourceCounts {
        Default::default()
    }
}

/// A Geneve Virtual Network Identifier
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(transparent)]
pub struct Vni(pub u32);
impl ::std::ops::Deref for Vni {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl ::std::convert::From<Vni> for u32 {
    fn from(value: Vni) -> Self {
        value.0
    }
}

impl ::std::convert::From<u32> for Vni {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl ::std::fmt::Display for Vni {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for Vni {
    type Err = <u32 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}

/// View of a VPC
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct Vpc {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// The name used for the VPC in DNS.
    pub dns_name: Name,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The unique local IPv6 address range for subnets in this VPC
    pub ipv6_prefix: Ipv6Net,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// ID for the project containing this VPC
    pub project_id: ::uuid::Uuid,
    /// ID for the system router where subnet default routes are registered
    pub system_router_id: ::uuid::Uuid,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl Vpc {
    pub fn builder() -> builder::Vpc {
        Default::default()
    }
}

/// Create-time parameters for a `Vpc`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcCreate {
    /// Default resources to create in the VPC
    ///
    /// Omit this field  or pass `null`  to create all defaults: currently, the
    /// default subnet. Pass an object to specify which resources to create.
    /// `{}` creates none.
    ///
    /// This does not affect the system router, default firewall rules, or
    /// default internet gateway, which are always created and do not block
    /// deletion of the VPC.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub defaults: ::std::option::Option<VpcCreateDefaults>,
    pub description: ::std::string::String,
    pub dns_name: Name,
    /// The IPv6 prefix for this VPC
    ///
    /// All IPv6 subnets created from this VPC must be taken from this range,
    /// which should be a Unique Local Address in the range `fd00::/48`. The
    /// default subnet, if requested, will take the first `/64` range from this
    /// prefix.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ipv6_prefix: ::std::option::Option<Ipv6Net>,
    pub name: Name,
}

impl VpcCreate {
    pub fn builder() -> builder::VpcCreate {
        Default::default()
    }
}

/// Default resources to create in a VPC
///
/// Each field corresponds to one resource. Set a field to an object to create
/// that resource. Omit it or pass `null` to skip it.
///
/// This does not affect the system router, default firewall rules, or default
/// internet gateway, which are always created and do not block deletion of the
/// VPC.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct VpcCreateDefaults {
    /// Create the default subnet. Pass `{}` to create it and omit this field
    /// (or pass `null`) to skip it.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subnet: ::std::option::Option<SubnetCreateDefaults>,
}

impl VpcCreateDefaults {
    pub fn builder() -> builder::VpcCreateDefaults {
        Default::default()
    }
}

/// Default resources to create in a VPC
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "defaults")]
pub enum VpcCreateDefaultsSelection {
    #[serde(rename = "all")]
    All,
    /// Create only the default resources listed in `defaults`. Pass `{}` as
    /// `defaults` to skip them all.
    #[serde(rename = "explicit")]
    Explicit(VpcCreateDefaults),
}

impl ::std::convert::From<VpcCreateDefaults> for VpcCreateDefaultsSelection {
    fn from(value: VpcCreateDefaults) -> Self {
        Self::Explicit(value)
    }
}

/// `VpcFirewallIcmpFilter`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcFirewallIcmpFilter {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<IcmpParamRange>,
    pub icmp_type: u8,
}

impl VpcFirewallIcmpFilter {
    pub fn builder() -> builder::VpcFirewallIcmpFilter {
        Default::default()
    }
}

/// A single rule in a VPC firewall
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcFirewallRule {
    /// Whether traffic matching the rule should be allowed or dropped
    pub action: VpcFirewallRuleAction,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Whether this rule is for incoming or outgoing traffic
    pub direction: VpcFirewallRuleDirection,
    /// Reductions on the scope of the rule
    pub filters: VpcFirewallRuleFilter,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// The relative priority of this rule
    pub priority: u16,
    /// Whether this rule is in effect
    pub status: VpcFirewallRuleStatus,
    /// Determine the set of instances that the rule applies to
    pub targets: ::std::vec::Vec<VpcFirewallRuleTarget>,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The VPC to which this rule belongs
    pub vpc_id: ::uuid::Uuid,
}

impl VpcFirewallRule {
    pub fn builder() -> builder::VpcFirewallRule {
        Default::default()
    }
}

/// `VpcFirewallRuleAction`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum VpcFirewallRuleAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

impl ::std::fmt::Display for VpcFirewallRuleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allow => f.write_str("allow"),
            Self::Deny => f.write_str("deny"),
        }
    }
}

impl ::std::str::FromStr for VpcFirewallRuleAction {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow" => Ok(Self::Allow),
            "deny" => Ok(Self::Deny),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for VpcFirewallRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for VpcFirewallRuleAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// `VpcFirewallRuleDirection`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum VpcFirewallRuleDirection {
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    Outbound,
}

impl ::std::fmt::Display for VpcFirewallRuleDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inbound => f.write_str("inbound"),
            Self::Outbound => f.write_str("outbound"),
        }
    }
}

impl ::std::str::FromStr for VpcFirewallRuleDirection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inbound" => Ok(Self::Inbound),
            "outbound" => Ok(Self::Outbound),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for VpcFirewallRuleDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for VpcFirewallRuleDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// Filters reduce the scope of a firewall rule. Without filters, the rule
/// applies to all packets to the targets (or from the targets, if it's an
/// outbound rule). With multiple filters, the rule applies only to packets
/// matching ALL filters. The maximum number of each type of filter is 256.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct VpcFirewallRuleFilter {
    /// If present, host filters match the "other end" of traffic from the
    /// target’s perspective: for an inbound rule, they match the source of
    /// traffic. For an outbound rule, they match the destination.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hosts: ::std::option::Option<::std::vec::Vec<VpcFirewallRuleHostFilter>>,
    /// If present, the destination ports or port ranges this rule applies to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ports: ::std::option::Option<::std::vec::Vec<L4PortRange>>,
    /// If present, the networking protocols this rule applies to.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub protocols: ::std::option::Option<::std::vec::Vec<VpcFirewallRuleProtocol>>,
}

impl VpcFirewallRuleFilter {
    pub fn builder() -> builder::VpcFirewallRuleFilter {
        Default::default()
    }
}

/// The `VpcFirewallRuleHostFilter` is used to filter traffic on the basis of
/// its source or destination host.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum VpcFirewallRuleHostFilter {
    /// The rule applies to traffic from/to all instances in the VPC
    #[serde(rename = "vpc")]
    Vpc(Name),
    /// The rule applies to traffic from/to all instances in the VPC Subnet
    #[serde(rename = "subnet")]
    Subnet(Name),
    /// The rule applies to traffic from/to this specific instance
    #[serde(rename = "instance")]
    Instance(Name),
    /// The rule applies to traffic from/to a specific IP address
    #[serde(rename = "ip")]
    Ip(::std::net::IpAddr),
    /// The rule applies to traffic from/to a specific IP subnet
    #[serde(rename = "ip_net")]
    IpNet(IpNet),
}

impl ::std::convert::From<::std::net::IpAddr> for VpcFirewallRuleHostFilter {
    fn from(value: ::std::net::IpAddr) -> Self {
        Self::Ip(value)
    }
}

impl ::std::convert::From<IpNet> for VpcFirewallRuleHostFilter {
    fn from(value: IpNet) -> Self {
        Self::IpNet(value)
    }
}

/// The protocols that may be specified in a firewall rule's filter
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum VpcFirewallRuleProtocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "icmp")]
    Icmp(::std::option::Option<VpcFirewallIcmpFilter>),
    #[serde(rename = "icmp6")]
    Icmp6(::std::option::Option<VpcFirewallIcmpFilter>),
}

/// `VpcFirewallRuleStatus`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum VpcFirewallRuleStatus {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
}

impl ::std::fmt::Display for VpcFirewallRuleStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Disabled => f.write_str("disabled"),
            Self::Enabled => f.write_str("enabled"),
        }
    }
}

impl ::std::str::FromStr for VpcFirewallRuleStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "disabled" => Ok(Self::Disabled),
            "enabled" => Ok(Self::Enabled),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for VpcFirewallRuleStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for VpcFirewallRuleStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A `VpcFirewallRuleTarget` is used to specify the set of instances to which a
/// firewall rule applies. You can target instances directly by name, or specify
/// a VPC, VPC subnet, IP, or IP subnet, which will apply the rule to traffic
/// going to all matching instances. Targets are additive: the rule applies to
/// instances matching ANY target.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum VpcFirewallRuleTarget {
    /// The rule applies to all instances in the VPC
    #[serde(rename = "vpc")]
    Vpc(Name),
    /// The rule applies to all instances in the VPC Subnet
    #[serde(rename = "subnet")]
    Subnet(Name),
    /// The rule applies to this specific instance
    #[serde(rename = "instance")]
    Instance(Name),
    /// The rule applies to a specific IP address
    #[serde(rename = "ip")]
    Ip(::std::net::IpAddr),
    /// The rule applies to a specific IP subnet
    #[serde(rename = "ip_net")]
    IpNet(IpNet),
}

impl ::std::convert::From<::std::net::IpAddr> for VpcFirewallRuleTarget {
    fn from(value: ::std::net::IpAddr) -> Self {
        Self::Ip(value)
    }
}

impl ::std::convert::From<IpNet> for VpcFirewallRuleTarget {
    fn from(value: IpNet) -> Self {
        Self::IpNet(value)
    }
}

/// A single rule in a VPC firewall
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcFirewallRuleUpdate {
    /// Whether traffic matching the rule should be allowed or dropped
    pub action: VpcFirewallRuleAction,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Whether this rule is for incoming or outgoing traffic
    pub direction: VpcFirewallRuleDirection,
    /// Reductions on the scope of the rule
    pub filters: VpcFirewallRuleFilter,
    /// Name of the rule, unique to this VPC
    pub name: Name,
    /// The relative priority of this rule
    pub priority: u16,
    /// Whether this rule is in effect
    pub status: VpcFirewallRuleStatus,
    /// Determine the set of instances that the rule applies to
    pub targets: ::std::vec::Vec<VpcFirewallRuleTarget>,
}

impl VpcFirewallRuleUpdate {
    pub fn builder() -> builder::VpcFirewallRuleUpdate {
        Default::default()
    }
}

/// Updated list of firewall rules. Will replace all existing rules.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct VpcFirewallRuleUpdateParams {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub rules: ::std::vec::Vec<VpcFirewallRuleUpdate>,
}

impl VpcFirewallRuleUpdateParams {
    pub fn builder() -> builder::VpcFirewallRuleUpdateParams {
        Default::default()
    }
}

/// Collection of a Vpc's firewall rules
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcFirewallRules {
    pub rules: ::std::vec::Vec<VpcFirewallRule>,
}

impl VpcFirewallRules {
    pub fn builder() -> builder::VpcFirewallRules {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<Vpc>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl VpcResultsPage {
    pub fn builder() -> builder::VpcResultsPage {
        Default::default()
    }
}

/// A VPC router defines a series of rules that indicate where traffic should be
/// sent depending on its destination.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcRouter {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    pub kind: VpcRouterKind,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The VPC to which the router belongs.
    pub vpc_id: ::uuid::Uuid,
}

impl VpcRouter {
    pub fn builder() -> builder::VpcRouter {
        Default::default()
    }
}

/// Create-time parameters for a `VpcRouter`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcRouterCreate {
    pub description: ::std::string::String,
    pub name: Name,
}

impl VpcRouterCreate {
    pub fn builder() -> builder::VpcRouterCreate {
        Default::default()
    }
}

/// `VpcRouterKind`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum VpcRouterKind {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "custom")]
    Custom,
}

impl ::std::fmt::Display for VpcRouterKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::System => f.write_str("system"),
            Self::Custom => f.write_str("custom"),
        }
    }
}

impl ::std::str::FromStr for VpcRouterKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "system" => Ok(Self::System),
            "custom" => Ok(Self::Custom),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for VpcRouterKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for VpcRouterKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcRouterResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<VpcRouter>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl VpcRouterResultsPage {
    pub fn builder() -> builder::VpcRouterResultsPage {
        Default::default()
    }
}

/// Updateable properties of a `VpcRouter`
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct VpcRouterUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl VpcRouterUpdate {
    pub fn builder() -> builder::VpcRouterUpdate {
        Default::default()
    }
}

/// A VPC subnet represents a logical grouping for instances that allows network
/// traffic between them, within an IPv4 subnetwork or optionally an IPv6
/// subnetwork.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcSubnet {
    /// ID for an attached custom router.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_router_id: ::std::option::Option<::uuid::Uuid>,
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// The IPv4 subnet CIDR block.
    pub ipv4_block: Ipv4Net,
    /// The IPv6 subnet CIDR block.
    pub ipv6_block: Ipv6Net,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The VPC to which the subnet belongs.
    pub vpc_id: ::uuid::Uuid,
}

impl VpcSubnet {
    pub fn builder() -> builder::VpcSubnet {
        Default::default()
    }
}

/// Create-time parameters for a `VpcSubnet`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcSubnetCreate {
    /// An optional router, used to direct packets sent from hosts in this
    /// subnet to any destination address.
    ///
    /// Custom routers apply in addition to the VPC-wide *system* router, and
    /// have higher priority than the system router for an otherwise
    /// equal-prefix-length match.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_router: ::std::option::Option<NameOrId>,
    pub description: ::std::string::String,
    /// The IPv4 address range for this subnet.
    ///
    /// It must be allocated from an RFC 1918 private address range, and must
    /// not overlap with any other existing subnet in the VPC.
    pub ipv4_block: Ipv4Net,
    /// The IPv6 address range for this subnet.
    ///
    /// It must be allocated from the RFC 4193 Unique Local Address range, with
    /// the prefix equal to the parent VPC's prefix. A random `/64` block will
    /// be assigned if one is not provided. It must not overlap with any
    /// existing subnet in the VPC.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ipv6_block: ::std::option::Option<Ipv6Net>,
    pub name: Name,
}

impl VpcSubnetCreate {
    pub fn builder() -> builder::VpcSubnetCreate {
        Default::default()
    }
}

/// A single page of results
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct VpcSubnetResultsPage {
    /// list of items on this page of results
    pub items: ::std::vec::Vec<VpcSubnet>,
    /// token used to fetch the next page of results (if any)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next_page: ::std::option::Option<::std::string::String>,
}

impl VpcSubnetResultsPage {
    pub fn builder() -> builder::VpcSubnetResultsPage {
        Default::default()
    }
}

/// Updateable properties of a `VpcSubnet`
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct VpcSubnetUpdate {
    /// An optional router, used to direct packets sent from hosts in this
    /// subnet to any destination address.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom_router: ::std::option::Option<NameOrId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl VpcSubnetUpdate {
    pub fn builder() -> builder::VpcSubnetUpdate {
        Default::default()
    }
}

/// Updateable properties of a `Vpc`
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct VpcUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dns_name: ::std::option::Option<Name>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl VpcUpdate {
    pub fn builder() -> builder::VpcUpdate {
        Default::default()
    }
}

/// Create-time identity-related parameters
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct WebhookCreate {
    pub description: ::std::string::String,
    /// The URL that webhook notification requests should be sent to
    pub endpoint: ::std::string::String,
    pub name: Name,
    /// A non-empty list of secret keys used to sign webhook payloads.
    pub secrets: ::std::vec::Vec<::std::string::String>,
    /// A list of webhook event class subscriptions.
    ///
    /// If this list is empty or is not included in the request body, the
    /// webhook will not be subscribed to any events.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub subscriptions: ::std::vec::Vec<AlertSubscription>,
}

impl WebhookCreate {
    pub fn builder() -> builder::WebhookCreate {
        Default::default()
    }
}

/// An individual delivery attempt for a webhook event.
///
/// This represents a single HTTP request that was sent to the receiver, and its
/// outcome.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct WebhookDeliveryAttempt {
    /// The attempt number.
    pub attempt: u32,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response: ::std::option::Option<WebhookDeliveryResponse>,
    /// The outcome of this delivery attempt: either the event was delivered
    /// successfully, or the request failed for one of several reasons.
    pub result: WebhookDeliveryAttemptResult,
    /// The time at which the webhook delivery was attempted.
    pub time_sent: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl WebhookDeliveryAttempt {
    pub fn builder() -> builder::WebhookDeliveryAttempt {
        Default::default()
    }
}

/// `WebhookDeliveryAttemptResult`
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    schemars :: JsonSchema,
)]
pub enum WebhookDeliveryAttemptResult {
    /// The webhook event has been delivered successfully.
    #[serde(rename = "succeeded")]
    Succeeded,
    /// A webhook request was sent to the endpoint, and it returned a HTTP error
    /// status code indicating an error.
    #[serde(rename = "failed_http_error")]
    FailedHttpError,
    /// The webhook request could not be sent to the receiver endpoint.
    #[serde(rename = "failed_unreachable")]
    FailedUnreachable,
    /// A connection to the receiver endpoint was successfully established, but
    /// no response was received within the delivery timeout.
    #[serde(rename = "failed_timeout")]
    FailedTimeout,
}

impl ::std::fmt::Display for WebhookDeliveryAttemptResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Succeeded => f.write_str("succeeded"),
            Self::FailedHttpError => f.write_str("failed_http_error"),
            Self::FailedUnreachable => f.write_str("failed_unreachable"),
            Self::FailedTimeout => f.write_str("failed_timeout"),
        }
    }
}

impl ::std::str::FromStr for WebhookDeliveryAttemptResult {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "succeeded" => Ok(Self::Succeeded),
            "failed_http_error" => Ok(Self::FailedHttpError),
            "failed_unreachable" => Ok(Self::FailedUnreachable),
            "failed_timeout" => Ok(Self::FailedTimeout),
            _ => Err("invalid value".into()),
        }
    }
}

impl ::std::convert::TryFrom<&str> for WebhookDeliveryAttemptResult {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for WebhookDeliveryAttemptResult {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

/// The response received from a webhook receiver endpoint.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct WebhookDeliveryResponse {
    /// The response time of the webhook endpoint, in milliseconds.
    pub duration_ms: u32,
    /// The HTTP status code returned from the webhook endpoint.
    pub status: u16,
}

impl WebhookDeliveryResponse {
    pub fn builder() -> builder::WebhookDeliveryResponse {
        Default::default()
    }
}

/// The configuration for a webhook alert receiver.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct WebhookReceiver {
    /// Human-readable free-form text about a resource
    pub description: ::std::string::String,
    /// The URL that webhook notification requests are sent to.
    pub endpoint: ::std::string::String,
    /// Unique, immutable, system-controlled identifier for each resource
    pub id: ::uuid::Uuid,
    /// Unique, mutable, user-controlled identifier for each resource
    pub name: Name,
    /// A list containing the IDs of the secret keys used to sign payloads sent
    /// to this receiver.
    pub secrets: ::std::vec::Vec<WebhookSecret>,
    /// The list of alert classes to which this receiver is subscribed.
    pub subscriptions: ::std::vec::Vec<AlertSubscription>,
    /// Timestamp when this resource was created
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Timestamp when this resource was last modified
    pub time_modified: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl WebhookReceiver {
    pub fn builder() -> builder::WebhookReceiver {
        Default::default()
    }
}

/// Parameters to update a webhook configuration.
#[derive(
    :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default, schemars :: JsonSchema,
)]
pub struct WebhookReceiverUpdate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    /// The URL that webhook notification requests should be sent to
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub endpoint: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<Name>,
}

impl WebhookReceiverUpdate {
    pub fn builder() -> builder::WebhookReceiverUpdate {
        Default::default()
    }
}

/// A view of a shared secret key assigned to a webhook receiver.
///
/// Once a secret is created, the value of the secret is not available in the
/// API, as it must remain secret. Instead, secrets are referenced by their
/// unique IDs assigned when they are created.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct WebhookSecret {
    /// The public unique ID of the secret.
    pub id: ::uuid::Uuid,
    /// The UTC timestamp at which this secret was created.
    pub time_created: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl WebhookSecret {
    pub fn builder() -> builder::WebhookSecret {
        Default::default()
    }
}

/// `WebhookSecretCreate`
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct WebhookSecretCreate {
    /// The value of the shared secret key.
    pub secret: ::std::string::String,
}

impl WebhookSecretCreate {
    pub fn builder() -> builder::WebhookSecretCreate {
        Default::default()
    }
}

/// A list of the IDs of secrets associated with a webhook receiver.
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema)]
pub struct WebhookSecrets {
    pub secrets: ::std::vec::Vec<WebhookSecret>,
}

impl WebhookSecrets {
    pub fn builder() -> builder::WebhookSecrets {
        Default::default()
    }
}
