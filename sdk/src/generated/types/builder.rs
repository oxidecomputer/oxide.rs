// The contents of this file are generated; do not modify them.

#[derive(Clone, Debug)]
pub struct Address {
    address: ::std::result::Result<super::IpNet, ::std::string::String>,
    address_lot: ::std::result::Result<super::NameOrId, ::std::string::String>,
    vlan_id: ::std::result::Result<::std::option::Option<u16>, ::std::string::String>,
}

impl ::std::default::Default for Address {
    fn default() -> Self {
        Self {
            address: Err("no value supplied for address".to_string()),
            address_lot: Err("no value supplied for address_lot".to_string()),
            vlan_id: Ok(Default::default()),
        }
    }
}

impl Address {
    pub fn address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address: {e}"));
        self
    }

    pub fn address_lot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.address_lot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_lot: {e}"));
        self
    }

    pub fn vlan_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u16>>,
        T::Error: ::std::fmt::Display,
    {
        self.vlan_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vlan_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Address> for super::Address {
    type Error = super::error::ConversionError;
    fn try_from(value: Address) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            address: value.address?,
            address_lot: value.address_lot?,
            vlan_id: value.vlan_id?,
        })
    }
}

impl ::std::convert::From<super::Address> for Address {
    fn from(value: super::Address) -> Self {
        Self {
            address: Ok(value.address),
            address_lot: Ok(value.address_lot),
            vlan_id: Ok(value.vlan_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressConfig {
    addresses: ::std::result::Result<::std::vec::Vec<super::Address>, ::std::string::String>,
    link_name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for AddressConfig {
    fn default() -> Self {
        Self {
            addresses: Err("no value supplied for addresses".to_string()),
            link_name: Err("no value supplied for link_name".to_string()),
        }
    }
}

impl AddressConfig {
    pub fn addresses<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Address>>,
        T::Error: ::std::fmt::Display,
    {
        self.addresses = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for addresses: {e}"));
        self
    }

    pub fn link_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.link_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AddressConfig> for super::AddressConfig {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AddressConfig,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            addresses: value.addresses?,
            link_name: value.link_name?,
        })
    }
}

impl ::std::convert::From<super::AddressConfig> for AddressConfig {
    fn from(value: super::AddressConfig) -> Self {
        Self {
            addresses: Ok(value.addresses),
            link_name: Ok(value.link_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressLot {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    kind: ::std::result::Result<super::AddressLotKind, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for AddressLot {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            kind: Err("no value supplied for kind".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl AddressLot {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AddressLotKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AddressLot> for super::AddressLot {
    type Error = super::error::ConversionError;
    fn try_from(value: AddressLot) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            kind: value.kind?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::AddressLot> for AddressLot {
    fn from(value: super::AddressLot) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            kind: Ok(value.kind),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressLotBlock {
    first_address: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    last_address: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
}

impl ::std::default::Default for AddressLotBlock {
    fn default() -> Self {
        Self {
            first_address: Err("no value supplied for first_address".to_string()),
            id: Err("no value supplied for id".to_string()),
            last_address: Err("no value supplied for last_address".to_string()),
        }
    }
}

impl AddressLotBlock {
    pub fn first_address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.first_address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for first_address: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn last_address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.last_address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for last_address: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AddressLotBlock> for super::AddressLotBlock {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AddressLotBlock,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            first_address: value.first_address?,
            id: value.id?,
            last_address: value.last_address?,
        })
    }
}

impl ::std::convert::From<super::AddressLotBlock> for AddressLotBlock {
    fn from(value: super::AddressLotBlock) -> Self {
        Self {
            first_address: Ok(value.first_address),
            id: Ok(value.id),
            last_address: Ok(value.last_address),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressLotBlockCreate {
    first_address: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    last_address: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
}

impl ::std::default::Default for AddressLotBlockCreate {
    fn default() -> Self {
        Self {
            first_address: Err("no value supplied for first_address".to_string()),
            last_address: Err("no value supplied for last_address".to_string()),
        }
    }
}

impl AddressLotBlockCreate {
    pub fn first_address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.first_address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for first_address: {e}"));
        self
    }

    pub fn last_address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.last_address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for last_address: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AddressLotBlockCreate> for super::AddressLotBlockCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AddressLotBlockCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            first_address: value.first_address?,
            last_address: value.last_address?,
        })
    }
}

impl ::std::convert::From<super::AddressLotBlockCreate> for AddressLotBlockCreate {
    fn from(value: super::AddressLotBlockCreate) -> Self {
        Self {
            first_address: Ok(value.first_address),
            last_address: Ok(value.last_address),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressLotBlockResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::AddressLotBlock>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AddressLotBlockResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AddressLotBlockResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AddressLotBlock>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AddressLotBlockResultsPage> for super::AddressLotBlockResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AddressLotBlockResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AddressLotBlockResultsPage> for AddressLotBlockResultsPage {
    fn from(value: super::AddressLotBlockResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressLotCreate {
    blocks:
        ::std::result::Result<::std::vec::Vec<super::AddressLotBlockCreate>, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    kind: ::std::result::Result<super::AddressLotKind, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for AddressLotCreate {
    fn default() -> Self {
        Self {
            blocks: Err("no value supplied for blocks".to_string()),
            description: Err("no value supplied for description".to_string()),
            kind: Err("no value supplied for kind".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl AddressLotCreate {
    pub fn blocks<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AddressLotBlockCreate>>,
        T::Error: ::std::fmt::Display,
    {
        self.blocks = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for blocks: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AddressLotKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AddressLotCreate> for super::AddressLotCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AddressLotCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            blocks: value.blocks?,
            description: value.description?,
            kind: value.kind?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::AddressLotCreate> for AddressLotCreate {
    fn from(value: super::AddressLotCreate) -> Self {
        Self {
            blocks: Ok(value.blocks),
            description: Ok(value.description),
            kind: Ok(value.kind),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressLotCreateResponse {
    blocks: ::std::result::Result<::std::vec::Vec<super::AddressLotBlock>, ::std::string::String>,
    lot: ::std::result::Result<super::AddressLot, ::std::string::String>,
}

impl ::std::default::Default for AddressLotCreateResponse {
    fn default() -> Self {
        Self {
            blocks: Err("no value supplied for blocks".to_string()),
            lot: Err("no value supplied for lot".to_string()),
        }
    }
}

impl AddressLotCreateResponse {
    pub fn blocks<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AddressLotBlock>>,
        T::Error: ::std::fmt::Display,
    {
        self.blocks = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for blocks: {e}"));
        self
    }

    pub fn lot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AddressLot>,
        T::Error: ::std::fmt::Display,
    {
        self.lot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for lot: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AddressLotCreateResponse> for super::AddressLotCreateResponse {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AddressLotCreateResponse,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            blocks: value.blocks?,
            lot: value.lot?,
        })
    }
}

impl ::std::convert::From<super::AddressLotCreateResponse> for AddressLotCreateResponse {
    fn from(value: super::AddressLotCreateResponse) -> Self {
        Self {
            blocks: Ok(value.blocks),
            lot: Ok(value.lot),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressLotResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::AddressLot>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AddressLotResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AddressLotResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AddressLot>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AddressLotResultsPage> for super::AddressLotResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AddressLotResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AddressLotResultsPage> for AddressLotResultsPage {
    fn from(value: super::AddressLotResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddressLotViewResponse {
    blocks: ::std::result::Result<::std::vec::Vec<super::AddressLotBlock>, ::std::string::String>,
    lot: ::std::result::Result<super::AddressLot, ::std::string::String>,
}

impl ::std::default::Default for AddressLotViewResponse {
    fn default() -> Self {
        Self {
            blocks: Err("no value supplied for blocks".to_string()),
            lot: Err("no value supplied for lot".to_string()),
        }
    }
}

impl AddressLotViewResponse {
    pub fn blocks<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AddressLotBlock>>,
        T::Error: ::std::fmt::Display,
    {
        self.blocks = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for blocks: {e}"));
        self
    }

    pub fn lot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AddressLot>,
        T::Error: ::std::fmt::Display,
    {
        self.lot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for lot: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AddressLotViewResponse> for super::AddressLotViewResponse {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AddressLotViewResponse,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            blocks: value.blocks?,
            lot: value.lot?,
        })
    }
}

impl ::std::convert::From<super::AddressLotViewResponse> for AddressLotViewResponse {
    fn from(value: super::AddressLotViewResponse) -> Self {
        Self {
            blocks: Ok(value.blocks),
            lot: Ok(value.lot),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AffinityGroup {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    failure_domain: ::std::result::Result<super::FailureDomain, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    policy: ::std::result::Result<super::AffinityPolicy, ::std::string::String>,
    project_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for AffinityGroup {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            failure_domain: Err("no value supplied for failure_domain".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            policy: Err("no value supplied for policy".to_string()),
            project_id: Err("no value supplied for project_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl AffinityGroup {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn failure_domain<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::FailureDomain>,
        T::Error: ::std::fmt::Display,
    {
        self.failure_domain = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for failure_domain: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn policy<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AffinityPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.policy = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for policy: {e}"));
        self
    }

    pub fn project_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.project_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AffinityGroup> for super::AffinityGroup {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AffinityGroup,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            failure_domain: value.failure_domain?,
            id: value.id?,
            name: value.name?,
            policy: value.policy?,
            project_id: value.project_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::AffinityGroup> for AffinityGroup {
    fn from(value: super::AffinityGroup) -> Self {
        Self {
            description: Ok(value.description),
            failure_domain: Ok(value.failure_domain),
            id: Ok(value.id),
            name: Ok(value.name),
            policy: Ok(value.policy),
            project_id: Ok(value.project_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AffinityGroupCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    failure_domain: ::std::result::Result<super::FailureDomain, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    policy: ::std::result::Result<super::AffinityPolicy, ::std::string::String>,
}

impl ::std::default::Default for AffinityGroupCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            failure_domain: Err("no value supplied for failure_domain".to_string()),
            name: Err("no value supplied for name".to_string()),
            policy: Err("no value supplied for policy".to_string()),
        }
    }
}

impl AffinityGroupCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn failure_domain<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::FailureDomain>,
        T::Error: ::std::fmt::Display,
    {
        self.failure_domain = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for failure_domain: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn policy<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AffinityPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.policy = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for policy: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AffinityGroupCreate> for super::AffinityGroupCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AffinityGroupCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            failure_domain: value.failure_domain?,
            name: value.name?,
            policy: value.policy?,
        })
    }
}

impl ::std::convert::From<super::AffinityGroupCreate> for AffinityGroupCreate {
    fn from(value: super::AffinityGroupCreate) -> Self {
        Self {
            description: Ok(value.description),
            failure_domain: Ok(value.failure_domain),
            name: Ok(value.name),
            policy: Ok(value.policy),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AffinityGroupMemberResultsPage {
    items:
        ::std::result::Result<::std::vec::Vec<super::AffinityGroupMember>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AffinityGroupMemberResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AffinityGroupMemberResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AffinityGroupMember>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AffinityGroupMemberResultsPage>
    for super::AffinityGroupMemberResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: AffinityGroupMemberResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AffinityGroupMemberResultsPage>
    for AffinityGroupMemberResultsPage
{
    fn from(value: super::AffinityGroupMemberResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AffinityGroupResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::AffinityGroup>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AffinityGroupResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AffinityGroupResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AffinityGroup>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AffinityGroupResultsPage> for super::AffinityGroupResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AffinityGroupResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AffinityGroupResultsPage> for AffinityGroupResultsPage {
    fn from(value: super::AffinityGroupResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AffinityGroupUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for AffinityGroupUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl AffinityGroupUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AffinityGroupUpdate> for super::AffinityGroupUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AffinityGroupUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::AffinityGroupUpdate> for AffinityGroupUpdate {
    fn from(value: super::AffinityGroupUpdate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AggregateBgpMessageHistory {
    switch_histories:
        ::std::result::Result<::std::vec::Vec<super::SwitchBgpHistory>, ::std::string::String>,
}

impl ::std::default::Default for AggregateBgpMessageHistory {
    fn default() -> Self {
        Self {
            switch_histories: Err("no value supplied for switch_histories".to_string()),
        }
    }
}

impl AggregateBgpMessageHistory {
    pub fn switch_histories<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SwitchBgpHistory>>,
        T::Error: ::std::fmt::Display,
    {
        self.switch_histories = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch_histories: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AggregateBgpMessageHistory> for super::AggregateBgpMessageHistory {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AggregateBgpMessageHistory,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            switch_histories: value.switch_histories?,
        })
    }
}

impl ::std::convert::From<super::AggregateBgpMessageHistory> for AggregateBgpMessageHistory {
    fn from(value: super::AggregateBgpMessageHistory) -> Self {
        Self {
            switch_histories: Ok(value.switch_histories),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Alert {
    class: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    payload: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    version: ::std::result::Result<u32, ::std::string::String>,
}

impl ::std::default::Default for Alert {
    fn default() -> Self {
        Self {
            class: Err("no value supplied for class".to_string()),
            id: Err("no value supplied for id".to_string()),
            payload: Err("no value supplied for payload".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            version: Err("no value supplied for version".to_string()),
        }
    }
}

impl Alert {
    pub fn class<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.class = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for class: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn payload<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::serde_json::Value>,
        T::Error: ::std::fmt::Display,
    {
        self.payload = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for payload: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for version: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Alert> for super::Alert {
    type Error = super::error::ConversionError;
    fn try_from(value: Alert) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            class: value.class?,
            id: value.id?,
            payload: value.payload?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            version: value.version?,
        })
    }
}

impl ::std::convert::From<super::Alert> for Alert {
    fn from(value: super::Alert) -> Self {
        Self {
            class: Ok(value.class),
            id: Ok(value.id),
            payload: Ok(value.payload),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            version: Ok(value.version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertClass {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for AlertClass {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl AlertClass {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertClass> for super::AlertClass {
    type Error = super::error::ConversionError;
    fn try_from(value: AlertClass) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::AlertClass> for AlertClass {
    fn from(value: super::AlertClass) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertClassResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::AlertClass>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AlertClassResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AlertClassResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AlertClass>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertClassResultsPage> for super::AlertClassResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertClassResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AlertClassResultsPage> for AlertClassResultsPage {
    fn from(value: super::AlertClassResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertDelivery {
    alert_class: ::std::result::Result<::std::string::String, ::std::string::String>,
    alert_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    attempts: ::std::result::Result<super::AlertDeliveryAttempts, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    receiver_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    state: ::std::result::Result<super::AlertDeliveryState, ::std::string::String>,
    time_started:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    trigger: ::std::result::Result<super::AlertDeliveryTrigger, ::std::string::String>,
}

impl ::std::default::Default for AlertDelivery {
    fn default() -> Self {
        Self {
            alert_class: Err("no value supplied for alert_class".to_string()),
            alert_id: Err("no value supplied for alert_id".to_string()),
            attempts: Err("no value supplied for attempts".to_string()),
            id: Err("no value supplied for id".to_string()),
            receiver_id: Err("no value supplied for receiver_id".to_string()),
            state: Err("no value supplied for state".to_string()),
            time_started: Err("no value supplied for time_started".to_string()),
            trigger: Err("no value supplied for trigger".to_string()),
        }
    }
}

impl AlertDelivery {
    pub fn alert_class<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.alert_class = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for alert_class: {e}"));
        self
    }

    pub fn alert_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.alert_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for alert_id: {e}"));
        self
    }

    pub fn attempts<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AlertDeliveryAttempts>,
        T::Error: ::std::fmt::Display,
    {
        self.attempts = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for attempts: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn receiver_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.receiver_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for receiver_id: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AlertDeliveryState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_started<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_started = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_started: {e}"));
        self
    }

    pub fn trigger<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AlertDeliveryTrigger>,
        T::Error: ::std::fmt::Display,
    {
        self.trigger = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for trigger: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertDelivery> for super::AlertDelivery {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertDelivery,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            alert_class: value.alert_class?,
            alert_id: value.alert_id?,
            attempts: value.attempts?,
            id: value.id?,
            receiver_id: value.receiver_id?,
            state: value.state?,
            time_started: value.time_started?,
            trigger: value.trigger?,
        })
    }
}

impl ::std::convert::From<super::AlertDelivery> for AlertDelivery {
    fn from(value: super::AlertDelivery) -> Self {
        Self {
            alert_class: Ok(value.alert_class),
            alert_id: Ok(value.alert_id),
            attempts: Ok(value.attempts),
            id: Ok(value.id),
            receiver_id: Ok(value.receiver_id),
            state: Ok(value.state),
            time_started: Ok(value.time_started),
            trigger: Ok(value.trigger),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertDeliveryId {
    delivery_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for AlertDeliveryId {
    fn default() -> Self {
        Self {
            delivery_id: Err("no value supplied for delivery_id".to_string()),
        }
    }
}

impl AlertDeliveryId {
    pub fn delivery_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.delivery_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for delivery_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertDeliveryId> for super::AlertDeliveryId {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertDeliveryId,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            delivery_id: value.delivery_id?,
        })
    }
}

impl ::std::convert::From<super::AlertDeliveryId> for AlertDeliveryId {
    fn from(value: super::AlertDeliveryId) -> Self {
        Self {
            delivery_id: Ok(value.delivery_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertDeliveryResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::AlertDelivery>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AlertDeliveryResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AlertDeliveryResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AlertDelivery>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertDeliveryResultsPage> for super::AlertDeliveryResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertDeliveryResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AlertDeliveryResultsPage> for AlertDeliveryResultsPage {
    fn from(value: super::AlertDeliveryResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertProbeResult {
    probe: ::std::result::Result<super::AlertDelivery, ::std::string::String>,
    resends_started: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
}

impl ::std::default::Default for AlertProbeResult {
    fn default() -> Self {
        Self {
            probe: Err("no value supplied for probe".to_string()),
            resends_started: Ok(Default::default()),
        }
    }
}

impl AlertProbeResult {
    pub fn probe<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AlertDelivery>,
        T::Error: ::std::fmt::Display,
    {
        self.probe = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for probe: {e}"));
        self
    }

    pub fn resends_started<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u32>>,
        T::Error: ::std::fmt::Display,
    {
        self.resends_started = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for resends_started: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertProbeResult> for super::AlertProbeResult {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertProbeResult,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            probe: value.probe?,
            resends_started: value.resends_started?,
        })
    }
}

impl ::std::convert::From<super::AlertProbeResult> for AlertProbeResult {
    fn from(value: super::AlertProbeResult) -> Self {
        Self {
            probe: Ok(value.probe),
            resends_started: Ok(value.resends_started),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertReceiver {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    kind: ::std::result::Result<super::AlertReceiverKind, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    subscriptions:
        ::std::result::Result<::std::vec::Vec<super::AlertSubscription>, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for AlertReceiver {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            kind: Err("no value supplied for kind".to_string()),
            name: Err("no value supplied for name".to_string()),
            subscriptions: Err("no value supplied for subscriptions".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl AlertReceiver {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AlertReceiverKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn subscriptions<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AlertSubscription>>,
        T::Error: ::std::fmt::Display,
    {
        self.subscriptions = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subscriptions: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertReceiver> for super::AlertReceiver {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertReceiver,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            kind: value.kind?,
            name: value.name?,
            subscriptions: value.subscriptions?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::AlertReceiver> for AlertReceiver {
    fn from(value: super::AlertReceiver) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            kind: Ok(value.kind),
            name: Ok(value.name),
            subscriptions: Ok(value.subscriptions),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertReceiverResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::AlertReceiver>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AlertReceiverResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AlertReceiverResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AlertReceiver>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertReceiverResultsPage> for super::AlertReceiverResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertReceiverResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AlertReceiverResultsPage> for AlertReceiverResultsPage {
    fn from(value: super::AlertReceiverResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Alert>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AlertResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AlertResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Alert>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertResultsPage> for super::AlertResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AlertResultsPage> for AlertResultsPage {
    fn from(value: super::AlertResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertSubscriptionCreate {
    subscription: ::std::result::Result<super::AlertSubscription, ::std::string::String>,
}

impl ::std::default::Default for AlertSubscriptionCreate {
    fn default() -> Self {
        Self {
            subscription: Err("no value supplied for subscription".to_string()),
        }
    }
}

impl AlertSubscriptionCreate {
    pub fn subscription<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AlertSubscription>,
        T::Error: ::std::fmt::Display,
    {
        self.subscription = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subscription: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertSubscriptionCreate> for super::AlertSubscriptionCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertSubscriptionCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            subscription: value.subscription?,
        })
    }
}

impl ::std::convert::From<super::AlertSubscriptionCreate> for AlertSubscriptionCreate {
    fn from(value: super::AlertSubscriptionCreate) -> Self {
        Self {
            subscription: Ok(value.subscription),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertSubscriptionCreated {
    subscription: ::std::result::Result<super::AlertSubscription, ::std::string::String>,
}

impl ::std::default::Default for AlertSubscriptionCreated {
    fn default() -> Self {
        Self {
            subscription: Err("no value supplied for subscription".to_string()),
        }
    }
}

impl AlertSubscriptionCreated {
    pub fn subscription<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AlertSubscription>,
        T::Error: ::std::fmt::Display,
    {
        self.subscription = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subscription: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AlertSubscriptionCreated> for super::AlertSubscriptionCreated {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AlertSubscriptionCreated,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            subscription: value.subscription?,
        })
    }
}

impl ::std::convert::From<super::AlertSubscriptionCreated> for AlertSubscriptionCreated {
    fn from(value: super::AlertSubscriptionCreated) -> Self {
        Self {
            subscription: Ok(value.subscription),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AllowList {
    allowed_ips: ::std::result::Result<super::AllowedSourceIps, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for AllowList {
    fn default() -> Self {
        Self {
            allowed_ips: Err("no value supplied for allowed_ips".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl AllowList {
    pub fn allowed_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AllowedSourceIps>,
        T::Error: ::std::fmt::Display,
    {
        self.allowed_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for allowed_ips: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AllowList> for super::AllowList {
    type Error = super::error::ConversionError;
    fn try_from(value: AllowList) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            allowed_ips: value.allowed_ips?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::AllowList> for AllowList {
    fn from(value: super::AllowList) -> Self {
        Self {
            allowed_ips: Ok(value.allowed_ips),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AllowListUpdate {
    allowed_ips: ::std::result::Result<super::AllowedSourceIps, ::std::string::String>,
}

impl ::std::default::Default for AllowListUpdate {
    fn default() -> Self {
        Self {
            allowed_ips: Err("no value supplied for allowed_ips".to_string()),
        }
    }
}

impl AllowListUpdate {
    pub fn allowed_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AllowedSourceIps>,
        T::Error: ::std::fmt::Display,
    {
        self.allowed_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for allowed_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AllowListUpdate> for super::AllowListUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AllowListUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            allowed_ips: value.allowed_ips?,
        })
    }
}

impl ::std::convert::From<super::AllowListUpdate> for AllowListUpdate {
    fn from(value: super::AllowListUpdate) -> Self {
        Self {
            allowed_ips: Ok(value.allowed_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AntiAffinityGroup {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    failure_domain: ::std::result::Result<super::FailureDomain, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    policy: ::std::result::Result<super::AffinityPolicy, ::std::string::String>,
    project_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for AntiAffinityGroup {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            failure_domain: Err("no value supplied for failure_domain".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            policy: Err("no value supplied for policy".to_string()),
            project_id: Err("no value supplied for project_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl AntiAffinityGroup {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn failure_domain<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::FailureDomain>,
        T::Error: ::std::fmt::Display,
    {
        self.failure_domain = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for failure_domain: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn policy<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AffinityPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.policy = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for policy: {e}"));
        self
    }

    pub fn project_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.project_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AntiAffinityGroup> for super::AntiAffinityGroup {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AntiAffinityGroup,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            failure_domain: value.failure_domain?,
            id: value.id?,
            name: value.name?,
            policy: value.policy?,
            project_id: value.project_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::AntiAffinityGroup> for AntiAffinityGroup {
    fn from(value: super::AntiAffinityGroup) -> Self {
        Self {
            description: Ok(value.description),
            failure_domain: Ok(value.failure_domain),
            id: Ok(value.id),
            name: Ok(value.name),
            policy: Ok(value.policy),
            project_id: Ok(value.project_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AntiAffinityGroupCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    failure_domain: ::std::result::Result<super::FailureDomain, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    policy: ::std::result::Result<super::AffinityPolicy, ::std::string::String>,
}

impl ::std::default::Default for AntiAffinityGroupCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            failure_domain: Err("no value supplied for failure_domain".to_string()),
            name: Err("no value supplied for name".to_string()),
            policy: Err("no value supplied for policy".to_string()),
        }
    }
}

impl AntiAffinityGroupCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn failure_domain<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::FailureDomain>,
        T::Error: ::std::fmt::Display,
    {
        self.failure_domain = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for failure_domain: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn policy<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AffinityPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.policy = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for policy: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AntiAffinityGroupCreate> for super::AntiAffinityGroupCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AntiAffinityGroupCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            failure_domain: value.failure_domain?,
            name: value.name?,
            policy: value.policy?,
        })
    }
}

impl ::std::convert::From<super::AntiAffinityGroupCreate> for AntiAffinityGroupCreate {
    fn from(value: super::AntiAffinityGroupCreate) -> Self {
        Self {
            description: Ok(value.description),
            failure_domain: Ok(value.failure_domain),
            name: Ok(value.name),
            policy: Ok(value.policy),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AntiAffinityGroupMemberResultsPage {
    items: ::std::result::Result<
        ::std::vec::Vec<super::AntiAffinityGroupMember>,
        ::std::string::String,
    >,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AntiAffinityGroupMemberResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AntiAffinityGroupMemberResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AntiAffinityGroupMember>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AntiAffinityGroupMemberResultsPage>
    for super::AntiAffinityGroupMemberResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: AntiAffinityGroupMemberResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AntiAffinityGroupMemberResultsPage>
    for AntiAffinityGroupMemberResultsPage
{
    fn from(value: super::AntiAffinityGroupMemberResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AntiAffinityGroupResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::AntiAffinityGroup>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AntiAffinityGroupResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AntiAffinityGroupResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AntiAffinityGroup>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AntiAffinityGroupResultsPage> for super::AntiAffinityGroupResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AntiAffinityGroupResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AntiAffinityGroupResultsPage> for AntiAffinityGroupResultsPage {
    fn from(value: super::AntiAffinityGroupResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AntiAffinityGroupUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for AntiAffinityGroupUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl AntiAffinityGroupUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AntiAffinityGroupUpdate> for super::AntiAffinityGroupUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AntiAffinityGroupUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::AntiAffinityGroupUpdate> for AntiAffinityGroupUpdate {
    fn from(value: super::AntiAffinityGroupUpdate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AuditLogEntry {
    actor: ::std::result::Result<super::AuditLogEntryActor, ::std::string::String>,
    auth_method:
        ::std::result::Result<::std::option::Option<super::AuthMethod>, ::std::string::String>,
    credential_id:
        ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    operation_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    request_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    result: ::std::result::Result<super::AuditLogEntryResult, ::std::string::String>,
    source_ip: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    time_completed:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_started:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    user_agent:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AuditLogEntry {
    fn default() -> Self {
        Self {
            actor: Err("no value supplied for actor".to_string()),
            auth_method: Ok(Default::default()),
            credential_id: Ok(Default::default()),
            id: Err("no value supplied for id".to_string()),
            operation_id: Err("no value supplied for operation_id".to_string()),
            request_id: Err("no value supplied for request_id".to_string()),
            request_uri: Err("no value supplied for request_uri".to_string()),
            result: Err("no value supplied for result".to_string()),
            source_ip: Err("no value supplied for source_ip".to_string()),
            time_completed: Err("no value supplied for time_completed".to_string()),
            time_started: Err("no value supplied for time_started".to_string()),
            user_agent: Ok(Default::default()),
        }
    }
}

impl AuditLogEntry {
    pub fn actor<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AuditLogEntryActor>,
        T::Error: ::std::fmt::Display,
    {
        self.actor = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for actor: {e}"));
        self
    }

    pub fn auth_method<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::AuthMethod>>,
        T::Error: ::std::fmt::Display,
    {
        self.auth_method = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for auth_method: {e}"));
        self
    }

    pub fn credential_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.credential_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for credential_id: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn operation_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.operation_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for operation_id: {e}"));
        self
    }

    pub fn request_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.request_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for request_id: {e}"));
        self
    }

    pub fn request_uri<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.request_uri = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for request_uri: {e}"));
        self
    }

    pub fn result<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AuditLogEntryResult>,
        T::Error: ::std::fmt::Display,
    {
        self.result = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for result: {e}"));
        self
    }

    pub fn source_ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.source_ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for source_ip: {e}"));
        self
    }

    pub fn time_completed<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_completed = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_completed: {e}"));
        self
    }

    pub fn time_started<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_started = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_started: {e}"));
        self
    }

    pub fn user_agent<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.user_agent = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for user_agent: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AuditLogEntry> for super::AuditLogEntry {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AuditLogEntry,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            actor: value.actor?,
            auth_method: value.auth_method?,
            credential_id: value.credential_id?,
            id: value.id?,
            operation_id: value.operation_id?,
            request_id: value.request_id?,
            request_uri: value.request_uri?,
            result: value.result?,
            source_ip: value.source_ip?,
            time_completed: value.time_completed?,
            time_started: value.time_started?,
            user_agent: value.user_agent?,
        })
    }
}

impl ::std::convert::From<super::AuditLogEntry> for AuditLogEntry {
    fn from(value: super::AuditLogEntry) -> Self {
        Self {
            actor: Ok(value.actor),
            auth_method: Ok(value.auth_method),
            credential_id: Ok(value.credential_id),
            id: Ok(value.id),
            operation_id: Ok(value.operation_id),
            request_id: Ok(value.request_id),
            request_uri: Ok(value.request_uri),
            result: Ok(value.result),
            source_ip: Ok(value.source_ip),
            time_completed: Ok(value.time_completed),
            time_started: Ok(value.time_started),
            user_agent: Ok(value.user_agent),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AuditLogEntryResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::AuditLogEntry>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for AuditLogEntryResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl AuditLogEntryResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AuditLogEntry>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<AuditLogEntryResultsPage> for super::AuditLogEntryResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: AuditLogEntryResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::AuditLogEntryResultsPage> for AuditLogEntryResultsPage {
    fn from(value: super::AuditLogEntryResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Baseboard {
    part: ::std::result::Result<::std::string::String, ::std::string::String>,
    revision: ::std::result::Result<u32, ::std::string::String>,
    serial: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for Baseboard {
    fn default() -> Self {
        Self {
            part: Err("no value supplied for part".to_string()),
            revision: Err("no value supplied for revision".to_string()),
            serial: Err("no value supplied for serial".to_string()),
        }
    }
}

impl Baseboard {
    pub fn part<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.part = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for part: {e}"));
        self
    }

    pub fn revision<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.revision = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for revision: {e}"));
        self
    }

    pub fn serial<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.serial = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for serial: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Baseboard> for super::Baseboard {
    type Error = super::error::ConversionError;
    fn try_from(value: Baseboard) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            part: value.part?,
            revision: value.revision?,
            serial: value.serial?,
        })
    }
}

impl ::std::convert::From<super::Baseboard> for Baseboard {
    fn from(value: super::Baseboard) -> Self {
        Self {
            part: Ok(value.part),
            revision: Ok(value.revision),
            serial: Ok(value.serial),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BaseboardId {
    part_number: ::std::result::Result<::std::string::String, ::std::string::String>,
    serial_number: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for BaseboardId {
    fn default() -> Self {
        Self {
            part_number: Err("no value supplied for part_number".to_string()),
            serial_number: Err("no value supplied for serial_number".to_string()),
        }
    }
}

impl BaseboardId {
    pub fn part_number<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.part_number = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for part_number: {e}"));
        self
    }

    pub fn serial_number<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.serial_number = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for serial_number: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BaseboardId> for super::BaseboardId {
    type Error = super::error::ConversionError;
    fn try_from(value: BaseboardId) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            part_number: value.part_number?,
            serial_number: value.serial_number?,
        })
    }
}

impl ::std::convert::From<super::BaseboardId> for BaseboardId {
    fn from(value: super::BaseboardId) -> Self {
        Self {
            part_number: Ok(value.part_number),
            serial_number: Ok(value.serial_number),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BfdSessionDisable {
    remote: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    switch_slot: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for BfdSessionDisable {
    fn default() -> Self {
        Self {
            remote: Err("no value supplied for remote".to_string()),
            switch_slot: Err("no value supplied for switch_slot".to_string()),
        }
    }
}

impl BfdSessionDisable {
    pub fn remote<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.remote = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for remote: {e}"));
        self
    }

    pub fn switch_slot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch_slot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch_slot: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BfdSessionDisable> for super::BfdSessionDisable {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BfdSessionDisable,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            remote: value.remote?,
            switch_slot: value.switch_slot?,
        })
    }
}

impl ::std::convert::From<super::BfdSessionDisable> for BfdSessionDisable {
    fn from(value: super::BfdSessionDisable) -> Self {
        Self {
            remote: Ok(value.remote),
            switch_slot: Ok(value.switch_slot),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BfdSessionEnable {
    detection_threshold: ::std::result::Result<u8, ::std::string::String>,
    local: ::std::result::Result<::std::option::Option<::std::net::IpAddr>, ::std::string::String>,
    mode: ::std::result::Result<super::BfdMode, ::std::string::String>,
    remote: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    required_rx: ::std::result::Result<u64, ::std::string::String>,
    switch_slot: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for BfdSessionEnable {
    fn default() -> Self {
        Self {
            detection_threshold: Err("no value supplied for detection_threshold".to_string()),
            local: Ok(Default::default()),
            mode: Err("no value supplied for mode".to_string()),
            remote: Err("no value supplied for remote".to_string()),
            required_rx: Err("no value supplied for required_rx".to_string()),
            switch_slot: Err("no value supplied for switch_slot".to_string()),
        }
    }
}

impl BfdSessionEnable {
    pub fn detection_threshold<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u8>,
        T::Error: ::std::fmt::Display,
    {
        self.detection_threshold = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for detection_threshold: {e}"));
        self
    }

    pub fn local<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::net::IpAddr>>,
        T::Error: ::std::fmt::Display,
    {
        self.local = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for local: {e}"));
        self
    }

    pub fn mode<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BfdMode>,
        T::Error: ::std::fmt::Display,
    {
        self.mode = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for mode: {e}"));
        self
    }

    pub fn remote<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.remote = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for remote: {e}"));
        self
    }

    pub fn required_rx<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.required_rx = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for required_rx: {e}"));
        self
    }

    pub fn switch_slot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch_slot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch_slot: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BfdSessionEnable> for super::BfdSessionEnable {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BfdSessionEnable,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            detection_threshold: value.detection_threshold?,
            local: value.local?,
            mode: value.mode?,
            remote: value.remote?,
            required_rx: value.required_rx?,
            switch_slot: value.switch_slot?,
        })
    }
}

impl ::std::convert::From<super::BfdSessionEnable> for BfdSessionEnable {
    fn from(value: super::BfdSessionEnable) -> Self {
        Self {
            detection_threshold: Ok(value.detection_threshold),
            local: Ok(value.local),
            mode: Ok(value.mode),
            remote: Ok(value.remote),
            required_rx: Ok(value.required_rx),
            switch_slot: Ok(value.switch_slot),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BfdStatus {
    detection_threshold: ::std::result::Result<u8, ::std::string::String>,
    local: ::std::result::Result<::std::option::Option<::std::net::IpAddr>, ::std::string::String>,
    mode: ::std::result::Result<super::BfdMode, ::std::string::String>,
    peer: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    required_rx: ::std::result::Result<u64, ::std::string::String>,
    state: ::std::result::Result<super::BfdState, ::std::string::String>,
    switch_slot: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for BfdStatus {
    fn default() -> Self {
        Self {
            detection_threshold: Err("no value supplied for detection_threshold".to_string()),
            local: Ok(Default::default()),
            mode: Err("no value supplied for mode".to_string()),
            peer: Err("no value supplied for peer".to_string()),
            required_rx: Err("no value supplied for required_rx".to_string()),
            state: Err("no value supplied for state".to_string()),
            switch_slot: Err("no value supplied for switch_slot".to_string()),
        }
    }
}

impl BfdStatus {
    pub fn detection_threshold<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u8>,
        T::Error: ::std::fmt::Display,
    {
        self.detection_threshold = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for detection_threshold: {e}"));
        self
    }

    pub fn local<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::net::IpAddr>>,
        T::Error: ::std::fmt::Display,
    {
        self.local = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for local: {e}"));
        self
    }

    pub fn mode<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BfdMode>,
        T::Error: ::std::fmt::Display,
    {
        self.mode = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for mode: {e}"));
        self
    }

    pub fn peer<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.peer = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for peer: {e}"));
        self
    }

    pub fn required_rx<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.required_rx = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for required_rx: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BfdState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn switch_slot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch_slot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch_slot: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BfdStatus> for super::BfdStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: BfdStatus) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            detection_threshold: value.detection_threshold?,
            local: value.local?,
            mode: value.mode?,
            peer: value.peer?,
            required_rx: value.required_rx?,
            state: value.state?,
            switch_slot: value.switch_slot?,
        })
    }
}

impl ::std::convert::From<super::BfdStatus> for BfdStatus {
    fn from(value: super::BfdStatus) -> Self {
        Self {
            detection_threshold: Ok(value.detection_threshold),
            local: Ok(value.local),
            mode: Ok(value.mode),
            peer: Ok(value.peer),
            required_rx: Ok(value.required_rx),
            state: Ok(value.state),
            switch_slot: Ok(value.switch_slot),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpAnnounceSet {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for BgpAnnounceSet {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl BgpAnnounceSet {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpAnnounceSet> for super::BgpAnnounceSet {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BgpAnnounceSet,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::BgpAnnounceSet> for BgpAnnounceSet {
    fn from(value: super::BgpAnnounceSet) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpAnnounceSetCreate {
    announcement:
        ::std::result::Result<::std::vec::Vec<super::BgpAnnouncementCreate>, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for BgpAnnounceSetCreate {
    fn default() -> Self {
        Self {
            announcement: Err("no value supplied for announcement".to_string()),
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl BgpAnnounceSetCreate {
    pub fn announcement<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::BgpAnnouncementCreate>>,
        T::Error: ::std::fmt::Display,
    {
        self.announcement = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for announcement: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpAnnounceSetCreate> for super::BgpAnnounceSetCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BgpAnnounceSetCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            announcement: value.announcement?,
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::BgpAnnounceSetCreate> for BgpAnnounceSetCreate {
    fn from(value: super::BgpAnnounceSetCreate) -> Self {
        Self {
            announcement: Ok(value.announcement),
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpAnnouncement {
    address_lot_block_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    announce_set_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    network: ::std::result::Result<super::IpNet, ::std::string::String>,
}

impl ::std::default::Default for BgpAnnouncement {
    fn default() -> Self {
        Self {
            address_lot_block_id: Err("no value supplied for address_lot_block_id".to_string()),
            announce_set_id: Err("no value supplied for announce_set_id".to_string()),
            network: Err("no value supplied for network".to_string()),
        }
    }
}

impl BgpAnnouncement {
    pub fn address_lot_block_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.address_lot_block_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_lot_block_id: {e}"));
        self
    }

    pub fn announce_set_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.announce_set_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for announce_set_id: {e}"));
        self
    }

    pub fn network<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.network = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for network: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpAnnouncement> for super::BgpAnnouncement {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BgpAnnouncement,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            address_lot_block_id: value.address_lot_block_id?,
            announce_set_id: value.announce_set_id?,
            network: value.network?,
        })
    }
}

impl ::std::convert::From<super::BgpAnnouncement> for BgpAnnouncement {
    fn from(value: super::BgpAnnouncement) -> Self {
        Self {
            address_lot_block_id: Ok(value.address_lot_block_id),
            announce_set_id: Ok(value.announce_set_id),
            network: Ok(value.network),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpAnnouncementCreate {
    address_lot_block: ::std::result::Result<super::NameOrId, ::std::string::String>,
    network: ::std::result::Result<super::IpNet, ::std::string::String>,
}

impl ::std::default::Default for BgpAnnouncementCreate {
    fn default() -> Self {
        Self {
            address_lot_block: Err("no value supplied for address_lot_block".to_string()),
            network: Err("no value supplied for network".to_string()),
        }
    }
}

impl BgpAnnouncementCreate {
    pub fn address_lot_block<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.address_lot_block = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_lot_block: {e}"));
        self
    }

    pub fn network<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.network = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for network: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpAnnouncementCreate> for super::BgpAnnouncementCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BgpAnnouncementCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            address_lot_block: value.address_lot_block?,
            network: value.network?,
        })
    }
}

impl ::std::convert::From<super::BgpAnnouncementCreate> for BgpAnnouncementCreate {
    fn from(value: super::BgpAnnouncementCreate) -> Self {
        Self {
            address_lot_block: Ok(value.address_lot_block),
            network: Ok(value.network),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpConfig {
    asn: ::std::result::Result<u32, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    max_paths: ::std::result::Result<super::MaxPathConfig, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    vrf: ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for BgpConfig {
    fn default() -> Self {
        Self {
            asn: Err("no value supplied for asn".to_string()),
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            max_paths: Err("no value supplied for max_paths".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            vrf: Ok(Default::default()),
        }
    }
}

impl BgpConfig {
    pub fn asn<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.asn = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for asn: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn max_paths<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::MaxPathConfig>,
        T::Error: ::std::fmt::Display,
    {
        self.max_paths = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max_paths: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn vrf<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.vrf = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vrf: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpConfig> for super::BgpConfig {
    type Error = super::error::ConversionError;
    fn try_from(value: BgpConfig) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            asn: value.asn?,
            description: value.description?,
            id: value.id?,
            max_paths: value.max_paths?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            vrf: value.vrf?,
        })
    }
}

impl ::std::convert::From<super::BgpConfig> for BgpConfig {
    fn from(value: super::BgpConfig) -> Self {
        Self {
            asn: Ok(value.asn),
            description: Ok(value.description),
            id: Ok(value.id),
            max_paths: Ok(value.max_paths),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            vrf: Ok(value.vrf),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpConfigCreate {
    asn: ::std::result::Result<u32, ::std::string::String>,
    bgp_announce_set_id: ::std::result::Result<super::NameOrId, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    max_paths: ::std::result::Result<super::MaxPathConfig, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    vrf: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for BgpConfigCreate {
    fn default() -> Self {
        Self {
            asn: Err("no value supplied for asn".to_string()),
            bgp_announce_set_id: Err("no value supplied for bgp_announce_set_id".to_string()),
            description: Err("no value supplied for description".to_string()),
            max_paths: Ok(super::defaults::bgp_config_create_max_paths()),
            name: Err("no value supplied for name".to_string()),
            vrf: Ok(Default::default()),
        }
    }
}

impl BgpConfigCreate {
    pub fn asn<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.asn = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for asn: {e}"));
        self
    }

    pub fn bgp_announce_set_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.bgp_announce_set_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bgp_announce_set_id: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn max_paths<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::MaxPathConfig>,
        T::Error: ::std::fmt::Display,
    {
        self.max_paths = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max_paths: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn vrf<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.vrf = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vrf: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpConfigCreate> for super::BgpConfigCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BgpConfigCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            asn: value.asn?,
            bgp_announce_set_id: value.bgp_announce_set_id?,
            description: value.description?,
            max_paths: value.max_paths?,
            name: value.name?,
            vrf: value.vrf?,
        })
    }
}

impl ::std::convert::From<super::BgpConfigCreate> for BgpConfigCreate {
    fn from(value: super::BgpConfigCreate) -> Self {
        Self {
            asn: Ok(value.asn),
            bgp_announce_set_id: Ok(value.bgp_announce_set_id),
            description: Ok(value.description),
            max_paths: Ok(value.max_paths),
            name: Ok(value.name),
            vrf: Ok(value.vrf),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpConfigResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::BgpConfig>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for BgpConfigResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl BgpConfigResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::BgpConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpConfigResultsPage> for super::BgpConfigResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BgpConfigResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::BgpConfigResultsPage> for BgpConfigResultsPage {
    fn from(value: super::BgpConfigResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpConfigUpdate {
    bgp_announce_set_id:
        ::std::result::Result<::std::option::Option<super::NameOrId>, ::std::string::String>,
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    max_paths:
        ::std::result::Result<::std::option::Option<super::MaxPathConfig>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for BgpConfigUpdate {
    fn default() -> Self {
        Self {
            bgp_announce_set_id: Ok(Default::default()),
            description: Ok(Default::default()),
            max_paths: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl BgpConfigUpdate {
    pub fn bgp_announce_set_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::NameOrId>>,
        T::Error: ::std::fmt::Display,
    {
        self.bgp_announce_set_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bgp_announce_set_id: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn max_paths<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::MaxPathConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.max_paths = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max_paths: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpConfigUpdate> for super::BgpConfigUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BgpConfigUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bgp_announce_set_id: value.bgp_announce_set_id?,
            description: value.description?,
            max_paths: value.max_paths?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::BgpConfigUpdate> for BgpConfigUpdate {
    fn from(value: super::BgpConfigUpdate) -> Self {
        Self {
            bgp_announce_set_id: Ok(value.bgp_announce_set_id),
            description: Ok(value.description),
            max_paths: Ok(value.max_paths),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpExported {
    peer_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    prefix: ::std::result::Result<super::IpNet, ::std::string::String>,
    switch: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for BgpExported {
    fn default() -> Self {
        Self {
            peer_id: Err("no value supplied for peer_id".to_string()),
            prefix: Err("no value supplied for prefix".to_string()),
            switch: Err("no value supplied for switch".to_string()),
        }
    }
}

impl BgpExported {
    pub fn peer_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.peer_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for peer_id: {e}"));
        self
    }

    pub fn prefix<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.prefix = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for prefix: {e}"));
        self
    }

    pub fn switch<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpExported> for super::BgpExported {
    type Error = super::error::ConversionError;
    fn try_from(value: BgpExported) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            peer_id: value.peer_id?,
            prefix: value.prefix?,
            switch: value.switch?,
        })
    }
}

impl ::std::convert::From<super::BgpExported> for BgpExported {
    fn from(value: super::BgpExported) -> Self {
        Self {
            peer_id: Ok(value.peer_id),
            prefix: Ok(value.prefix),
            switch: Ok(value.switch),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpImported {
    id: ::std::result::Result<u32, ::std::string::String>,
    nexthop: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    prefix: ::std::result::Result<super::IpNet, ::std::string::String>,
    switch: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for BgpImported {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            nexthop: Err("no value supplied for nexthop".to_string()),
            prefix: Err("no value supplied for prefix".to_string()),
            switch: Err("no value supplied for switch".to_string()),
        }
    }
}

impl BgpImported {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn nexthop<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.nexthop = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for nexthop: {e}"));
        self
    }

    pub fn prefix<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.prefix = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for prefix: {e}"));
        self
    }

    pub fn switch<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpImported> for super::BgpImported {
    type Error = super::error::ConversionError;
    fn try_from(value: BgpImported) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            nexthop: value.nexthop?,
            prefix: value.prefix?,
            switch: value.switch?,
        })
    }
}

impl ::std::convert::From<super::BgpImported> for BgpImported {
    fn from(value: super::BgpImported) -> Self {
        Self {
            id: Ok(value.id),
            nexthop: Ok(value.nexthop),
            prefix: Ok(value.prefix),
            switch: Ok(value.switch),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpPeer {
    addr: ::std::result::Result<super::RouterPeerType, ::std::string::String>,
    allowed_export: ::std::result::Result<super::ImportExportPolicy, ::std::string::String>,
    allowed_import: ::std::result::Result<super::ImportExportPolicy, ::std::string::String>,
    bgp_config: ::std::result::Result<super::NameOrId, ::std::string::String>,
    communities: ::std::result::Result<::std::vec::Vec<u32>, ::std::string::String>,
    connect_retry: ::std::result::Result<u32, ::std::string::String>,
    delay_open: ::std::result::Result<u32, ::std::string::String>,
    enforce_first_as: ::std::result::Result<bool, ::std::string::String>,
    hold_time: ::std::result::Result<u32, ::std::string::String>,
    idle_hold_time: ::std::result::Result<u32, ::std::string::String>,
    keepalive: ::std::result::Result<u32, ::std::string::String>,
    local_pref: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
    md5_auth_key:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    min_ttl: ::std::result::Result<::std::option::Option<u8>, ::std::string::String>,
    multi_exit_discriminator:
        ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
    remote_asn: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
    vlan_id: ::std::result::Result<::std::option::Option<u16>, ::std::string::String>,
}

impl ::std::default::Default for BgpPeer {
    fn default() -> Self {
        Self {
            addr: Err("no value supplied for addr".to_string()),
            allowed_export: Err("no value supplied for allowed_export".to_string()),
            allowed_import: Err("no value supplied for allowed_import".to_string()),
            bgp_config: Err("no value supplied for bgp_config".to_string()),
            communities: Err("no value supplied for communities".to_string()),
            connect_retry: Err("no value supplied for connect_retry".to_string()),
            delay_open: Err("no value supplied for delay_open".to_string()),
            enforce_first_as: Err("no value supplied for enforce_first_as".to_string()),
            hold_time: Err("no value supplied for hold_time".to_string()),
            idle_hold_time: Err("no value supplied for idle_hold_time".to_string()),
            keepalive: Err("no value supplied for keepalive".to_string()),
            local_pref: Ok(Default::default()),
            md5_auth_key: Ok(Default::default()),
            min_ttl: Ok(Default::default()),
            multi_exit_discriminator: Ok(Default::default()),
            remote_asn: Ok(Default::default()),
            vlan_id: Ok(Default::default()),
        }
    }
}

impl BgpPeer {
    pub fn addr<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RouterPeerType>,
        T::Error: ::std::fmt::Display,
    {
        self.addr = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for addr: {e}"));
        self
    }

    pub fn allowed_export<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ImportExportPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.allowed_export = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for allowed_export: {e}"));
        self
    }

    pub fn allowed_import<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ImportExportPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.allowed_import = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for allowed_import: {e}"));
        self
    }

    pub fn bgp_config<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.bgp_config = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bgp_config: {e}"));
        self
    }

    pub fn communities<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<u32>>,
        T::Error: ::std::fmt::Display,
    {
        self.communities = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for communities: {e}"));
        self
    }

    pub fn connect_retry<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.connect_retry = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for connect_retry: {e}"));
        self
    }

    pub fn delay_open<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.delay_open = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for delay_open: {e}"));
        self
    }

    pub fn enforce_first_as<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.enforce_first_as = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for enforce_first_as: {e}"));
        self
    }

    pub fn hold_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.hold_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for hold_time: {e}"));
        self
    }

    pub fn idle_hold_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.idle_hold_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for idle_hold_time: {e}"));
        self
    }

    pub fn keepalive<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.keepalive = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for keepalive: {e}"));
        self
    }

    pub fn local_pref<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u32>>,
        T::Error: ::std::fmt::Display,
    {
        self.local_pref = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for local_pref: {e}"));
        self
    }

    pub fn md5_auth_key<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.md5_auth_key = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for md5_auth_key: {e}"));
        self
    }

    pub fn min_ttl<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u8>>,
        T::Error: ::std::fmt::Display,
    {
        self.min_ttl = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min_ttl: {e}"));
        self
    }

    pub fn multi_exit_discriminator<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u32>>,
        T::Error: ::std::fmt::Display,
    {
        self.multi_exit_discriminator = value.try_into().map_err(|e| {
            format!("error converting supplied value for multi_exit_discriminator: {e}")
        });
        self
    }

    pub fn remote_asn<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u32>>,
        T::Error: ::std::fmt::Display,
    {
        self.remote_asn = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for remote_asn: {e}"));
        self
    }

    pub fn vlan_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u16>>,
        T::Error: ::std::fmt::Display,
    {
        self.vlan_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vlan_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpPeer> for super::BgpPeer {
    type Error = super::error::ConversionError;
    fn try_from(value: BgpPeer) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            addr: value.addr?,
            allowed_export: value.allowed_export?,
            allowed_import: value.allowed_import?,
            bgp_config: value.bgp_config?,
            communities: value.communities?,
            connect_retry: value.connect_retry?,
            delay_open: value.delay_open?,
            enforce_first_as: value.enforce_first_as?,
            hold_time: value.hold_time?,
            idle_hold_time: value.idle_hold_time?,
            keepalive: value.keepalive?,
            local_pref: value.local_pref?,
            md5_auth_key: value.md5_auth_key?,
            min_ttl: value.min_ttl?,
            multi_exit_discriminator: value.multi_exit_discriminator?,
            remote_asn: value.remote_asn?,
            vlan_id: value.vlan_id?,
        })
    }
}

impl ::std::convert::From<super::BgpPeer> for BgpPeer {
    fn from(value: super::BgpPeer) -> Self {
        Self {
            addr: Ok(value.addr),
            allowed_export: Ok(value.allowed_export),
            allowed_import: Ok(value.allowed_import),
            bgp_config: Ok(value.bgp_config),
            communities: Ok(value.communities),
            connect_retry: Ok(value.connect_retry),
            delay_open: Ok(value.delay_open),
            enforce_first_as: Ok(value.enforce_first_as),
            hold_time: Ok(value.hold_time),
            idle_hold_time: Ok(value.idle_hold_time),
            keepalive: Ok(value.keepalive),
            local_pref: Ok(value.local_pref),
            md5_auth_key: Ok(value.md5_auth_key),
            min_ttl: Ok(value.min_ttl),
            multi_exit_discriminator: Ok(value.multi_exit_discriminator),
            remote_asn: Ok(value.remote_asn),
            vlan_id: Ok(value.vlan_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpPeerConfig {
    link_name: ::std::result::Result<super::Name, ::std::string::String>,
    peers: ::std::result::Result<::std::vec::Vec<super::BgpPeer>, ::std::string::String>,
}

impl ::std::default::Default for BgpPeerConfig {
    fn default() -> Self {
        Self {
            link_name: Err("no value supplied for link_name".to_string()),
            peers: Err("no value supplied for peers".to_string()),
        }
    }
}

impl BgpPeerConfig {
    pub fn link_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.link_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_name: {e}"));
        self
    }

    pub fn peers<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::BgpPeer>>,
        T::Error: ::std::fmt::Display,
    {
        self.peers = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for peers: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpPeerConfig> for super::BgpPeerConfig {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BgpPeerConfig,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            link_name: value.link_name?,
            peers: value.peers?,
        })
    }
}

impl ::std::convert::From<super::BgpPeerConfig> for BgpPeerConfig {
    fn from(value: super::BgpPeerConfig) -> Self {
        Self {
            link_name: Ok(value.link_name),
            peers: Ok(value.peers),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BgpPeerStatus {
    addr: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    local_asn: ::std::result::Result<u32, ::std::string::String>,
    peer_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    remote_asn: ::std::result::Result<u32, ::std::string::String>,
    state: ::std::result::Result<super::BgpPeerState, ::std::string::String>,
    state_duration_millis: ::std::result::Result<u64, ::std::string::String>,
    switch: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for BgpPeerStatus {
    fn default() -> Self {
        Self {
            addr: Err("no value supplied for addr".to_string()),
            local_asn: Err("no value supplied for local_asn".to_string()),
            peer_id: Err("no value supplied for peer_id".to_string()),
            remote_asn: Err("no value supplied for remote_asn".to_string()),
            state: Err("no value supplied for state".to_string()),
            state_duration_millis: Err("no value supplied for state_duration_millis".to_string()),
            switch: Err("no value supplied for switch".to_string()),
        }
    }
}

impl BgpPeerStatus {
    pub fn addr<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.addr = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for addr: {e}"));
        self
    }

    pub fn local_asn<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.local_asn = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for local_asn: {e}"));
        self
    }

    pub fn peer_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.peer_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for peer_id: {e}"));
        self
    }

    pub fn remote_asn<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.remote_asn = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for remote_asn: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BgpPeerState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn state_duration_millis<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.state_duration_millis = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state_duration_millis: {e}"));
        self
    }

    pub fn switch<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<BgpPeerStatus> for super::BgpPeerStatus {
    type Error = super::error::ConversionError;
    fn try_from(
        value: BgpPeerStatus,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            addr: value.addr?,
            local_asn: value.local_asn?,
            peer_id: value.peer_id?,
            remote_asn: value.remote_asn?,
            state: value.state?,
            state_duration_millis: value.state_duration_millis?,
            switch: value.switch?,
        })
    }
}

impl ::std::convert::From<super::BgpPeerStatus> for BgpPeerStatus {
    fn from(value: super::BgpPeerStatus) -> Self {
        Self {
            addr: Ok(value.addr),
            local_asn: Ok(value.local_asn),
            peer_id: Ok(value.peer_id),
            remote_asn: Ok(value.remote_asn),
            state: Ok(value.state),
            state_duration_millis: Ok(value.state_duration_millis),
            switch: Ok(value.switch),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bindouble {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangedouble, ::std::string::String>,
}

impl ::std::default::Default for Bindouble {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Bindouble {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangedouble>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Bindouble> for super::Bindouble {
    type Error = super::error::ConversionError;
    fn try_from(value: Bindouble) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Bindouble> for Bindouble {
    fn from(value: super::Bindouble) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binfloat {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangefloat, ::std::string::String>,
}

impl ::std::default::Default for Binfloat {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Binfloat {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangefloat>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Binfloat> for super::Binfloat {
    type Error = super::error::ConversionError;
    fn try_from(value: Binfloat) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Binfloat> for Binfloat {
    fn from(value: super::Binfloat) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binint16 {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangeint16, ::std::string::String>,
}

impl ::std::default::Default for Binint16 {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Binint16 {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangeint16>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Binint16> for super::Binint16 {
    type Error = super::error::ConversionError;
    fn try_from(value: Binint16) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Binint16> for Binint16 {
    fn from(value: super::Binint16) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binint32 {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangeint32, ::std::string::String>,
}

impl ::std::default::Default for Binint32 {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Binint32 {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangeint32>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Binint32> for super::Binint32 {
    type Error = super::error::ConversionError;
    fn try_from(value: Binint32) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Binint32> for Binint32 {
    fn from(value: super::Binint32) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binint64 {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangeint64, ::std::string::String>,
}

impl ::std::default::Default for Binint64 {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Binint64 {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangeint64>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Binint64> for super::Binint64 {
    type Error = super::error::ConversionError;
    fn try_from(value: Binint64) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Binint64> for Binint64 {
    fn from(value: super::Binint64) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binint8 {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangeint8, ::std::string::String>,
}

impl ::std::default::Default for Binint8 {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Binint8 {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangeint8>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Binint8> for super::Binint8 {
    type Error = super::error::ConversionError;
    fn try_from(value: Binint8) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Binint8> for Binint8 {
    fn from(value: super::Binint8) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binuint16 {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangeuint16, ::std::string::String>,
}

impl ::std::default::Default for Binuint16 {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Binuint16 {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangeuint16>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Binuint16> for super::Binuint16 {
    type Error = super::error::ConversionError;
    fn try_from(value: Binuint16) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Binuint16> for Binuint16 {
    fn from(value: super::Binuint16) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binuint32 {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangeuint32, ::std::string::String>,
}

impl ::std::default::Default for Binuint32 {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Binuint32 {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangeuint32>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Binuint32> for super::Binuint32 {
    type Error = super::error::ConversionError;
    fn try_from(value: Binuint32) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Binuint32> for Binuint32 {
    fn from(value: super::Binuint32) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binuint64 {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangeuint64, ::std::string::String>,
}

impl ::std::default::Default for Binuint64 {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Binuint64 {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangeuint64>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Binuint64> for super::Binuint64 {
    type Error = super::error::ConversionError;
    fn try_from(value: Binuint64) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Binuint64> for Binuint64 {
    fn from(value: super::Binuint64) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binuint8 {
    count: ::std::result::Result<u64, ::std::string::String>,
    range: ::std::result::Result<super::BinRangeuint8, ::std::string::String>,
}

impl ::std::default::Default for Binuint8 {
    fn default() -> Self {
        Self {
            count: Err("no value supplied for count".to_string()),
            range: Err("no value supplied for range".to_string()),
        }
    }
}

impl Binuint8 {
    pub fn count<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.count = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for count: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BinRangeuint8>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Binuint8> for super::Binuint8 {
    type Error = super::error::ConversionError;
    fn try_from(value: Binuint8) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            count: value.count?,
            range: value.range?,
        })
    }
}

impl ::std::convert::From<super::Binuint8> for Binuint8 {
    fn from(value: super::Binuint8) -> Self {
        Self {
            count: Ok(value.count),
            range: Ok(value.range),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Certificate {
    cert: ::std::result::Result<::std::string::String, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    service: ::std::result::Result<super::ServiceUsingCertificate, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Certificate {
    fn default() -> Self {
        Self {
            cert: Err("no value supplied for cert".to_string()),
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            service: Err("no value supplied for service".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Certificate {
    pub fn cert<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.cert = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cert: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn service<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ServiceUsingCertificate>,
        T::Error: ::std::fmt::Display,
    {
        self.service = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for service: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Certificate> for super::Certificate {
    type Error = super::error::ConversionError;
    fn try_from(value: Certificate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            cert: value.cert?,
            description: value.description?,
            id: value.id?,
            name: value.name?,
            service: value.service?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Certificate> for Certificate {
    fn from(value: super::Certificate) -> Self {
        Self {
            cert: Ok(value.cert),
            description: Ok(value.description),
            id: Ok(value.id),
            name: Ok(value.name),
            service: Ok(value.service),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CertificateCreate {
    cert: ::std::result::Result<::std::string::String, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    key: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    service: ::std::result::Result<super::ServiceUsingCertificate, ::std::string::String>,
}

impl ::std::default::Default for CertificateCreate {
    fn default() -> Self {
        Self {
            cert: Err("no value supplied for cert".to_string()),
            description: Err("no value supplied for description".to_string()),
            key: Err("no value supplied for key".to_string()),
            name: Err("no value supplied for name".to_string()),
            service: Err("no value supplied for service".to_string()),
        }
    }
}

impl CertificateCreate {
    pub fn cert<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.cert = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cert: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn key<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.key = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for key: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn service<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ServiceUsingCertificate>,
        T::Error: ::std::fmt::Display,
    {
        self.service = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for service: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<CertificateCreate> for super::CertificateCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: CertificateCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            cert: value.cert?,
            description: value.description?,
            key: value.key?,
            name: value.name?,
            service: value.service?,
        })
    }
}

impl ::std::convert::From<super::CertificateCreate> for CertificateCreate {
    fn from(value: super::CertificateCreate) -> Self {
        Self {
            cert: Ok(value.cert),
            description: Ok(value.description),
            key: Ok(value.key),
            name: Ok(value.name),
            service: Ok(value.service),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CertificateResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Certificate>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for CertificateResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl CertificateResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Certificate>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<CertificateResultsPage> for super::CertificateResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: CertificateResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::CertificateResultsPage> for CertificateResultsPage {
    fn from(value: super::CertificateResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConsoleSession {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_last_used:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for ConsoleSession {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_last_used: Err("no value supplied for time_last_used".to_string()),
        }
    }
}

impl ConsoleSession {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_last_used<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_last_used = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_last_used: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ConsoleSession> for super::ConsoleSession {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ConsoleSession,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            time_created: value.time_created?,
            time_last_used: value.time_last_used?,
        })
    }
}

impl ::std::convert::From<super::ConsoleSession> for ConsoleSession {
    fn from(value: super::ConsoleSession) -> Self {
        Self {
            id: Ok(value.id),
            time_created: Ok(value.time_created),
            time_last_used: Ok(value.time_last_used),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConsoleSessionResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::ConsoleSession>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for ConsoleSessionResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl ConsoleSessionResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::ConsoleSession>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ConsoleSessionResultsPage> for super::ConsoleSessionResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ConsoleSessionResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::ConsoleSessionResultsPage> for ConsoleSessionResultsPage {
    fn from(value: super::ConsoleSessionResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cumulativedouble {
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    value: ::std::result::Result<f64, ::std::string::String>,
}

impl ::std::default::Default for Cumulativedouble {
    fn default() -> Self {
        Self {
            start_time: Err("no value supplied for start_time".to_string()),
            value: Err("no value supplied for value".to_string()),
        }
    }
}

impl Cumulativedouble {
    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.value = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for value: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Cumulativedouble> for super::Cumulativedouble {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Cumulativedouble,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            start_time: value.start_time?,
            value: value.value?,
        })
    }
}

impl ::std::convert::From<super::Cumulativedouble> for Cumulativedouble {
    fn from(value: super::Cumulativedouble) -> Self {
        Self {
            start_time: Ok(value.start_time),
            value: Ok(value.value),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cumulativefloat {
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    value: ::std::result::Result<f32, ::std::string::String>,
}

impl ::std::default::Default for Cumulativefloat {
    fn default() -> Self {
        Self {
            start_time: Err("no value supplied for start_time".to_string()),
            value: Err("no value supplied for value".to_string()),
        }
    }
}

impl Cumulativefloat {
    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f32>,
        T::Error: ::std::fmt::Display,
    {
        self.value = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for value: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Cumulativefloat> for super::Cumulativefloat {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Cumulativefloat,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            start_time: value.start_time?,
            value: value.value?,
        })
    }
}

impl ::std::convert::From<super::Cumulativefloat> for Cumulativefloat {
    fn from(value: super::Cumulativefloat) -> Self {
        Self {
            start_time: Ok(value.start_time),
            value: Ok(value.value),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cumulativeint64 {
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    value: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Cumulativeint64 {
    fn default() -> Self {
        Self {
            start_time: Err("no value supplied for start_time".to_string()),
            value: Err("no value supplied for value".to_string()),
        }
    }
}

impl Cumulativeint64 {
    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.value = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for value: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Cumulativeint64> for super::Cumulativeint64 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Cumulativeint64,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            start_time: value.start_time?,
            value: value.value?,
        })
    }
}

impl ::std::convert::From<super::Cumulativeint64> for Cumulativeint64 {
    fn from(value: super::Cumulativeint64) -> Self {
        Self {
            start_time: Ok(value.start_time),
            value: Ok(value.value),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cumulativeuint64 {
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    value: ::std::result::Result<u64, ::std::string::String>,
}

impl ::std::default::Default for Cumulativeuint64 {
    fn default() -> Self {
        Self {
            start_time: Err("no value supplied for start_time".to_string()),
            value: Err("no value supplied for value".to_string()),
        }
    }
}

impl Cumulativeuint64 {
    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.value = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for value: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Cumulativeuint64> for super::Cumulativeuint64 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Cumulativeuint64,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            start_time: value.start_time?,
            value: value.value?,
        })
    }
}

impl ::std::convert::From<super::Cumulativeuint64> for Cumulativeuint64 {
    fn from(value: super::Cumulativeuint64) -> Self {
        Self {
            start_time: Ok(value.start_time),
            value: Ok(value.value),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    display_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    fleet_viewer: ::std::result::Result<bool, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    silo_admin: ::std::result::Result<bool, ::std::string::String>,
    silo_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    silo_name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for CurrentUser {
    fn default() -> Self {
        Self {
            display_name: Err("no value supplied for display_name".to_string()),
            fleet_viewer: Err("no value supplied for fleet_viewer".to_string()),
            id: Err("no value supplied for id".to_string()),
            silo_admin: Err("no value supplied for silo_admin".to_string()),
            silo_id: Err("no value supplied for silo_id".to_string()),
            silo_name: Err("no value supplied for silo_name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl CurrentUser {
    pub fn display_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.display_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for display_name: {e}"));
        self
    }

    pub fn fleet_viewer<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.fleet_viewer = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for fleet_viewer: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn silo_admin<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_admin = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_admin: {e}"));
        self
    }

    pub fn silo_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_id: {e}"));
        self
    }

    pub fn silo_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<CurrentUser> for super::CurrentUser {
    type Error = super::error::ConversionError;
    fn try_from(value: CurrentUser) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            display_name: value.display_name?,
            fleet_viewer: value.fleet_viewer?,
            id: value.id?,
            silo_admin: value.silo_admin?,
            silo_id: value.silo_id?,
            silo_name: value.silo_name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::CurrentUser> for CurrentUser {
    fn from(value: super::CurrentUser) -> Self {
        Self {
            display_name: Ok(value.display_name),
            fleet_viewer: Ok(value.fleet_viewer),
            id: Ok(value.id),
            silo_admin: Ok(value.silo_admin),
            silo_id: Ok(value.silo_id),
            silo_name: Ok(value.silo_name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DerEncodedKeyPair {
    private_key: ::std::result::Result<::std::string::String, ::std::string::String>,
    public_cert: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for DerEncodedKeyPair {
    fn default() -> Self {
        Self {
            private_key: Err("no value supplied for private_key".to_string()),
            public_cert: Err("no value supplied for public_cert".to_string()),
        }
    }
}

impl DerEncodedKeyPair {
    pub fn private_key<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.private_key = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for private_key: {e}"));
        self
    }

    pub fn public_cert<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.public_cert = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for public_cert: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<DerEncodedKeyPair> for super::DerEncodedKeyPair {
    type Error = super::error::ConversionError;
    fn try_from(
        value: DerEncodedKeyPair,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            private_key: value.private_key?,
            public_cert: value.public_cert?,
        })
    }
}

impl ::std::convert::From<super::DerEncodedKeyPair> for DerEncodedKeyPair {
    fn from(value: super::DerEncodedKeyPair) -> Self {
        Self {
            private_key: Ok(value.private_key),
            public_cert: Ok(value.public_cert),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceAccessToken {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_expires: ::std::result::Result<
        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for DeviceAccessToken {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_expires: Ok(Default::default()),
        }
    }
}

impl DeviceAccessToken {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_expires<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.time_expires = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_expires: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<DeviceAccessToken> for super::DeviceAccessToken {
    type Error = super::error::ConversionError;
    fn try_from(
        value: DeviceAccessToken,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            time_created: value.time_created?,
            time_expires: value.time_expires?,
        })
    }
}

impl ::std::convert::From<super::DeviceAccessToken> for DeviceAccessToken {
    fn from(value: super::DeviceAccessToken) -> Self {
        Self {
            id: Ok(value.id),
            time_created: Ok(value.time_created),
            time_expires: Ok(value.time_expires),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceAccessTokenRequest {
    client_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    device_code: ::std::result::Result<::std::string::String, ::std::string::String>,
    grant_type: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for DeviceAccessTokenRequest {
    fn default() -> Self {
        Self {
            client_id: Err("no value supplied for client_id".to_string()),
            device_code: Err("no value supplied for device_code".to_string()),
            grant_type: Err("no value supplied for grant_type".to_string()),
        }
    }
}

impl DeviceAccessTokenRequest {
    pub fn client_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.client_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for client_id: {e}"));
        self
    }

    pub fn device_code<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.device_code = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for device_code: {e}"));
        self
    }

    pub fn grant_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.grant_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for grant_type: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<DeviceAccessTokenRequest> for super::DeviceAccessTokenRequest {
    type Error = super::error::ConversionError;
    fn try_from(
        value: DeviceAccessTokenRequest,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            client_id: value.client_id?,
            device_code: value.device_code?,
            grant_type: value.grant_type?,
        })
    }
}

impl ::std::convert::From<super::DeviceAccessTokenRequest> for DeviceAccessTokenRequest {
    fn from(value: super::DeviceAccessTokenRequest) -> Self {
        Self {
            client_id: Ok(value.client_id),
            device_code: Ok(value.device_code),
            grant_type: Ok(value.grant_type),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceAccessTokenResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::DeviceAccessToken>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for DeviceAccessTokenResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl DeviceAccessTokenResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::DeviceAccessToken>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<DeviceAccessTokenResultsPage> for super::DeviceAccessTokenResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: DeviceAccessTokenResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::DeviceAccessTokenResultsPage> for DeviceAccessTokenResultsPage {
    fn from(value: super::DeviceAccessTokenResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceAuthRequest {
    client_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ttl_seconds:
        ::std::result::Result<::std::option::Option<::std::num::NonZeroU32>, ::std::string::String>,
}

impl ::std::default::Default for DeviceAuthRequest {
    fn default() -> Self {
        Self {
            client_id: Err("no value supplied for client_id".to_string()),
            ttl_seconds: Ok(Default::default()),
        }
    }
}

impl DeviceAuthRequest {
    pub fn client_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.client_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for client_id: {e}"));
        self
    }

    pub fn ttl_seconds<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU32>>,
        T::Error: ::std::fmt::Display,
    {
        self.ttl_seconds = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ttl_seconds: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<DeviceAuthRequest> for super::DeviceAuthRequest {
    type Error = super::error::ConversionError;
    fn try_from(
        value: DeviceAuthRequest,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            client_id: value.client_id?,
            ttl_seconds: value.ttl_seconds?,
        })
    }
}

impl ::std::convert::From<super::DeviceAuthRequest> for DeviceAuthRequest {
    fn from(value: super::DeviceAuthRequest) -> Self {
        Self {
            client_id: Ok(value.client_id),
            ttl_seconds: Ok(value.ttl_seconds),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceAuthVerify {
    user_code: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for DeviceAuthVerify {
    fn default() -> Self {
        Self {
            user_code: Err("no value supplied for user_code".to_string()),
        }
    }
}

impl DeviceAuthVerify {
    pub fn user_code<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.user_code = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for user_code: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<DeviceAuthVerify> for super::DeviceAuthVerify {
    type Error = super::error::ConversionError;
    fn try_from(
        value: DeviceAuthVerify,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            user_code: value.user_code?,
        })
    }
}

impl ::std::convert::From<super::DeviceAuthVerify> for DeviceAuthVerify {
    fn from(value: super::DeviceAuthVerify) -> Self {
        Self {
            user_code: Ok(value.user_code),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Disk {
    block_size: ::std::result::Result<super::BlockSize, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    device_path: ::std::result::Result<::std::string::String, ::std::string::String>,
    disk_type: ::std::result::Result<super::DiskType, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    image_id: ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    project_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    read_only: ::std::result::Result<bool, ::std::string::String>,
    size: ::std::result::Result<super::ByteCount, ::std::string::String>,
    snapshot_id: ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    state: ::std::result::Result<super::DiskState, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Disk {
    fn default() -> Self {
        Self {
            block_size: Err("no value supplied for block_size".to_string()),
            description: Err("no value supplied for description".to_string()),
            device_path: Err("no value supplied for device_path".to_string()),
            disk_type: Err("no value supplied for disk_type".to_string()),
            id: Err("no value supplied for id".to_string()),
            image_id: Ok(Default::default()),
            name: Err("no value supplied for name".to_string()),
            project_id: Err("no value supplied for project_id".to_string()),
            read_only: Err("no value supplied for read_only".to_string()),
            size: Err("no value supplied for size".to_string()),
            snapshot_id: Ok(Default::default()),
            state: Err("no value supplied for state".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Disk {
    pub fn block_size<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BlockSize>,
        T::Error: ::std::fmt::Display,
    {
        self.block_size = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for block_size: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn device_path<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.device_path = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for device_path: {e}"));
        self
    }

    pub fn disk_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::DiskType>,
        T::Error: ::std::fmt::Display,
    {
        self.disk_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for disk_type: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn image_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.image_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for image_id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn project_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.project_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_id: {e}"));
        self
    }

    pub fn read_only<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.read_only = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for read_only: {e}"));
        self
    }

    pub fn size<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.size = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for size: {e}"));
        self
    }

    pub fn snapshot_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.snapshot_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for snapshot_id: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::DiskState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Disk> for super::Disk {
    type Error = super::error::ConversionError;
    fn try_from(value: Disk) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            block_size: value.block_size?,
            description: value.description?,
            device_path: value.device_path?,
            disk_type: value.disk_type?,
            id: value.id?,
            image_id: value.image_id?,
            name: value.name?,
            project_id: value.project_id?,
            read_only: value.read_only?,
            size: value.size?,
            snapshot_id: value.snapshot_id?,
            state: value.state?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Disk> for Disk {
    fn from(value: super::Disk) -> Self {
        Self {
            block_size: Ok(value.block_size),
            description: Ok(value.description),
            device_path: Ok(value.device_path),
            disk_type: Ok(value.disk_type),
            id: Ok(value.id),
            image_id: Ok(value.image_id),
            name: Ok(value.name),
            project_id: Ok(value.project_id),
            read_only: Ok(value.read_only),
            size: Ok(value.size),
            snapshot_id: Ok(value.snapshot_id),
            state: Ok(value.state),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiskCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    disk_backend: ::std::result::Result<super::DiskBackend, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    size: ::std::result::Result<super::ByteCount, ::std::string::String>,
}

impl ::std::default::Default for DiskCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            disk_backend: Err("no value supplied for disk_backend".to_string()),
            name: Err("no value supplied for name".to_string()),
            size: Err("no value supplied for size".to_string()),
        }
    }
}

impl DiskCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn disk_backend<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::DiskBackend>,
        T::Error: ::std::fmt::Display,
    {
        self.disk_backend = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for disk_backend: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn size<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.size = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for size: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<DiskCreate> for super::DiskCreate {
    type Error = super::error::ConversionError;
    fn try_from(value: DiskCreate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            disk_backend: value.disk_backend?,
            name: value.name?,
            size: value.size?,
        })
    }
}

impl ::std::convert::From<super::DiskCreate> for DiskCreate {
    fn from(value: super::DiskCreate) -> Self {
        Self {
            description: Ok(value.description),
            disk_backend: Ok(value.disk_backend),
            name: Ok(value.name),
            size: Ok(value.size),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiskPath {
    disk: ::std::result::Result<super::NameOrId, ::std::string::String>,
}

impl ::std::default::Default for DiskPath {
    fn default() -> Self {
        Self {
            disk: Err("no value supplied for disk".to_string()),
        }
    }
}

impl DiskPath {
    pub fn disk<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.disk = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for disk: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<DiskPath> for super::DiskPath {
    type Error = super::error::ConversionError;
    fn try_from(value: DiskPath) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self { disk: value.disk? })
    }
}

impl ::std::convert::From<super::DiskPath> for DiskPath {
    fn from(value: super::DiskPath) -> Self {
        Self {
            disk: Ok(value.disk),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiskResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Disk>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for DiskResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl DiskResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Disk>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<DiskResultsPage> for super::DiskResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: DiskResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::DiskResultsPage> for DiskResultsPage {
    fn from(value: super::DiskResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Distributiondouble {
    bins: ::std::result::Result<::std::vec::Vec<f64>, ::std::string::String>,
    counts: ::std::result::Result<::std::vec::Vec<u64>, ::std::string::String>,
    max: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    p50: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    p90: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    p99: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    sum_of_samples: ::std::result::Result<f64, ::std::string::String>,
}

impl ::std::default::Default for Distributiondouble {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            counts: Err("no value supplied for counts".to_string()),
            max: Ok(Default::default()),
            min: Ok(Default::default()),
            p50: Ok(Default::default()),
            p90: Ok(Default::default()),
            p99: Ok(Default::default()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Distributiondouble {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<f64>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn counts<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<u64>>,
        T::Error: ::std::fmt::Display,
    {
        self.counts = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for counts: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<f64>>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<f64>>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<f64>>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<f64>>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<f64>>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Distributiondouble> for super::Distributiondouble {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Distributiondouble,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            counts: value.counts?,
            max: value.max?,
            min: value.min?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Distributiondouble> for Distributiondouble {
    fn from(value: super::Distributiondouble) -> Self {
        Self {
            bins: Ok(value.bins),
            counts: Ok(value.counts),
            max: Ok(value.max),
            min: Ok(value.min),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Distributionint64 {
    bins: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
    counts: ::std::result::Result<::std::vec::Vec<u64>, ::std::string::String>,
    max: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    min: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    p50: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    p90: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    p99: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    sum_of_samples: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Distributionint64 {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            counts: Err("no value supplied for counts".to_string()),
            max: Ok(Default::default()),
            min: Ok(Default::default()),
            p50: Ok(Default::default()),
            p90: Ok(Default::default()),
            p99: Ok(Default::default()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Distributionint64 {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn counts<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<u64>>,
        T::Error: ::std::fmt::Display,
    {
        self.counts = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for counts: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<i64>>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<i64>>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<f64>>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<f64>>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<f64>>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Distributionint64> for super::Distributionint64 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Distributionint64,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            counts: value.counts?,
            max: value.max?,
            min: value.min?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Distributionint64> for Distributionint64 {
    fn from(value: super::Distributionint64) -> Self {
        Self {
            bins: Ok(value.bins),
            counts: Ok(value.counts),
            max: Ok(value.max),
            min: Ok(value.min),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EphemeralIpCreate {
    pool_selector: ::std::result::Result<super::PoolSelector, ::std::string::String>,
}

impl ::std::default::Default for EphemeralIpCreate {
    fn default() -> Self {
        Self {
            pool_selector: Ok(super::defaults::ephemeral_ip_create_pool_selector()),
        }
    }
}

impl EphemeralIpCreate {
    pub fn pool_selector<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PoolSelector>,
        T::Error: ::std::fmt::Display,
    {
        self.pool_selector = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for pool_selector: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<EphemeralIpCreate> for super::EphemeralIpCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: EphemeralIpCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            pool_selector: value.pool_selector?,
        })
    }
}

impl ::std::convert::From<super::EphemeralIpCreate> for EphemeralIpCreate {
    fn from(value: super::EphemeralIpCreate) -> Self {
        Self {
            pool_selector: Ok(value.pool_selector),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Error {
    error_code:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    message: ::std::result::Result<::std::string::String, ::std::string::String>,
    request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for Error {
    fn default() -> Self {
        Self {
            error_code: Ok(Default::default()),
            message: Err("no value supplied for message".to_string()),
            request_id: Err("no value supplied for request_id".to_string()),
        }
    }
}

impl Error {
    pub fn error_code<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.error_code = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for error_code: {e}"));
        self
    }

    pub fn message<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.message = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for message: {e}"));
        self
    }

    pub fn request_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.request_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for request_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Error> for super::Error {
    type Error = super::error::ConversionError;
    fn try_from(value: Error) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            error_code: value.error_code?,
            message: value.message?,
            request_id: value.request_id?,
        })
    }
}

impl ::std::convert::From<super::Error> for Error {
    fn from(value: super::Error) -> Self {
        Self {
            error_code: Ok(value.error_code),
            message: Ok(value.message),
            request_id: Ok(value.request_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExternalIpResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::ExternalIp>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for ExternalIpResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl ExternalIpResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalIp>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ExternalIpResultsPage> for super::ExternalIpResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ExternalIpResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::ExternalIpResultsPage> for ExternalIpResultsPage {
    fn from(value: super::ExternalIpResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExternalSubnet {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    instance_id: ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    project_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    subnet: ::std::result::Result<super::IpNet, ::std::string::String>,
    subnet_pool_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    subnet_pool_member_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for ExternalSubnet {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            instance_id: Ok(Default::default()),
            name: Err("no value supplied for name".to_string()),
            project_id: Err("no value supplied for project_id".to_string()),
            subnet: Err("no value supplied for subnet".to_string()),
            subnet_pool_id: Err("no value supplied for subnet_pool_id".to_string()),
            subnet_pool_member_id: Err("no value supplied for subnet_pool_member_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl ExternalSubnet {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn instance_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.instance_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for instance_id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn project_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.project_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_id: {e}"));
        self
    }

    pub fn subnet<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet: {e}"));
        self
    }

    pub fn subnet_pool_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet_pool_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet_pool_id: {e}"));
        self
    }

    pub fn subnet_pool_member_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet_pool_member_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet_pool_member_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ExternalSubnet> for super::ExternalSubnet {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ExternalSubnet,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            instance_id: value.instance_id?,
            name: value.name?,
            project_id: value.project_id?,
            subnet: value.subnet?,
            subnet_pool_id: value.subnet_pool_id?,
            subnet_pool_member_id: value.subnet_pool_member_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::ExternalSubnet> for ExternalSubnet {
    fn from(value: super::ExternalSubnet) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            instance_id: Ok(value.instance_id),
            name: Ok(value.name),
            project_id: Ok(value.project_id),
            subnet: Ok(value.subnet),
            subnet_pool_id: Ok(value.subnet_pool_id),
            subnet_pool_member_id: Ok(value.subnet_pool_member_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExternalSubnetAttach {
    instance: ::std::result::Result<super::NameOrId, ::std::string::String>,
}

impl ::std::default::Default for ExternalSubnetAttach {
    fn default() -> Self {
        Self {
            instance: Err("no value supplied for instance".to_string()),
        }
    }
}

impl ExternalSubnetAttach {
    pub fn instance<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.instance = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for instance: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ExternalSubnetAttach> for super::ExternalSubnetAttach {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ExternalSubnetAttach,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            instance: value.instance?,
        })
    }
}

impl ::std::convert::From<super::ExternalSubnetAttach> for ExternalSubnetAttach {
    fn from(value: super::ExternalSubnetAttach) -> Self {
        Self {
            instance: Ok(value.instance),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExternalSubnetCreate {
    allocator: ::std::result::Result<super::ExternalSubnetAllocator, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for ExternalSubnetCreate {
    fn default() -> Self {
        Self {
            allocator: Err("no value supplied for allocator".to_string()),
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl ExternalSubnetCreate {
    pub fn allocator<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ExternalSubnetAllocator>,
        T::Error: ::std::fmt::Display,
    {
        self.allocator = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for allocator: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ExternalSubnetCreate> for super::ExternalSubnetCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ExternalSubnetCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            allocator: value.allocator?,
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::ExternalSubnetCreate> for ExternalSubnetCreate {
    fn from(value: super::ExternalSubnetCreate) -> Self {
        Self {
            allocator: Ok(value.allocator),
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExternalSubnetResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::ExternalSubnet>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for ExternalSubnetResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl ExternalSubnetResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalSubnet>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ExternalSubnetResultsPage> for super::ExternalSubnetResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ExternalSubnetResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::ExternalSubnetResultsPage> for ExternalSubnetResultsPage {
    fn from(value: super::ExternalSubnetResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExternalSubnetUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for ExternalSubnetUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl ExternalSubnetUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ExternalSubnetUpdate> for super::ExternalSubnetUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ExternalSubnetUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::ExternalSubnetUpdate> for ExternalSubnetUpdate {
    fn from(value: super::ExternalSubnetUpdate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldSchema {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    field_type: ::std::result::Result<super::FieldType, ::std::string::String>,
    name: ::std::result::Result<::std::string::String, ::std::string::String>,
    source: ::std::result::Result<super::FieldSource, ::std::string::String>,
}

impl ::std::default::Default for FieldSchema {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            field_type: Err("no value supplied for field_type".to_string()),
            name: Err("no value supplied for name".to_string()),
            source: Err("no value supplied for source".to_string()),
        }
    }
}

impl FieldSchema {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn field_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::FieldType>,
        T::Error: ::std::fmt::Display,
    {
        self.field_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for field_type: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn source<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::FieldSource>,
        T::Error: ::std::fmt::Display,
    {
        self.source = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for source: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<FieldSchema> for super::FieldSchema {
    type Error = super::error::ConversionError;
    fn try_from(value: FieldSchema) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            field_type: value.field_type?,
            name: value.name?,
            source: value.source?,
        })
    }
}

impl ::std::convert::From<super::FieldSchema> for FieldSchema {
    fn from(value: super::FieldSchema) -> Self {
        Self {
            description: Ok(value.description),
            field_type: Ok(value.field_type),
            name: Ok(value.name),
            source: Ok(value.source),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FinalizeDisk {
    snapshot_name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for FinalizeDisk {
    fn default() -> Self {
        Self {
            snapshot_name: Ok(Default::default()),
        }
    }
}

impl FinalizeDisk {
    pub fn snapshot_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.snapshot_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for snapshot_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<FinalizeDisk> for super::FinalizeDisk {
    type Error = super::error::ConversionError;
    fn try_from(value: FinalizeDisk) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            snapshot_name: value.snapshot_name?,
        })
    }
}

impl ::std::convert::From<super::FinalizeDisk> for FinalizeDisk {
    fn from(value: super::FinalizeDisk) -> Self {
        Self {
            snapshot_name: Ok(value.snapshot_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FleetRolePolicy {
    role_assignments: ::std::result::Result<
        ::std::vec::Vec<super::FleetRoleRoleAssignment>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for FleetRolePolicy {
    fn default() -> Self {
        Self {
            role_assignments: Err("no value supplied for role_assignments".to_string()),
        }
    }
}

impl FleetRolePolicy {
    pub fn role_assignments<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::FleetRoleRoleAssignment>>,
        T::Error: ::std::fmt::Display,
    {
        self.role_assignments = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for role_assignments: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<FleetRolePolicy> for super::FleetRolePolicy {
    type Error = super::error::ConversionError;
    fn try_from(
        value: FleetRolePolicy,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            role_assignments: value.role_assignments?,
        })
    }
}

impl ::std::convert::From<super::FleetRolePolicy> for FleetRolePolicy {
    fn from(value: super::FleetRolePolicy) -> Self {
        Self {
            role_assignments: Ok(value.role_assignments),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FleetRoleRoleAssignment {
    identity_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    identity_type: ::std::result::Result<super::IdentityType, ::std::string::String>,
    role_name: ::std::result::Result<super::FleetRole, ::std::string::String>,
}

impl ::std::default::Default for FleetRoleRoleAssignment {
    fn default() -> Self {
        Self {
            identity_id: Err("no value supplied for identity_id".to_string()),
            identity_type: Err("no value supplied for identity_type".to_string()),
            role_name: Err("no value supplied for role_name".to_string()),
        }
    }
}

impl FleetRoleRoleAssignment {
    pub fn identity_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.identity_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_id: {e}"));
        self
    }

    pub fn identity_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IdentityType>,
        T::Error: ::std::fmt::Display,
    {
        self.identity_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_type: {e}"));
        self
    }

    pub fn role_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::FleetRole>,
        T::Error: ::std::fmt::Display,
    {
        self.role_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for role_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<FleetRoleRoleAssignment> for super::FleetRoleRoleAssignment {
    type Error = super::error::ConversionError;
    fn try_from(
        value: FleetRoleRoleAssignment,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            identity_id: value.identity_id?,
            identity_type: value.identity_type?,
            role_name: value.role_name?,
        })
    }
}

impl ::std::convert::From<super::FleetRoleRoleAssignment> for FleetRoleRoleAssignment {
    fn from(value: super::FleetRoleRoleAssignment) -> Self {
        Self {
            identity_id: Ok(value.identity_id),
            identity_type: Ok(value.identity_type),
            role_name: Ok(value.role_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloatingIp {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    instance_id: ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    ip: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    ip_pool_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    project_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for FloatingIp {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            instance_id: Ok(Default::default()),
            ip: Err("no value supplied for ip".to_string()),
            ip_pool_id: Err("no value supplied for ip_pool_id".to_string()),
            name: Err("no value supplied for name".to_string()),
            project_id: Err("no value supplied for project_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl FloatingIp {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn instance_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.instance_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for instance_id: {e}"));
        self
    }

    pub fn ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip: {e}"));
        self
    }

    pub fn ip_pool_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_pool_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_pool_id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn project_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.project_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<FloatingIp> for super::FloatingIp {
    type Error = super::error::ConversionError;
    fn try_from(value: FloatingIp) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            instance_id: value.instance_id?,
            ip: value.ip?,
            ip_pool_id: value.ip_pool_id?,
            name: value.name?,
            project_id: value.project_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::FloatingIp> for FloatingIp {
    fn from(value: super::FloatingIp) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            instance_id: Ok(value.instance_id),
            ip: Ok(value.ip),
            ip_pool_id: Ok(value.ip_pool_id),
            name: Ok(value.name),
            project_id: Ok(value.project_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloatingIpAttach {
    kind: ::std::result::Result<super::FloatingIpParentKind, ::std::string::String>,
    parent: ::std::result::Result<super::NameOrId, ::std::string::String>,
}

impl ::std::default::Default for FloatingIpAttach {
    fn default() -> Self {
        Self {
            kind: Err("no value supplied for kind".to_string()),
            parent: Err("no value supplied for parent".to_string()),
        }
    }
}

impl FloatingIpAttach {
    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::FloatingIpParentKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn parent<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.parent = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for parent: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<FloatingIpAttach> for super::FloatingIpAttach {
    type Error = super::error::ConversionError;
    fn try_from(
        value: FloatingIpAttach,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            kind: value.kind?,
            parent: value.parent?,
        })
    }
}

impl ::std::convert::From<super::FloatingIpAttach> for FloatingIpAttach {
    fn from(value: super::FloatingIpAttach) -> Self {
        Self {
            kind: Ok(value.kind),
            parent: Ok(value.parent),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloatingIpCreate {
    address_allocator: ::std::result::Result<super::AddressAllocator, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for FloatingIpCreate {
    fn default() -> Self {
        Self {
            address_allocator: Ok(super::defaults::floating_ip_create_address_allocator()),
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl FloatingIpCreate {
    pub fn address_allocator<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AddressAllocator>,
        T::Error: ::std::fmt::Display,
    {
        self.address_allocator = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_allocator: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<FloatingIpCreate> for super::FloatingIpCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: FloatingIpCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            address_allocator: value.address_allocator?,
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::FloatingIpCreate> for FloatingIpCreate {
    fn from(value: super::FloatingIpCreate) -> Self {
        Self {
            address_allocator: Ok(value.address_allocator),
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloatingIpResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::FloatingIp>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for FloatingIpResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl FloatingIpResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::FloatingIp>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<FloatingIpResultsPage> for super::FloatingIpResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: FloatingIpResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::FloatingIpResultsPage> for FloatingIpResultsPage {
    fn from(value: super::FloatingIpResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloatingIpUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for FloatingIpUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl FloatingIpUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<FloatingIpUpdate> for super::FloatingIpUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: FloatingIpUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::FloatingIpUpdate> for FloatingIpUpdate {
    fn from(value: super::FloatingIpUpdate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Group {
    display_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    silo_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Group {
    fn default() -> Self {
        Self {
            display_name: Err("no value supplied for display_name".to_string()),
            id: Err("no value supplied for id".to_string()),
            silo_id: Err("no value supplied for silo_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Group {
    pub fn display_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.display_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for display_name: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn silo_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Group> for super::Group {
    type Error = super::error::ConversionError;
    fn try_from(value: Group) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            display_name: value.display_name?,
            id: value.id?,
            silo_id: value.silo_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Group> for Group {
    fn from(value: super::Group) -> Self {
        Self {
            display_name: Ok(value.display_name),
            id: Ok(value.id),
            silo_id: Ok(value.silo_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GroupResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Group>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for GroupResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl GroupResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Group>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<GroupResultsPage> for super::GroupResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: GroupResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::GroupResultsPage> for GroupResultsPage {
    fn from(value: super::GroupResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramdouble {
    bins: ::std::result::Result<::std::vec::Vec<super::Bindouble>, ::std::string::String>,
    max: ::std::result::Result<f64, ::std::string::String>,
    min: ::std::result::Result<f64, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<f64, ::std::string::String>,
}

impl ::std::default::Default for Histogramdouble {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramdouble {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Bindouble>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramdouble> for super::Histogramdouble {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramdouble,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramdouble> for Histogramdouble {
    fn from(value: super::Histogramdouble) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramfloat {
    bins: ::std::result::Result<::std::vec::Vec<super::Binfloat>, ::std::string::String>,
    max: ::std::result::Result<f32, ::std::string::String>,
    min: ::std::result::Result<f32, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<f64, ::std::string::String>,
}

impl ::std::default::Default for Histogramfloat {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramfloat {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Binfloat>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f32>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f32>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramfloat> for super::Histogramfloat {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramfloat,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramfloat> for Histogramfloat {
    fn from(value: super::Histogramfloat) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramint16 {
    bins: ::std::result::Result<::std::vec::Vec<super::Binint16>, ::std::string::String>,
    max: ::std::result::Result<i16, ::std::string::String>,
    min: ::std::result::Result<i16, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Histogramint16 {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramint16 {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Binint16>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i16>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i16>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramint16> for super::Histogramint16 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramint16,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramint16> for Histogramint16 {
    fn from(value: super::Histogramint16) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramint32 {
    bins: ::std::result::Result<::std::vec::Vec<super::Binint32>, ::std::string::String>,
    max: ::std::result::Result<i32, ::std::string::String>,
    min: ::std::result::Result<i32, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Histogramint32 {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramint32 {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Binint32>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i32>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i32>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramint32> for super::Histogramint32 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramint32,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramint32> for Histogramint32 {
    fn from(value: super::Histogramint32) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramint64 {
    bins: ::std::result::Result<::std::vec::Vec<super::Binint64>, ::std::string::String>,
    max: ::std::result::Result<i64, ::std::string::String>,
    min: ::std::result::Result<i64, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Histogramint64 {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramint64 {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Binint64>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramint64> for super::Histogramint64 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramint64,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramint64> for Histogramint64 {
    fn from(value: super::Histogramint64) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramint8 {
    bins: ::std::result::Result<::std::vec::Vec<super::Binint8>, ::std::string::String>,
    max: ::std::result::Result<i8, ::std::string::String>,
    min: ::std::result::Result<i8, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Histogramint8 {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramint8 {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Binint8>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i8>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i8>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramint8> for super::Histogramint8 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramint8,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramint8> for Histogramint8 {
    fn from(value: super::Histogramint8) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramuint16 {
    bins: ::std::result::Result<::std::vec::Vec<super::Binuint16>, ::std::string::String>,
    max: ::std::result::Result<u16, ::std::string::String>,
    min: ::std::result::Result<u16, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Histogramuint16 {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramuint16 {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Binuint16>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramuint16> for super::Histogramuint16 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramuint16,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramuint16> for Histogramuint16 {
    fn from(value: super::Histogramuint16) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramuint32 {
    bins: ::std::result::Result<::std::vec::Vec<super::Binuint32>, ::std::string::String>,
    max: ::std::result::Result<u32, ::std::string::String>,
    min: ::std::result::Result<u32, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Histogramuint32 {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramuint32 {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Binuint32>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramuint32> for super::Histogramuint32 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramuint32,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramuint32> for Histogramuint32 {
    fn from(value: super::Histogramuint32) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramuint64 {
    bins: ::std::result::Result<::std::vec::Vec<super::Binuint64>, ::std::string::String>,
    max: ::std::result::Result<u64, ::std::string::String>,
    min: ::std::result::Result<u64, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Histogramuint64 {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramuint64 {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Binuint64>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramuint64> for super::Histogramuint64 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramuint64,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramuint64> for Histogramuint64 {
    fn from(value: super::Histogramuint64) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Histogramuint8 {
    bins: ::std::result::Result<::std::vec::Vec<super::Binuint8>, ::std::string::String>,
    max: ::std::result::Result<u8, ::std::string::String>,
    min: ::std::result::Result<u8, ::std::string::String>,
    n_samples: ::std::result::Result<u64, ::std::string::String>,
    p50: ::std::result::Result<super::Quantile, ::std::string::String>,
    p90: ::std::result::Result<super::Quantile, ::std::string::String>,
    p99: ::std::result::Result<super::Quantile, ::std::string::String>,
    squared_mean: ::std::result::Result<f64, ::std::string::String>,
    start_time:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    sum_of_samples: ::std::result::Result<i64, ::std::string::String>,
}

impl ::std::default::Default for Histogramuint8 {
    fn default() -> Self {
        Self {
            bins: Err("no value supplied for bins".to_string()),
            max: Err("no value supplied for max".to_string()),
            min: Err("no value supplied for min".to_string()),
            n_samples: Err("no value supplied for n_samples".to_string()),
            p50: Err("no value supplied for p50".to_string()),
            p90: Err("no value supplied for p90".to_string()),
            p99: Err("no value supplied for p99".to_string()),
            squared_mean: Err("no value supplied for squared_mean".to_string()),
            start_time: Err("no value supplied for start_time".to_string()),
            sum_of_samples: Err("no value supplied for sum_of_samples".to_string()),
        }
    }
}

impl Histogramuint8 {
    pub fn bins<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Binuint8>>,
        T::Error: ::std::fmt::Display,
    {
        self.bins = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bins: {e}"));
        self
    }

    pub fn max<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u8>,
        T::Error: ::std::fmt::Display,
    {
        self.max = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max: {e}"));
        self
    }

    pub fn min<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u8>,
        T::Error: ::std::fmt::Display,
    {
        self.min = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min: {e}"));
        self
    }

    pub fn n_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.n_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for n_samples: {e}"));
        self
    }

    pub fn p50<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p50 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p50: {e}"));
        self
    }

    pub fn p90<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p90 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p90: {e}"));
        self
    }

    pub fn p99<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Quantile>,
        T::Error: ::std::fmt::Display,
    {
        self.p99 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p99: {e}"));
        self
    }

    pub fn squared_mean<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.squared_mean = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for squared_mean: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }

    pub fn sum_of_samples<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.sum_of_samples = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sum_of_samples: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Histogramuint8> for super::Histogramuint8 {
    type Error = super::error::ConversionError;
    fn try_from(
        value: Histogramuint8,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bins: value.bins?,
            max: value.max?,
            min: value.min?,
            n_samples: value.n_samples?,
            p50: value.p50?,
            p90: value.p90?,
            p99: value.p99?,
            squared_mean: value.squared_mean?,
            start_time: value.start_time?,
            sum_of_samples: value.sum_of_samples?,
        })
    }
}

impl ::std::convert::From<super::Histogramuint8> for Histogramuint8 {
    fn from(value: super::Histogramuint8) -> Self {
        Self {
            bins: Ok(value.bins),
            max: Ok(value.max),
            min: Ok(value.min),
            n_samples: Ok(value.n_samples),
            p50: Ok(value.p50),
            p90: Ok(value.p90),
            p99: Ok(value.p99),
            squared_mean: Ok(value.squared_mean),
            start_time: Ok(value.start_time),
            sum_of_samples: Ok(value.sum_of_samples),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IdentityProvider {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    provider_type: ::std::result::Result<super::IdentityProviderType, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for IdentityProvider {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            provider_type: Err("no value supplied for provider_type".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl IdentityProvider {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn provider_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IdentityProviderType>,
        T::Error: ::std::fmt::Display,
    {
        self.provider_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for provider_type: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IdentityProvider> for super::IdentityProvider {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IdentityProvider,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            name: value.name?,
            provider_type: value.provider_type?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::IdentityProvider> for IdentityProvider {
    fn from(value: super::IdentityProvider) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            name: Ok(value.name),
            provider_type: Ok(value.provider_type),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IdentityProviderResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::IdentityProvider>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for IdentityProviderResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl IdentityProviderResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::IdentityProvider>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IdentityProviderResultsPage> for super::IdentityProviderResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IdentityProviderResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::IdentityProviderResultsPage> for IdentityProviderResultsPage {
    fn from(value: super::IdentityProviderResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Image {
    block_size: ::std::result::Result<super::BlockSize, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    digest: ::std::result::Result<::std::option::Option<super::Digest>, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    os: ::std::result::Result<::std::string::String, ::std::string::String>,
    project_id: ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    size: ::std::result::Result<super::ByteCount, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    version: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for Image {
    fn default() -> Self {
        Self {
            block_size: Err("no value supplied for block_size".to_string()),
            description: Err("no value supplied for description".to_string()),
            digest: Ok(Default::default()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            os: Err("no value supplied for os".to_string()),
            project_id: Ok(Default::default()),
            size: Err("no value supplied for size".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            version: Err("no value supplied for version".to_string()),
        }
    }
}

impl Image {
    pub fn block_size<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::BlockSize>,
        T::Error: ::std::fmt::Display,
    {
        self.block_size = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for block_size: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn digest<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Digest>>,
        T::Error: ::std::fmt::Display,
    {
        self.digest = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for digest: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn os<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.os = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for os: {e}"));
        self
    }

    pub fn project_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.project_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_id: {e}"));
        self
    }

    pub fn size<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.size = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for size: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for version: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Image> for super::Image {
    type Error = super::error::ConversionError;
    fn try_from(value: Image) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            block_size: value.block_size?,
            description: value.description?,
            digest: value.digest?,
            id: value.id?,
            name: value.name?,
            os: value.os?,
            project_id: value.project_id?,
            size: value.size?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            version: value.version?,
        })
    }
}

impl ::std::convert::From<super::Image> for Image {
    fn from(value: super::Image) -> Self {
        Self {
            block_size: Ok(value.block_size),
            description: Ok(value.description),
            digest: Ok(value.digest),
            id: Ok(value.id),
            name: Ok(value.name),
            os: Ok(value.os),
            project_id: Ok(value.project_id),
            size: Ok(value.size),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            version: Ok(value.version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    os: ::std::result::Result<::std::string::String, ::std::string::String>,
    source: ::std::result::Result<super::ImageSource, ::std::string::String>,
    version: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for ImageCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
            os: Err("no value supplied for os".to_string()),
            source: Err("no value supplied for source".to_string()),
            version: Err("no value supplied for version".to_string()),
        }
    }
}

impl ImageCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn os<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.os = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for os: {e}"));
        self
    }

    pub fn source<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ImageSource>,
        T::Error: ::std::fmt::Display,
    {
        self.source = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for source: {e}"));
        self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for version: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ImageCreate> for super::ImageCreate {
    type Error = super::error::ConversionError;
    fn try_from(value: ImageCreate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
            os: value.os?,
            source: value.source?,
            version: value.version?,
        })
    }
}

impl ::std::convert::From<super::ImageCreate> for ImageCreate {
    fn from(value: super::ImageCreate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
            os: Ok(value.os),
            source: Ok(value.source),
            version: Ok(value.version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Image>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for ImageResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl ImageResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Image>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ImageResultsPage> for super::ImageResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ImageResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::ImageResultsPage> for ImageResultsPage {
    fn from(value: super::ImageResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImportBlocksBulkWrite {
    base64_encoded_data: ::std::result::Result<::std::string::String, ::std::string::String>,
    offset: ::std::result::Result<u64, ::std::string::String>,
}

impl ::std::default::Default for ImportBlocksBulkWrite {
    fn default() -> Self {
        Self {
            base64_encoded_data: Err("no value supplied for base64_encoded_data".to_string()),
            offset: Err("no value supplied for offset".to_string()),
        }
    }
}

impl ImportBlocksBulkWrite {
    pub fn base64_encoded_data<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.base64_encoded_data = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for base64_encoded_data: {e}"));
        self
    }

    pub fn offset<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.offset = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for offset: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ImportBlocksBulkWrite> for super::ImportBlocksBulkWrite {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ImportBlocksBulkWrite,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            base64_encoded_data: value.base64_encoded_data?,
            offset: value.offset?,
        })
    }
}

impl ::std::convert::From<super::ImportBlocksBulkWrite> for ImportBlocksBulkWrite {
    fn from(value: super::ImportBlocksBulkWrite) -> Self {
        Self {
            base64_encoded_data: Ok(value.base64_encoded_data),
            offset: Ok(value.offset),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instance {
    auto_restart_cooldown_expiration: ::std::result::Result<
        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ::std::string::String,
    >,
    auto_restart_enabled: ::std::result::Result<bool, ::std::string::String>,
    auto_restart_policy: ::std::result::Result<
        ::std::option::Option<super::InstanceAutoRestartPolicy>,
        ::std::string::String,
    >,
    boot_disk_id: ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    cpu_platform: ::std::result::Result<
        ::std::option::Option<super::InstanceCpuPlatform>,
        ::std::string::String,
    >,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    enable_jumbo_frames: ::std::result::Result<bool, ::std::string::String>,
    hostname: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    memory: ::std::result::Result<super::ByteCount, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    ncpus: ::std::result::Result<super::InstanceCpuCount, ::std::string::String>,
    project_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    run_state: ::std::result::Result<super::InstanceState, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_last_auto_restarted: ::std::result::Result<
        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ::std::string::String,
    >,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_run_state_updated:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Instance {
    fn default() -> Self {
        Self {
            auto_restart_cooldown_expiration: Ok(Default::default()),
            auto_restart_enabled: Err("no value supplied for auto_restart_enabled".to_string()),
            auto_restart_policy: Ok(Default::default()),
            boot_disk_id: Ok(Default::default()),
            cpu_platform: Ok(Default::default()),
            description: Err("no value supplied for description".to_string()),
            enable_jumbo_frames: Err("no value supplied for enable_jumbo_frames".to_string()),
            hostname: Err("no value supplied for hostname".to_string()),
            id: Err("no value supplied for id".to_string()),
            memory: Err("no value supplied for memory".to_string()),
            name: Err("no value supplied for name".to_string()),
            ncpus: Err("no value supplied for ncpus".to_string()),
            project_id: Err("no value supplied for project_id".to_string()),
            run_state: Err("no value supplied for run_state".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_last_auto_restarted: Ok(Default::default()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            time_run_state_updated: Err("no value supplied for time_run_state_updated".to_string()),
        }
    }
}

impl Instance {
    pub fn auto_restart_cooldown_expiration<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.auto_restart_cooldown_expiration = value.try_into().map_err(|e| {
            format!("error converting supplied value for auto_restart_cooldown_expiration: {e}")
        });
        self
    }

    pub fn auto_restart_enabled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.auto_restart_enabled = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for auto_restart_enabled: {e}"));
        self
    }

    pub fn auto_restart_policy<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::InstanceAutoRestartPolicy>>,
        T::Error: ::std::fmt::Display,
    {
        self.auto_restart_policy = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for auto_restart_policy: {e}"));
        self
    }

    pub fn boot_disk_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.boot_disk_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for boot_disk_id: {e}"));
        self
    }

    pub fn cpu_platform<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::InstanceCpuPlatform>>,
        T::Error: ::std::fmt::Display,
    {
        self.cpu_platform = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cpu_platform: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn enable_jumbo_frames<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.enable_jumbo_frames = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for enable_jumbo_frames: {e}"));
        self
    }

    pub fn hostname<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.hostname = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for hostname: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn memory<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.memory = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for memory: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn ncpus<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::InstanceCpuCount>,
        T::Error: ::std::fmt::Display,
    {
        self.ncpus = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ncpus: {e}"));
        self
    }

    pub fn project_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.project_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_id: {e}"));
        self
    }

    pub fn run_state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::InstanceState>,
        T::Error: ::std::fmt::Display,
    {
        self.run_state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for run_state: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_last_auto_restarted<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.time_last_auto_restarted = value.try_into().map_err(|e| {
            format!("error converting supplied value for time_last_auto_restarted: {e}")
        });
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn time_run_state_updated<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_run_state_updated = value.try_into().map_err(|e| {
            format!("error converting supplied value for time_run_state_updated: {e}")
        });
        self
    }
}

impl ::std::convert::TryFrom<Instance> for super::Instance {
    type Error = super::error::ConversionError;
    fn try_from(value: Instance) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            auto_restart_cooldown_expiration: value.auto_restart_cooldown_expiration?,
            auto_restart_enabled: value.auto_restart_enabled?,
            auto_restart_policy: value.auto_restart_policy?,
            boot_disk_id: value.boot_disk_id?,
            cpu_platform: value.cpu_platform?,
            description: value.description?,
            enable_jumbo_frames: value.enable_jumbo_frames?,
            hostname: value.hostname?,
            id: value.id?,
            memory: value.memory?,
            name: value.name?,
            ncpus: value.ncpus?,
            project_id: value.project_id?,
            run_state: value.run_state?,
            time_created: value.time_created?,
            time_last_auto_restarted: value.time_last_auto_restarted?,
            time_modified: value.time_modified?,
            time_run_state_updated: value.time_run_state_updated?,
        })
    }
}

impl ::std::convert::From<super::Instance> for Instance {
    fn from(value: super::Instance) -> Self {
        Self {
            auto_restart_cooldown_expiration: Ok(value.auto_restart_cooldown_expiration),
            auto_restart_enabled: Ok(value.auto_restart_enabled),
            auto_restart_policy: Ok(value.auto_restart_policy),
            boot_disk_id: Ok(value.boot_disk_id),
            cpu_platform: Ok(value.cpu_platform),
            description: Ok(value.description),
            enable_jumbo_frames: Ok(value.enable_jumbo_frames),
            hostname: Ok(value.hostname),
            id: Ok(value.id),
            memory: Ok(value.memory),
            name: Ok(value.name),
            ncpus: Ok(value.ncpus),
            project_id: Ok(value.project_id),
            run_state: Ok(value.run_state),
            time_created: Ok(value.time_created),
            time_last_auto_restarted: Ok(value.time_last_auto_restarted),
            time_modified: Ok(value.time_modified),
            time_run_state_updated: Ok(value.time_run_state_updated),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceCreate {
    anti_affinity_groups:
        ::std::result::Result<::std::vec::Vec<super::NameOrId>, ::std::string::String>,
    auto_restart_policy: ::std::result::Result<
        ::std::option::Option<super::InstanceAutoRestartPolicy>,
        ::std::string::String,
    >,
    boot_disk: ::std::result::Result<
        ::std::option::Option<super::InstanceDiskAttachment>,
        ::std::string::String,
    >,
    cpu_platform: ::std::result::Result<
        ::std::option::Option<super::InstanceCpuPlatform>,
        ::std::string::String,
    >,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    disks: ::std::result::Result<
        ::std::vec::Vec<super::InstanceDiskAttachment>,
        ::std::string::String,
    >,
    enable_jumbo_frames: ::std::result::Result<bool, ::std::string::String>,
    external_ips:
        ::std::result::Result<::std::vec::Vec<super::ExternalIpCreate>, ::std::string::String>,
    hostname: ::std::result::Result<super::Hostname, ::std::string::String>,
    memory: ::std::result::Result<super::ByteCount, ::std::string::String>,
    multicast_groups: ::std::result::Result<
        ::std::vec::Vec<super::MulticastGroupJoinSpec>,
        ::std::string::String,
    >,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    ncpus: ::std::result::Result<super::InstanceCpuCount, ::std::string::String>,
    network_interfaces:
        ::std::result::Result<super::InstanceNetworkInterfaceAttachment, ::std::string::String>,
    ssh_public_keys: ::std::result::Result<
        ::std::option::Option<::std::vec::Vec<super::NameOrId>>,
        ::std::string::String,
    >,
    start: ::std::result::Result<bool, ::std::string::String>,
    user_data: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for InstanceCreate {
    fn default() -> Self {
        Self {
            anti_affinity_groups: Ok(Default::default()),
            auto_restart_policy: Ok(Default::default()),
            boot_disk: Ok(Default::default()),
            cpu_platform: Ok(Default::default()),
            description: Err("no value supplied for description".to_string()),
            disks: Ok(Default::default()),
            enable_jumbo_frames: Ok(Default::default()),
            external_ips: Ok(Default::default()),
            hostname: Err("no value supplied for hostname".to_string()),
            memory: Err("no value supplied for memory".to_string()),
            multicast_groups: Ok(Default::default()),
            name: Err("no value supplied for name".to_string()),
            ncpus: Err("no value supplied for ncpus".to_string()),
            network_interfaces: Ok(super::defaults::instance_create_network_interfaces()),
            ssh_public_keys: Ok(Default::default()),
            start: Ok(super::defaults::default_bool::<true>()),
            user_data: Ok(Default::default()),
        }
    }
}

impl InstanceCreate {
    pub fn anti_affinity_groups<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::NameOrId>>,
        T::Error: ::std::fmt::Display,
    {
        self.anti_affinity_groups = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for anti_affinity_groups: {e}"));
        self
    }

    pub fn auto_restart_policy<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::InstanceAutoRestartPolicy>>,
        T::Error: ::std::fmt::Display,
    {
        self.auto_restart_policy = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for auto_restart_policy: {e}"));
        self
    }

    pub fn boot_disk<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::InstanceDiskAttachment>>,
        T::Error: ::std::fmt::Display,
    {
        self.boot_disk = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for boot_disk: {e}"));
        self
    }

    pub fn cpu_platform<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::InstanceCpuPlatform>>,
        T::Error: ::std::fmt::Display,
    {
        self.cpu_platform = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cpu_platform: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn disks<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::InstanceDiskAttachment>>,
        T::Error: ::std::fmt::Display,
    {
        self.disks = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for disks: {e}"));
        self
    }

    pub fn enable_jumbo_frames<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.enable_jumbo_frames = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for enable_jumbo_frames: {e}"));
        self
    }

    pub fn external_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::ExternalIpCreate>>,
        T::Error: ::std::fmt::Display,
    {
        self.external_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for external_ips: {e}"));
        self
    }

    pub fn hostname<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Hostname>,
        T::Error: ::std::fmt::Display,
    {
        self.hostname = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for hostname: {e}"));
        self
    }

    pub fn memory<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.memory = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for memory: {e}"));
        self
    }

    pub fn multicast_groups<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::MulticastGroupJoinSpec>>,
        T::Error: ::std::fmt::Display,
    {
        self.multicast_groups = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for multicast_groups: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn ncpus<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::InstanceCpuCount>,
        T::Error: ::std::fmt::Display,
    {
        self.ncpus = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ncpus: {e}"));
        self
    }

    pub fn network_interfaces<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::InstanceNetworkInterfaceAttachment>,
        T::Error: ::std::fmt::Display,
    {
        self.network_interfaces = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for network_interfaces: {e}"));
        self
    }

    pub fn ssh_public_keys<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::NameOrId>>>,
        T::Error: ::std::fmt::Display,
    {
        self.ssh_public_keys = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ssh_public_keys: {e}"));
        self
    }

    pub fn start<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.start = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start: {e}"));
        self
    }

    pub fn user_data<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.user_data = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for user_data: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InstanceCreate> for super::InstanceCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InstanceCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            anti_affinity_groups: value.anti_affinity_groups?,
            auto_restart_policy: value.auto_restart_policy?,
            boot_disk: value.boot_disk?,
            cpu_platform: value.cpu_platform?,
            description: value.description?,
            disks: value.disks?,
            enable_jumbo_frames: value.enable_jumbo_frames?,
            external_ips: value.external_ips?,
            hostname: value.hostname?,
            memory: value.memory?,
            multicast_groups: value.multicast_groups?,
            name: value.name?,
            ncpus: value.ncpus?,
            network_interfaces: value.network_interfaces?,
            ssh_public_keys: value.ssh_public_keys?,
            start: value.start?,
            user_data: value.user_data?,
        })
    }
}

impl ::std::convert::From<super::InstanceCreate> for InstanceCreate {
    fn from(value: super::InstanceCreate) -> Self {
        Self {
            anti_affinity_groups: Ok(value.anti_affinity_groups),
            auto_restart_policy: Ok(value.auto_restart_policy),
            boot_disk: Ok(value.boot_disk),
            cpu_platform: Ok(value.cpu_platform),
            description: Ok(value.description),
            disks: Ok(value.disks),
            enable_jumbo_frames: Ok(value.enable_jumbo_frames),
            external_ips: Ok(value.external_ips),
            hostname: Ok(value.hostname),
            memory: Ok(value.memory),
            multicast_groups: Ok(value.multicast_groups),
            name: Ok(value.name),
            ncpus: Ok(value.ncpus),
            network_interfaces: Ok(value.network_interfaces),
            ssh_public_keys: Ok(value.ssh_public_keys),
            start: Ok(value.start),
            user_data: Ok(value.user_data),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceMulticastGroupJoin {
    ip_version:
        ::std::result::Result<::std::option::Option<super::IpVersion>, ::std::string::String>,
    source_ips: ::std::result::Result<
        ::std::option::Option<::std::vec::Vec<::std::net::IpAddr>>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for InstanceMulticastGroupJoin {
    fn default() -> Self {
        Self {
            ip_version: Ok(Default::default()),
            source_ips: Ok(Default::default()),
        }
    }
}

impl InstanceMulticastGroupJoin {
    pub fn ip_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::IpVersion>>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_version: {e}"));
        self
    }

    pub fn source_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<::std::net::IpAddr>>>,
        T::Error: ::std::fmt::Display,
    {
        self.source_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for source_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InstanceMulticastGroupJoin> for super::InstanceMulticastGroupJoin {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InstanceMulticastGroupJoin,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            ip_version: value.ip_version?,
            source_ips: value.source_ips?,
        })
    }
}

impl ::std::convert::From<super::InstanceMulticastGroupJoin> for InstanceMulticastGroupJoin {
    fn from(value: super::InstanceMulticastGroupJoin) -> Self {
        Self {
            ip_version: Ok(value.ip_version),
            source_ips: Ok(value.source_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceNetworkInterface {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    instance_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ip_stack: ::std::result::Result<super::PrivateIpStack, ::std::string::String>,
    mac: ::std::result::Result<super::MacAddr, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    primary: ::std::result::Result<bool, ::std::string::String>,
    subnet_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    vpc_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for InstanceNetworkInterface {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            instance_id: Err("no value supplied for instance_id".to_string()),
            ip_stack: Err("no value supplied for ip_stack".to_string()),
            mac: Err("no value supplied for mac".to_string()),
            name: Err("no value supplied for name".to_string()),
            primary: Err("no value supplied for primary".to_string()),
            subnet_id: Err("no value supplied for subnet_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            vpc_id: Err("no value supplied for vpc_id".to_string()),
        }
    }
}

impl InstanceNetworkInterface {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn instance_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.instance_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for instance_id: {e}"));
        self
    }

    pub fn ip_stack<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PrivateIpStack>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_stack = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_stack: {e}"));
        self
    }

    pub fn mac<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::MacAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.mac = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for mac: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn primary<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.primary = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for primary: {e}"));
        self
    }

    pub fn subnet_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn vpc_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.vpc_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vpc_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InstanceNetworkInterface> for super::InstanceNetworkInterface {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InstanceNetworkInterface,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            instance_id: value.instance_id?,
            ip_stack: value.ip_stack?,
            mac: value.mac?,
            name: value.name?,
            primary: value.primary?,
            subnet_id: value.subnet_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            vpc_id: value.vpc_id?,
        })
    }
}

impl ::std::convert::From<super::InstanceNetworkInterface> for InstanceNetworkInterface {
    fn from(value: super::InstanceNetworkInterface) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            instance_id: Ok(value.instance_id),
            ip_stack: Ok(value.ip_stack),
            mac: Ok(value.mac),
            name: Ok(value.name),
            primary: Ok(value.primary),
            subnet_id: Ok(value.subnet_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            vpc_id: Ok(value.vpc_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceNetworkInterfaceCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    ip_config: ::std::result::Result<super::PrivateIpStackCreate, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    subnet_name: ::std::result::Result<super::Name, ::std::string::String>,
    vpc_name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for InstanceNetworkInterfaceCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            ip_config: Ok(super::defaults::instance_network_interface_create_ip_config()),
            name: Err("no value supplied for name".to_string()),
            subnet_name: Err("no value supplied for subnet_name".to_string()),
            vpc_name: Err("no value supplied for vpc_name".to_string()),
        }
    }
}

impl InstanceNetworkInterfaceCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn ip_config<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PrivateIpStackCreate>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_config = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_config: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn subnet_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet_name: {e}"));
        self
    }

    pub fn vpc_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.vpc_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vpc_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InstanceNetworkInterfaceCreate>
    for super::InstanceNetworkInterfaceCreate
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: InstanceNetworkInterfaceCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            ip_config: value.ip_config?,
            name: value.name?,
            subnet_name: value.subnet_name?,
            vpc_name: value.vpc_name?,
        })
    }
}

impl ::std::convert::From<super::InstanceNetworkInterfaceCreate>
    for InstanceNetworkInterfaceCreate
{
    fn from(value: super::InstanceNetworkInterfaceCreate) -> Self {
        Self {
            description: Ok(value.description),
            ip_config: Ok(value.ip_config),
            name: Ok(value.name),
            subnet_name: Ok(value.subnet_name),
            vpc_name: Ok(value.vpc_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceNetworkInterfaceResultsPage {
    items: ::std::result::Result<
        ::std::vec::Vec<super::InstanceNetworkInterface>,
        ::std::string::String,
    >,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for InstanceNetworkInterfaceResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl InstanceNetworkInterfaceResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::InstanceNetworkInterface>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InstanceNetworkInterfaceResultsPage>
    for super::InstanceNetworkInterfaceResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: InstanceNetworkInterfaceResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::InstanceNetworkInterfaceResultsPage>
    for InstanceNetworkInterfaceResultsPage
{
    fn from(value: super::InstanceNetworkInterfaceResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceNetworkInterfaceUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
    primary: ::std::result::Result<bool, ::std::string::String>,
    transit_ips: ::std::result::Result<::std::vec::Vec<super::IpNet>, ::std::string::String>,
}

impl ::std::default::Default for InstanceNetworkInterfaceUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            name: Ok(Default::default()),
            primary: Ok(Default::default()),
            transit_ips: Ok(Default::default()),
        }
    }
}

impl InstanceNetworkInterfaceUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn primary<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.primary = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for primary: {e}"));
        self
    }

    pub fn transit_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::IpNet>>,
        T::Error: ::std::fmt::Display,
    {
        self.transit_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for transit_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InstanceNetworkInterfaceUpdate>
    for super::InstanceNetworkInterfaceUpdate
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: InstanceNetworkInterfaceUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
            primary: value.primary?,
            transit_ips: value.transit_ips?,
        })
    }
}

impl ::std::convert::From<super::InstanceNetworkInterfaceUpdate>
    for InstanceNetworkInterfaceUpdate
{
    fn from(value: super::InstanceNetworkInterfaceUpdate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
            primary: Ok(value.primary),
            transit_ips: Ok(value.transit_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Instance>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for InstanceResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl InstanceResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Instance>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InstanceResultsPage> for super::InstanceResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InstanceResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::InstanceResultsPage> for InstanceResultsPage {
    fn from(value: super::InstanceResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceSerialConsoleData {
    data: ::std::result::Result<::std::vec::Vec<u8>, ::std::string::String>,
    last_byte_offset: ::std::result::Result<u64, ::std::string::String>,
}

impl ::std::default::Default for InstanceSerialConsoleData {
    fn default() -> Self {
        Self {
            data: Err("no value supplied for data".to_string()),
            last_byte_offset: Err("no value supplied for last_byte_offset".to_string()),
        }
    }
}

impl InstanceSerialConsoleData {
    pub fn data<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<u8>>,
        T::Error: ::std::fmt::Display,
    {
        self.data = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for data: {e}"));
        self
    }

    pub fn last_byte_offset<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u64>,
        T::Error: ::std::fmt::Display,
    {
        self.last_byte_offset = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for last_byte_offset: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InstanceSerialConsoleData> for super::InstanceSerialConsoleData {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InstanceSerialConsoleData,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            data: value.data?,
            last_byte_offset: value.last_byte_offset?,
        })
    }
}

impl ::std::convert::From<super::InstanceSerialConsoleData> for InstanceSerialConsoleData {
    fn from(value: super::InstanceSerialConsoleData) -> Self {
        Self {
            data: Ok(value.data),
            last_byte_offset: Ok(value.last_byte_offset),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceUpdate {
    auto_restart_policy: ::std::result::Result<
        ::std::option::Option<super::InstanceAutoRestartPolicy>,
        ::std::string::String,
    >,
    boot_disk: ::std::result::Result<::std::option::Option<super::NameOrId>, ::std::string::String>,
    cpu_platform: ::std::result::Result<
        ::std::option::Option<super::InstanceCpuPlatform>,
        ::std::string::String,
    >,
    enable_jumbo_frames: ::std::result::Result<bool, ::std::string::String>,
    memory: ::std::result::Result<super::ByteCount, ::std::string::String>,
    multicast_groups: ::std::result::Result<
        ::std::option::Option<::std::vec::Vec<super::MulticastGroupJoinSpec>>,
        ::std::string::String,
    >,
    ncpus: ::std::result::Result<super::InstanceCpuCount, ::std::string::String>,
}

impl ::std::default::Default for InstanceUpdate {
    fn default() -> Self {
        Self {
            auto_restart_policy: Err("no value supplied for auto_restart_policy".to_string()),
            boot_disk: Err("no value supplied for boot_disk".to_string()),
            cpu_platform: Err("no value supplied for cpu_platform".to_string()),
            enable_jumbo_frames: Err("no value supplied for enable_jumbo_frames".to_string()),
            memory: Err("no value supplied for memory".to_string()),
            multicast_groups: Ok(Default::default()),
            ncpus: Err("no value supplied for ncpus".to_string()),
        }
    }
}

impl InstanceUpdate {
    pub fn auto_restart_policy<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::InstanceAutoRestartPolicy>>,
        T::Error: ::std::fmt::Display,
    {
        self.auto_restart_policy = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for auto_restart_policy: {e}"));
        self
    }

    pub fn boot_disk<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::NameOrId>>,
        T::Error: ::std::fmt::Display,
    {
        self.boot_disk = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for boot_disk: {e}"));
        self
    }

    pub fn cpu_platform<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::InstanceCpuPlatform>>,
        T::Error: ::std::fmt::Display,
    {
        self.cpu_platform = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cpu_platform: {e}"));
        self
    }

    pub fn enable_jumbo_frames<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.enable_jumbo_frames = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for enable_jumbo_frames: {e}"));
        self
    }

    pub fn memory<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.memory = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for memory: {e}"));
        self
    }

    pub fn multicast_groups<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::std::vec::Vec<super::MulticastGroupJoinSpec>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.multicast_groups = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for multicast_groups: {e}"));
        self
    }

    pub fn ncpus<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::InstanceCpuCount>,
        T::Error: ::std::fmt::Display,
    {
        self.ncpus = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ncpus: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InstanceUpdate> for super::InstanceUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InstanceUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            auto_restart_policy: value.auto_restart_policy?,
            boot_disk: value.boot_disk?,
            cpu_platform: value.cpu_platform?,
            enable_jumbo_frames: value.enable_jumbo_frames?,
            memory: value.memory?,
            multicast_groups: value.multicast_groups?,
            ncpus: value.ncpus?,
        })
    }
}

impl ::std::convert::From<super::InstanceUpdate> for InstanceUpdate {
    fn from(value: super::InstanceUpdate) -> Self {
        Self {
            auto_restart_policy: Ok(value.auto_restart_policy),
            boot_disk: Ok(value.boot_disk),
            cpu_platform: Ok(value.cpu_platform),
            enable_jumbo_frames: Ok(value.enable_jumbo_frames),
            memory: Ok(value.memory),
            multicast_groups: Ok(value.multicast_groups),
            ncpus: Ok(value.ncpus),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InternetGateway {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    vpc_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for InternetGateway {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            vpc_id: Err("no value supplied for vpc_id".to_string()),
        }
    }
}

impl InternetGateway {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn vpc_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.vpc_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vpc_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InternetGateway> for super::InternetGateway {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InternetGateway,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            vpc_id: value.vpc_id?,
        })
    }
}

impl ::std::convert::From<super::InternetGateway> for InternetGateway {
    fn from(value: super::InternetGateway) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            vpc_id: Ok(value.vpc_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InternetGatewayCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for InternetGatewayCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl InternetGatewayCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InternetGatewayCreate> for super::InternetGatewayCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InternetGatewayCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::InternetGatewayCreate> for InternetGatewayCreate {
    fn from(value: super::InternetGatewayCreate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InternetGatewayIpAddress {
    address: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    internet_gateway_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for InternetGatewayIpAddress {
    fn default() -> Self {
        Self {
            address: Err("no value supplied for address".to_string()),
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            internet_gateway_id: Err("no value supplied for internet_gateway_id".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl InternetGatewayIpAddress {
    pub fn address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn internet_gateway_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.internet_gateway_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for internet_gateway_id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InternetGatewayIpAddress> for super::InternetGatewayIpAddress {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InternetGatewayIpAddress,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            address: value.address?,
            description: value.description?,
            id: value.id?,
            internet_gateway_id: value.internet_gateway_id?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::InternetGatewayIpAddress> for InternetGatewayIpAddress {
    fn from(value: super::InternetGatewayIpAddress) -> Self {
        Self {
            address: Ok(value.address),
            description: Ok(value.description),
            id: Ok(value.id),
            internet_gateway_id: Ok(value.internet_gateway_id),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InternetGatewayIpAddressCreate {
    address: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for InternetGatewayIpAddressCreate {
    fn default() -> Self {
        Self {
            address: Err("no value supplied for address".to_string()),
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl InternetGatewayIpAddressCreate {
    pub fn address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InternetGatewayIpAddressCreate>
    for super::InternetGatewayIpAddressCreate
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: InternetGatewayIpAddressCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            address: value.address?,
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::InternetGatewayIpAddressCreate>
    for InternetGatewayIpAddressCreate
{
    fn from(value: super::InternetGatewayIpAddressCreate) -> Self {
        Self {
            address: Ok(value.address),
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InternetGatewayIpAddressResultsPage {
    items: ::std::result::Result<
        ::std::vec::Vec<super::InternetGatewayIpAddress>,
        ::std::string::String,
    >,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for InternetGatewayIpAddressResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl InternetGatewayIpAddressResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::InternetGatewayIpAddress>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InternetGatewayIpAddressResultsPage>
    for super::InternetGatewayIpAddressResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: InternetGatewayIpAddressResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::InternetGatewayIpAddressResultsPage>
    for InternetGatewayIpAddressResultsPage
{
    fn from(value: super::InternetGatewayIpAddressResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InternetGatewayIpPool {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    internet_gateway_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ip_pool_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for InternetGatewayIpPool {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            internet_gateway_id: Err("no value supplied for internet_gateway_id".to_string()),
            ip_pool_id: Err("no value supplied for ip_pool_id".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl InternetGatewayIpPool {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn internet_gateway_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.internet_gateway_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for internet_gateway_id: {e}"));
        self
    }

    pub fn ip_pool_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_pool_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_pool_id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InternetGatewayIpPool> for super::InternetGatewayIpPool {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InternetGatewayIpPool,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            internet_gateway_id: value.internet_gateway_id?,
            ip_pool_id: value.ip_pool_id?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::InternetGatewayIpPool> for InternetGatewayIpPool {
    fn from(value: super::InternetGatewayIpPool) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            internet_gateway_id: Ok(value.internet_gateway_id),
            ip_pool_id: Ok(value.ip_pool_id),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InternetGatewayIpPoolCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    ip_pool: ::std::result::Result<super::NameOrId, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for InternetGatewayIpPoolCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            ip_pool: Err("no value supplied for ip_pool".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl InternetGatewayIpPoolCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn ip_pool<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_pool = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_pool: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InternetGatewayIpPoolCreate> for super::InternetGatewayIpPoolCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InternetGatewayIpPoolCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            ip_pool: value.ip_pool?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::InternetGatewayIpPoolCreate> for InternetGatewayIpPoolCreate {
    fn from(value: super::InternetGatewayIpPoolCreate) -> Self {
        Self {
            description: Ok(value.description),
            ip_pool: Ok(value.ip_pool),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InternetGatewayIpPoolResultsPage {
    items:
        ::std::result::Result<::std::vec::Vec<super::InternetGatewayIpPool>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for InternetGatewayIpPoolResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl InternetGatewayIpPoolResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::InternetGatewayIpPool>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InternetGatewayIpPoolResultsPage>
    for super::InternetGatewayIpPoolResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: InternetGatewayIpPoolResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::InternetGatewayIpPoolResultsPage>
    for InternetGatewayIpPoolResultsPage
{
    fn from(value: super::InternetGatewayIpPoolResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InternetGatewayResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::InternetGateway>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for InternetGatewayResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl InternetGatewayResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::InternetGateway>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<InternetGatewayResultsPage> for super::InternetGatewayResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: InternetGatewayResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::InternetGatewayResultsPage> for InternetGatewayResultsPage {
    fn from(value: super::InternetGatewayResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPool {
    assignment: ::std::result::Result<super::IpPoolAssignment, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ip_version: ::std::result::Result<super::IpVersion, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    pool_type: ::std::result::Result<super::IpPoolType, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for IpPool {
    fn default() -> Self {
        Self {
            assignment: Err("no value supplied for assignment".to_string()),
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            ip_version: Err("no value supplied for ip_version".to_string()),
            name: Err("no value supplied for name".to_string()),
            pool_type: Err("no value supplied for pool_type".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl IpPool {
    pub fn assignment<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpPoolAssignment>,
        T::Error: ::std::fmt::Display,
    {
        self.assignment = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for assignment: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn ip_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_version: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn pool_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpPoolType>,
        T::Error: ::std::fmt::Display,
    {
        self.pool_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for pool_type: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPool> for super::IpPool {
    type Error = super::error::ConversionError;
    fn try_from(value: IpPool) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            assignment: value.assignment?,
            description: value.description?,
            id: value.id?,
            ip_version: value.ip_version?,
            name: value.name?,
            pool_type: value.pool_type?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::IpPool> for IpPool {
    fn from(value: super::IpPool) -> Self {
        Self {
            assignment: Ok(value.assignment),
            description: Ok(value.description),
            id: Ok(value.id),
            ip_version: Ok(value.ip_version),
            name: Ok(value.name),
            pool_type: Ok(value.pool_type),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolAssignParam {
    assignment: ::std::result::Result<super::IpPoolAssignment, ::std::string::String>,
}

impl ::std::default::Default for IpPoolAssignParam {
    fn default() -> Self {
        Self {
            assignment: Err("no value supplied for assignment".to_string()),
        }
    }
}

impl IpPoolAssignParam {
    pub fn assignment<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpPoolAssignment>,
        T::Error: ::std::fmt::Display,
    {
        self.assignment = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for assignment: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolAssignParam> for super::IpPoolAssignParam {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IpPoolAssignParam,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            assignment: value.assignment?,
        })
    }
}

impl ::std::convert::From<super::IpPoolAssignParam> for IpPoolAssignParam {
    fn from(value: super::IpPoolAssignParam) -> Self {
        Self {
            assignment: Ok(value.assignment),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolCreate {
    assignment: ::std::result::Result<super::IpPoolAssignment, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    ip_version: ::std::result::Result<super::IpVersion, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    pool_type: ::std::result::Result<super::IpPoolType, ::std::string::String>,
}

impl ::std::default::Default for IpPoolCreate {
    fn default() -> Self {
        Self {
            assignment: Ok(super::defaults::ip_pool_create_assignment()),
            description: Err("no value supplied for description".to_string()),
            ip_version: Ok(super::defaults::ip_pool_create_ip_version()),
            name: Err("no value supplied for name".to_string()),
            pool_type: Ok(super::defaults::ip_pool_create_pool_type()),
        }
    }
}

impl IpPoolCreate {
    pub fn assignment<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpPoolAssignment>,
        T::Error: ::std::fmt::Display,
    {
        self.assignment = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for assignment: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn ip_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_version: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn pool_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpPoolType>,
        T::Error: ::std::fmt::Display,
    {
        self.pool_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for pool_type: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolCreate> for super::IpPoolCreate {
    type Error = super::error::ConversionError;
    fn try_from(value: IpPoolCreate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            assignment: value.assignment?,
            description: value.description?,
            ip_version: value.ip_version?,
            name: value.name?,
            pool_type: value.pool_type?,
        })
    }
}

impl ::std::convert::From<super::IpPoolCreate> for IpPoolCreate {
    fn from(value: super::IpPoolCreate) -> Self {
        Self {
            assignment: Ok(value.assignment),
            description: Ok(value.description),
            ip_version: Ok(value.ip_version),
            name: Ok(value.name),
            pool_type: Ok(value.pool_type),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolLinkSilo {
    is_default: ::std::result::Result<bool, ::std::string::String>,
    silo: ::std::result::Result<super::NameOrId, ::std::string::String>,
}

impl ::std::default::Default for IpPoolLinkSilo {
    fn default() -> Self {
        Self {
            is_default: Err("no value supplied for is_default".to_string()),
            silo: Err("no value supplied for silo".to_string()),
        }
    }
}

impl IpPoolLinkSilo {
    pub fn is_default<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.is_default = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for is_default: {e}"));
        self
    }

    pub fn silo<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.silo = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolLinkSilo> for super::IpPoolLinkSilo {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IpPoolLinkSilo,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            is_default: value.is_default?,
            silo: value.silo?,
        })
    }
}

impl ::std::convert::From<super::IpPoolLinkSilo> for IpPoolLinkSilo {
    fn from(value: super::IpPoolLinkSilo) -> Self {
        Self {
            is_default: Ok(value.is_default),
            silo: Ok(value.silo),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolRange {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ip_pool_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    range: ::std::result::Result<super::IpRange, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for IpPoolRange {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            ip_pool_id: Err("no value supplied for ip_pool_id".to_string()),
            range: Err("no value supplied for range".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
        }
    }
}

impl IpPoolRange {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn ip_pool_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_pool_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_pool_id: {e}"));
        self
    }

    pub fn range<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpRange>,
        T::Error: ::std::fmt::Display,
    {
        self.range = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for range: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolRange> for super::IpPoolRange {
    type Error = super::error::ConversionError;
    fn try_from(value: IpPoolRange) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            ip_pool_id: value.ip_pool_id?,
            range: value.range?,
            time_created: value.time_created?,
        })
    }
}

impl ::std::convert::From<super::IpPoolRange> for IpPoolRange {
    fn from(value: super::IpPoolRange) -> Self {
        Self {
            id: Ok(value.id),
            ip_pool_id: Ok(value.ip_pool_id),
            range: Ok(value.range),
            time_created: Ok(value.time_created),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolRangeResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::IpPoolRange>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for IpPoolRangeResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl IpPoolRangeResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::IpPoolRange>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolRangeResultsPage> for super::IpPoolRangeResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IpPoolRangeResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::IpPoolRangeResultsPage> for IpPoolRangeResultsPage {
    fn from(value: super::IpPoolRangeResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::IpPool>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for IpPoolResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl IpPoolResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::IpPool>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolResultsPage> for super::IpPoolResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IpPoolResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::IpPoolResultsPage> for IpPoolResultsPage {
    fn from(value: super::IpPoolResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolSiloLink {
    ip_pool_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    is_default: ::std::result::Result<bool, ::std::string::String>,
    silo_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for IpPoolSiloLink {
    fn default() -> Self {
        Self {
            ip_pool_id: Err("no value supplied for ip_pool_id".to_string()),
            is_default: Err("no value supplied for is_default".to_string()),
            silo_id: Err("no value supplied for silo_id".to_string()),
        }
    }
}

impl IpPoolSiloLink {
    pub fn ip_pool_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_pool_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_pool_id: {e}"));
        self
    }

    pub fn is_default<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.is_default = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for is_default: {e}"));
        self
    }

    pub fn silo_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolSiloLink> for super::IpPoolSiloLink {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IpPoolSiloLink,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            ip_pool_id: value.ip_pool_id?,
            is_default: value.is_default?,
            silo_id: value.silo_id?,
        })
    }
}

impl ::std::convert::From<super::IpPoolSiloLink> for IpPoolSiloLink {
    fn from(value: super::IpPoolSiloLink) -> Self {
        Self {
            ip_pool_id: Ok(value.ip_pool_id),
            is_default: Ok(value.is_default),
            silo_id: Ok(value.silo_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolSiloLinkResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::IpPoolSiloLink>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for IpPoolSiloLinkResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl IpPoolSiloLinkResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::IpPoolSiloLink>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolSiloLinkResultsPage> for super::IpPoolSiloLinkResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IpPoolSiloLinkResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::IpPoolSiloLinkResultsPage> for IpPoolSiloLinkResultsPage {
    fn from(value: super::IpPoolSiloLinkResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolSiloUpdate {
    is_default: ::std::result::Result<bool, ::std::string::String>,
}

impl ::std::default::Default for IpPoolSiloUpdate {
    fn default() -> Self {
        Self {
            is_default: Err("no value supplied for is_default".to_string()),
        }
    }
}

impl IpPoolSiloUpdate {
    pub fn is_default<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.is_default = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for is_default: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolSiloUpdate> for super::IpPoolSiloUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IpPoolSiloUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            is_default: value.is_default?,
        })
    }
}

impl ::std::convert::From<super::IpPoolSiloUpdate> for IpPoolSiloUpdate {
    fn from(value: super::IpPoolSiloUpdate) -> Self {
        Self {
            is_default: Ok(value.is_default),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for IpPoolUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl IpPoolUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolUpdate> for super::IpPoolUpdate {
    type Error = super::error::ConversionError;
    fn try_from(value: IpPoolUpdate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::IpPoolUpdate> for IpPoolUpdate {
    fn from(value: super::IpPoolUpdate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IpPoolUtilization {
    capacity: ::std::result::Result<f64, ::std::string::String>,
    remaining: ::std::result::Result<f64, ::std::string::String>,
}

impl ::std::default::Default for IpPoolUtilization {
    fn default() -> Self {
        Self {
            capacity: Err("no value supplied for capacity".to_string()),
            remaining: Err("no value supplied for remaining".to_string()),
        }
    }
}

impl IpPoolUtilization {
    pub fn capacity<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.capacity = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for capacity: {e}"));
        self
    }

    pub fn remaining<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.remaining = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for remaining: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<IpPoolUtilization> for super::IpPoolUtilization {
    type Error = super::error::ConversionError;
    fn try_from(
        value: IpPoolUtilization,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            capacity: value.capacity?,
            remaining: value.remaining?,
        })
    }
}

impl ::std::convert::From<super::IpPoolUtilization> for IpPoolUtilization {
    fn from(value: super::IpPoolUtilization) -> Self {
        Self {
            capacity: Ok(value.capacity),
            remaining: Ok(value.remaining),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ipv4Range {
    first: ::std::result::Result<::std::net::Ipv4Addr, ::std::string::String>,
    last: ::std::result::Result<::std::net::Ipv4Addr, ::std::string::String>,
}

impl ::std::default::Default for Ipv4Range {
    fn default() -> Self {
        Self {
            first: Err("no value supplied for first".to_string()),
            last: Err("no value supplied for last".to_string()),
        }
    }
}

impl Ipv4Range {
    pub fn first<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::Ipv4Addr>,
        T::Error: ::std::fmt::Display,
    {
        self.first = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for first: {e}"));
        self
    }

    pub fn last<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::Ipv4Addr>,
        T::Error: ::std::fmt::Display,
    {
        self.last = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for last: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Ipv4Range> for super::Ipv4Range {
    type Error = super::error::ConversionError;
    fn try_from(value: Ipv4Range) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            first: value.first?,
            last: value.last?,
        })
    }
}

impl ::std::convert::From<super::Ipv4Range> for Ipv4Range {
    fn from(value: super::Ipv4Range) -> Self {
        Self {
            first: Ok(value.first),
            last: Ok(value.last),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ipv6Range {
    first: ::std::result::Result<::std::net::Ipv6Addr, ::std::string::String>,
    last: ::std::result::Result<::std::net::Ipv6Addr, ::std::string::String>,
}

impl ::std::default::Default for Ipv6Range {
    fn default() -> Self {
        Self {
            first: Err("no value supplied for first".to_string()),
            last: Err("no value supplied for last".to_string()),
        }
    }
}

impl Ipv6Range {
    pub fn first<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::Ipv6Addr>,
        T::Error: ::std::fmt::Display,
    {
        self.first = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for first: {e}"));
        self
    }

    pub fn last<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::Ipv6Addr>,
        T::Error: ::std::fmt::Display,
    {
        self.last = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for last: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Ipv6Range> for super::Ipv6Range {
    type Error = super::error::ConversionError;
    fn try_from(value: Ipv6Range) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            first: value.first?,
            last: value.last?,
        })
    }
}

impl ::std::convert::From<super::Ipv6Range> for Ipv6Range {
    fn from(value: super::Ipv6Range) -> Self {
        Self {
            first: Ok(value.first),
            last: Ok(value.last),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LinkConfigCreate {
    autoneg: ::std::result::Result<bool, ::std::string::String>,
    fec: ::std::result::Result<::std::option::Option<super::LinkFec>, ::std::string::String>,
    link_name: ::std::result::Result<super::Name, ::std::string::String>,
    lldp: ::std::result::Result<super::LldpLinkConfigCreate, ::std::string::String>,
    mtu: ::std::result::Result<u16, ::std::string::String>,
    speed: ::std::result::Result<super::LinkSpeed, ::std::string::String>,
    tx_eq: ::std::result::Result<::std::option::Option<super::TxEqConfig>, ::std::string::String>,
}

impl ::std::default::Default for LinkConfigCreate {
    fn default() -> Self {
        Self {
            autoneg: Err("no value supplied for autoneg".to_string()),
            fec: Ok(Default::default()),
            link_name: Err("no value supplied for link_name".to_string()),
            lldp: Err("no value supplied for lldp".to_string()),
            mtu: Err("no value supplied for mtu".to_string()),
            speed: Err("no value supplied for speed".to_string()),
            tx_eq: Ok(Default::default()),
        }
    }
}

impl LinkConfigCreate {
    pub fn autoneg<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.autoneg = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for autoneg: {e}"));
        self
    }

    pub fn fec<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::LinkFec>>,
        T::Error: ::std::fmt::Display,
    {
        self.fec = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for fec: {e}"));
        self
    }

    pub fn link_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.link_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_name: {e}"));
        self
    }

    pub fn lldp<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::LldpLinkConfigCreate>,
        T::Error: ::std::fmt::Display,
    {
        self.lldp = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for lldp: {e}"));
        self
    }

    pub fn mtu<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.mtu = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for mtu: {e}"));
        self
    }

    pub fn speed<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::LinkSpeed>,
        T::Error: ::std::fmt::Display,
    {
        self.speed = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for speed: {e}"));
        self
    }

    pub fn tx_eq<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::TxEqConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.tx_eq = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for tx_eq: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<LinkConfigCreate> for super::LinkConfigCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: LinkConfigCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            autoneg: value.autoneg?,
            fec: value.fec?,
            link_name: value.link_name?,
            lldp: value.lldp?,
            mtu: value.mtu?,
            speed: value.speed?,
            tx_eq: value.tx_eq?,
        })
    }
}

impl ::std::convert::From<super::LinkConfigCreate> for LinkConfigCreate {
    fn from(value: super::LinkConfigCreate) -> Self {
        Self {
            autoneg: Ok(value.autoneg),
            fec: Ok(value.fec),
            link_name: Ok(value.link_name),
            lldp: Ok(value.lldp),
            mtu: Ok(value.mtu),
            speed: Ok(value.speed),
            tx_eq: Ok(value.tx_eq),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LldpLinkConfig {
    chassis_id:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    enabled: ::std::result::Result<bool, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    link_description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    link_name:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    management_ip:
        ::std::result::Result<::std::option::Option<::std::net::IpAddr>, ::std::string::String>,
    system_description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    system_name:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for LldpLinkConfig {
    fn default() -> Self {
        Self {
            chassis_id: Ok(Default::default()),
            enabled: Err("no value supplied for enabled".to_string()),
            id: Err("no value supplied for id".to_string()),
            link_description: Ok(Default::default()),
            link_name: Ok(Default::default()),
            management_ip: Ok(Default::default()),
            system_description: Ok(Default::default()),
            system_name: Ok(Default::default()),
        }
    }
}

impl LldpLinkConfig {
    pub fn chassis_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.chassis_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for chassis_id: {e}"));
        self
    }

    pub fn enabled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.enabled = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for enabled: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn link_description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.link_description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_description: {e}"));
        self
    }

    pub fn link_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.link_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_name: {e}"));
        self
    }

    pub fn management_ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::net::IpAddr>>,
        T::Error: ::std::fmt::Display,
    {
        self.management_ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for management_ip: {e}"));
        self
    }

    pub fn system_description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.system_description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for system_description: {e}"));
        self
    }

    pub fn system_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.system_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for system_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<LldpLinkConfig> for super::LldpLinkConfig {
    type Error = super::error::ConversionError;
    fn try_from(
        value: LldpLinkConfig,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            chassis_id: value.chassis_id?,
            enabled: value.enabled?,
            id: value.id?,
            link_description: value.link_description?,
            link_name: value.link_name?,
            management_ip: value.management_ip?,
            system_description: value.system_description?,
            system_name: value.system_name?,
        })
    }
}

impl ::std::convert::From<super::LldpLinkConfig> for LldpLinkConfig {
    fn from(value: super::LldpLinkConfig) -> Self {
        Self {
            chassis_id: Ok(value.chassis_id),
            enabled: Ok(value.enabled),
            id: Ok(value.id),
            link_description: Ok(value.link_description),
            link_name: Ok(value.link_name),
            management_ip: Ok(value.management_ip),
            system_description: Ok(value.system_description),
            system_name: Ok(value.system_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LldpLinkConfigCreate {
    chassis_id:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    enabled: ::std::result::Result<bool, ::std::string::String>,
    link_description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    link_name:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    management_ip:
        ::std::result::Result<::std::option::Option<::std::net::IpAddr>, ::std::string::String>,
    system_description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    system_name:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for LldpLinkConfigCreate {
    fn default() -> Self {
        Self {
            chassis_id: Ok(Default::default()),
            enabled: Err("no value supplied for enabled".to_string()),
            link_description: Ok(Default::default()),
            link_name: Ok(Default::default()),
            management_ip: Ok(Default::default()),
            system_description: Ok(Default::default()),
            system_name: Ok(Default::default()),
        }
    }
}

impl LldpLinkConfigCreate {
    pub fn chassis_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.chassis_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for chassis_id: {e}"));
        self
    }

    pub fn enabled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.enabled = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for enabled: {e}"));
        self
    }

    pub fn link_description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.link_description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_description: {e}"));
        self
    }

    pub fn link_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.link_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_name: {e}"));
        self
    }

    pub fn management_ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::net::IpAddr>>,
        T::Error: ::std::fmt::Display,
    {
        self.management_ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for management_ip: {e}"));
        self
    }

    pub fn system_description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.system_description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for system_description: {e}"));
        self
    }

    pub fn system_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.system_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for system_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<LldpLinkConfigCreate> for super::LldpLinkConfigCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: LldpLinkConfigCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            chassis_id: value.chassis_id?,
            enabled: value.enabled?,
            link_description: value.link_description?,
            link_name: value.link_name?,
            management_ip: value.management_ip?,
            system_description: value.system_description?,
            system_name: value.system_name?,
        })
    }
}

impl ::std::convert::From<super::LldpLinkConfigCreate> for LldpLinkConfigCreate {
    fn from(value: super::LldpLinkConfigCreate) -> Self {
        Self {
            chassis_id: Ok(value.chassis_id),
            enabled: Ok(value.enabled),
            link_description: Ok(value.link_description),
            link_name: Ok(value.link_name),
            management_ip: Ok(value.management_ip),
            system_description: Ok(value.system_description),
            system_name: Ok(value.system_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LldpNeighbor {
    chassis_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    first_seen:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    last_seen:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    link_description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    link_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    local_port: ::std::result::Result<::std::string::String, ::std::string::String>,
    management_ip:
        ::std::result::Result<::std::vec::Vec<super::ManagementAddress>, ::std::string::String>,
    system_description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    system_name:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for LldpNeighbor {
    fn default() -> Self {
        Self {
            chassis_id: Err("no value supplied for chassis_id".to_string()),
            first_seen: Err("no value supplied for first_seen".to_string()),
            last_seen: Err("no value supplied for last_seen".to_string()),
            link_description: Ok(Default::default()),
            link_name: Err("no value supplied for link_name".to_string()),
            local_port: Err("no value supplied for local_port".to_string()),
            management_ip: Err("no value supplied for management_ip".to_string()),
            system_description: Ok(Default::default()),
            system_name: Ok(Default::default()),
        }
    }
}

impl LldpNeighbor {
    pub fn chassis_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.chassis_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for chassis_id: {e}"));
        self
    }

    pub fn first_seen<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.first_seen = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for first_seen: {e}"));
        self
    }

    pub fn last_seen<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.last_seen = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for last_seen: {e}"));
        self
    }

    pub fn link_description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.link_description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_description: {e}"));
        self
    }

    pub fn link_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.link_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_name: {e}"));
        self
    }

    pub fn local_port<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.local_port = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for local_port: {e}"));
        self
    }

    pub fn management_ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::ManagementAddress>>,
        T::Error: ::std::fmt::Display,
    {
        self.management_ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for management_ip: {e}"));
        self
    }

    pub fn system_description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.system_description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for system_description: {e}"));
        self
    }

    pub fn system_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.system_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for system_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<LldpNeighbor> for super::LldpNeighbor {
    type Error = super::error::ConversionError;
    fn try_from(value: LldpNeighbor) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            chassis_id: value.chassis_id?,
            first_seen: value.first_seen?,
            last_seen: value.last_seen?,
            link_description: value.link_description?,
            link_name: value.link_name?,
            local_port: value.local_port?,
            management_ip: value.management_ip?,
            system_description: value.system_description?,
            system_name: value.system_name?,
        })
    }
}

impl ::std::convert::From<super::LldpNeighbor> for LldpNeighbor {
    fn from(value: super::LldpNeighbor) -> Self {
        Self {
            chassis_id: Ok(value.chassis_id),
            first_seen: Ok(value.first_seen),
            last_seen: Ok(value.last_seen),
            link_description: Ok(value.link_description),
            link_name: Ok(value.link_name),
            local_port: Ok(value.local_port),
            management_ip: Ok(value.management_ip),
            system_description: Ok(value.system_description),
            system_name: Ok(value.system_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LldpNeighborResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::LldpNeighbor>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for LldpNeighborResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl LldpNeighborResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::LldpNeighbor>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<LldpNeighborResultsPage> for super::LldpNeighborResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: LldpNeighborResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::LldpNeighborResultsPage> for LldpNeighborResultsPage {
    fn from(value: super::LldpNeighborResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LoopbackAddress {
    address: ::std::result::Result<super::IpNet, ::std::string::String>,
    address_lot_block_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    rack_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    switch_slot: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for LoopbackAddress {
    fn default() -> Self {
        Self {
            address: Err("no value supplied for address".to_string()),
            address_lot_block_id: Err("no value supplied for address_lot_block_id".to_string()),
            id: Err("no value supplied for id".to_string()),
            rack_id: Err("no value supplied for rack_id".to_string()),
            switch_slot: Err("no value supplied for switch_slot".to_string()),
        }
    }
}

impl LoopbackAddress {
    pub fn address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address: {e}"));
        self
    }

    pub fn address_lot_block_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.address_lot_block_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_lot_block_id: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn rack_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.rack_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rack_id: {e}"));
        self
    }

    pub fn switch_slot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch_slot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch_slot: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<LoopbackAddress> for super::LoopbackAddress {
    type Error = super::error::ConversionError;
    fn try_from(
        value: LoopbackAddress,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            address: value.address?,
            address_lot_block_id: value.address_lot_block_id?,
            id: value.id?,
            rack_id: value.rack_id?,
            switch_slot: value.switch_slot?,
        })
    }
}

impl ::std::convert::From<super::LoopbackAddress> for LoopbackAddress {
    fn from(value: super::LoopbackAddress) -> Self {
        Self {
            address: Ok(value.address),
            address_lot_block_id: Ok(value.address_lot_block_id),
            id: Ok(value.id),
            rack_id: Ok(value.rack_id),
            switch_slot: Ok(value.switch_slot),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LoopbackAddressCreate {
    address: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    address_lot: ::std::result::Result<super::NameOrId, ::std::string::String>,
    anycast: ::std::result::Result<bool, ::std::string::String>,
    mask: ::std::result::Result<u8, ::std::string::String>,
    rack_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    switch_slot: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for LoopbackAddressCreate {
    fn default() -> Self {
        Self {
            address: Err("no value supplied for address".to_string()),
            address_lot: Err("no value supplied for address_lot".to_string()),
            anycast: Err("no value supplied for anycast".to_string()),
            mask: Err("no value supplied for mask".to_string()),
            rack_id: Err("no value supplied for rack_id".to_string()),
            switch_slot: Err("no value supplied for switch_slot".to_string()),
        }
    }
}

impl LoopbackAddressCreate {
    pub fn address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address: {e}"));
        self
    }

    pub fn address_lot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.address_lot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_lot: {e}"));
        self
    }

    pub fn anycast<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.anycast = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for anycast: {e}"));
        self
    }

    pub fn mask<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u8>,
        T::Error: ::std::fmt::Display,
    {
        self.mask = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for mask: {e}"));
        self
    }

    pub fn rack_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.rack_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rack_id: {e}"));
        self
    }

    pub fn switch_slot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch_slot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch_slot: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<LoopbackAddressCreate> for super::LoopbackAddressCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: LoopbackAddressCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            address: value.address?,
            address_lot: value.address_lot?,
            anycast: value.anycast?,
            mask: value.mask?,
            rack_id: value.rack_id?,
            switch_slot: value.switch_slot?,
        })
    }
}

impl ::std::convert::From<super::LoopbackAddressCreate> for LoopbackAddressCreate {
    fn from(value: super::LoopbackAddressCreate) -> Self {
        Self {
            address: Ok(value.address),
            address_lot: Ok(value.address_lot),
            anycast: Ok(value.anycast),
            mask: Ok(value.mask),
            rack_id: Ok(value.rack_id),
            switch_slot: Ok(value.switch_slot),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LoopbackAddressResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::LoopbackAddress>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for LoopbackAddressResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl LoopbackAddressResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::LoopbackAddress>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<LoopbackAddressResultsPage> for super::LoopbackAddressResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: LoopbackAddressResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::LoopbackAddressResultsPage> for LoopbackAddressResultsPage {
    fn from(value: super::LoopbackAddressResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ManagementAddress {
    addr: ::std::result::Result<super::NetworkAddress, ::std::string::String>,
    interface_num: ::std::result::Result<super::InterfaceNum, ::std::string::String>,
    oid: ::std::result::Result<::std::option::Option<::std::vec::Vec<u8>>, ::std::string::String>,
}

impl ::std::default::Default for ManagementAddress {
    fn default() -> Self {
        Self {
            addr: Err("no value supplied for addr".to_string()),
            interface_num: Err("no value supplied for interface_num".to_string()),
            oid: Ok(Default::default()),
        }
    }
}

impl ManagementAddress {
    pub fn addr<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NetworkAddress>,
        T::Error: ::std::fmt::Display,
    {
        self.addr = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for addr: {e}"));
        self
    }

    pub fn interface_num<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::InterfaceNum>,
        T::Error: ::std::fmt::Display,
    {
        self.interface_num = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for interface_num: {e}"));
        self
    }

    pub fn oid<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<u8>>>,
        T::Error: ::std::fmt::Display,
    {
        self.oid = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for oid: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ManagementAddress> for super::ManagementAddress {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ManagementAddress,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            addr: value.addr?,
            interface_num: value.interface_num?,
            oid: value.oid?,
        })
    }
}

impl ::std::convert::From<super::ManagementAddress> for ManagementAddress {
    fn from(value: super::ManagementAddress) -> Self {
        Self {
            addr: Ok(value.addr),
            interface_num: Ok(value.interface_num),
            oid: Ok(value.oid),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Measurement {
    datum: ::std::result::Result<super::Datum, ::std::string::String>,
    timestamp:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Measurement {
    fn default() -> Self {
        Self {
            datum: Err("no value supplied for datum".to_string()),
            timestamp: Err("no value supplied for timestamp".to_string()),
        }
    }
}

impl Measurement {
    pub fn datum<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Datum>,
        T::Error: ::std::fmt::Display,
    {
        self.datum = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for datum: {e}"));
        self
    }

    pub fn timestamp<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.timestamp = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for timestamp: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Measurement> for super::Measurement {
    type Error = super::error::ConversionError;
    fn try_from(value: Measurement) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            datum: value.datum?,
            timestamp: value.timestamp?,
        })
    }
}

impl ::std::convert::From<super::Measurement> for Measurement {
    fn from(value: super::Measurement) -> Self {
        Self {
            datum: Ok(value.datum),
            timestamp: Ok(value.timestamp),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MeasurementResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Measurement>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for MeasurementResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl MeasurementResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Measurement>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<MeasurementResultsPage> for super::MeasurementResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: MeasurementResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::MeasurementResultsPage> for MeasurementResultsPage {
    fn from(value: super::MeasurementResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MissingDatum {
    datum_type: ::std::result::Result<super::DatumType, ::std::string::String>,
    start_time: ::std::result::Result<
        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for MissingDatum {
    fn default() -> Self {
        Self {
            datum_type: Err("no value supplied for datum_type".to_string()),
            start_time: Ok(Default::default()),
        }
    }
}

impl MissingDatum {
    pub fn datum_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::DatumType>,
        T::Error: ::std::fmt::Display,
    {
        self.datum_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for datum_type: {e}"));
        self
    }

    pub fn start_time<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.start_time = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_time: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<MissingDatum> for super::MissingDatum {
    type Error = super::error::ConversionError;
    fn try_from(value: MissingDatum) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            datum_type: value.datum_type?,
            start_time: value.start_time?,
        })
    }
}

impl ::std::convert::From<super::MissingDatum> for MissingDatum {
    fn from(value: super::MissingDatum) -> Self {
        Self {
            datum_type: Ok(value.datum_type),
            start_time: Ok(value.start_time),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MulticastGroup {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    has_any_source_member: ::std::result::Result<bool, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ip_pool_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    multicast_ip: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    source_ips: ::std::result::Result<::std::vec::Vec<::std::net::IpAddr>, ::std::string::String>,
    state: ::std::result::Result<::std::string::String, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for MulticastGroup {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            has_any_source_member: Err("no value supplied for has_any_source_member".to_string()),
            id: Err("no value supplied for id".to_string()),
            ip_pool_id: Err("no value supplied for ip_pool_id".to_string()),
            multicast_ip: Err("no value supplied for multicast_ip".to_string()),
            name: Err("no value supplied for name".to_string()),
            source_ips: Err("no value supplied for source_ips".to_string()),
            state: Err("no value supplied for state".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl MulticastGroup {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn has_any_source_member<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.has_any_source_member = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for has_any_source_member: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn ip_pool_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_pool_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_pool_id: {e}"));
        self
    }

    pub fn multicast_ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.multicast_ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for multicast_ip: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn source_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<::std::net::IpAddr>>,
        T::Error: ::std::fmt::Display,
    {
        self.source_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for source_ips: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<MulticastGroup> for super::MulticastGroup {
    type Error = super::error::ConversionError;
    fn try_from(
        value: MulticastGroup,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            has_any_source_member: value.has_any_source_member?,
            id: value.id?,
            ip_pool_id: value.ip_pool_id?,
            multicast_ip: value.multicast_ip?,
            name: value.name?,
            source_ips: value.source_ips?,
            state: value.state?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::MulticastGroup> for MulticastGroup {
    fn from(value: super::MulticastGroup) -> Self {
        Self {
            description: Ok(value.description),
            has_any_source_member: Ok(value.has_any_source_member),
            id: Ok(value.id),
            ip_pool_id: Ok(value.ip_pool_id),
            multicast_ip: Ok(value.multicast_ip),
            name: Ok(value.name),
            source_ips: Ok(value.source_ips),
            state: Ok(value.state),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MulticastGroupJoinSpec {
    group: ::std::result::Result<super::MulticastGroupIdentifier, ::std::string::String>,
    ip_version:
        ::std::result::Result<::std::option::Option<super::IpVersion>, ::std::string::String>,
    source_ips: ::std::result::Result<
        ::std::option::Option<::std::vec::Vec<::std::net::IpAddr>>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for MulticastGroupJoinSpec {
    fn default() -> Self {
        Self {
            group: Err("no value supplied for group".to_string()),
            ip_version: Ok(Default::default()),
            source_ips: Ok(Default::default()),
        }
    }
}

impl MulticastGroupJoinSpec {
    pub fn group<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::MulticastGroupIdentifier>,
        T::Error: ::std::fmt::Display,
    {
        self.group = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for group: {e}"));
        self
    }

    pub fn ip_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::IpVersion>>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_version: {e}"));
        self
    }

    pub fn source_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<::std::net::IpAddr>>>,
        T::Error: ::std::fmt::Display,
    {
        self.source_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for source_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<MulticastGroupJoinSpec> for super::MulticastGroupJoinSpec {
    type Error = super::error::ConversionError;
    fn try_from(
        value: MulticastGroupJoinSpec,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            group: value.group?,
            ip_version: value.ip_version?,
            source_ips: value.source_ips?,
        })
    }
}

impl ::std::convert::From<super::MulticastGroupJoinSpec> for MulticastGroupJoinSpec {
    fn from(value: super::MulticastGroupJoinSpec) -> Self {
        Self {
            group: Ok(value.group),
            ip_version: Ok(value.ip_version),
            source_ips: Ok(value.source_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MulticastGroupMember {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    instance_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    multicast_group_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    multicast_ip: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    source_ips: ::std::result::Result<::std::vec::Vec<::std::net::IpAddr>, ::std::string::String>,
    state: ::std::result::Result<::std::string::String, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for MulticastGroupMember {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            instance_id: Err("no value supplied for instance_id".to_string()),
            multicast_group_id: Err("no value supplied for multicast_group_id".to_string()),
            multicast_ip: Err("no value supplied for multicast_ip".to_string()),
            name: Err("no value supplied for name".to_string()),
            source_ips: Err("no value supplied for source_ips".to_string()),
            state: Err("no value supplied for state".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl MulticastGroupMember {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn instance_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.instance_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for instance_id: {e}"));
        self
    }

    pub fn multicast_group_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.multicast_group_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for multicast_group_id: {e}"));
        self
    }

    pub fn multicast_ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.multicast_ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for multicast_ip: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn source_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<::std::net::IpAddr>>,
        T::Error: ::std::fmt::Display,
    {
        self.source_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for source_ips: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<MulticastGroupMember> for super::MulticastGroupMember {
    type Error = super::error::ConversionError;
    fn try_from(
        value: MulticastGroupMember,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            instance_id: value.instance_id?,
            multicast_group_id: value.multicast_group_id?,
            multicast_ip: value.multicast_ip?,
            name: value.name?,
            source_ips: value.source_ips?,
            state: value.state?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::MulticastGroupMember> for MulticastGroupMember {
    fn from(value: super::MulticastGroupMember) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            instance_id: Ok(value.instance_id),
            multicast_group_id: Ok(value.multicast_group_id),
            multicast_ip: Ok(value.multicast_ip),
            name: Ok(value.name),
            source_ips: Ok(value.source_ips),
            state: Ok(value.state),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MulticastGroupMemberResultsPage {
    items:
        ::std::result::Result<::std::vec::Vec<super::MulticastGroupMember>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for MulticastGroupMemberResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl MulticastGroupMemberResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::MulticastGroupMember>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<MulticastGroupMemberResultsPage>
    for super::MulticastGroupMemberResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: MulticastGroupMemberResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::MulticastGroupMemberResultsPage>
    for MulticastGroupMemberResultsPage
{
    fn from(value: super::MulticastGroupMemberResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MulticastGroupResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::MulticastGroup>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for MulticastGroupResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl MulticastGroupResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::MulticastGroup>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<MulticastGroupResultsPage> for super::MulticastGroupResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: MulticastGroupResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::MulticastGroupResultsPage> for MulticastGroupResultsPage {
    fn from(value: super::MulticastGroupResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NetworkInterface {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ip_config: ::std::result::Result<super::PrivateIpConfig, ::std::string::String>,
    kind: ::std::result::Result<super::NetworkInterfaceKind, ::std::string::String>,
    mac: ::std::result::Result<super::MacAddr, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    primary: ::std::result::Result<bool, ::std::string::String>,
    slot: ::std::result::Result<u8, ::std::string::String>,
    vni: ::std::result::Result<super::Vni, ::std::string::String>,
}

impl ::std::default::Default for NetworkInterface {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            ip_config: Err("no value supplied for ip_config".to_string()),
            kind: Err("no value supplied for kind".to_string()),
            mac: Err("no value supplied for mac".to_string()),
            name: Err("no value supplied for name".to_string()),
            primary: Err("no value supplied for primary".to_string()),
            slot: Err("no value supplied for slot".to_string()),
            vni: Err("no value supplied for vni".to_string()),
        }
    }
}

impl NetworkInterface {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn ip_config<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PrivateIpConfig>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_config = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_config: {e}"));
        self
    }

    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NetworkInterfaceKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn mac<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::MacAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.mac = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for mac: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn primary<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.primary = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for primary: {e}"));
        self
    }

    pub fn slot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u8>,
        T::Error: ::std::fmt::Display,
    {
        self.slot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for slot: {e}"));
        self
    }

    pub fn vni<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Vni>,
        T::Error: ::std::fmt::Display,
    {
        self.vni = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vni: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<NetworkInterface> for super::NetworkInterface {
    type Error = super::error::ConversionError;
    fn try_from(
        value: NetworkInterface,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            ip_config: value.ip_config?,
            kind: value.kind?,
            mac: value.mac?,
            name: value.name?,
            primary: value.primary?,
            slot: value.slot?,
            vni: value.vni?,
        })
    }
}

impl ::std::convert::From<super::NetworkInterface> for NetworkInterface {
    fn from(value: super::NetworkInterface) -> Self {
        Self {
            id: Ok(value.id),
            ip_config: Ok(value.ip_config),
            kind: Ok(value.kind),
            mac: Ok(value.mac),
            name: Ok(value.name),
            primary: Ok(value.primary),
            slot: Ok(value.slot),
            vni: Ok(value.vni),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OxqlQueryResult {
    tables: ::std::result::Result<::std::vec::Vec<super::OxqlTable>, ::std::string::String>,
}

impl ::std::default::Default for OxqlQueryResult {
    fn default() -> Self {
        Self {
            tables: Err("no value supplied for tables".to_string()),
        }
    }
}

impl OxqlQueryResult {
    pub fn tables<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::OxqlTable>>,
        T::Error: ::std::fmt::Display,
    {
        self.tables = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for tables: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<OxqlQueryResult> for super::OxqlQueryResult {
    type Error = super::error::ConversionError;
    fn try_from(
        value: OxqlQueryResult,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            tables: value.tables?,
        })
    }
}

impl ::std::convert::From<super::OxqlQueryResult> for OxqlQueryResult {
    fn from(value: super::OxqlQueryResult) -> Self {
        Self {
            tables: Ok(value.tables),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OxqlTable {
    name: ::std::result::Result<::std::string::String, ::std::string::String>,
    timeseries: ::std::result::Result<::std::vec::Vec<super::Timeseries>, ::std::string::String>,
}

impl ::std::default::Default for OxqlTable {
    fn default() -> Self {
        Self {
            name: Err("no value supplied for name".to_string()),
            timeseries: Err("no value supplied for timeseries".to_string()),
        }
    }
}

impl OxqlTable {
    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn timeseries<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Timeseries>>,
        T::Error: ::std::fmt::Display,
    {
        self.timeseries = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for timeseries: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<OxqlTable> for super::OxqlTable {
    type Error = super::error::ConversionError;
    fn try_from(value: OxqlTable) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            name: value.name?,
            timeseries: value.timeseries?,
        })
    }
}

impl ::std::convert::From<super::OxqlTable> for OxqlTable {
    fn from(value: super::OxqlTable) -> Self {
        Self {
            name: Ok(value.name),
            timeseries: Ok(value.timeseries),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PhysicalDisk {
    form_factor: ::std::result::Result<super::PhysicalDiskKind, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    model: ::std::result::Result<::std::string::String, ::std::string::String>,
    policy: ::std::result::Result<super::PhysicalDiskPolicy, ::std::string::String>,
    serial: ::std::result::Result<::std::string::String, ::std::string::String>,
    sled_id: ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    state: ::std::result::Result<super::PhysicalDiskState, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    vendor: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for PhysicalDisk {
    fn default() -> Self {
        Self {
            form_factor: Err("no value supplied for form_factor".to_string()),
            id: Err("no value supplied for id".to_string()),
            model: Err("no value supplied for model".to_string()),
            policy: Err("no value supplied for policy".to_string()),
            serial: Err("no value supplied for serial".to_string()),
            sled_id: Ok(Default::default()),
            state: Err("no value supplied for state".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            vendor: Err("no value supplied for vendor".to_string()),
        }
    }
}

impl PhysicalDisk {
    pub fn form_factor<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PhysicalDiskKind>,
        T::Error: ::std::fmt::Display,
    {
        self.form_factor = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for form_factor: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn model<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.model = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for model: {e}"));
        self
    }

    pub fn policy<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PhysicalDiskPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.policy = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for policy: {e}"));
        self
    }

    pub fn serial<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.serial = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for serial: {e}"));
        self
    }

    pub fn sled_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.sled_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sled_id: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PhysicalDiskState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn vendor<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.vendor = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vendor: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PhysicalDisk> for super::PhysicalDisk {
    type Error = super::error::ConversionError;
    fn try_from(value: PhysicalDisk) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            form_factor: value.form_factor?,
            id: value.id?,
            model: value.model?,
            policy: value.policy?,
            serial: value.serial?,
            sled_id: value.sled_id?,
            state: value.state?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            vendor: value.vendor?,
        })
    }
}

impl ::std::convert::From<super::PhysicalDisk> for PhysicalDisk {
    fn from(value: super::PhysicalDisk) -> Self {
        Self {
            form_factor: Ok(value.form_factor),
            id: Ok(value.id),
            model: Ok(value.model),
            policy: Ok(value.policy),
            serial: Ok(value.serial),
            sled_id: Ok(value.sled_id),
            state: Ok(value.state),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            vendor: Ok(value.vendor),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PhysicalDiskAdoptionRequest {
    disk_id: ::std::result::Result<super::PhysicalDiskManufacturerIdentity, ::std::string::String>,
    id: ::std::result::Result<super::PhysicalDiskAdoptionRequestUuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for PhysicalDiskAdoptionRequest {
    fn default() -> Self {
        Self {
            disk_id: Err("no value supplied for disk_id".to_string()),
            id: Err("no value supplied for id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
        }
    }
}

impl PhysicalDiskAdoptionRequest {
    pub fn disk_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PhysicalDiskManufacturerIdentity>,
        T::Error: ::std::fmt::Display,
    {
        self.disk_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for disk_id: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PhysicalDiskAdoptionRequestUuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PhysicalDiskAdoptionRequest> for super::PhysicalDiskAdoptionRequest {
    type Error = super::error::ConversionError;
    fn try_from(
        value: PhysicalDiskAdoptionRequest,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            disk_id: value.disk_id?,
            id: value.id?,
            time_created: value.time_created?,
        })
    }
}

impl ::std::convert::From<super::PhysicalDiskAdoptionRequest> for PhysicalDiskAdoptionRequest {
    fn from(value: super::PhysicalDiskAdoptionRequest) -> Self {
        Self {
            disk_id: Ok(value.disk_id),
            id: Ok(value.id),
            time_created: Ok(value.time_created),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PhysicalDiskAdoptionRequestResultsPage {
    items: ::std::result::Result<
        ::std::vec::Vec<super::PhysicalDiskAdoptionRequest>,
        ::std::string::String,
    >,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for PhysicalDiskAdoptionRequestResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl PhysicalDiskAdoptionRequestResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::PhysicalDiskAdoptionRequest>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PhysicalDiskAdoptionRequestResultsPage>
    for super::PhysicalDiskAdoptionRequestResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: PhysicalDiskAdoptionRequestResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::PhysicalDiskAdoptionRequestResultsPage>
    for PhysicalDiskAdoptionRequestResultsPage
{
    fn from(value: super::PhysicalDiskAdoptionRequestResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PhysicalDiskManufacturerIdentity {
    model: ::std::result::Result<::std::string::String, ::std::string::String>,
    serial: ::std::result::Result<::std::string::String, ::std::string::String>,
    vendor: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for PhysicalDiskManufacturerIdentity {
    fn default() -> Self {
        Self {
            model: Err("no value supplied for model".to_string()),
            serial: Err("no value supplied for serial".to_string()),
            vendor: Err("no value supplied for vendor".to_string()),
        }
    }
}

impl PhysicalDiskManufacturerIdentity {
    pub fn model<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.model = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for model: {e}"));
        self
    }

    pub fn serial<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.serial = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for serial: {e}"));
        self
    }

    pub fn vendor<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.vendor = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vendor: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PhysicalDiskManufacturerIdentity>
    for super::PhysicalDiskManufacturerIdentity
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: PhysicalDiskManufacturerIdentity,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            model: value.model?,
            serial: value.serial?,
            vendor: value.vendor?,
        })
    }
}

impl ::std::convert::From<super::PhysicalDiskManufacturerIdentity>
    for PhysicalDiskManufacturerIdentity
{
    fn from(value: super::PhysicalDiskManufacturerIdentity) -> Self {
        Self {
            model: Ok(value.model),
            serial: Ok(value.serial),
            vendor: Ok(value.vendor),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PhysicalDiskResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::PhysicalDisk>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for PhysicalDiskResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl PhysicalDiskResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::PhysicalDisk>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PhysicalDiskResultsPage> for super::PhysicalDiskResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: PhysicalDiskResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::PhysicalDiskResultsPage> for PhysicalDiskResultsPage {
    fn from(value: super::PhysicalDiskResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ping {
    status: ::std::result::Result<super::PingStatus, ::std::string::String>,
}

impl ::std::default::Default for Ping {
    fn default() -> Self {
        Self {
            status: Err("no value supplied for status".to_string()),
        }
    }
}

impl Ping {
    pub fn status<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PingStatus>,
        T::Error: ::std::fmt::Display,
    {
        self.status = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for status: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Ping> for super::Ping {
    type Error = super::error::ConversionError;
    fn try_from(value: Ping) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            status: value.status?,
        })
    }
}

impl ::std::convert::From<super::Ping> for Ping {
    fn from(value: super::Ping) -> Self {
        Self {
            status: Ok(value.status),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Points {
    start_times: ::std::result::Result<
        ::std::option::Option<::std::vec::Vec<::chrono::DateTime<::chrono::offset::Utc>>>,
        ::std::string::String,
    >,
    timestamps: ::std::result::Result<
        ::std::vec::Vec<::chrono::DateTime<::chrono::offset::Utc>>,
        ::std::string::String,
    >,
    values: ::std::result::Result<::std::vec::Vec<super::Values>, ::std::string::String>,
}

impl ::std::default::Default for Points {
    fn default() -> Self {
        Self {
            start_times: Ok(Default::default()),
            timestamps: Err("no value supplied for timestamps".to_string()),
            values: Err("no value supplied for values".to_string()),
        }
    }
}

impl Points {
    pub fn start_times<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::std::vec::Vec<::chrono::DateTime<::chrono::offset::Utc>>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.start_times = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for start_times: {e}"));
        self
    }

    pub fn timestamps<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<::chrono::DateTime<::chrono::offset::Utc>>>,
        T::Error: ::std::fmt::Display,
    {
        self.timestamps = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for timestamps: {e}"));
        self
    }

    pub fn values<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Values>>,
        T::Error: ::std::fmt::Display,
    {
        self.values = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for values: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Points> for super::Points {
    type Error = super::error::ConversionError;
    fn try_from(value: Points) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            start_times: value.start_times?,
            timestamps: value.timestamps?,
            values: value.values?,
        })
    }
}

impl ::std::convert::From<super::Points> for Points {
    fn from(value: super::Points) -> Self {
        Self {
            start_times: Ok(value.start_times),
            timestamps: Ok(value.timestamps),
            values: Ok(value.values),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrivateIpv4Config {
    ip: ::std::result::Result<::std::net::Ipv4Addr, ::std::string::String>,
    subnet: ::std::result::Result<super::Ipv4Net, ::std::string::String>,
    transit_ips: ::std::result::Result<::std::vec::Vec<super::Ipv4Net>, ::std::string::String>,
}

impl ::std::default::Default for PrivateIpv4Config {
    fn default() -> Self {
        Self {
            ip: Err("no value supplied for ip".to_string()),
            subnet: Err("no value supplied for subnet".to_string()),
            transit_ips: Ok(Default::default()),
        }
    }
}

impl PrivateIpv4Config {
    pub fn ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::Ipv4Addr>,
        T::Error: ::std::fmt::Display,
    {
        self.ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip: {e}"));
        self
    }

    pub fn subnet<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Ipv4Net>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet: {e}"));
        self
    }

    pub fn transit_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Ipv4Net>>,
        T::Error: ::std::fmt::Display,
    {
        self.transit_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for transit_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PrivateIpv4Config> for super::PrivateIpv4Config {
    type Error = super::error::ConversionError;
    fn try_from(
        value: PrivateIpv4Config,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            ip: value.ip?,
            subnet: value.subnet?,
            transit_ips: value.transit_ips?,
        })
    }
}

impl ::std::convert::From<super::PrivateIpv4Config> for PrivateIpv4Config {
    fn from(value: super::PrivateIpv4Config) -> Self {
        Self {
            ip: Ok(value.ip),
            subnet: Ok(value.subnet),
            transit_ips: Ok(value.transit_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrivateIpv4Stack {
    ip: ::std::result::Result<::std::net::Ipv4Addr, ::std::string::String>,
    transit_ips: ::std::result::Result<::std::vec::Vec<super::Ipv4Net>, ::std::string::String>,
}

impl ::std::default::Default for PrivateIpv4Stack {
    fn default() -> Self {
        Self {
            ip: Err("no value supplied for ip".to_string()),
            transit_ips: Err("no value supplied for transit_ips".to_string()),
        }
    }
}

impl PrivateIpv4Stack {
    pub fn ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::Ipv4Addr>,
        T::Error: ::std::fmt::Display,
    {
        self.ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip: {e}"));
        self
    }

    pub fn transit_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Ipv4Net>>,
        T::Error: ::std::fmt::Display,
    {
        self.transit_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for transit_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PrivateIpv4Stack> for super::PrivateIpv4Stack {
    type Error = super::error::ConversionError;
    fn try_from(
        value: PrivateIpv4Stack,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            ip: value.ip?,
            transit_ips: value.transit_ips?,
        })
    }
}

impl ::std::convert::From<super::PrivateIpv4Stack> for PrivateIpv4Stack {
    fn from(value: super::PrivateIpv4Stack) -> Self {
        Self {
            ip: Ok(value.ip),
            transit_ips: Ok(value.transit_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrivateIpv4StackCreate {
    ip: ::std::result::Result<super::Ipv4Assignment, ::std::string::String>,
    transit_ips: ::std::result::Result<::std::vec::Vec<super::Ipv4Net>, ::std::string::String>,
}

impl ::std::default::Default for PrivateIpv4StackCreate {
    fn default() -> Self {
        Self {
            ip: Err("no value supplied for ip".to_string()),
            transit_ips: Ok(Default::default()),
        }
    }
}

impl PrivateIpv4StackCreate {
    pub fn ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Ipv4Assignment>,
        T::Error: ::std::fmt::Display,
    {
        self.ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip: {e}"));
        self
    }

    pub fn transit_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Ipv4Net>>,
        T::Error: ::std::fmt::Display,
    {
        self.transit_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for transit_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PrivateIpv4StackCreate> for super::PrivateIpv4StackCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: PrivateIpv4StackCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            ip: value.ip?,
            transit_ips: value.transit_ips?,
        })
    }
}

impl ::std::convert::From<super::PrivateIpv4StackCreate> for PrivateIpv4StackCreate {
    fn from(value: super::PrivateIpv4StackCreate) -> Self {
        Self {
            ip: Ok(value.ip),
            transit_ips: Ok(value.transit_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrivateIpv6Config {
    ip: ::std::result::Result<::std::net::Ipv6Addr, ::std::string::String>,
    subnet: ::std::result::Result<super::Ipv6Net, ::std::string::String>,
    transit_ips: ::std::result::Result<::std::vec::Vec<super::Ipv6Net>, ::std::string::String>,
}

impl ::std::default::Default for PrivateIpv6Config {
    fn default() -> Self {
        Self {
            ip: Err("no value supplied for ip".to_string()),
            subnet: Err("no value supplied for subnet".to_string()),
            transit_ips: Err("no value supplied for transit_ips".to_string()),
        }
    }
}

impl PrivateIpv6Config {
    pub fn ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::Ipv6Addr>,
        T::Error: ::std::fmt::Display,
    {
        self.ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip: {e}"));
        self
    }

    pub fn subnet<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Ipv6Net>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet: {e}"));
        self
    }

    pub fn transit_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Ipv6Net>>,
        T::Error: ::std::fmt::Display,
    {
        self.transit_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for transit_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PrivateIpv6Config> for super::PrivateIpv6Config {
    type Error = super::error::ConversionError;
    fn try_from(
        value: PrivateIpv6Config,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            ip: value.ip?,
            subnet: value.subnet?,
            transit_ips: value.transit_ips?,
        })
    }
}

impl ::std::convert::From<super::PrivateIpv6Config> for PrivateIpv6Config {
    fn from(value: super::PrivateIpv6Config) -> Self {
        Self {
            ip: Ok(value.ip),
            subnet: Ok(value.subnet),
            transit_ips: Ok(value.transit_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrivateIpv6Stack {
    ip: ::std::result::Result<::std::net::Ipv6Addr, ::std::string::String>,
    transit_ips: ::std::result::Result<::std::vec::Vec<super::Ipv6Net>, ::std::string::String>,
}

impl ::std::default::Default for PrivateIpv6Stack {
    fn default() -> Self {
        Self {
            ip: Err("no value supplied for ip".to_string()),
            transit_ips: Err("no value supplied for transit_ips".to_string()),
        }
    }
}

impl PrivateIpv6Stack {
    pub fn ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::Ipv6Addr>,
        T::Error: ::std::fmt::Display,
    {
        self.ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip: {e}"));
        self
    }

    pub fn transit_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Ipv6Net>>,
        T::Error: ::std::fmt::Display,
    {
        self.transit_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for transit_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PrivateIpv6Stack> for super::PrivateIpv6Stack {
    type Error = super::error::ConversionError;
    fn try_from(
        value: PrivateIpv6Stack,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            ip: value.ip?,
            transit_ips: value.transit_ips?,
        })
    }
}

impl ::std::convert::From<super::PrivateIpv6Stack> for PrivateIpv6Stack {
    fn from(value: super::PrivateIpv6Stack) -> Self {
        Self {
            ip: Ok(value.ip),
            transit_ips: Ok(value.transit_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrivateIpv6StackCreate {
    ip: ::std::result::Result<super::Ipv6Assignment, ::std::string::String>,
    transit_ips: ::std::result::Result<::std::vec::Vec<super::Ipv6Net>, ::std::string::String>,
}

impl ::std::default::Default for PrivateIpv6StackCreate {
    fn default() -> Self {
        Self {
            ip: Err("no value supplied for ip".to_string()),
            transit_ips: Ok(Default::default()),
        }
    }
}

impl PrivateIpv6StackCreate {
    pub fn ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Ipv6Assignment>,
        T::Error: ::std::fmt::Display,
    {
        self.ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip: {e}"));
        self
    }

    pub fn transit_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Ipv6Net>>,
        T::Error: ::std::fmt::Display,
    {
        self.transit_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for transit_ips: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<PrivateIpv6StackCreate> for super::PrivateIpv6StackCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: PrivateIpv6StackCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            ip: value.ip?,
            transit_ips: value.transit_ips?,
        })
    }
}

impl ::std::convert::From<super::PrivateIpv6StackCreate> for PrivateIpv6StackCreate {
    fn from(value: super::PrivateIpv6StackCreate) -> Self {
        Self {
            ip: Ok(value.ip),
            transit_ips: Ok(value.transit_ips),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Probe {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    sled: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Probe {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            sled: Err("no value supplied for sled".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Probe {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn sled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.sled = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sled: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Probe> for super::Probe {
    type Error = super::error::ConversionError;
    fn try_from(value: Probe) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            name: value.name?,
            sled: value.sled?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Probe> for Probe {
    fn from(value: super::Probe) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            name: Ok(value.name),
            sled: Ok(value.sled),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProbeCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    pool_selector: ::std::result::Result<super::PoolSelector, ::std::string::String>,
    sled: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for ProbeCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
            pool_selector: Ok(super::defaults::probe_create_pool_selector()),
            sled: Err("no value supplied for sled".to_string()),
        }
    }
}

impl ProbeCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn pool_selector<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PoolSelector>,
        T::Error: ::std::fmt::Display,
    {
        self.pool_selector = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for pool_selector: {e}"));
        self
    }

    pub fn sled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.sled = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sled: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProbeCreate> for super::ProbeCreate {
    type Error = super::error::ConversionError;
    fn try_from(value: ProbeCreate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
            pool_selector: value.pool_selector?,
            sled: value.sled?,
        })
    }
}

impl ::std::convert::From<super::ProbeCreate> for ProbeCreate {
    fn from(value: super::ProbeCreate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
            pool_selector: Ok(value.pool_selector),
            sled: Ok(value.sled),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProbeExternalIp {
    first_port: ::std::result::Result<u16, ::std::string::String>,
    ip: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    kind: ::std::result::Result<super::ProbeExternalIpKind, ::std::string::String>,
    last_port: ::std::result::Result<u16, ::std::string::String>,
}

impl ::std::default::Default for ProbeExternalIp {
    fn default() -> Self {
        Self {
            first_port: Err("no value supplied for first_port".to_string()),
            ip: Err("no value supplied for ip".to_string()),
            kind: Err("no value supplied for kind".to_string()),
            last_port: Err("no value supplied for last_port".to_string()),
        }
    }
}

impl ProbeExternalIp {
    pub fn first_port<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.first_port = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for first_port: {e}"));
        self
    }

    pub fn ip<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.ip = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip: {e}"));
        self
    }

    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ProbeExternalIpKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn last_port<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.last_port = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for last_port: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProbeExternalIp> for super::ProbeExternalIp {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ProbeExternalIp,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            first_port: value.first_port?,
            ip: value.ip?,
            kind: value.kind?,
            last_port: value.last_port?,
        })
    }
}

impl ::std::convert::From<super::ProbeExternalIp> for ProbeExternalIp {
    fn from(value: super::ProbeExternalIp) -> Self {
        Self {
            first_port: Ok(value.first_port),
            ip: Ok(value.ip),
            kind: Ok(value.kind),
            last_port: Ok(value.last_port),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProbeInfo {
    external_ips:
        ::std::result::Result<::std::vec::Vec<super::ProbeExternalIp>, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    interface: ::std::result::Result<super::NetworkInterface, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    sled: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for ProbeInfo {
    fn default() -> Self {
        Self {
            external_ips: Err("no value supplied for external_ips".to_string()),
            id: Err("no value supplied for id".to_string()),
            interface: Err("no value supplied for interface".to_string()),
            name: Err("no value supplied for name".to_string()),
            sled: Err("no value supplied for sled".to_string()),
        }
    }
}

impl ProbeInfo {
    pub fn external_ips<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::ProbeExternalIp>>,
        T::Error: ::std::fmt::Display,
    {
        self.external_ips = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for external_ips: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn interface<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NetworkInterface>,
        T::Error: ::std::fmt::Display,
    {
        self.interface = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for interface: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn sled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.sled = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sled: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProbeInfo> for super::ProbeInfo {
    type Error = super::error::ConversionError;
    fn try_from(value: ProbeInfo) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            external_ips: value.external_ips?,
            id: value.id?,
            interface: value.interface?,
            name: value.name?,
            sled: value.sled?,
        })
    }
}

impl ::std::convert::From<super::ProbeInfo> for ProbeInfo {
    fn from(value: super::ProbeInfo) -> Self {
        Self {
            external_ips: Ok(value.external_ips),
            id: Ok(value.id),
            interface: Ok(value.interface),
            name: Ok(value.name),
            sled: Ok(value.sled),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProbeInfoResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::ProbeInfo>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for ProbeInfoResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl ProbeInfoResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::ProbeInfo>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProbeInfoResultsPage> for super::ProbeInfoResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ProbeInfoResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::ProbeInfoResultsPage> for ProbeInfoResultsPage {
    fn from(value: super::ProbeInfoResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Project {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Project {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Project {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Project> for super::Project {
    type Error = super::error::ConversionError;
    fn try_from(value: Project) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Project> for Project {
    fn from(value: super::Project) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProjectCreate {
    defaults: ::std::result::Result<
        ::std::option::Option<super::ProjectCreateDefaults>,
        ::std::string::String,
    >,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for ProjectCreate {
    fn default() -> Self {
        Self {
            defaults: Ok(Default::default()),
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl ProjectCreate {
    pub fn defaults<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::ProjectCreateDefaults>>,
        T::Error: ::std::fmt::Display,
    {
        self.defaults = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for defaults: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProjectCreate> for super::ProjectCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ProjectCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            defaults: value.defaults?,
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::ProjectCreate> for ProjectCreate {
    fn from(value: super::ProjectCreate) -> Self {
        Self {
            defaults: Ok(value.defaults),
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProjectCreateDefaults {
    vpc: ::std::result::Result<
        ::std::option::Option<super::VpcCreateDefaultsSelection>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for ProjectCreateDefaults {
    fn default() -> Self {
        Self {
            vpc: Ok(Default::default()),
        }
    }
}

impl ProjectCreateDefaults {
    pub fn vpc<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::VpcCreateDefaultsSelection>>,
        T::Error: ::std::fmt::Display,
    {
        self.vpc = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vpc: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProjectCreateDefaults> for super::ProjectCreateDefaults {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ProjectCreateDefaults,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self { vpc: value.vpc? })
    }
}

impl ::std::convert::From<super::ProjectCreateDefaults> for ProjectCreateDefaults {
    fn from(value: super::ProjectCreateDefaults) -> Self {
        Self { vpc: Ok(value.vpc) }
    }
}

#[derive(Clone, Debug)]
pub struct ProjectResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Project>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for ProjectResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl ProjectResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Project>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProjectResultsPage> for super::ProjectResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ProjectResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::ProjectResultsPage> for ProjectResultsPage {
    fn from(value: super::ProjectResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProjectRolePolicy {
    role_assignments: ::std::result::Result<
        ::std::vec::Vec<super::ProjectRoleRoleAssignment>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for ProjectRolePolicy {
    fn default() -> Self {
        Self {
            role_assignments: Err("no value supplied for role_assignments".to_string()),
        }
    }
}

impl ProjectRolePolicy {
    pub fn role_assignments<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::ProjectRoleRoleAssignment>>,
        T::Error: ::std::fmt::Display,
    {
        self.role_assignments = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for role_assignments: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProjectRolePolicy> for super::ProjectRolePolicy {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ProjectRolePolicy,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            role_assignments: value.role_assignments?,
        })
    }
}

impl ::std::convert::From<super::ProjectRolePolicy> for ProjectRolePolicy {
    fn from(value: super::ProjectRolePolicy) -> Self {
        Self {
            role_assignments: Ok(value.role_assignments),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProjectRoleRoleAssignment {
    identity_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    identity_type: ::std::result::Result<super::IdentityType, ::std::string::String>,
    role_name: ::std::result::Result<super::ProjectRole, ::std::string::String>,
}

impl ::std::default::Default for ProjectRoleRoleAssignment {
    fn default() -> Self {
        Self {
            identity_id: Err("no value supplied for identity_id".to_string()),
            identity_type: Err("no value supplied for identity_type".to_string()),
            role_name: Err("no value supplied for role_name".to_string()),
        }
    }
}

impl ProjectRoleRoleAssignment {
    pub fn identity_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.identity_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_id: {e}"));
        self
    }

    pub fn identity_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IdentityType>,
        T::Error: ::std::fmt::Display,
    {
        self.identity_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_type: {e}"));
        self
    }

    pub fn role_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ProjectRole>,
        T::Error: ::std::fmt::Display,
    {
        self.role_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for role_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProjectRoleRoleAssignment> for super::ProjectRoleRoleAssignment {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ProjectRoleRoleAssignment,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            identity_id: value.identity_id?,
            identity_type: value.identity_type?,
            role_name: value.role_name?,
        })
    }
}

impl ::std::convert::From<super::ProjectRoleRoleAssignment> for ProjectRoleRoleAssignment {
    fn from(value: super::ProjectRoleRoleAssignment) -> Self {
        Self {
            identity_id: Ok(value.identity_id),
            identity_type: Ok(value.identity_type),
            role_name: Ok(value.role_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProjectUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for ProjectUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl ProjectUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ProjectUpdate> for super::ProjectUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ProjectUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::ProjectUpdate> for ProjectUpdate {
    fn from(value: super::ProjectUpdate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Quantile {
    desired_marker_positions: ::std::result::Result<[f64; 5usize], ::std::string::String>,
    marker_heights: ::std::result::Result<[f64; 5usize], ::std::string::String>,
    marker_positions: ::std::result::Result<[u64; 5usize], ::std::string::String>,
    p: ::std::result::Result<f64, ::std::string::String>,
}

impl ::std::default::Default for Quantile {
    fn default() -> Self {
        Self {
            desired_marker_positions: Err(
                "no value supplied for desired_marker_positions".to_string()
            ),
            marker_heights: Err("no value supplied for marker_heights".to_string()),
            marker_positions: Err("no value supplied for marker_positions".to_string()),
            p: Err("no value supplied for p".to_string()),
        }
    }
}

impl Quantile {
    pub fn desired_marker_positions<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<[f64; 5usize]>,
        T::Error: ::std::fmt::Display,
    {
        self.desired_marker_positions = value.try_into().map_err(|e| {
            format!("error converting supplied value for desired_marker_positions: {e}")
        });
        self
    }

    pub fn marker_heights<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<[f64; 5usize]>,
        T::Error: ::std::fmt::Display,
    {
        self.marker_heights = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for marker_heights: {e}"));
        self
    }

    pub fn marker_positions<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<[u64; 5usize]>,
        T::Error: ::std::fmt::Display,
    {
        self.marker_positions = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for marker_positions: {e}"));
        self
    }

    pub fn p<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.p = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for p: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Quantile> for super::Quantile {
    type Error = super::error::ConversionError;
    fn try_from(value: Quantile) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            desired_marker_positions: value.desired_marker_positions?,
            marker_heights: value.marker_heights?,
            marker_positions: value.marker_positions?,
            p: value.p?,
        })
    }
}

impl ::std::convert::From<super::Quantile> for Quantile {
    fn from(value: super::Quantile) -> Self {
        Self {
            desired_marker_positions: Ok(value.desired_marker_positions),
            marker_heights: Ok(value.marker_heights),
            marker_positions: Ok(value.marker_positions),
            p: Ok(value.p),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rack {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Rack {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Rack {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Rack> for super::Rack {
    type Error = super::error::ConversionError;
    fn try_from(value: Rack) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Rack> for Rack {
    fn from(value: super::Rack) -> Self {
        Self {
            id: Ok(value.id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RackMembershipAddSledsRequest {
    sled_ids: ::std::result::Result<::std::vec::Vec<super::BaseboardId>, ::std::string::String>,
}

impl ::std::default::Default for RackMembershipAddSledsRequest {
    fn default() -> Self {
        Self {
            sled_ids: Err("no value supplied for sled_ids".to_string()),
        }
    }
}

impl RackMembershipAddSledsRequest {
    pub fn sled_ids<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::BaseboardId>>,
        T::Error: ::std::fmt::Display,
    {
        self.sled_ids = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sled_ids: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<RackMembershipAddSledsRequest>
    for super::RackMembershipAddSledsRequest
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: RackMembershipAddSledsRequest,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            sled_ids: value.sled_ids?,
        })
    }
}

impl ::std::convert::From<super::RackMembershipAddSledsRequest> for RackMembershipAddSledsRequest {
    fn from(value: super::RackMembershipAddSledsRequest) -> Self {
        Self {
            sled_ids: Ok(value.sled_ids),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RackMembershipStatus {
    members: ::std::result::Result<::std::vec::Vec<super::BaseboardId>, ::std::string::String>,
    rack_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    state: ::std::result::Result<super::RackMembershipChangeState, ::std::string::String>,
    time_aborted: ::std::result::Result<
        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ::std::string::String,
    >,
    time_committed: ::std::result::Result<
        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ::std::string::String,
    >,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    unacknowledged_members:
        ::std::result::Result<::std::vec::Vec<super::BaseboardId>, ::std::string::String>,
    version: ::std::result::Result<super::RackMembershipVersion, ::std::string::String>,
}

impl ::std::default::Default for RackMembershipStatus {
    fn default() -> Self {
        Self {
            members: Err("no value supplied for members".to_string()),
            rack_id: Err("no value supplied for rack_id".to_string()),
            state: Err("no value supplied for state".to_string()),
            time_aborted: Ok(Default::default()),
            time_committed: Ok(Default::default()),
            time_created: Err("no value supplied for time_created".to_string()),
            unacknowledged_members: Err("no value supplied for unacknowledged_members".to_string()),
            version: Err("no value supplied for version".to_string()),
        }
    }
}

impl RackMembershipStatus {
    pub fn members<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::BaseboardId>>,
        T::Error: ::std::fmt::Display,
    {
        self.members = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for members: {e}"));
        self
    }

    pub fn rack_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.rack_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rack_id: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RackMembershipChangeState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_aborted<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.time_aborted = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_aborted: {e}"));
        self
    }

    pub fn time_committed<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.time_committed = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_committed: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn unacknowledged_members<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::BaseboardId>>,
        T::Error: ::std::fmt::Display,
    {
        self.unacknowledged_members = value.try_into().map_err(|e| {
            format!("error converting supplied value for unacknowledged_members: {e}")
        });
        self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RackMembershipVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for version: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<RackMembershipStatus> for super::RackMembershipStatus {
    type Error = super::error::ConversionError;
    fn try_from(
        value: RackMembershipStatus,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            members: value.members?,
            rack_id: value.rack_id?,
            state: value.state?,
            time_aborted: value.time_aborted?,
            time_committed: value.time_committed?,
            time_created: value.time_created?,
            unacknowledged_members: value.unacknowledged_members?,
            version: value.version?,
        })
    }
}

impl ::std::convert::From<super::RackMembershipStatus> for RackMembershipStatus {
    fn from(value: super::RackMembershipStatus) -> Self {
        Self {
            members: Ok(value.members),
            rack_id: Ok(value.rack_id),
            state: Ok(value.state),
            time_aborted: Ok(value.time_aborted),
            time_committed: Ok(value.time_committed),
            time_created: Ok(value.time_created),
            unacknowledged_members: Ok(value.unacknowledged_members),
            version: Ok(value.version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RackResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Rack>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for RackResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl RackResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Rack>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<RackResultsPage> for super::RackResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: RackResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::RackResultsPage> for RackResultsPage {
    fn from(value: super::RackResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Route {
    dst: ::std::result::Result<super::IpNet, ::std::string::String>,
    gw: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    rib_priority: ::std::result::Result<::std::option::Option<u8>, ::std::string::String>,
    vid: ::std::result::Result<::std::option::Option<u16>, ::std::string::String>,
}

impl ::std::default::Default for Route {
    fn default() -> Self {
        Self {
            dst: Err("no value supplied for dst".to_string()),
            gw: Err("no value supplied for gw".to_string()),
            rib_priority: Ok(Default::default()),
            vid: Ok(Default::default()),
        }
    }
}

impl Route {
    pub fn dst<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.dst = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for dst: {e}"));
        self
    }

    pub fn gw<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.gw = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for gw: {e}"));
        self
    }

    pub fn rib_priority<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u8>>,
        T::Error: ::std::fmt::Display,
    {
        self.rib_priority = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rib_priority: {e}"));
        self
    }

    pub fn vid<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u16>>,
        T::Error: ::std::fmt::Display,
    {
        self.vid = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vid: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Route> for super::Route {
    type Error = super::error::ConversionError;
    fn try_from(value: Route) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            dst: value.dst?,
            gw: value.gw?,
            rib_priority: value.rib_priority?,
            vid: value.vid?,
        })
    }
}

impl ::std::convert::From<super::Route> for Route {
    fn from(value: super::Route) -> Self {
        Self {
            dst: Ok(value.dst),
            gw: Ok(value.gw),
            rib_priority: Ok(value.rib_priority),
            vid: Ok(value.vid),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RouteConfig {
    link_name: ::std::result::Result<super::Name, ::std::string::String>,
    routes: ::std::result::Result<::std::vec::Vec<super::Route>, ::std::string::String>,
}

impl ::std::default::Default for RouteConfig {
    fn default() -> Self {
        Self {
            link_name: Err("no value supplied for link_name".to_string()),
            routes: Err("no value supplied for routes".to_string()),
        }
    }
}

impl RouteConfig {
    pub fn link_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.link_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_name: {e}"));
        self
    }

    pub fn routes<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Route>>,
        T::Error: ::std::fmt::Display,
    {
        self.routes = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for routes: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<RouteConfig> for super::RouteConfig {
    type Error = super::error::ConversionError;
    fn try_from(value: RouteConfig) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            link_name: value.link_name?,
            routes: value.routes?,
        })
    }
}

impl ::std::convert::From<super::RouteConfig> for RouteConfig {
    fn from(value: super::RouteConfig) -> Self {
        Self {
            link_name: Ok(value.link_name),
            routes: Ok(value.routes),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RouterRoute {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    destination: ::std::result::Result<super::RouteDestination, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    kind: ::std::result::Result<super::RouterRouteKind, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    target: ::std::result::Result<super::RouteTarget, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    vpc_router_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for RouterRoute {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            destination: Err("no value supplied for destination".to_string()),
            id: Err("no value supplied for id".to_string()),
            kind: Err("no value supplied for kind".to_string()),
            name: Err("no value supplied for name".to_string()),
            target: Err("no value supplied for target".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            vpc_router_id: Err("no value supplied for vpc_router_id".to_string()),
        }
    }
}

impl RouterRoute {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn destination<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RouteDestination>,
        T::Error: ::std::fmt::Display,
    {
        self.destination = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for destination: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RouterRouteKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn target<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RouteTarget>,
        T::Error: ::std::fmt::Display,
    {
        self.target = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn vpc_router_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.vpc_router_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vpc_router_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<RouterRoute> for super::RouterRoute {
    type Error = super::error::ConversionError;
    fn try_from(value: RouterRoute) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            destination: value.destination?,
            id: value.id?,
            kind: value.kind?,
            name: value.name?,
            target: value.target?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            vpc_router_id: value.vpc_router_id?,
        })
    }
}

impl ::std::convert::From<super::RouterRoute> for RouterRoute {
    fn from(value: super::RouterRoute) -> Self {
        Self {
            description: Ok(value.description),
            destination: Ok(value.destination),
            id: Ok(value.id),
            kind: Ok(value.kind),
            name: Ok(value.name),
            target: Ok(value.target),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            vpc_router_id: Ok(value.vpc_router_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RouterRouteCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    destination: ::std::result::Result<super::RouteDestination, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    target: ::std::result::Result<super::RouteTarget, ::std::string::String>,
}

impl ::std::default::Default for RouterRouteCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            destination: Err("no value supplied for destination".to_string()),
            name: Err("no value supplied for name".to_string()),
            target: Err("no value supplied for target".to_string()),
        }
    }
}

impl RouterRouteCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn destination<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RouteDestination>,
        T::Error: ::std::fmt::Display,
    {
        self.destination = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for destination: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn target<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RouteTarget>,
        T::Error: ::std::fmt::Display,
    {
        self.target = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<RouterRouteCreate> for super::RouterRouteCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: RouterRouteCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            destination: value.destination?,
            name: value.name?,
            target: value.target?,
        })
    }
}

impl ::std::convert::From<super::RouterRouteCreate> for RouterRouteCreate {
    fn from(value: super::RouterRouteCreate) -> Self {
        Self {
            description: Ok(value.description),
            destination: Ok(value.destination),
            name: Ok(value.name),
            target: Ok(value.target),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RouterRouteResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::RouterRoute>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for RouterRouteResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl RouterRouteResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::RouterRoute>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<RouterRouteResultsPage> for super::RouterRouteResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: RouterRouteResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::RouterRouteResultsPage> for RouterRouteResultsPage {
    fn from(value: super::RouterRouteResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RouterRouteUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    destination: ::std::result::Result<super::RouteDestination, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
    target: ::std::result::Result<super::RouteTarget, ::std::string::String>,
}

impl ::std::default::Default for RouterRouteUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            destination: Err("no value supplied for destination".to_string()),
            name: Ok(Default::default()),
            target: Err("no value supplied for target".to_string()),
        }
    }
}

impl RouterRouteUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn destination<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RouteDestination>,
        T::Error: ::std::fmt::Display,
    {
        self.destination = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for destination: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn target<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::RouteTarget>,
        T::Error: ::std::fmt::Display,
    {
        self.target = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<RouterRouteUpdate> for super::RouterRouteUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: RouterRouteUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            destination: value.destination?,
            name: value.name?,
            target: value.target?,
        })
    }
}

impl ::std::convert::From<super::RouterRouteUpdate> for RouterRouteUpdate {
    fn from(value: super::RouterRouteUpdate) -> Self {
        Self {
            description: Ok(value.description),
            destination: Ok(value.destination),
            name: Ok(value.name),
            target: Ok(value.target),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SamlIdentityProvider {
    acs_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    group_attribute_name:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    idp_entity_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    public_cert:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    slo_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    sp_client_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    technical_contact_email: ::std::result::Result<::std::string::String, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for SamlIdentityProvider {
    fn default() -> Self {
        Self {
            acs_url: Err("no value supplied for acs_url".to_string()),
            description: Err("no value supplied for description".to_string()),
            group_attribute_name: Ok(Default::default()),
            id: Err("no value supplied for id".to_string()),
            idp_entity_id: Err("no value supplied for idp_entity_id".to_string()),
            name: Err("no value supplied for name".to_string()),
            public_cert: Ok(Default::default()),
            slo_url: Err("no value supplied for slo_url".to_string()),
            sp_client_id: Err("no value supplied for sp_client_id".to_string()),
            technical_contact_email: Err(
                "no value supplied for technical_contact_email".to_string()
            ),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl SamlIdentityProvider {
    pub fn acs_url<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.acs_url = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for acs_url: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn group_attribute_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.group_attribute_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for group_attribute_name: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn idp_entity_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.idp_entity_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for idp_entity_id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn public_cert<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.public_cert = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for public_cert: {e}"));
        self
    }

    pub fn slo_url<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.slo_url = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for slo_url: {e}"));
        self
    }

    pub fn sp_client_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.sp_client_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sp_client_id: {e}"));
        self
    }

    pub fn technical_contact_email<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.technical_contact_email = value.try_into().map_err(|e| {
            format!("error converting supplied value for technical_contact_email: {e}")
        });
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SamlIdentityProvider> for super::SamlIdentityProvider {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SamlIdentityProvider,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            acs_url: value.acs_url?,
            description: value.description?,
            group_attribute_name: value.group_attribute_name?,
            id: value.id?,
            idp_entity_id: value.idp_entity_id?,
            name: value.name?,
            public_cert: value.public_cert?,
            slo_url: value.slo_url?,
            sp_client_id: value.sp_client_id?,
            technical_contact_email: value.technical_contact_email?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::SamlIdentityProvider> for SamlIdentityProvider {
    fn from(value: super::SamlIdentityProvider) -> Self {
        Self {
            acs_url: Ok(value.acs_url),
            description: Ok(value.description),
            group_attribute_name: Ok(value.group_attribute_name),
            id: Ok(value.id),
            idp_entity_id: Ok(value.idp_entity_id),
            name: Ok(value.name),
            public_cert: Ok(value.public_cert),
            slo_url: Ok(value.slo_url),
            sp_client_id: Ok(value.sp_client_id),
            technical_contact_email: Ok(value.technical_contact_email),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SamlIdentityProviderCreate {
    acs_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    group_attribute_name:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    idp_entity_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    idp_metadata_source: ::std::result::Result<super::IdpMetadataSource, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    signing_keypair: ::std::result::Result<
        ::std::option::Option<super::DerEncodedKeyPair>,
        ::std::string::String,
    >,
    slo_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    sp_client_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    technical_contact_email: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for SamlIdentityProviderCreate {
    fn default() -> Self {
        Self {
            acs_url: Err("no value supplied for acs_url".to_string()),
            description: Err("no value supplied for description".to_string()),
            group_attribute_name: Ok(Default::default()),
            idp_entity_id: Err("no value supplied for idp_entity_id".to_string()),
            idp_metadata_source: Err("no value supplied for idp_metadata_source".to_string()),
            name: Err("no value supplied for name".to_string()),
            signing_keypair: Ok(Default::default()),
            slo_url: Err("no value supplied for slo_url".to_string()),
            sp_client_id: Err("no value supplied for sp_client_id".to_string()),
            technical_contact_email: Err(
                "no value supplied for technical_contact_email".to_string()
            ),
        }
    }
}

impl SamlIdentityProviderCreate {
    pub fn acs_url<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.acs_url = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for acs_url: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn group_attribute_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.group_attribute_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for group_attribute_name: {e}"));
        self
    }

    pub fn idp_entity_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.idp_entity_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for idp_entity_id: {e}"));
        self
    }

    pub fn idp_metadata_source<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IdpMetadataSource>,
        T::Error: ::std::fmt::Display,
    {
        self.idp_metadata_source = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for idp_metadata_source: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn signing_keypair<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::DerEncodedKeyPair>>,
        T::Error: ::std::fmt::Display,
    {
        self.signing_keypair = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for signing_keypair: {e}"));
        self
    }

    pub fn slo_url<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.slo_url = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for slo_url: {e}"));
        self
    }

    pub fn sp_client_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.sp_client_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sp_client_id: {e}"));
        self
    }

    pub fn technical_contact_email<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.technical_contact_email = value.try_into().map_err(|e| {
            format!("error converting supplied value for technical_contact_email: {e}")
        });
        self
    }
}

impl ::std::convert::TryFrom<SamlIdentityProviderCreate> for super::SamlIdentityProviderCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SamlIdentityProviderCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            acs_url: value.acs_url?,
            description: value.description?,
            group_attribute_name: value.group_attribute_name?,
            idp_entity_id: value.idp_entity_id?,
            idp_metadata_source: value.idp_metadata_source?,
            name: value.name?,
            signing_keypair: value.signing_keypair?,
            slo_url: value.slo_url?,
            sp_client_id: value.sp_client_id?,
            technical_contact_email: value.technical_contact_email?,
        })
    }
}

impl ::std::convert::From<super::SamlIdentityProviderCreate> for SamlIdentityProviderCreate {
    fn from(value: super::SamlIdentityProviderCreate) -> Self {
        Self {
            acs_url: Ok(value.acs_url),
            description: Ok(value.description),
            group_attribute_name: Ok(value.group_attribute_name),
            idp_entity_id: Ok(value.idp_entity_id),
            idp_metadata_source: Ok(value.idp_metadata_source),
            name: Ok(value.name),
            signing_keypair: Ok(value.signing_keypair),
            slo_url: Ok(value.slo_url),
            sp_client_id: Ok(value.sp_client_id),
            technical_contact_email: Ok(value.technical_contact_email),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScimClientBearerToken {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_expires: ::std::result::Result<
        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for ScimClientBearerToken {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_expires: Ok(Default::default()),
        }
    }
}

impl ScimClientBearerToken {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_expires<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.time_expires = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_expires: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ScimClientBearerToken> for super::ScimClientBearerToken {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ScimClientBearerToken,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            time_created: value.time_created?,
            time_expires: value.time_expires?,
        })
    }
}

impl ::std::convert::From<super::ScimClientBearerToken> for ScimClientBearerToken {
    fn from(value: super::ScimClientBearerToken) -> Self {
        Self {
            id: Ok(value.id),
            time_created: Ok(value.time_created),
            time_expires: Ok(value.time_expires),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScimClientBearerTokenValue {
    bearer_token: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_expires: ::std::result::Result<
        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for ScimClientBearerTokenValue {
    fn default() -> Self {
        Self {
            bearer_token: Err("no value supplied for bearer_token".to_string()),
            id: Err("no value supplied for id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_expires: Ok(Default::default()),
        }
    }
}

impl ScimClientBearerTokenValue {
    pub fn bearer_token<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.bearer_token = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bearer_token: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_expires<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.time_expires = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_expires: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ScimClientBearerTokenValue> for super::ScimClientBearerTokenValue {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ScimClientBearerTokenValue,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            bearer_token: value.bearer_token?,
            id: value.id?,
            time_created: value.time_created?,
            time_expires: value.time_expires?,
        })
    }
}

impl ::std::convert::From<super::ScimClientBearerTokenValue> for ScimClientBearerTokenValue {
    fn from(value: super::ScimClientBearerTokenValue) -> Self {
        Self {
            bearer_token: Ok(value.bearer_token),
            id: Ok(value.id),
            time_created: Ok(value.time_created),
            time_expires: Ok(value.time_expires),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServiceIcmpConfig {
    enabled: ::std::result::Result<bool, ::std::string::String>,
}

impl ::std::default::Default for ServiceIcmpConfig {
    fn default() -> Self {
        Self {
            enabled: Err("no value supplied for enabled".to_string()),
        }
    }
}

impl ServiceIcmpConfig {
    pub fn enabled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.enabled = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for enabled: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<ServiceIcmpConfig> for super::ServiceIcmpConfig {
    type Error = super::error::ConversionError;
    fn try_from(
        value: ServiceIcmpConfig,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            enabled: value.enabled?,
        })
    }
}

impl ::std::convert::From<super::ServiceIcmpConfig> for ServiceIcmpConfig {
    fn from(value: super::ServiceIcmpConfig) -> Self {
        Self {
            enabled: Ok(value.enabled),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SetTargetReleaseParams {
    system_version:
        ::std::result::Result<super::SetTargetReleaseParamsSystemVersion, ::std::string::String>,
}

impl ::std::default::Default for SetTargetReleaseParams {
    fn default() -> Self {
        Self {
            system_version: Err("no value supplied for system_version".to_string()),
        }
    }
}

impl SetTargetReleaseParams {
    pub fn system_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SetTargetReleaseParamsSystemVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.system_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for system_version: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SetTargetReleaseParams> for super::SetTargetReleaseParams {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SetTargetReleaseParams,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            system_version: value.system_version?,
        })
    }
}

impl ::std::convert::From<super::SetTargetReleaseParams> for SetTargetReleaseParams {
    fn from(value: super::SetTargetReleaseParams) -> Self {
        Self {
            system_version: Ok(value.system_version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Silo {
    admin_group_name:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    discoverable: ::std::result::Result<bool, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    identity_mode: ::std::result::Result<super::SiloIdentityMode, ::std::string::String>,
    mapped_fleet_roles: ::std::result::Result<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::FleetRole>>,
        ::std::string::String,
    >,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Silo {
    fn default() -> Self {
        Self {
            admin_group_name: Ok(Default::default()),
            description: Err("no value supplied for description".to_string()),
            discoverable: Err("no value supplied for discoverable".to_string()),
            id: Err("no value supplied for id".to_string()),
            identity_mode: Err("no value supplied for identity_mode".to_string()),
            mapped_fleet_roles: Err("no value supplied for mapped_fleet_roles".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Silo {
    pub fn admin_group_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.admin_group_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for admin_group_name: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn discoverable<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.discoverable = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for discoverable: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn identity_mode<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SiloIdentityMode>,
        T::Error: ::std::fmt::Display,
    {
        self.identity_mode = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_mode: {e}"));
        self
    }

    pub fn mapped_fleet_roles<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::FleetRole>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.mapped_fleet_roles = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for mapped_fleet_roles: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Silo> for super::Silo {
    type Error = super::error::ConversionError;
    fn try_from(value: Silo) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            admin_group_name: value.admin_group_name?,
            description: value.description?,
            discoverable: value.discoverable?,
            id: value.id?,
            identity_mode: value.identity_mode?,
            mapped_fleet_roles: value.mapped_fleet_roles?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Silo> for Silo {
    fn from(value: super::Silo) -> Self {
        Self {
            admin_group_name: Ok(value.admin_group_name),
            description: Ok(value.description),
            discoverable: Ok(value.discoverable),
            id: Ok(value.id),
            identity_mode: Ok(value.identity_mode),
            mapped_fleet_roles: Ok(value.mapped_fleet_roles),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloAuthSettings {
    device_token_max_ttl_seconds:
        ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
    silo_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for SiloAuthSettings {
    fn default() -> Self {
        Self {
            device_token_max_ttl_seconds: Ok(Default::default()),
            silo_id: Err("no value supplied for silo_id".to_string()),
        }
    }
}

impl SiloAuthSettings {
    pub fn device_token_max_ttl_seconds<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u32>>,
        T::Error: ::std::fmt::Display,
    {
        self.device_token_max_ttl_seconds = value.try_into().map_err(|e| {
            format!("error converting supplied value for device_token_max_ttl_seconds: {e}")
        });
        self
    }

    pub fn silo_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloAuthSettings> for super::SiloAuthSettings {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloAuthSettings,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            device_token_max_ttl_seconds: value.device_token_max_ttl_seconds?,
            silo_id: value.silo_id?,
        })
    }
}

impl ::std::convert::From<super::SiloAuthSettings> for SiloAuthSettings {
    fn from(value: super::SiloAuthSettings) -> Self {
        Self {
            device_token_max_ttl_seconds: Ok(value.device_token_max_ttl_seconds),
            silo_id: Ok(value.silo_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloAuthSettingsUpdate {
    device_token_max_ttl_seconds:
        ::std::result::Result<::std::option::Option<::std::num::NonZeroU32>, ::std::string::String>,
}

impl ::std::default::Default for SiloAuthSettingsUpdate {
    fn default() -> Self {
        Self {
            device_token_max_ttl_seconds: Err(
                "no value supplied for device_token_max_ttl_seconds".to_string()
            ),
        }
    }
}

impl SiloAuthSettingsUpdate {
    pub fn device_token_max_ttl_seconds<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU32>>,
        T::Error: ::std::fmt::Display,
    {
        self.device_token_max_ttl_seconds = value.try_into().map_err(|e| {
            format!("error converting supplied value for device_token_max_ttl_seconds: {e}")
        });
        self
    }
}

impl ::std::convert::TryFrom<SiloAuthSettingsUpdate> for super::SiloAuthSettingsUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloAuthSettingsUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            device_token_max_ttl_seconds: value.device_token_max_ttl_seconds?,
        })
    }
}

impl ::std::convert::From<super::SiloAuthSettingsUpdate> for SiloAuthSettingsUpdate {
    fn from(value: super::SiloAuthSettingsUpdate) -> Self {
        Self {
            device_token_max_ttl_seconds: Ok(value.device_token_max_ttl_seconds),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloCreate {
    admin_group_name:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    identity_mode: ::std::result::Result<super::SiloIdentityMode, ::std::string::String>,
    mapped_fleet_roles: ::std::result::Result<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::FleetRole>>,
        ::std::string::String,
    >,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    quotas: ::std::result::Result<super::SiloQuotasCreate, ::std::string::String>,
    tls_certificates:
        ::std::result::Result<::std::vec::Vec<super::CertificateCreate>, ::std::string::String>,
}

impl ::std::default::Default for SiloCreate {
    fn default() -> Self {
        Self {
            admin_group_name: Ok(Default::default()),
            description: Err("no value supplied for description".to_string()),
            identity_mode: Err("no value supplied for identity_mode".to_string()),
            mapped_fleet_roles: Ok(Default::default()),
            name: Err("no value supplied for name".to_string()),
            quotas: Err("no value supplied for quotas".to_string()),
            tls_certificates: Err("no value supplied for tls_certificates".to_string()),
        }
    }
}

impl SiloCreate {
    pub fn admin_group_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.admin_group_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for admin_group_name: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn identity_mode<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SiloIdentityMode>,
        T::Error: ::std::fmt::Display,
    {
        self.identity_mode = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_mode: {e}"));
        self
    }

    pub fn mapped_fleet_roles<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<super::FleetRole>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.mapped_fleet_roles = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for mapped_fleet_roles: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn quotas<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SiloQuotasCreate>,
        T::Error: ::std::fmt::Display,
    {
        self.quotas = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for quotas: {e}"));
        self
    }

    pub fn tls_certificates<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::CertificateCreate>>,
        T::Error: ::std::fmt::Display,
    {
        self.tls_certificates = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for tls_certificates: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloCreate> for super::SiloCreate {
    type Error = super::error::ConversionError;
    fn try_from(value: SiloCreate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            admin_group_name: value.admin_group_name?,
            description: value.description?,
            identity_mode: value.identity_mode?,
            mapped_fleet_roles: value.mapped_fleet_roles?,
            name: value.name?,
            quotas: value.quotas?,
            tls_certificates: value.tls_certificates?,
        })
    }
}

impl ::std::convert::From<super::SiloCreate> for SiloCreate {
    fn from(value: super::SiloCreate) -> Self {
        Self {
            admin_group_name: Ok(value.admin_group_name),
            description: Ok(value.description),
            identity_mode: Ok(value.identity_mode),
            mapped_fleet_roles: Ok(value.mapped_fleet_roles),
            name: Ok(value.name),
            quotas: Ok(value.quotas),
            tls_certificates: Ok(value.tls_certificates),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloIpPool {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ip_version: ::std::result::Result<super::IpVersion, ::std::string::String>,
    is_default: ::std::result::Result<bool, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    pool_type: ::std::result::Result<super::IpPoolType, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for SiloIpPool {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            ip_version: Err("no value supplied for ip_version".to_string()),
            is_default: Err("no value supplied for is_default".to_string()),
            name: Err("no value supplied for name".to_string()),
            pool_type: Err("no value supplied for pool_type".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl SiloIpPool {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn ip_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_version: {e}"));
        self
    }

    pub fn is_default<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.is_default = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for is_default: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn pool_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpPoolType>,
        T::Error: ::std::fmt::Display,
    {
        self.pool_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for pool_type: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloIpPool> for super::SiloIpPool {
    type Error = super::error::ConversionError;
    fn try_from(value: SiloIpPool) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            ip_version: value.ip_version?,
            is_default: value.is_default?,
            name: value.name?,
            pool_type: value.pool_type?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::SiloIpPool> for SiloIpPool {
    fn from(value: super::SiloIpPool) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            ip_version: Ok(value.ip_version),
            is_default: Ok(value.is_default),
            name: Ok(value.name),
            pool_type: Ok(value.pool_type),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloIpPoolResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SiloIpPool>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SiloIpPoolResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SiloIpPoolResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SiloIpPool>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloIpPoolResultsPage> for super::SiloIpPoolResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloIpPoolResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SiloIpPoolResultsPage> for SiloIpPoolResultsPage {
    fn from(value: super::SiloIpPoolResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloQuotas {
    cpus: ::std::result::Result<i64, ::std::string::String>,
    memory: ::std::result::Result<super::ByteCount, ::std::string::String>,
    silo_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    storage: ::std::result::Result<super::ByteCount, ::std::string::String>,
}

impl ::std::default::Default for SiloQuotas {
    fn default() -> Self {
        Self {
            cpus: Err("no value supplied for cpus".to_string()),
            memory: Err("no value supplied for memory".to_string()),
            silo_id: Err("no value supplied for silo_id".to_string()),
            storage: Err("no value supplied for storage".to_string()),
        }
    }
}

impl SiloQuotas {
    pub fn cpus<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.cpus = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cpus: {e}"));
        self
    }

    pub fn memory<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.memory = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for memory: {e}"));
        self
    }

    pub fn silo_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_id: {e}"));
        self
    }

    pub fn storage<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.storage = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for storage: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloQuotas> for super::SiloQuotas {
    type Error = super::error::ConversionError;
    fn try_from(value: SiloQuotas) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            cpus: value.cpus?,
            memory: value.memory?,
            silo_id: value.silo_id?,
            storage: value.storage?,
        })
    }
}

impl ::std::convert::From<super::SiloQuotas> for SiloQuotas {
    fn from(value: super::SiloQuotas) -> Self {
        Self {
            cpus: Ok(value.cpus),
            memory: Ok(value.memory),
            silo_id: Ok(value.silo_id),
            storage: Ok(value.storage),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloQuotasCreate {
    cpus: ::std::result::Result<i64, ::std::string::String>,
    memory: ::std::result::Result<super::ByteCount, ::std::string::String>,
    storage: ::std::result::Result<super::ByteCount, ::std::string::String>,
}

impl ::std::default::Default for SiloQuotasCreate {
    fn default() -> Self {
        Self {
            cpus: Err("no value supplied for cpus".to_string()),
            memory: Err("no value supplied for memory".to_string()),
            storage: Err("no value supplied for storage".to_string()),
        }
    }
}

impl SiloQuotasCreate {
    pub fn cpus<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.cpus = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cpus: {e}"));
        self
    }

    pub fn memory<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.memory = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for memory: {e}"));
        self
    }

    pub fn storage<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.storage = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for storage: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloQuotasCreate> for super::SiloQuotasCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloQuotasCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            cpus: value.cpus?,
            memory: value.memory?,
            storage: value.storage?,
        })
    }
}

impl ::std::convert::From<super::SiloQuotasCreate> for SiloQuotasCreate {
    fn from(value: super::SiloQuotasCreate) -> Self {
        Self {
            cpus: Ok(value.cpus),
            memory: Ok(value.memory),
            storage: Ok(value.storage),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloQuotasResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SiloQuotas>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SiloQuotasResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SiloQuotasResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SiloQuotas>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloQuotasResultsPage> for super::SiloQuotasResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloQuotasResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SiloQuotasResultsPage> for SiloQuotasResultsPage {
    fn from(value: super::SiloQuotasResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloQuotasUpdate {
    cpus: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    memory: ::std::result::Result<::std::option::Option<super::ByteCount>, ::std::string::String>,
    storage: ::std::result::Result<::std::option::Option<super::ByteCount>, ::std::string::String>,
}

impl ::std::default::Default for SiloQuotasUpdate {
    fn default() -> Self {
        Self {
            cpus: Ok(Default::default()),
            memory: Ok(Default::default()),
            storage: Ok(Default::default()),
        }
    }
}

impl SiloQuotasUpdate {
    pub fn cpus<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<i64>>,
        T::Error: ::std::fmt::Display,
    {
        self.cpus = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cpus: {e}"));
        self
    }

    pub fn memory<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::ByteCount>>,
        T::Error: ::std::fmt::Display,
    {
        self.memory = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for memory: {e}"));
        self
    }

    pub fn storage<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::ByteCount>>,
        T::Error: ::std::fmt::Display,
    {
        self.storage = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for storage: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloQuotasUpdate> for super::SiloQuotasUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloQuotasUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            cpus: value.cpus?,
            memory: value.memory?,
            storage: value.storage?,
        })
    }
}

impl ::std::convert::From<super::SiloQuotasUpdate> for SiloQuotasUpdate {
    fn from(value: super::SiloQuotasUpdate) -> Self {
        Self {
            cpus: Ok(value.cpus),
            memory: Ok(value.memory),
            storage: Ok(value.storage),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Silo>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SiloResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SiloResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Silo>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloResultsPage> for super::SiloResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SiloResultsPage> for SiloResultsPage {
    fn from(value: super::SiloResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloRolePolicy {
    role_assignments: ::std::result::Result<
        ::std::vec::Vec<super::SiloRoleRoleAssignment>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for SiloRolePolicy {
    fn default() -> Self {
        Self {
            role_assignments: Err("no value supplied for role_assignments".to_string()),
        }
    }
}

impl SiloRolePolicy {
    pub fn role_assignments<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SiloRoleRoleAssignment>>,
        T::Error: ::std::fmt::Display,
    {
        self.role_assignments = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for role_assignments: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloRolePolicy> for super::SiloRolePolicy {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloRolePolicy,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            role_assignments: value.role_assignments?,
        })
    }
}

impl ::std::convert::From<super::SiloRolePolicy> for SiloRolePolicy {
    fn from(value: super::SiloRolePolicy) -> Self {
        Self {
            role_assignments: Ok(value.role_assignments),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloRoleRoleAssignment {
    identity_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    identity_type: ::std::result::Result<super::IdentityType, ::std::string::String>,
    role_name: ::std::result::Result<super::SiloRole, ::std::string::String>,
}

impl ::std::default::Default for SiloRoleRoleAssignment {
    fn default() -> Self {
        Self {
            identity_id: Err("no value supplied for identity_id".to_string()),
            identity_type: Err("no value supplied for identity_type".to_string()),
            role_name: Err("no value supplied for role_name".to_string()),
        }
    }
}

impl SiloRoleRoleAssignment {
    pub fn identity_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.identity_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_id: {e}"));
        self
    }

    pub fn identity_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IdentityType>,
        T::Error: ::std::fmt::Display,
    {
        self.identity_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for identity_type: {e}"));
        self
    }

    pub fn role_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SiloRole>,
        T::Error: ::std::fmt::Display,
    {
        self.role_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for role_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloRoleRoleAssignment> for super::SiloRoleRoleAssignment {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloRoleRoleAssignment,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            identity_id: value.identity_id?,
            identity_type: value.identity_type?,
            role_name: value.role_name?,
        })
    }
}

impl ::std::convert::From<super::SiloRoleRoleAssignment> for SiloRoleRoleAssignment {
    fn from(value: super::SiloRoleRoleAssignment) -> Self {
        Self {
            identity_id: Ok(value.identity_id),
            identity_type: Ok(value.identity_type),
            role_name: Ok(value.role_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloSubnetPool {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ip_version: ::std::result::Result<super::IpVersion, ::std::string::String>,
    is_default: ::std::result::Result<bool, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for SiloSubnetPool {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            ip_version: Err("no value supplied for ip_version".to_string()),
            is_default: Err("no value supplied for is_default".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl SiloSubnetPool {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn ip_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_version: {e}"));
        self
    }

    pub fn is_default<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.is_default = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for is_default: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloSubnetPool> for super::SiloSubnetPool {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloSubnetPool,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            ip_version: value.ip_version?,
            is_default: value.is_default?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::SiloSubnetPool> for SiloSubnetPool {
    fn from(value: super::SiloSubnetPool) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            ip_version: Ok(value.ip_version),
            is_default: Ok(value.is_default),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloSubnetPoolResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SiloSubnetPool>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SiloSubnetPoolResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SiloSubnetPoolResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SiloSubnetPool>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloSubnetPoolResultsPage> for super::SiloSubnetPoolResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloSubnetPoolResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SiloSubnetPoolResultsPage> for SiloSubnetPoolResultsPage {
    fn from(value: super::SiloSubnetPoolResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloUtilization {
    allocated: ::std::result::Result<super::VirtualResourceCounts, ::std::string::String>,
    provisioned: ::std::result::Result<super::VirtualResourceCounts, ::std::string::String>,
    silo_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    silo_name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for SiloUtilization {
    fn default() -> Self {
        Self {
            allocated: Err("no value supplied for allocated".to_string()),
            provisioned: Err("no value supplied for provisioned".to_string()),
            silo_id: Err("no value supplied for silo_id".to_string()),
            silo_name: Err("no value supplied for silo_name".to_string()),
        }
    }
}

impl SiloUtilization {
    pub fn allocated<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VirtualResourceCounts>,
        T::Error: ::std::fmt::Display,
    {
        self.allocated = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for allocated: {e}"));
        self
    }

    pub fn provisioned<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VirtualResourceCounts>,
        T::Error: ::std::fmt::Display,
    {
        self.provisioned = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for provisioned: {e}"));
        self
    }

    pub fn silo_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_id: {e}"));
        self
    }

    pub fn silo_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloUtilization> for super::SiloUtilization {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloUtilization,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            allocated: value.allocated?,
            provisioned: value.provisioned?,
            silo_id: value.silo_id?,
            silo_name: value.silo_name?,
        })
    }
}

impl ::std::convert::From<super::SiloUtilization> for SiloUtilization {
    fn from(value: super::SiloUtilization) -> Self {
        Self {
            allocated: Ok(value.allocated),
            provisioned: Ok(value.provisioned),
            silo_id: Ok(value.silo_id),
            silo_name: Ok(value.silo_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SiloUtilizationResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SiloUtilization>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SiloUtilizationResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SiloUtilizationResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SiloUtilization>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SiloUtilizationResultsPage> for super::SiloUtilizationResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SiloUtilizationResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SiloUtilizationResultsPage> for SiloUtilizationResultsPage {
    fn from(value: super::SiloUtilizationResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sled {
    baseboard: ::std::result::Result<super::Baseboard, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    policy: ::std::result::Result<super::SledPolicy, ::std::string::String>,
    rack_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    slot: ::std::result::Result<::std::option::Option<u16>, ::std::string::String>,
    state: ::std::result::Result<super::SledState, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    usable_hardware_threads: ::std::result::Result<u32, ::std::string::String>,
    usable_physical_ram: ::std::result::Result<super::ByteCount, ::std::string::String>,
}

impl ::std::default::Default for Sled {
    fn default() -> Self {
        Self {
            baseboard: Err("no value supplied for baseboard".to_string()),
            id: Err("no value supplied for id".to_string()),
            policy: Err("no value supplied for policy".to_string()),
            rack_id: Err("no value supplied for rack_id".to_string()),
            slot: Ok(Default::default()),
            state: Err("no value supplied for state".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            usable_hardware_threads: Err(
                "no value supplied for usable_hardware_threads".to_string()
            ),
            usable_physical_ram: Err("no value supplied for usable_physical_ram".to_string()),
        }
    }
}

impl Sled {
    pub fn baseboard<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Baseboard>,
        T::Error: ::std::fmt::Display,
    {
        self.baseboard = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for baseboard: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn policy<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SledPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.policy = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for policy: {e}"));
        self
    }

    pub fn rack_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.rack_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rack_id: {e}"));
        self
    }

    pub fn slot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u16>>,
        T::Error: ::std::fmt::Display,
    {
        self.slot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for slot: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SledState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn usable_hardware_threads<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.usable_hardware_threads = value.try_into().map_err(|e| {
            format!("error converting supplied value for usable_hardware_threads: {e}")
        });
        self
    }

    pub fn usable_physical_ram<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.usable_physical_ram = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for usable_physical_ram: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Sled> for super::Sled {
    type Error = super::error::ConversionError;
    fn try_from(value: Sled) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            baseboard: value.baseboard?,
            id: value.id?,
            policy: value.policy?,
            rack_id: value.rack_id?,
            slot: value.slot?,
            state: value.state?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            usable_hardware_threads: value.usable_hardware_threads?,
            usable_physical_ram: value.usable_physical_ram?,
        })
    }
}

impl ::std::convert::From<super::Sled> for Sled {
    fn from(value: super::Sled) -> Self {
        Self {
            baseboard: Ok(value.baseboard),
            id: Ok(value.id),
            policy: Ok(value.policy),
            rack_id: Ok(value.rack_id),
            slot: Ok(value.slot),
            state: Ok(value.state),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            usable_hardware_threads: Ok(value.usable_hardware_threads),
            usable_physical_ram: Ok(value.usable_physical_ram),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SledInstance {
    active_sled_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    memory: ::std::result::Result<i64, ::std::string::String>,
    migration_id: ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    ncpus: ::std::result::Result<i64, ::std::string::String>,
    project_name: ::std::result::Result<super::Name, ::std::string::String>,
    silo_name: ::std::result::Result<super::Name, ::std::string::String>,
    state: ::std::result::Result<super::InstanceState, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for SledInstance {
    fn default() -> Self {
        Self {
            active_sled_id: Err("no value supplied for active_sled_id".to_string()),
            id: Err("no value supplied for id".to_string()),
            memory: Err("no value supplied for memory".to_string()),
            migration_id: Ok(Default::default()),
            name: Err("no value supplied for name".to_string()),
            ncpus: Err("no value supplied for ncpus".to_string()),
            project_name: Err("no value supplied for project_name".to_string()),
            silo_name: Err("no value supplied for silo_name".to_string()),
            state: Err("no value supplied for state".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl SledInstance {
    pub fn active_sled_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.active_sled_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for active_sled_id: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn memory<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.memory = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for memory: {e}"));
        self
    }

    pub fn migration_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.migration_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for migration_id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn ncpus<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.ncpus = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ncpus: {e}"));
        self
    }

    pub fn project_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.project_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_name: {e}"));
        self
    }

    pub fn silo_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_name: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::InstanceState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SledInstance> for super::SledInstance {
    type Error = super::error::ConversionError;
    fn try_from(value: SledInstance) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            active_sled_id: value.active_sled_id?,
            id: value.id?,
            memory: value.memory?,
            migration_id: value.migration_id?,
            name: value.name?,
            ncpus: value.ncpus?,
            project_name: value.project_name?,
            silo_name: value.silo_name?,
            state: value.state?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::SledInstance> for SledInstance {
    fn from(value: super::SledInstance) -> Self {
        Self {
            active_sled_id: Ok(value.active_sled_id),
            id: Ok(value.id),
            memory: Ok(value.memory),
            migration_id: Ok(value.migration_id),
            name: Ok(value.name),
            ncpus: Ok(value.ncpus),
            project_name: Ok(value.project_name),
            silo_name: Ok(value.silo_name),
            state: Ok(value.state),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SledInstanceResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SledInstance>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SledInstanceResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SledInstanceResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SledInstance>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SledInstanceResultsPage> for super::SledInstanceResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SledInstanceResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SledInstanceResultsPage> for SledInstanceResultsPage {
    fn from(value: super::SledInstanceResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SledProvisionPolicyParams {
    state: ::std::result::Result<super::SledProvisionPolicy, ::std::string::String>,
}

impl ::std::default::Default for SledProvisionPolicyParams {
    fn default() -> Self {
        Self {
            state: Err("no value supplied for state".to_string()),
        }
    }
}

impl SledProvisionPolicyParams {
    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SledProvisionPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SledProvisionPolicyParams> for super::SledProvisionPolicyParams {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SledProvisionPolicyParams,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            state: value.state?,
        })
    }
}

impl ::std::convert::From<super::SledProvisionPolicyParams> for SledProvisionPolicyParams {
    fn from(value: super::SledProvisionPolicyParams) -> Self {
        Self {
            state: Ok(value.state),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SledProvisionPolicyResponse {
    new_state: ::std::result::Result<super::SledProvisionPolicy, ::std::string::String>,
    old_state: ::std::result::Result<super::SledProvisionPolicy, ::std::string::String>,
}

impl ::std::default::Default for SledProvisionPolicyResponse {
    fn default() -> Self {
        Self {
            new_state: Err("no value supplied for new_state".to_string()),
            old_state: Err("no value supplied for old_state".to_string()),
        }
    }
}

impl SledProvisionPolicyResponse {
    pub fn new_state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SledProvisionPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.new_state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for new_state: {e}"));
        self
    }

    pub fn old_state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SledProvisionPolicy>,
        T::Error: ::std::fmt::Display,
    {
        self.old_state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for old_state: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SledProvisionPolicyResponse> for super::SledProvisionPolicyResponse {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SledProvisionPolicyResponse,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            new_state: value.new_state?,
            old_state: value.old_state?,
        })
    }
}

impl ::std::convert::From<super::SledProvisionPolicyResponse> for SledProvisionPolicyResponse {
    fn from(value: super::SledProvisionPolicyResponse) -> Self {
        Self {
            new_state: Ok(value.new_state),
            old_state: Ok(value.old_state),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SledResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Sled>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SledResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SledResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Sled>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SledResultsPage> for super::SledResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SledResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SledResultsPage> for SledResultsPage {
    fn from(value: super::SledResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Snapshot {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    disk_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    project_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    size: ::std::result::Result<super::ByteCount, ::std::string::String>,
    state: ::std::result::Result<super::SnapshotState, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Snapshot {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            disk_id: Err("no value supplied for disk_id".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            project_id: Err("no value supplied for project_id".to_string()),
            size: Err("no value supplied for size".to_string()),
            state: Err("no value supplied for state".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Snapshot {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn disk_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.disk_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for disk_id: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn project_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.project_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_id: {e}"));
        self
    }

    pub fn size<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.size = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for size: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SnapshotState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Snapshot> for super::Snapshot {
    type Error = super::error::ConversionError;
    fn try_from(value: Snapshot) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            disk_id: value.disk_id?,
            id: value.id?,
            name: value.name?,
            project_id: value.project_id?,
            size: value.size?,
            state: value.state?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Snapshot> for Snapshot {
    fn from(value: super::Snapshot) -> Self {
        Self {
            description: Ok(value.description),
            disk_id: Ok(value.disk_id),
            id: Ok(value.id),
            name: Ok(value.name),
            project_id: Ok(value.project_id),
            size: Ok(value.size),
            state: Ok(value.state),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SnapshotCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    disk: ::std::result::Result<super::NameOrId, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for SnapshotCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            disk: Err("no value supplied for disk".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl SnapshotCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn disk<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.disk = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for disk: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SnapshotCreate> for super::SnapshotCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SnapshotCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            disk: value.disk?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::SnapshotCreate> for SnapshotCreate {
    fn from(value: super::SnapshotCreate) -> Self {
        Self {
            description: Ok(value.description),
            disk: Ok(value.disk),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SnapshotResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Snapshot>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SnapshotResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SnapshotResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Snapshot>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SnapshotResultsPage> for super::SnapshotResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SnapshotResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SnapshotResultsPage> for SnapshotResultsPage {
    fn from(value: super::SnapshotResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SshKey {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    public_key: ::std::result::Result<::std::string::String, ::std::string::String>,
    silo_user_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for SshKey {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            public_key: Err("no value supplied for public_key".to_string()),
            silo_user_id: Err("no value supplied for silo_user_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl SshKey {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn public_key<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.public_key = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for public_key: {e}"));
        self
    }

    pub fn silo_user_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_user_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_user_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SshKey> for super::SshKey {
    type Error = super::error::ConversionError;
    fn try_from(value: SshKey) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            name: value.name?,
            public_key: value.public_key?,
            silo_user_id: value.silo_user_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::SshKey> for SshKey {
    fn from(value: super::SshKey) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            name: Ok(value.name),
            public_key: Ok(value.public_key),
            silo_user_id: Ok(value.silo_user_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SshKeyCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    public_key: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for SshKeyCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
            public_key: Err("no value supplied for public_key".to_string()),
        }
    }
}

impl SshKeyCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn public_key<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.public_key = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for public_key: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SshKeyCreate> for super::SshKeyCreate {
    type Error = super::error::ConversionError;
    fn try_from(value: SshKeyCreate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
            public_key: value.public_key?,
        })
    }
}

impl ::std::convert::From<super::SshKeyCreate> for SshKeyCreate {
    fn from(value: super::SshKeyCreate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
            public_key: Ok(value.public_key),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SshKeyResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SshKey>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SshKeyResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SshKeyResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SshKey>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SshKeyResultsPage> for super::SshKeyResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SshKeyResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SshKeyResultsPage> for SshKeyResultsPage {
    fn from(value: super::SshKeyResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetCreateDefaults {}
impl ::std::default::Default for SubnetCreateDefaults {
    fn default() -> Self {
        Self {}
    }
}

impl SubnetCreateDefaults {}
impl ::std::convert::TryFrom<SubnetCreateDefaults> for super::SubnetCreateDefaults {
    type Error = super::error::ConversionError;
    fn try_from(
        _value: SubnetCreateDefaults,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {})
    }
}

impl ::std::convert::From<super::SubnetCreateDefaults> for SubnetCreateDefaults {
    fn from(_value: super::SubnetCreateDefaults) -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPool {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ip_version: ::std::result::Result<super::IpVersion, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for SubnetPool {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            ip_version: Err("no value supplied for ip_version".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl SubnetPool {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn ip_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_version: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPool> for super::SubnetPool {
    type Error = super::error::ConversionError;
    fn try_from(value: SubnetPool) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            ip_version: value.ip_version?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::SubnetPool> for SubnetPool {
    fn from(value: super::SubnetPool) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            ip_version: Ok(value.ip_version),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    ip_version: ::std::result::Result<super::IpVersion, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            ip_version: Err("no value supplied for ip_version".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl SubnetPoolCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn ip_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.ip_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ip_version: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolCreate> for super::SubnetPoolCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            ip_version: value.ip_version?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolCreate> for SubnetPoolCreate {
    fn from(value: super::SubnetPoolCreate) -> Self {
        Self {
            description: Ok(value.description),
            ip_version: Ok(value.ip_version),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolLinkSilo {
    is_default: ::std::result::Result<bool, ::std::string::String>,
    silo: ::std::result::Result<super::NameOrId, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolLinkSilo {
    fn default() -> Self {
        Self {
            is_default: Err("no value supplied for is_default".to_string()),
            silo: Err("no value supplied for silo".to_string()),
        }
    }
}

impl SubnetPoolLinkSilo {
    pub fn is_default<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.is_default = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for is_default: {e}"));
        self
    }

    pub fn silo<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.silo = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolLinkSilo> for super::SubnetPoolLinkSilo {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolLinkSilo,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            is_default: value.is_default?,
            silo: value.silo?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolLinkSilo> for SubnetPoolLinkSilo {
    fn from(value: super::SubnetPoolLinkSilo) -> Self {
        Self {
            is_default: Ok(value.is_default),
            silo: Ok(value.silo),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolMember {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    max_prefix_length: ::std::result::Result<u8, ::std::string::String>,
    min_prefix_length: ::std::result::Result<u8, ::std::string::String>,
    subnet: ::std::result::Result<super::IpNet, ::std::string::String>,
    subnet_pool_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolMember {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            max_prefix_length: Err("no value supplied for max_prefix_length".to_string()),
            min_prefix_length: Err("no value supplied for min_prefix_length".to_string()),
            subnet: Err("no value supplied for subnet".to_string()),
            subnet_pool_id: Err("no value supplied for subnet_pool_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
        }
    }
}

impl SubnetPoolMember {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn max_prefix_length<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u8>,
        T::Error: ::std::fmt::Display,
    {
        self.max_prefix_length = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max_prefix_length: {e}"));
        self
    }

    pub fn min_prefix_length<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u8>,
        T::Error: ::std::fmt::Display,
    {
        self.min_prefix_length = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min_prefix_length: {e}"));
        self
    }

    pub fn subnet<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet: {e}"));
        self
    }

    pub fn subnet_pool_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet_pool_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet_pool_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolMember> for super::SubnetPoolMember {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolMember,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            max_prefix_length: value.max_prefix_length?,
            min_prefix_length: value.min_prefix_length?,
            subnet: value.subnet?,
            subnet_pool_id: value.subnet_pool_id?,
            time_created: value.time_created?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolMember> for SubnetPoolMember {
    fn from(value: super::SubnetPoolMember) -> Self {
        Self {
            id: Ok(value.id),
            max_prefix_length: Ok(value.max_prefix_length),
            min_prefix_length: Ok(value.min_prefix_length),
            subnet: Ok(value.subnet),
            subnet_pool_id: Ok(value.subnet_pool_id),
            time_created: Ok(value.time_created),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolMemberAdd {
    max_prefix_length: ::std::result::Result<::std::option::Option<u8>, ::std::string::String>,
    min_prefix_length: ::std::result::Result<::std::option::Option<u8>, ::std::string::String>,
    subnet: ::std::result::Result<super::IpNet, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolMemberAdd {
    fn default() -> Self {
        Self {
            max_prefix_length: Ok(Default::default()),
            min_prefix_length: Ok(Default::default()),
            subnet: Err("no value supplied for subnet".to_string()),
        }
    }
}

impl SubnetPoolMemberAdd {
    pub fn max_prefix_length<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u8>>,
        T::Error: ::std::fmt::Display,
    {
        self.max_prefix_length = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for max_prefix_length: {e}"));
        self
    }

    pub fn min_prefix_length<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u8>>,
        T::Error: ::std::fmt::Display,
    {
        self.min_prefix_length = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for min_prefix_length: {e}"));
        self
    }

    pub fn subnet<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolMemberAdd> for super::SubnetPoolMemberAdd {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolMemberAdd,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            max_prefix_length: value.max_prefix_length?,
            min_prefix_length: value.min_prefix_length?,
            subnet: value.subnet?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolMemberAdd> for SubnetPoolMemberAdd {
    fn from(value: super::SubnetPoolMemberAdd) -> Self {
        Self {
            max_prefix_length: Ok(value.max_prefix_length),
            min_prefix_length: Ok(value.min_prefix_length),
            subnet: Ok(value.subnet),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolMemberRemove {
    subnet: ::std::result::Result<super::IpNet, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolMemberRemove {
    fn default() -> Self {
        Self {
            subnet: Err("no value supplied for subnet".to_string()),
        }
    }
}

impl SubnetPoolMemberRemove {
    pub fn subnet<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolMemberRemove> for super::SubnetPoolMemberRemove {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolMemberRemove,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            subnet: value.subnet?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolMemberRemove> for SubnetPoolMemberRemove {
    fn from(value: super::SubnetPoolMemberRemove) -> Self {
        Self {
            subnet: Ok(value.subnet),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolMemberResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SubnetPoolMember>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolMemberResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SubnetPoolMemberResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SubnetPoolMember>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolMemberResultsPage> for super::SubnetPoolMemberResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolMemberResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolMemberResultsPage> for SubnetPoolMemberResultsPage {
    fn from(value: super::SubnetPoolMemberResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SubnetPool>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SubnetPoolResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SubnetPool>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolResultsPage> for super::SubnetPoolResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolResultsPage> for SubnetPoolResultsPage {
    fn from(value: super::SubnetPoolResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolSiloLink {
    is_default: ::std::result::Result<bool, ::std::string::String>,
    silo_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    subnet_pool_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolSiloLink {
    fn default() -> Self {
        Self {
            is_default: Err("no value supplied for is_default".to_string()),
            silo_id: Err("no value supplied for silo_id".to_string()),
            subnet_pool_id: Err("no value supplied for subnet_pool_id".to_string()),
        }
    }
}

impl SubnetPoolSiloLink {
    pub fn is_default<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.is_default = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for is_default: {e}"));
        self
    }

    pub fn silo_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_id: {e}"));
        self
    }

    pub fn subnet_pool_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet_pool_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet_pool_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolSiloLink> for super::SubnetPoolSiloLink {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolSiloLink,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            is_default: value.is_default?,
            silo_id: value.silo_id?,
            subnet_pool_id: value.subnet_pool_id?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolSiloLink> for SubnetPoolSiloLink {
    fn from(value: super::SubnetPoolSiloLink) -> Self {
        Self {
            is_default: Ok(value.is_default),
            silo_id: Ok(value.silo_id),
            subnet_pool_id: Ok(value.subnet_pool_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolSiloLinkResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SubnetPoolSiloLink>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolSiloLinkResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SubnetPoolSiloLinkResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SubnetPoolSiloLink>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolSiloLinkResultsPage>
    for super::SubnetPoolSiloLinkResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolSiloLinkResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolSiloLinkResultsPage> for SubnetPoolSiloLinkResultsPage {
    fn from(value: super::SubnetPoolSiloLinkResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolSiloUpdate {
    is_default: ::std::result::Result<bool, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolSiloUpdate {
    fn default() -> Self {
        Self {
            is_default: Err("no value supplied for is_default".to_string()),
        }
    }
}

impl SubnetPoolSiloUpdate {
    pub fn is_default<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.is_default = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for is_default: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolSiloUpdate> for super::SubnetPoolSiloUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolSiloUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            is_default: value.is_default?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolSiloUpdate> for SubnetPoolSiloUpdate {
    fn from(value: super::SubnetPoolSiloUpdate) -> Self {
        Self {
            is_default: Ok(value.is_default),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl SubnetPoolUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolUpdate> for super::SubnetPoolUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolUpdate> for SubnetPoolUpdate {
    fn from(value: super::SubnetPoolUpdate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetPoolUtilization {
    capacity: ::std::result::Result<f64, ::std::string::String>,
    remaining: ::std::result::Result<f64, ::std::string::String>,
}

impl ::std::default::Default for SubnetPoolUtilization {
    fn default() -> Self {
        Self {
            capacity: Err("no value supplied for capacity".to_string()),
            remaining: Err("no value supplied for remaining".to_string()),
        }
    }
}

impl SubnetPoolUtilization {
    pub fn capacity<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.capacity = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for capacity: {e}"));
        self
    }

    pub fn remaining<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<f64>,
        T::Error: ::std::fmt::Display,
    {
        self.remaining = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for remaining: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SubnetPoolUtilization> for super::SubnetPoolUtilization {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SubnetPoolUtilization,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            capacity: value.capacity?,
            remaining: value.remaining?,
        })
    }
}

impl ::std::convert::From<super::SubnetPoolUtilization> for SubnetPoolUtilization {
    fn from(value: super::SubnetPoolUtilization) -> Self {
        Self {
            capacity: Ok(value.capacity),
            remaining: Ok(value.remaining),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SupportBundleCreate {
    user_comment:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SupportBundleCreate {
    fn default() -> Self {
        Self {
            user_comment: Ok(Default::default()),
        }
    }
}

impl SupportBundleCreate {
    pub fn user_comment<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.user_comment = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for user_comment: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SupportBundleCreate> for super::SupportBundleCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SupportBundleCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            user_comment: value.user_comment?,
        })
    }
}

impl ::std::convert::From<super::SupportBundleCreate> for SupportBundleCreate {
    fn from(value: super::SupportBundleCreate) -> Self {
        Self {
            user_comment: Ok(value.user_comment),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SupportBundleInfo {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    reason_for_creation: ::std::result::Result<::std::string::String, ::std::string::String>,
    reason_for_failure:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    state: ::std::result::Result<super::SupportBundleState, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    user_comment:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SupportBundleInfo {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            reason_for_creation: Err("no value supplied for reason_for_creation".to_string()),
            reason_for_failure: Ok(Default::default()),
            state: Err("no value supplied for state".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            user_comment: Ok(Default::default()),
        }
    }
}

impl SupportBundleInfo {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn reason_for_creation<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.reason_for_creation = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for reason_for_creation: {e}"));
        self
    }

    pub fn reason_for_failure<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.reason_for_failure = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for reason_for_failure: {e}"));
        self
    }

    pub fn state<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SupportBundleState>,
        T::Error: ::std::fmt::Display,
    {
        self.state = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for state: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn user_comment<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.user_comment = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for user_comment: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SupportBundleInfo> for super::SupportBundleInfo {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SupportBundleInfo,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            reason_for_creation: value.reason_for_creation?,
            reason_for_failure: value.reason_for_failure?,
            state: value.state?,
            time_created: value.time_created?,
            user_comment: value.user_comment?,
        })
    }
}

impl ::std::convert::From<super::SupportBundleInfo> for SupportBundleInfo {
    fn from(value: super::SupportBundleInfo) -> Self {
        Self {
            id: Ok(value.id),
            reason_for_creation: Ok(value.reason_for_creation),
            reason_for_failure: Ok(value.reason_for_failure),
            state: Ok(value.state),
            time_created: Ok(value.time_created),
            user_comment: Ok(value.user_comment),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SupportBundleInfoResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SupportBundleInfo>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SupportBundleInfoResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SupportBundleInfoResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SupportBundleInfo>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SupportBundleInfoResultsPage> for super::SupportBundleInfoResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SupportBundleInfoResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SupportBundleInfoResultsPage> for SupportBundleInfoResultsPage {
    fn from(value: super::SupportBundleInfoResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SupportBundleUpdate {
    user_comment:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SupportBundleUpdate {
    fn default() -> Self {
        Self {
            user_comment: Ok(Default::default()),
        }
    }
}

impl SupportBundleUpdate {
    pub fn user_comment<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.user_comment = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for user_comment: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SupportBundleUpdate> for super::SupportBundleUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SupportBundleUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            user_comment: value.user_comment?,
        })
    }
}

impl ::std::convert::From<super::SupportBundleUpdate> for SupportBundleUpdate {
    fn from(value: super::SupportBundleUpdate) -> Self {
        Self {
            user_comment: Ok(value.user_comment),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Switch {
    baseboard: ::std::result::Result<super::Baseboard, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    rack_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Switch {
    fn default() -> Self {
        Self {
            baseboard: Err("no value supplied for baseboard".to_string()),
            id: Err("no value supplied for id".to_string()),
            rack_id: Err("no value supplied for rack_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Switch {
    pub fn baseboard<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Baseboard>,
        T::Error: ::std::fmt::Display,
    {
        self.baseboard = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for baseboard: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn rack_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.rack_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rack_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Switch> for super::Switch {
    type Error = super::error::ConversionError;
    fn try_from(value: Switch) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            baseboard: value.baseboard?,
            id: value.id?,
            rack_id: value.rack_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Switch> for Switch {
    fn from(value: super::Switch) -> Self {
        Self {
            baseboard: Ok(value.baseboard),
            id: Ok(value.id),
            rack_id: Ok(value.rack_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchBgpHistory {
    history: ::std::result::Result<
        ::std::collections::HashMap<::std::string::String, super::BgpMessageHistory>,
        ::std::string::String,
    >,
    switch: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for SwitchBgpHistory {
    fn default() -> Self {
        Self {
            history: Err("no value supplied for history".to_string()),
            switch: Err("no value supplied for switch".to_string()),
        }
    }
}

impl SwitchBgpHistory {
    pub fn history<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::collections::HashMap<::std::string::String, super::BgpMessageHistory>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.history = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for history: {e}"));
        self
    }

    pub fn switch<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchBgpHistory> for super::SwitchBgpHistory {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchBgpHistory,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            history: value.history?,
            switch: value.switch?,
        })
    }
}

impl ::std::convert::From<super::SwitchBgpHistory> for SwitchBgpHistory {
    fn from(value: super::SwitchBgpHistory) -> Self {
        Self {
            history: Ok(value.history),
            switch: Ok(value.switch),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchInterfaceConfig {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    interface_name: ::std::result::Result<super::Name, ::std::string::String>,
    kind: ::std::result::Result<super::SwitchInterfaceKind, ::std::string::String>,
    port_settings_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    v6_enabled: ::std::result::Result<bool, ::std::string::String>,
}

impl ::std::default::Default for SwitchInterfaceConfig {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            interface_name: Err("no value supplied for interface_name".to_string()),
            kind: Err("no value supplied for kind".to_string()),
            port_settings_id: Err("no value supplied for port_settings_id".to_string()),
            v6_enabled: Err("no value supplied for v6_enabled".to_string()),
        }
    }
}

impl SwitchInterfaceConfig {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn interface_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.interface_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for interface_name: {e}"));
        self
    }

    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchInterfaceKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn port_settings_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.port_settings_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_settings_id: {e}"));
        self
    }

    pub fn v6_enabled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.v6_enabled = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for v6_enabled: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchInterfaceConfig> for super::SwitchInterfaceConfig {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchInterfaceConfig,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            interface_name: value.interface_name?,
            kind: value.kind?,
            port_settings_id: value.port_settings_id?,
            v6_enabled: value.v6_enabled?,
        })
    }
}

impl ::std::convert::From<super::SwitchInterfaceConfig> for SwitchInterfaceConfig {
    fn from(value: super::SwitchInterfaceConfig) -> Self {
        Self {
            id: Ok(value.id),
            interface_name: Ok(value.interface_name),
            kind: Ok(value.kind),
            port_settings_id: Ok(value.port_settings_id),
            v6_enabled: Ok(value.v6_enabled),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchInterfaceConfigCreate {
    kind: ::std::result::Result<super::SwitchInterfaceKind, ::std::string::String>,
    link_name: ::std::result::Result<super::Name, ::std::string::String>,
    v6_enabled: ::std::result::Result<bool, ::std::string::String>,
}

impl ::std::default::Default for SwitchInterfaceConfigCreate {
    fn default() -> Self {
        Self {
            kind: Err("no value supplied for kind".to_string()),
            link_name: Err("no value supplied for link_name".to_string()),
            v6_enabled: Err("no value supplied for v6_enabled".to_string()),
        }
    }
}

impl SwitchInterfaceConfigCreate {
    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchInterfaceKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn link_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.link_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_name: {e}"));
        self
    }

    pub fn v6_enabled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.v6_enabled = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for v6_enabled: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchInterfaceConfigCreate> for super::SwitchInterfaceConfigCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchInterfaceConfigCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            kind: value.kind?,
            link_name: value.link_name?,
            v6_enabled: value.v6_enabled?,
        })
    }
}

impl ::std::convert::From<super::SwitchInterfaceConfigCreate> for SwitchInterfaceConfigCreate {
    fn from(value: super::SwitchInterfaceConfigCreate) -> Self {
        Self {
            kind: Ok(value.kind),
            link_name: Ok(value.link_name),
            v6_enabled: Ok(value.v6_enabled),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPort {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    port_name: ::std::result::Result<super::Name, ::std::string::String>,
    port_settings_id:
        ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    rack_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    switch_slot: ::std::result::Result<super::SwitchSlot, ::std::string::String>,
}

impl ::std::default::Default for SwitchPort {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            port_name: Err("no value supplied for port_name".to_string()),
            port_settings_id: Ok(Default::default()),
            rack_id: Err("no value supplied for rack_id".to_string()),
            switch_slot: Err("no value supplied for switch_slot".to_string()),
        }
    }
}

impl SwitchPort {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn port_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.port_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_name: {e}"));
        self
    }

    pub fn port_settings_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.port_settings_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_settings_id: {e}"));
        self
    }

    pub fn rack_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.rack_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rack_id: {e}"));
        self
    }

    pub fn switch_slot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchSlot>,
        T::Error: ::std::fmt::Display,
    {
        self.switch_slot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for switch_slot: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPort> for super::SwitchPort {
    type Error = super::error::ConversionError;
    fn try_from(value: SwitchPort) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            port_name: value.port_name?,
            port_settings_id: value.port_settings_id?,
            rack_id: value.rack_id?,
            switch_slot: value.switch_slot?,
        })
    }
}

impl ::std::convert::From<super::SwitchPort> for SwitchPort {
    fn from(value: super::SwitchPort) -> Self {
        Self {
            id: Ok(value.id),
            port_name: Ok(value.port_name),
            port_settings_id: Ok(value.port_settings_id),
            rack_id: Ok(value.rack_id),
            switch_slot: Ok(value.switch_slot),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortAddressView {
    address: ::std::result::Result<super::IpNet, ::std::string::String>,
    address_lot_block_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    address_lot_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    address_lot_name: ::std::result::Result<super::Name, ::std::string::String>,
    interface_name: ::std::result::Result<super::Name, ::std::string::String>,
    port_settings_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    vlan_id: ::std::result::Result<::std::option::Option<u16>, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortAddressView {
    fn default() -> Self {
        Self {
            address: Err("no value supplied for address".to_string()),
            address_lot_block_id: Err("no value supplied for address_lot_block_id".to_string()),
            address_lot_id: Err("no value supplied for address_lot_id".to_string()),
            address_lot_name: Err("no value supplied for address_lot_name".to_string()),
            interface_name: Err("no value supplied for interface_name".to_string()),
            port_settings_id: Err("no value supplied for port_settings_id".to_string()),
            vlan_id: Ok(Default::default()),
        }
    }
}

impl SwitchPortAddressView {
    pub fn address<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.address = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address: {e}"));
        self
    }

    pub fn address_lot_block_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.address_lot_block_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_lot_block_id: {e}"));
        self
    }

    pub fn address_lot_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.address_lot_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_lot_id: {e}"));
        self
    }

    pub fn address_lot_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.address_lot_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for address_lot_name: {e}"));
        self
    }

    pub fn interface_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.interface_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for interface_name: {e}"));
        self
    }

    pub fn port_settings_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.port_settings_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_settings_id: {e}"));
        self
    }

    pub fn vlan_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u16>>,
        T::Error: ::std::fmt::Display,
    {
        self.vlan_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vlan_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortAddressView> for super::SwitchPortAddressView {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortAddressView,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            address: value.address?,
            address_lot_block_id: value.address_lot_block_id?,
            address_lot_id: value.address_lot_id?,
            address_lot_name: value.address_lot_name?,
            interface_name: value.interface_name?,
            port_settings_id: value.port_settings_id?,
            vlan_id: value.vlan_id?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortAddressView> for SwitchPortAddressView {
    fn from(value: super::SwitchPortAddressView) -> Self {
        Self {
            address: Ok(value.address),
            address_lot_block_id: Ok(value.address_lot_block_id),
            address_lot_id: Ok(value.address_lot_id),
            address_lot_name: Ok(value.address_lot_name),
            interface_name: Ok(value.interface_name),
            port_settings_id: Ok(value.port_settings_id),
            vlan_id: Ok(value.vlan_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortApplySettings {
    port_settings: ::std::result::Result<super::NameOrId, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortApplySettings {
    fn default() -> Self {
        Self {
            port_settings: Err("no value supplied for port_settings".to_string()),
        }
    }
}

impl SwitchPortApplySettings {
    pub fn port_settings<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::NameOrId>,
        T::Error: ::std::fmt::Display,
    {
        self.port_settings = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_settings: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortApplySettings> for super::SwitchPortApplySettings {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortApplySettings,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            port_settings: value.port_settings?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortApplySettings> for SwitchPortApplySettings {
    fn from(value: super::SwitchPortApplySettings) -> Self {
        Self {
            port_settings: Ok(value.port_settings),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortConfig {
    geometry: ::std::result::Result<super::SwitchPortGeometry, ::std::string::String>,
    port_settings_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortConfig {
    fn default() -> Self {
        Self {
            geometry: Err("no value supplied for geometry".to_string()),
            port_settings_id: Err("no value supplied for port_settings_id".to_string()),
        }
    }
}

impl SwitchPortConfig {
    pub fn geometry<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchPortGeometry>,
        T::Error: ::std::fmt::Display,
    {
        self.geometry = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for geometry: {e}"));
        self
    }

    pub fn port_settings_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.port_settings_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_settings_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortConfig> for super::SwitchPortConfig {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortConfig,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            geometry: value.geometry?,
            port_settings_id: value.port_settings_id?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortConfig> for SwitchPortConfig {
    fn from(value: super::SwitchPortConfig) -> Self {
        Self {
            geometry: Ok(value.geometry),
            port_settings_id: Ok(value.port_settings_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortConfigCreate {
    geometry: ::std::result::Result<super::SwitchPortGeometry, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortConfigCreate {
    fn default() -> Self {
        Self {
            geometry: Err("no value supplied for geometry".to_string()),
        }
    }
}

impl SwitchPortConfigCreate {
    pub fn geometry<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchPortGeometry>,
        T::Error: ::std::fmt::Display,
    {
        self.geometry = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for geometry: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortConfigCreate> for super::SwitchPortConfigCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortConfigCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            geometry: value.geometry?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortConfigCreate> for SwitchPortConfigCreate {
    fn from(value: super::SwitchPortConfigCreate) -> Self {
        Self {
            geometry: Ok(value.geometry),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortLinkConfig {
    autoneg: ::std::result::Result<bool, ::std::string::String>,
    fec: ::std::result::Result<::std::option::Option<super::LinkFec>, ::std::string::String>,
    link_name: ::std::result::Result<super::Name, ::std::string::String>,
    lldp_link_config:
        ::std::result::Result<::std::option::Option<super::LldpLinkConfig>, ::std::string::String>,
    mtu: ::std::result::Result<u16, ::std::string::String>,
    port_settings_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    speed: ::std::result::Result<super::LinkSpeed, ::std::string::String>,
    tx_eq_config:
        ::std::result::Result<::std::option::Option<super::TxEqConfig>, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortLinkConfig {
    fn default() -> Self {
        Self {
            autoneg: Err("no value supplied for autoneg".to_string()),
            fec: Ok(Default::default()),
            link_name: Err("no value supplied for link_name".to_string()),
            lldp_link_config: Ok(Default::default()),
            mtu: Err("no value supplied for mtu".to_string()),
            port_settings_id: Err("no value supplied for port_settings_id".to_string()),
            speed: Err("no value supplied for speed".to_string()),
            tx_eq_config: Ok(Default::default()),
        }
    }
}

impl SwitchPortLinkConfig {
    pub fn autoneg<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.autoneg = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for autoneg: {e}"));
        self
    }

    pub fn fec<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::LinkFec>>,
        T::Error: ::std::fmt::Display,
    {
        self.fec = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for fec: {e}"));
        self
    }

    pub fn link_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.link_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for link_name: {e}"));
        self
    }

    pub fn lldp_link_config<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::LldpLinkConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.lldp_link_config = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for lldp_link_config: {e}"));
        self
    }

    pub fn mtu<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.mtu = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for mtu: {e}"));
        self
    }

    pub fn port_settings_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.port_settings_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_settings_id: {e}"));
        self
    }

    pub fn speed<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::LinkSpeed>,
        T::Error: ::std::fmt::Display,
    {
        self.speed = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for speed: {e}"));
        self
    }

    pub fn tx_eq_config<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::TxEqConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.tx_eq_config = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for tx_eq_config: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortLinkConfig> for super::SwitchPortLinkConfig {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortLinkConfig,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            autoneg: value.autoneg?,
            fec: value.fec?,
            link_name: value.link_name?,
            lldp_link_config: value.lldp_link_config?,
            mtu: value.mtu?,
            port_settings_id: value.port_settings_id?,
            speed: value.speed?,
            tx_eq_config: value.tx_eq_config?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortLinkConfig> for SwitchPortLinkConfig {
    fn from(value: super::SwitchPortLinkConfig) -> Self {
        Self {
            autoneg: Ok(value.autoneg),
            fec: Ok(value.fec),
            link_name: Ok(value.link_name),
            lldp_link_config: Ok(value.lldp_link_config),
            mtu: Ok(value.mtu),
            port_settings_id: Ok(value.port_settings_id),
            speed: Ok(value.speed),
            tx_eq_config: Ok(value.tx_eq_config),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::SwitchPort>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SwitchPortResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SwitchPort>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortResultsPage> for super::SwitchPortResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortResultsPage> for SwitchPortResultsPage {
    fn from(value: super::SwitchPortResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortRouteConfig {
    dst: ::std::result::Result<super::IpNet, ::std::string::String>,
    gw: ::std::result::Result<::std::net::IpAddr, ::std::string::String>,
    interface_name: ::std::result::Result<super::Name, ::std::string::String>,
    port_settings_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    rib_priority: ::std::result::Result<::std::option::Option<u8>, ::std::string::String>,
    vlan_id: ::std::result::Result<::std::option::Option<u16>, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortRouteConfig {
    fn default() -> Self {
        Self {
            dst: Err("no value supplied for dst".to_string()),
            gw: Err("no value supplied for gw".to_string()),
            interface_name: Err("no value supplied for interface_name".to_string()),
            port_settings_id: Err("no value supplied for port_settings_id".to_string()),
            rib_priority: Ok(Default::default()),
            vlan_id: Ok(Default::default()),
        }
    }
}

impl SwitchPortRouteConfig {
    pub fn dst<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::IpNet>,
        T::Error: ::std::fmt::Display,
    {
        self.dst = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for dst: {e}"));
        self
    }

    pub fn gw<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::net::IpAddr>,
        T::Error: ::std::fmt::Display,
    {
        self.gw = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for gw: {e}"));
        self
    }

    pub fn interface_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.interface_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for interface_name: {e}"));
        self
    }

    pub fn port_settings_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.port_settings_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_settings_id: {e}"));
        self
    }

    pub fn rib_priority<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u8>>,
        T::Error: ::std::fmt::Display,
    {
        self.rib_priority = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rib_priority: {e}"));
        self
    }

    pub fn vlan_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<u16>>,
        T::Error: ::std::fmt::Display,
    {
        self.vlan_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vlan_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortRouteConfig> for super::SwitchPortRouteConfig {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortRouteConfig,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            dst: value.dst?,
            gw: value.gw?,
            interface_name: value.interface_name?,
            port_settings_id: value.port_settings_id?,
            rib_priority: value.rib_priority?,
            vlan_id: value.vlan_id?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortRouteConfig> for SwitchPortRouteConfig {
    fn from(value: super::SwitchPortRouteConfig) -> Self {
        Self {
            dst: Ok(value.dst),
            gw: Ok(value.gw),
            interface_name: Ok(value.interface_name),
            port_settings_id: Ok(value.port_settings_id),
            rib_priority: Ok(value.rib_priority),
            vlan_id: Ok(value.vlan_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortSettings {
    addresses:
        ::std::result::Result<::std::vec::Vec<super::SwitchPortAddressView>, ::std::string::String>,
    bgp_peers: ::std::result::Result<::std::vec::Vec<super::BgpPeer>, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    groups: ::std::result::Result<
        ::std::vec::Vec<super::SwitchPortSettingsGroups>,
        ::std::string::String,
    >,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    interfaces:
        ::std::result::Result<::std::vec::Vec<super::SwitchInterfaceConfig>, ::std::string::String>,
    links:
        ::std::result::Result<::std::vec::Vec<super::SwitchPortLinkConfig>, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    port: ::std::result::Result<super::SwitchPortConfig, ::std::string::String>,
    routes:
        ::std::result::Result<::std::vec::Vec<super::SwitchPortRouteConfig>, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortSettings {
    fn default() -> Self {
        Self {
            addresses: Err("no value supplied for addresses".to_string()),
            bgp_peers: Err("no value supplied for bgp_peers".to_string()),
            description: Err("no value supplied for description".to_string()),
            groups: Err("no value supplied for groups".to_string()),
            id: Err("no value supplied for id".to_string()),
            interfaces: Err("no value supplied for interfaces".to_string()),
            links: Err("no value supplied for links".to_string()),
            name: Err("no value supplied for name".to_string()),
            port: Err("no value supplied for port".to_string()),
            routes: Err("no value supplied for routes".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl SwitchPortSettings {
    pub fn addresses<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SwitchPortAddressView>>,
        T::Error: ::std::fmt::Display,
    {
        self.addresses = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for addresses: {e}"));
        self
    }

    pub fn bgp_peers<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::BgpPeer>>,
        T::Error: ::std::fmt::Display,
    {
        self.bgp_peers = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bgp_peers: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn groups<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SwitchPortSettingsGroups>>,
        T::Error: ::std::fmt::Display,
    {
        self.groups = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for groups: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn interfaces<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SwitchInterfaceConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.interfaces = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for interfaces: {e}"));
        self
    }

    pub fn links<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SwitchPortLinkConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.links = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for links: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn port<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchPortConfig>,
        T::Error: ::std::fmt::Display,
    {
        self.port = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port: {e}"));
        self
    }

    pub fn routes<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SwitchPortRouteConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.routes = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for routes: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortSettings> for super::SwitchPortSettings {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortSettings,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            addresses: value.addresses?,
            bgp_peers: value.bgp_peers?,
            description: value.description?,
            groups: value.groups?,
            id: value.id?,
            interfaces: value.interfaces?,
            links: value.links?,
            name: value.name?,
            port: value.port?,
            routes: value.routes?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortSettings> for SwitchPortSettings {
    fn from(value: super::SwitchPortSettings) -> Self {
        Self {
            addresses: Ok(value.addresses),
            bgp_peers: Ok(value.bgp_peers),
            description: Ok(value.description),
            groups: Ok(value.groups),
            id: Ok(value.id),
            interfaces: Ok(value.interfaces),
            links: Ok(value.links),
            name: Ok(value.name),
            port: Ok(value.port),
            routes: Ok(value.routes),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortSettingsCreate {
    addresses: ::std::result::Result<::std::vec::Vec<super::AddressConfig>, ::std::string::String>,
    bgp_peers: ::std::result::Result<::std::vec::Vec<super::BgpPeerConfig>, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    groups: ::std::result::Result<::std::vec::Vec<super::NameOrId>, ::std::string::String>,
    interfaces: ::std::result::Result<
        ::std::vec::Vec<super::SwitchInterfaceConfigCreate>,
        ::std::string::String,
    >,
    links: ::std::result::Result<::std::vec::Vec<super::LinkConfigCreate>, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    port_config: ::std::result::Result<super::SwitchPortConfigCreate, ::std::string::String>,
    routes: ::std::result::Result<::std::vec::Vec<super::RouteConfig>, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortSettingsCreate {
    fn default() -> Self {
        Self {
            addresses: Err("no value supplied for addresses".to_string()),
            bgp_peers: Ok(Default::default()),
            description: Err("no value supplied for description".to_string()),
            groups: Ok(Default::default()),
            interfaces: Ok(Default::default()),
            links: Err("no value supplied for links".to_string()),
            name: Err("no value supplied for name".to_string()),
            port_config: Err("no value supplied for port_config".to_string()),
            routes: Ok(Default::default()),
        }
    }
}

impl SwitchPortSettingsCreate {
    pub fn addresses<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AddressConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.addresses = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for addresses: {e}"));
        self
    }

    pub fn bgp_peers<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::BgpPeerConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.bgp_peers = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for bgp_peers: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn groups<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::NameOrId>>,
        T::Error: ::std::fmt::Display,
    {
        self.groups = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for groups: {e}"));
        self
    }

    pub fn interfaces<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SwitchInterfaceConfigCreate>>,
        T::Error: ::std::fmt::Display,
    {
        self.interfaces = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for interfaces: {e}"));
        self
    }

    pub fn links<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::LinkConfigCreate>>,
        T::Error: ::std::fmt::Display,
    {
        self.links = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for links: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn port_config<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SwitchPortConfigCreate>,
        T::Error: ::std::fmt::Display,
    {
        self.port_config = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_config: {e}"));
        self
    }

    pub fn routes<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::RouteConfig>>,
        T::Error: ::std::fmt::Display,
    {
        self.routes = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for routes: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortSettingsCreate> for super::SwitchPortSettingsCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortSettingsCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            addresses: value.addresses?,
            bgp_peers: value.bgp_peers?,
            description: value.description?,
            groups: value.groups?,
            interfaces: value.interfaces?,
            links: value.links?,
            name: value.name?,
            port_config: value.port_config?,
            routes: value.routes?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortSettingsCreate> for SwitchPortSettingsCreate {
    fn from(value: super::SwitchPortSettingsCreate) -> Self {
        Self {
            addresses: Ok(value.addresses),
            bgp_peers: Ok(value.bgp_peers),
            description: Ok(value.description),
            groups: Ok(value.groups),
            interfaces: Ok(value.interfaces),
            links: Ok(value.links),
            name: Ok(value.name),
            port_config: Ok(value.port_config),
            routes: Ok(value.routes),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortSettingsGroups {
    port_settings_group_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    port_settings_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortSettingsGroups {
    fn default() -> Self {
        Self {
            port_settings_group_id: Err("no value supplied for port_settings_group_id".to_string()),
            port_settings_id: Err("no value supplied for port_settings_id".to_string()),
        }
    }
}

impl SwitchPortSettingsGroups {
    pub fn port_settings_group_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.port_settings_group_id = value.try_into().map_err(|e| {
            format!("error converting supplied value for port_settings_group_id: {e}")
        });
        self
    }

    pub fn port_settings_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.port_settings_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for port_settings_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortSettingsGroups> for super::SwitchPortSettingsGroups {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortSettingsGroups,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            port_settings_group_id: value.port_settings_group_id?,
            port_settings_id: value.port_settings_id?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortSettingsGroups> for SwitchPortSettingsGroups {
    fn from(value: super::SwitchPortSettingsGroups) -> Self {
        Self {
            port_settings_group_id: Ok(value.port_settings_group_id),
            port_settings_id: Ok(value.port_settings_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortSettingsIdentity {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortSettingsIdentity {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl SwitchPortSettingsIdentity {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortSettingsIdentity> for super::SwitchPortSettingsIdentity {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortSettingsIdentity,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortSettingsIdentity> for SwitchPortSettingsIdentity {
    fn from(value: super::SwitchPortSettingsIdentity) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchPortSettingsIdentityResultsPage {
    items: ::std::result::Result<
        ::std::vec::Vec<super::SwitchPortSettingsIdentity>,
        ::std::string::String,
    >,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SwitchPortSettingsIdentityResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SwitchPortSettingsIdentityResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::SwitchPortSettingsIdentity>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchPortSettingsIdentityResultsPage>
    for super::SwitchPortSettingsIdentityResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchPortSettingsIdentityResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SwitchPortSettingsIdentityResultsPage>
    for SwitchPortSettingsIdentityResultsPage
{
    fn from(value: super::SwitchPortSettingsIdentityResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwitchResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Switch>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for SwitchResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl SwitchResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Switch>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<SwitchResultsPage> for super::SwitchResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SwitchResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::SwitchResultsPage> for SwitchResultsPage {
    fn from(value: super::SwitchResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SystemNetworkingSettings {
    external_jumbo_frames_opt_in_enabled: ::std::result::Result<bool, ::std::string::String>,
}

impl ::std::default::Default for SystemNetworkingSettings {
    fn default() -> Self {
        Self {
            external_jumbo_frames_opt_in_enabled: Err("no value supplied for \
                                                       external_jumbo_frames_opt_in_enabled"
                .to_string()),
        }
    }
}

impl SystemNetworkingSettings {
    pub fn external_jumbo_frames_opt_in_enabled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.external_jumbo_frames_opt_in_enabled = value.try_into().map_err(|e| {
            format!("error converting supplied value for external_jumbo_frames_opt_in_enabled: {e}")
        });
        self
    }
}

impl ::std::convert::TryFrom<SystemNetworkingSettings> for super::SystemNetworkingSettings {
    type Error = super::error::ConversionError;
    fn try_from(
        value: SystemNetworkingSettings,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            external_jumbo_frames_opt_in_enabled: value.external_jumbo_frames_opt_in_enabled?,
        })
    }
}

impl ::std::convert::From<super::SystemNetworkingSettings> for SystemNetworkingSettings {
    fn from(value: super::SystemNetworkingSettings) -> Self {
        Self {
            external_jumbo_frames_opt_in_enabled: Ok(value.external_jumbo_frames_opt_in_enabled),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SystemNetworkingSettingsUpdate {
    external_jumbo_frames_opt_in_enabled: ::std::result::Result<bool, ::std::string::String>,
}

impl ::std::default::Default for SystemNetworkingSettingsUpdate {
    fn default() -> Self {
        Self {
            external_jumbo_frames_opt_in_enabled: Ok(Default::default()),
        }
    }
}

impl SystemNetworkingSettingsUpdate {
    pub fn external_jumbo_frames_opt_in_enabled<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.external_jumbo_frames_opt_in_enabled = value.try_into().map_err(|e| {
            format!("error converting supplied value for external_jumbo_frames_opt_in_enabled: {e}")
        });
        self
    }
}

impl ::std::convert::TryFrom<SystemNetworkingSettingsUpdate>
    for super::SystemNetworkingSettingsUpdate
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: SystemNetworkingSettingsUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            external_jumbo_frames_opt_in_enabled: value.external_jumbo_frames_opt_in_enabled?,
        })
    }
}

impl ::std::convert::From<super::SystemNetworkingSettingsUpdate>
    for SystemNetworkingSettingsUpdate
{
    fn from(value: super::SystemNetworkingSettingsUpdate) -> Self {
        Self {
            external_jumbo_frames_opt_in_enabled: Ok(value.external_jumbo_frames_opt_in_enabled),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TargetRelease {
    time_requested:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    version: ::std::result::Result<super::TargetReleaseVersion, ::std::string::String>,
}

impl ::std::default::Default for TargetRelease {
    fn default() -> Self {
        Self {
            time_requested: Err("no value supplied for time_requested".to_string()),
            version: Err("no value supplied for version".to_string()),
        }
    }
}

impl TargetRelease {
    pub fn time_requested<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_requested = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_requested: {e}"));
        self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::TargetReleaseVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for version: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<TargetRelease> for super::TargetRelease {
    type Error = super::error::ConversionError;
    fn try_from(
        value: TargetRelease,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            time_requested: value.time_requested?,
            version: value.version?,
        })
    }
}

impl ::std::convert::From<super::TargetRelease> for TargetRelease {
    fn from(value: super::TargetRelease) -> Self {
        Self {
            time_requested: Ok(value.time_requested),
            version: Ok(value.version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Timeseries {
    fields: ::std::result::Result<
        ::std::collections::HashMap<::std::string::String, super::FieldValue>,
        ::std::string::String,
    >,
    points: ::std::result::Result<super::Points, ::std::string::String>,
}

impl ::std::default::Default for Timeseries {
    fn default() -> Self {
        Self {
            fields: Err("no value supplied for fields".to_string()),
            points: Err("no value supplied for points".to_string()),
        }
    }
}

impl Timeseries {
    pub fn fields<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::collections::HashMap<::std::string::String, super::FieldValue>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.fields = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for fields: {e}"));
        self
    }

    pub fn points<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Points>,
        T::Error: ::std::fmt::Display,
    {
        self.points = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for points: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Timeseries> for super::Timeseries {
    type Error = super::error::ConversionError;
    fn try_from(value: Timeseries) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            fields: value.fields?,
            points: value.points?,
        })
    }
}

impl ::std::convert::From<super::Timeseries> for Timeseries {
    fn from(value: super::Timeseries) -> Self {
        Self {
            fields: Ok(value.fields),
            points: Ok(value.points),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeseriesDescription {
    metric: ::std::result::Result<::std::string::String, ::std::string::String>,
    target: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for TimeseriesDescription {
    fn default() -> Self {
        Self {
            metric: Err("no value supplied for metric".to_string()),
            target: Err("no value supplied for target".to_string()),
        }
    }
}

impl TimeseriesDescription {
    pub fn metric<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.metric = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for metric: {e}"));
        self
    }

    pub fn target<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.target = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<TimeseriesDescription> for super::TimeseriesDescription {
    type Error = super::error::ConversionError;
    fn try_from(
        value: TimeseriesDescription,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            metric: value.metric?,
            target: value.target?,
        })
    }
}

impl ::std::convert::From<super::TimeseriesDescription> for TimeseriesDescription {
    fn from(value: super::TimeseriesDescription) -> Self {
        Self {
            metric: Ok(value.metric),
            target: Ok(value.target),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeseriesQuery {
    query: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for TimeseriesQuery {
    fn default() -> Self {
        Self {
            query: Err("no value supplied for query".to_string()),
        }
    }
}

impl TimeseriesQuery {
    pub fn query<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.query = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for query: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<TimeseriesQuery> for super::TimeseriesQuery {
    type Error = super::error::ConversionError;
    fn try_from(
        value: TimeseriesQuery,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            query: value.query?,
        })
    }
}

impl ::std::convert::From<super::TimeseriesQuery> for TimeseriesQuery {
    fn from(value: super::TimeseriesQuery) -> Self {
        Self {
            query: Ok(value.query),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeseriesSchema {
    authz_scope: ::std::result::Result<super::AuthzScope, ::std::string::String>,
    created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    datum_type: ::std::result::Result<super::DatumType, ::std::string::String>,
    description: ::std::result::Result<super::TimeseriesDescription, ::std::string::String>,
    field_schema: ::std::result::Result<::std::vec::Vec<super::FieldSchema>, ::std::string::String>,
    timeseries_name: ::std::result::Result<super::TimeseriesName, ::std::string::String>,
    units: ::std::result::Result<super::Units, ::std::string::String>,
    version: ::std::result::Result<::std::num::NonZeroU8, ::std::string::String>,
}

impl ::std::default::Default for TimeseriesSchema {
    fn default() -> Self {
        Self {
            authz_scope: Err("no value supplied for authz_scope".to_string()),
            created: Err("no value supplied for created".to_string()),
            datum_type: Err("no value supplied for datum_type".to_string()),
            description: Err("no value supplied for description".to_string()),
            field_schema: Err("no value supplied for field_schema".to_string()),
            timeseries_name: Err("no value supplied for timeseries_name".to_string()),
            units: Err("no value supplied for units".to_string()),
            version: Err("no value supplied for version".to_string()),
        }
    }
}

impl TimeseriesSchema {
    pub fn authz_scope<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::AuthzScope>,
        T::Error: ::std::fmt::Display,
    {
        self.authz_scope = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for authz_scope: {e}"));
        self
    }

    pub fn created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for created: {e}"));
        self
    }

    pub fn datum_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::DatumType>,
        T::Error: ::std::fmt::Display,
    {
        self.datum_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for datum_type: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::TimeseriesDescription>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn field_schema<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::FieldSchema>>,
        T::Error: ::std::fmt::Display,
    {
        self.field_schema = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for field_schema: {e}"));
        self
    }

    pub fn timeseries_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::TimeseriesName>,
        T::Error: ::std::fmt::Display,
    {
        self.timeseries_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for timeseries_name: {e}"));
        self
    }

    pub fn units<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Units>,
        T::Error: ::std::fmt::Display,
    {
        self.units = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for units: {e}"));
        self
    }

    pub fn version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::num::NonZeroU8>,
        T::Error: ::std::fmt::Display,
    {
        self.version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for version: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<TimeseriesSchema> for super::TimeseriesSchema {
    type Error = super::error::ConversionError;
    fn try_from(
        value: TimeseriesSchema,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            authz_scope: value.authz_scope?,
            created: value.created?,
            datum_type: value.datum_type?,
            description: value.description?,
            field_schema: value.field_schema?,
            timeseries_name: value.timeseries_name?,
            units: value.units?,
            version: value.version?,
        })
    }
}

impl ::std::convert::From<super::TimeseriesSchema> for TimeseriesSchema {
    fn from(value: super::TimeseriesSchema) -> Self {
        Self {
            authz_scope: Ok(value.authz_scope),
            created: Ok(value.created),
            datum_type: Ok(value.datum_type),
            description: Ok(value.description),
            field_schema: Ok(value.field_schema),
            timeseries_name: Ok(value.timeseries_name),
            units: Ok(value.units),
            version: Ok(value.version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeseriesSchemaResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::TimeseriesSchema>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for TimeseriesSchemaResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl TimeseriesSchemaResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::TimeseriesSchema>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<TimeseriesSchemaResultsPage> for super::TimeseriesSchemaResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: TimeseriesSchemaResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::TimeseriesSchemaResultsPage> for TimeseriesSchemaResultsPage {
    fn from(value: super::TimeseriesSchemaResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TufRepo {
    file_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    hash: ::std::result::Result<super::TufRepoHash, ::std::string::String>,
    system_version: ::std::result::Result<super::TufRepoSystemVersion, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for TufRepo {
    fn default() -> Self {
        Self {
            file_name: Err("no value supplied for file_name".to_string()),
            hash: Err("no value supplied for hash".to_string()),
            system_version: Err("no value supplied for system_version".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
        }
    }
}

impl TufRepo {
    pub fn file_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.file_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for file_name: {e}"));
        self
    }

    pub fn hash<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::TufRepoHash>,
        T::Error: ::std::fmt::Display,
    {
        self.hash = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for hash: {e}"));
        self
    }

    pub fn system_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::TufRepoSystemVersion>,
        T::Error: ::std::fmt::Display,
    {
        self.system_version = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for system_version: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<TufRepo> for super::TufRepo {
    type Error = super::error::ConversionError;
    fn try_from(value: TufRepo) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            file_name: value.file_name?,
            hash: value.hash?,
            system_version: value.system_version?,
            time_created: value.time_created?,
        })
    }
}

impl ::std::convert::From<super::TufRepo> for TufRepo {
    fn from(value: super::TufRepo) -> Self {
        Self {
            file_name: Ok(value.file_name),
            hash: Ok(value.hash),
            system_version: Ok(value.system_version),
            time_created: Ok(value.time_created),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TufRepoResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::TufRepo>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for TufRepoResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl TufRepoResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::TufRepo>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<TufRepoResultsPage> for super::TufRepoResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: TufRepoResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::TufRepoResultsPage> for TufRepoResultsPage {
    fn from(value: super::TufRepoResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TufRepoUpload {
    repo: ::std::result::Result<super::TufRepo, ::std::string::String>,
    status: ::std::result::Result<super::TufRepoUploadStatus, ::std::string::String>,
}

impl ::std::default::Default for TufRepoUpload {
    fn default() -> Self {
        Self {
            repo: Err("no value supplied for repo".to_string()),
            status: Err("no value supplied for status".to_string()),
        }
    }
}

impl TufRepoUpload {
    pub fn repo<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::TufRepo>,
        T::Error: ::std::fmt::Display,
    {
        self.repo = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for repo: {e}"));
        self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::TufRepoUploadStatus>,
        T::Error: ::std::fmt::Display,
    {
        self.status = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for status: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<TufRepoUpload> for super::TufRepoUpload {
    type Error = super::error::ConversionError;
    fn try_from(
        value: TufRepoUpload,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            repo: value.repo?,
            status: value.status?,
        })
    }
}

impl ::std::convert::From<super::TufRepoUpload> for TufRepoUpload {
    fn from(value: super::TufRepoUpload) -> Self {
        Self {
            repo: Ok(value.repo),
            status: Ok(value.status),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TxEqConfig {
    main: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    post1: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    post2: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    pre1: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
    pre2: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
}

impl ::std::default::Default for TxEqConfig {
    fn default() -> Self {
        Self {
            main: Ok(Default::default()),
            post1: Ok(Default::default()),
            post2: Ok(Default::default()),
            pre1: Ok(Default::default()),
            pre2: Ok(Default::default()),
        }
    }
}

impl TxEqConfig {
    pub fn main<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<i32>>,
        T::Error: ::std::fmt::Display,
    {
        self.main = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for main: {e}"));
        self
    }

    pub fn post1<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<i32>>,
        T::Error: ::std::fmt::Display,
    {
        self.post1 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for post1: {e}"));
        self
    }

    pub fn post2<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<i32>>,
        T::Error: ::std::fmt::Display,
    {
        self.post2 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for post2: {e}"));
        self
    }

    pub fn pre1<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<i32>>,
        T::Error: ::std::fmt::Display,
    {
        self.pre1 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for pre1: {e}"));
        self
    }

    pub fn pre2<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<i32>>,
        T::Error: ::std::fmt::Display,
    {
        self.pre2 = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for pre2: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<TxEqConfig> for super::TxEqConfig {
    type Error = super::error::ConversionError;
    fn try_from(value: TxEqConfig) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            main: value.main?,
            post1: value.post1?,
            post2: value.post2?,
            pre1: value.pre1?,
            pre2: value.pre2?,
        })
    }
}

impl ::std::convert::From<super::TxEqConfig> for TxEqConfig {
    fn from(value: super::TxEqConfig) -> Self {
        Self {
            main: Ok(value.main),
            post1: Ok(value.post1),
            post2: Ok(value.post2),
            pre1: Ok(value.pre1),
            pre2: Ok(value.pre2),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnadoptedPhysicalDisk {
    disk_id: ::std::result::Result<super::PhysicalDiskManufacturerIdentity, ::std::string::String>,
    sled_id: ::std::result::Result<super::SledUuid, ::std::string::String>,
    slot: ::std::result::Result<i64, ::std::string::String>,
    variant: ::std::result::Result<super::PhysicalDiskKind, ::std::string::String>,
}

impl ::std::default::Default for UnadoptedPhysicalDisk {
    fn default() -> Self {
        Self {
            disk_id: Err("no value supplied for disk_id".to_string()),
            sled_id: Err("no value supplied for sled_id".to_string()),
            slot: Err("no value supplied for slot".to_string()),
            variant: Err("no value supplied for variant".to_string()),
        }
    }
}

impl UnadoptedPhysicalDisk {
    pub fn disk_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PhysicalDiskManufacturerIdentity>,
        T::Error: ::std::fmt::Display,
    {
        self.disk_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for disk_id: {e}"));
        self
    }

    pub fn sled_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::SledUuid>,
        T::Error: ::std::fmt::Display,
    {
        self.sled_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for sled_id: {e}"));
        self
    }

    pub fn slot<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.slot = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for slot: {e}"));
        self
    }

    pub fn variant<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::PhysicalDiskKind>,
        T::Error: ::std::fmt::Display,
    {
        self.variant = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for variant: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UnadoptedPhysicalDisk> for super::UnadoptedPhysicalDisk {
    type Error = super::error::ConversionError;
    fn try_from(
        value: UnadoptedPhysicalDisk,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            disk_id: value.disk_id?,
            sled_id: value.sled_id?,
            slot: value.slot?,
            variant: value.variant?,
        })
    }
}

impl ::std::convert::From<super::UnadoptedPhysicalDisk> for UnadoptedPhysicalDisk {
    fn from(value: super::UnadoptedPhysicalDisk) -> Self {
        Self {
            disk_id: Ok(value.disk_id),
            sled_id: Ok(value.sled_id),
            slot: Ok(value.slot),
            variant: Ok(value.variant),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnadoptedPhysicalDiskResultsPage {
    items:
        ::std::result::Result<::std::vec::Vec<super::UnadoptedPhysicalDisk>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for UnadoptedPhysicalDiskResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl UnadoptedPhysicalDiskResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::UnadoptedPhysicalDisk>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UnadoptedPhysicalDiskResultsPage>
    for super::UnadoptedPhysicalDiskResultsPage
{
    type Error = super::error::ConversionError;
    fn try_from(
        value: UnadoptedPhysicalDiskResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::UnadoptedPhysicalDiskResultsPage>
    for UnadoptedPhysicalDiskResultsPage
{
    fn from(value: super::UnadoptedPhysicalDiskResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UninitializedSled {
    baseboard: ::std::result::Result<super::Baseboard, ::std::string::String>,
    cubby: ::std::result::Result<u16, ::std::string::String>,
    rack_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for UninitializedSled {
    fn default() -> Self {
        Self {
            baseboard: Err("no value supplied for baseboard".to_string()),
            cubby: Err("no value supplied for cubby".to_string()),
            rack_id: Err("no value supplied for rack_id".to_string()),
        }
    }
}

impl UninitializedSled {
    pub fn baseboard<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Baseboard>,
        T::Error: ::std::fmt::Display,
    {
        self.baseboard = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for baseboard: {e}"));
        self
    }

    pub fn cubby<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.cubby = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cubby: {e}"));
        self
    }

    pub fn rack_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.rack_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rack_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UninitializedSled> for super::UninitializedSled {
    type Error = super::error::ConversionError;
    fn try_from(
        value: UninitializedSled,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            baseboard: value.baseboard?,
            cubby: value.cubby?,
            rack_id: value.rack_id?,
        })
    }
}

impl ::std::convert::From<super::UninitializedSled> for UninitializedSled {
    fn from(value: super::UninitializedSled) -> Self {
        Self {
            baseboard: Ok(value.baseboard),
            cubby: Ok(value.cubby),
            rack_id: Ok(value.rack_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UninitializedSledResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::UninitializedSled>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for UninitializedSledResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl UninitializedSledResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::UninitializedSled>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UninitializedSledResultsPage> for super::UninitializedSledResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: UninitializedSledResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::UninitializedSledResultsPage> for UninitializedSledResultsPage {
    fn from(value: super::UninitializedSledResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UpdateStatus {
    components_by_release_version: ::std::result::Result<
        ::std::collections::HashMap<::std::string::String, u32>,
        ::std::string::String,
    >,
    contact_support: ::std::result::Result<bool, ::std::string::String>,
    suspended: ::std::result::Result<bool, ::std::string::String>,
    target_release:
        ::std::result::Result<::std::option::Option<super::TargetRelease>, ::std::string::String>,
    time_last_step_planned:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for UpdateStatus {
    fn default() -> Self {
        Self {
            components_by_release_version: Err("no value supplied for \
                                                components_by_release_version"
                .to_string()),
            contact_support: Err("no value supplied for contact_support".to_string()),
            suspended: Err("no value supplied for suspended".to_string()),
            target_release: Err("no value supplied for target_release".to_string()),
            time_last_step_planned: Err("no value supplied for time_last_step_planned".to_string()),
        }
    }
}

impl UpdateStatus {
    pub fn components_by_release_version<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::collections::HashMap<::std::string::String, u32>>,
        T::Error: ::std::fmt::Display,
    {
        self.components_by_release_version = value.try_into().map_err(|e| {
            format!("error converting supplied value for components_by_release_version: {e}")
        });
        self
    }

    pub fn contact_support<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.contact_support = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for contact_support: {e}"));
        self
    }

    pub fn suspended<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<bool>,
        T::Error: ::std::fmt::Display,
    {
        self.suspended = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for suspended: {e}"));
        self
    }

    pub fn target_release<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::TargetRelease>>,
        T::Error: ::std::fmt::Display,
    {
        self.target_release = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for target_release: {e}"));
        self
    }

    pub fn time_last_step_planned<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_last_step_planned = value.try_into().map_err(|e| {
            format!("error converting supplied value for time_last_step_planned: {e}")
        });
        self
    }
}

impl ::std::convert::TryFrom<UpdateStatus> for super::UpdateStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: UpdateStatus) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            components_by_release_version: value.components_by_release_version?,
            contact_support: value.contact_support?,
            suspended: value.suspended?,
            target_release: value.target_release?,
            time_last_step_planned: value.time_last_step_planned?,
        })
    }
}

impl ::std::convert::From<super::UpdateStatus> for UpdateStatus {
    fn from(value: super::UpdateStatus) -> Self {
        Self {
            components_by_release_version: Ok(value.components_by_release_version),
            contact_support: Ok(value.contact_support),
            suspended: Ok(value.suspended),
            target_release: Ok(value.target_release),
            time_last_step_planned: Ok(value.time_last_step_planned),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UpdatesTrustRoot {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    root_role: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for UpdatesTrustRoot {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            root_role: Err("no value supplied for root_role".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
        }
    }
}

impl UpdatesTrustRoot {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn root_role<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::serde_json::Value>,
        T::Error: ::std::fmt::Display,
    {
        self.root_role = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for root_role: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UpdatesTrustRoot> for super::UpdatesTrustRoot {
    type Error = super::error::ConversionError;
    fn try_from(
        value: UpdatesTrustRoot,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            root_role: value.root_role?,
            time_created: value.time_created?,
        })
    }
}

impl ::std::convert::From<super::UpdatesTrustRoot> for UpdatesTrustRoot {
    fn from(value: super::UpdatesTrustRoot) -> Self {
        Self {
            id: Ok(value.id),
            root_role: Ok(value.root_role),
            time_created: Ok(value.time_created),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UpdatesTrustRootResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::UpdatesTrustRoot>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for UpdatesTrustRootResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl UpdatesTrustRootResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::UpdatesTrustRoot>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UpdatesTrustRootResultsPage> for super::UpdatesTrustRootResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: UpdatesTrustRootResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::UpdatesTrustRootResultsPage> for UpdatesTrustRootResultsPage {
    fn from(value: super::UpdatesTrustRootResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    display_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    silo_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for User {
    fn default() -> Self {
        Self {
            display_name: Err("no value supplied for display_name".to_string()),
            id: Err("no value supplied for id".to_string()),
            silo_id: Err("no value supplied for silo_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl User {
    pub fn display_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.display_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for display_name: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn silo_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.silo_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for silo_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<User> for super::User {
    type Error = super::error::ConversionError;
    fn try_from(value: User) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            display_name: value.display_name?,
            id: value.id?,
            silo_id: value.silo_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::User> for User {
    fn from(value: super::User) -> Self {
        Self {
            display_name: Ok(value.display_name),
            id: Ok(value.id),
            silo_id: Ok(value.silo_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserBuiltin {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for UserBuiltin {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl UserBuiltin {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UserBuiltin> for super::UserBuiltin {
    type Error = super::error::ConversionError;
    fn try_from(value: UserBuiltin) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::UserBuiltin> for UserBuiltin {
    fn from(value: super::UserBuiltin) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserBuiltinResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::UserBuiltin>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for UserBuiltinResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl UserBuiltinResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::UserBuiltin>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UserBuiltinResultsPage> for super::UserBuiltinResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: UserBuiltinResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::UserBuiltinResultsPage> for UserBuiltinResultsPage {
    fn from(value: super::UserBuiltinResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserCreate {
    external_id: ::std::result::Result<super::UserId, ::std::string::String>,
    password: ::std::result::Result<super::UserPassword, ::std::string::String>,
}

impl ::std::default::Default for UserCreate {
    fn default() -> Self {
        Self {
            external_id: Err("no value supplied for external_id".to_string()),
            password: Err("no value supplied for password".to_string()),
        }
    }
}

impl UserCreate {
    pub fn external_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::UserId>,
        T::Error: ::std::fmt::Display,
    {
        self.external_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for external_id: {e}"));
        self
    }

    pub fn password<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::UserPassword>,
        T::Error: ::std::fmt::Display,
    {
        self.password = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for password: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UserCreate> for super::UserCreate {
    type Error = super::error::ConversionError;
    fn try_from(value: UserCreate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            external_id: value.external_id?,
            password: value.password?,
        })
    }
}

impl ::std::convert::From<super::UserCreate> for UserCreate {
    fn from(value: super::UserCreate) -> Self {
        Self {
            external_id: Ok(value.external_id),
            password: Ok(value.password),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::User>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for UserResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl UserResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::User>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UserResultsPage> for super::UserResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: UserResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::UserResultsPage> for UserResultsPage {
    fn from(value: super::UserResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UsernamePasswordCredentials {
    password: ::std::result::Result<super::Password, ::std::string::String>,
    username: ::std::result::Result<super::UserId, ::std::string::String>,
}

impl ::std::default::Default for UsernamePasswordCredentials {
    fn default() -> Self {
        Self {
            password: Err("no value supplied for password".to_string()),
            username: Err("no value supplied for username".to_string()),
        }
    }
}

impl UsernamePasswordCredentials {
    pub fn password<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Password>,
        T::Error: ::std::fmt::Display,
    {
        self.password = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for password: {e}"));
        self
    }

    pub fn username<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::UserId>,
        T::Error: ::std::fmt::Display,
    {
        self.username = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for username: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<UsernamePasswordCredentials> for super::UsernamePasswordCredentials {
    type Error = super::error::ConversionError;
    fn try_from(
        value: UsernamePasswordCredentials,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            password: value.password?,
            username: value.username?,
        })
    }
}

impl ::std::convert::From<super::UsernamePasswordCredentials> for UsernamePasswordCredentials {
    fn from(value: super::UsernamePasswordCredentials) -> Self {
        Self {
            password: Ok(value.password),
            username: Ok(value.username),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Utilization {
    capacity: ::std::result::Result<super::VirtualResourceCounts, ::std::string::String>,
    provisioned: ::std::result::Result<super::VirtualResourceCounts, ::std::string::String>,
}

impl ::std::default::Default for Utilization {
    fn default() -> Self {
        Self {
            capacity: Err("no value supplied for capacity".to_string()),
            provisioned: Err("no value supplied for provisioned".to_string()),
        }
    }
}

impl Utilization {
    pub fn capacity<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VirtualResourceCounts>,
        T::Error: ::std::fmt::Display,
    {
        self.capacity = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for capacity: {e}"));
        self
    }

    pub fn provisioned<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VirtualResourceCounts>,
        T::Error: ::std::fmt::Display,
    {
        self.provisioned = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for provisioned: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Utilization> for super::Utilization {
    type Error = super::error::ConversionError;
    fn try_from(value: Utilization) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            capacity: value.capacity?,
            provisioned: value.provisioned?,
        })
    }
}

impl ::std::convert::From<super::Utilization> for Utilization {
    fn from(value: super::Utilization) -> Self {
        Self {
            capacity: Ok(value.capacity),
            provisioned: Ok(value.provisioned),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Values {
    metric_type: ::std::result::Result<super::MetricType, ::std::string::String>,
    values: ::std::result::Result<super::ValueArray, ::std::string::String>,
}

impl ::std::default::Default for Values {
    fn default() -> Self {
        Self {
            metric_type: Err("no value supplied for metric_type".to_string()),
            values: Err("no value supplied for values".to_string()),
        }
    }
}

impl Values {
    pub fn metric_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::MetricType>,
        T::Error: ::std::fmt::Display,
    {
        self.metric_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for metric_type: {e}"));
        self
    }

    pub fn values<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ValueArray>,
        T::Error: ::std::fmt::Display,
    {
        self.values = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for values: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Values> for super::Values {
    type Error = super::error::ConversionError;
    fn try_from(value: Values) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            metric_type: value.metric_type?,
            values: value.values?,
        })
    }
}

impl ::std::convert::From<super::Values> for Values {
    fn from(value: super::Values) -> Self {
        Self {
            metric_type: Ok(value.metric_type),
            values: Ok(value.values),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VirtualResourceCounts {
    cpus: ::std::result::Result<i64, ::std::string::String>,
    memory: ::std::result::Result<super::ByteCount, ::std::string::String>,
    storage: ::std::result::Result<super::ByteCount, ::std::string::String>,
}

impl ::std::default::Default for VirtualResourceCounts {
    fn default() -> Self {
        Self {
            cpus: Err("no value supplied for cpus".to_string()),
            memory: Err("no value supplied for memory".to_string()),
            storage: Err("no value supplied for storage".to_string()),
        }
    }
}

impl VirtualResourceCounts {
    pub fn cpus<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<i64>,
        T::Error: ::std::fmt::Display,
    {
        self.cpus = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for cpus: {e}"));
        self
    }

    pub fn memory<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.memory = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for memory: {e}"));
        self
    }

    pub fn storage<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::ByteCount>,
        T::Error: ::std::fmt::Display,
    {
        self.storage = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for storage: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VirtualResourceCounts> for super::VirtualResourceCounts {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VirtualResourceCounts,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            cpus: value.cpus?,
            memory: value.memory?,
            storage: value.storage?,
        })
    }
}

impl ::std::convert::From<super::VirtualResourceCounts> for VirtualResourceCounts {
    fn from(value: super::VirtualResourceCounts) -> Self {
        Self {
            cpus: Ok(value.cpus),
            memory: Ok(value.memory),
            storage: Ok(value.storage),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vpc {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    dns_name: ::std::result::Result<super::Name, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ipv6_prefix: ::std::result::Result<super::Ipv6Net, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    project_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    system_router_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for Vpc {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            dns_name: Err("no value supplied for dns_name".to_string()),
            id: Err("no value supplied for id".to_string()),
            ipv6_prefix: Err("no value supplied for ipv6_prefix".to_string()),
            name: Err("no value supplied for name".to_string()),
            project_id: Err("no value supplied for project_id".to_string()),
            system_router_id: Err("no value supplied for system_router_id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl Vpc {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn dns_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.dns_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for dns_name: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn ipv6_prefix<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Ipv6Net>,
        T::Error: ::std::fmt::Display,
    {
        self.ipv6_prefix = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ipv6_prefix: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn project_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.project_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for project_id: {e}"));
        self
    }

    pub fn system_router_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.system_router_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for system_router_id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<Vpc> for super::Vpc {
    type Error = super::error::ConversionError;
    fn try_from(value: Vpc) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            dns_name: value.dns_name?,
            id: value.id?,
            ipv6_prefix: value.ipv6_prefix?,
            name: value.name?,
            project_id: value.project_id?,
            system_router_id: value.system_router_id?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::Vpc> for Vpc {
    fn from(value: super::Vpc) -> Self {
        Self {
            description: Ok(value.description),
            dns_name: Ok(value.dns_name),
            id: Ok(value.id),
            ipv6_prefix: Ok(value.ipv6_prefix),
            name: Ok(value.name),
            project_id: Ok(value.project_id),
            system_router_id: Ok(value.system_router_id),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcCreate {
    defaults: ::std::result::Result<
        ::std::option::Option<super::VpcCreateDefaults>,
        ::std::string::String,
    >,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    dns_name: ::std::result::Result<super::Name, ::std::string::String>,
    ipv6_prefix:
        ::std::result::Result<::std::option::Option<super::Ipv6Net>, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for VpcCreate {
    fn default() -> Self {
        Self {
            defaults: Ok(Default::default()),
            description: Err("no value supplied for description".to_string()),
            dns_name: Err("no value supplied for dns_name".to_string()),
            ipv6_prefix: Ok(Default::default()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl VpcCreate {
    pub fn defaults<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::VpcCreateDefaults>>,
        T::Error: ::std::fmt::Display,
    {
        self.defaults = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for defaults: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn dns_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.dns_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for dns_name: {e}"));
        self
    }

    pub fn ipv6_prefix<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Ipv6Net>>,
        T::Error: ::std::fmt::Display,
    {
        self.ipv6_prefix = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ipv6_prefix: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcCreate> for super::VpcCreate {
    type Error = super::error::ConversionError;
    fn try_from(value: VpcCreate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            defaults: value.defaults?,
            description: value.description?,
            dns_name: value.dns_name?,
            ipv6_prefix: value.ipv6_prefix?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::VpcCreate> for VpcCreate {
    fn from(value: super::VpcCreate) -> Self {
        Self {
            defaults: Ok(value.defaults),
            description: Ok(value.description),
            dns_name: Ok(value.dns_name),
            ipv6_prefix: Ok(value.ipv6_prefix),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcCreateDefaults {
    subnet: ::std::result::Result<
        ::std::option::Option<super::SubnetCreateDefaults>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for VpcCreateDefaults {
    fn default() -> Self {
        Self {
            subnet: Ok(Default::default()),
        }
    }
}

impl VpcCreateDefaults {
    pub fn subnet<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::SubnetCreateDefaults>>,
        T::Error: ::std::fmt::Display,
    {
        self.subnet = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subnet: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcCreateDefaults> for super::VpcCreateDefaults {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcCreateDefaults,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            subnet: value.subnet?,
        })
    }
}

impl ::std::convert::From<super::VpcCreateDefaults> for VpcCreateDefaults {
    fn from(value: super::VpcCreateDefaults) -> Self {
        Self {
            subnet: Ok(value.subnet),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcFirewallIcmpFilter {
    code:
        ::std::result::Result<::std::option::Option<super::IcmpParamRange>, ::std::string::String>,
    icmp_type: ::std::result::Result<u8, ::std::string::String>,
}

impl ::std::default::Default for VpcFirewallIcmpFilter {
    fn default() -> Self {
        Self {
            code: Ok(Default::default()),
            icmp_type: Err("no value supplied for icmp_type".to_string()),
        }
    }
}

impl VpcFirewallIcmpFilter {
    pub fn code<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::IcmpParamRange>>,
        T::Error: ::std::fmt::Display,
    {
        self.code = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for code: {e}"));
        self
    }

    pub fn icmp_type<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u8>,
        T::Error: ::std::fmt::Display,
    {
        self.icmp_type = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for icmp_type: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcFirewallIcmpFilter> for super::VpcFirewallIcmpFilter {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcFirewallIcmpFilter,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            code: value.code?,
            icmp_type: value.icmp_type?,
        })
    }
}

impl ::std::convert::From<super::VpcFirewallIcmpFilter> for VpcFirewallIcmpFilter {
    fn from(value: super::VpcFirewallIcmpFilter) -> Self {
        Self {
            code: Ok(value.code),
            icmp_type: Ok(value.icmp_type),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcFirewallRule {
    action: ::std::result::Result<super::VpcFirewallRuleAction, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    direction: ::std::result::Result<super::VpcFirewallRuleDirection, ::std::string::String>,
    filters: ::std::result::Result<super::VpcFirewallRuleFilter, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    priority: ::std::result::Result<u16, ::std::string::String>,
    status: ::std::result::Result<super::VpcFirewallRuleStatus, ::std::string::String>,
    targets:
        ::std::result::Result<::std::vec::Vec<super::VpcFirewallRuleTarget>, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    vpc_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for VpcFirewallRule {
    fn default() -> Self {
        Self {
            action: Err("no value supplied for action".to_string()),
            description: Err("no value supplied for description".to_string()),
            direction: Err("no value supplied for direction".to_string()),
            filters: Err("no value supplied for filters".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            priority: Err("no value supplied for priority".to_string()),
            status: Err("no value supplied for status".to_string()),
            targets: Err("no value supplied for targets".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            vpc_id: Err("no value supplied for vpc_id".to_string()),
        }
    }
}

impl VpcFirewallRule {
    pub fn action<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VpcFirewallRuleAction>,
        T::Error: ::std::fmt::Display,
    {
        self.action = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for action: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn direction<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VpcFirewallRuleDirection>,
        T::Error: ::std::fmt::Display,
    {
        self.direction = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for direction: {e}"));
        self
    }

    pub fn filters<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VpcFirewallRuleFilter>,
        T::Error: ::std::fmt::Display,
    {
        self.filters = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for filters: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn priority<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.priority = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for priority: {e}"));
        self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VpcFirewallRuleStatus>,
        T::Error: ::std::fmt::Display,
    {
        self.status = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for status: {e}"));
        self
    }

    pub fn targets<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::VpcFirewallRuleTarget>>,
        T::Error: ::std::fmt::Display,
    {
        self.targets = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for targets: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn vpc_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.vpc_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vpc_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcFirewallRule> for super::VpcFirewallRule {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcFirewallRule,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            action: value.action?,
            description: value.description?,
            direction: value.direction?,
            filters: value.filters?,
            id: value.id?,
            name: value.name?,
            priority: value.priority?,
            status: value.status?,
            targets: value.targets?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            vpc_id: value.vpc_id?,
        })
    }
}

impl ::std::convert::From<super::VpcFirewallRule> for VpcFirewallRule {
    fn from(value: super::VpcFirewallRule) -> Self {
        Self {
            action: Ok(value.action),
            description: Ok(value.description),
            direction: Ok(value.direction),
            filters: Ok(value.filters),
            id: Ok(value.id),
            name: Ok(value.name),
            priority: Ok(value.priority),
            status: Ok(value.status),
            targets: Ok(value.targets),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            vpc_id: Ok(value.vpc_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcFirewallRuleFilter {
    hosts: ::std::result::Result<
        ::std::option::Option<::std::vec::Vec<super::VpcFirewallRuleHostFilter>>,
        ::std::string::String,
    >,
    ports: ::std::result::Result<
        ::std::option::Option<::std::vec::Vec<super::L4PortRange>>,
        ::std::string::String,
    >,
    protocols: ::std::result::Result<
        ::std::option::Option<::std::vec::Vec<super::VpcFirewallRuleProtocol>>,
        ::std::string::String,
    >,
}

impl ::std::default::Default for VpcFirewallRuleFilter {
    fn default() -> Self {
        Self {
            hosts: Ok(Default::default()),
            ports: Ok(Default::default()),
            protocols: Ok(Default::default()),
        }
    }
}

impl VpcFirewallRuleFilter {
    pub fn hosts<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::std::vec::Vec<super::VpcFirewallRuleHostFilter>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.hosts = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for hosts: {e}"));
        self
    }

    pub fn ports<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::vec::Vec<super::L4PortRange>>>,
        T::Error: ::std::fmt::Display,
    {
        self.ports = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ports: {e}"));
        self
    }

    pub fn protocols<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<
            ::std::option::Option<::std::vec::Vec<super::VpcFirewallRuleProtocol>>,
        >,
        T::Error: ::std::fmt::Display,
    {
        self.protocols = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for protocols: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcFirewallRuleFilter> for super::VpcFirewallRuleFilter {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcFirewallRuleFilter,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            hosts: value.hosts?,
            ports: value.ports?,
            protocols: value.protocols?,
        })
    }
}

impl ::std::convert::From<super::VpcFirewallRuleFilter> for VpcFirewallRuleFilter {
    fn from(value: super::VpcFirewallRuleFilter) -> Self {
        Self {
            hosts: Ok(value.hosts),
            ports: Ok(value.ports),
            protocols: Ok(value.protocols),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcFirewallRuleUpdate {
    action: ::std::result::Result<super::VpcFirewallRuleAction, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    direction: ::std::result::Result<super::VpcFirewallRuleDirection, ::std::string::String>,
    filters: ::std::result::Result<super::VpcFirewallRuleFilter, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    priority: ::std::result::Result<u16, ::std::string::String>,
    status: ::std::result::Result<super::VpcFirewallRuleStatus, ::std::string::String>,
    targets:
        ::std::result::Result<::std::vec::Vec<super::VpcFirewallRuleTarget>, ::std::string::String>,
}

impl ::std::default::Default for VpcFirewallRuleUpdate {
    fn default() -> Self {
        Self {
            action: Err("no value supplied for action".to_string()),
            description: Err("no value supplied for description".to_string()),
            direction: Err("no value supplied for direction".to_string()),
            filters: Err("no value supplied for filters".to_string()),
            name: Err("no value supplied for name".to_string()),
            priority: Err("no value supplied for priority".to_string()),
            status: Err("no value supplied for status".to_string()),
            targets: Err("no value supplied for targets".to_string()),
        }
    }
}

impl VpcFirewallRuleUpdate {
    pub fn action<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VpcFirewallRuleAction>,
        T::Error: ::std::fmt::Display,
    {
        self.action = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for action: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn direction<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VpcFirewallRuleDirection>,
        T::Error: ::std::fmt::Display,
    {
        self.direction = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for direction: {e}"));
        self
    }

    pub fn filters<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VpcFirewallRuleFilter>,
        T::Error: ::std::fmt::Display,
    {
        self.filters = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for filters: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn priority<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.priority = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for priority: {e}"));
        self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VpcFirewallRuleStatus>,
        T::Error: ::std::fmt::Display,
    {
        self.status = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for status: {e}"));
        self
    }

    pub fn targets<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::VpcFirewallRuleTarget>>,
        T::Error: ::std::fmt::Display,
    {
        self.targets = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for targets: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcFirewallRuleUpdate> for super::VpcFirewallRuleUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcFirewallRuleUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            action: value.action?,
            description: value.description?,
            direction: value.direction?,
            filters: value.filters?,
            name: value.name?,
            priority: value.priority?,
            status: value.status?,
            targets: value.targets?,
        })
    }
}

impl ::std::convert::From<super::VpcFirewallRuleUpdate> for VpcFirewallRuleUpdate {
    fn from(value: super::VpcFirewallRuleUpdate) -> Self {
        Self {
            action: Ok(value.action),
            description: Ok(value.description),
            direction: Ok(value.direction),
            filters: Ok(value.filters),
            name: Ok(value.name),
            priority: Ok(value.priority),
            status: Ok(value.status),
            targets: Ok(value.targets),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcFirewallRuleUpdateParams {
    rules:
        ::std::result::Result<::std::vec::Vec<super::VpcFirewallRuleUpdate>, ::std::string::String>,
}

impl ::std::default::Default for VpcFirewallRuleUpdateParams {
    fn default() -> Self {
        Self {
            rules: Ok(Default::default()),
        }
    }
}

impl VpcFirewallRuleUpdateParams {
    pub fn rules<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::VpcFirewallRuleUpdate>>,
        T::Error: ::std::fmt::Display,
    {
        self.rules = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rules: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcFirewallRuleUpdateParams> for super::VpcFirewallRuleUpdateParams {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcFirewallRuleUpdateParams,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            rules: value.rules?,
        })
    }
}

impl ::std::convert::From<super::VpcFirewallRuleUpdateParams> for VpcFirewallRuleUpdateParams {
    fn from(value: super::VpcFirewallRuleUpdateParams) -> Self {
        Self {
            rules: Ok(value.rules),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcFirewallRules {
    rules: ::std::result::Result<::std::vec::Vec<super::VpcFirewallRule>, ::std::string::String>,
}

impl ::std::default::Default for VpcFirewallRules {
    fn default() -> Self {
        Self {
            rules: Err("no value supplied for rules".to_string()),
        }
    }
}

impl VpcFirewallRules {
    pub fn rules<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::VpcFirewallRule>>,
        T::Error: ::std::fmt::Display,
    {
        self.rules = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for rules: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcFirewallRules> for super::VpcFirewallRules {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcFirewallRules,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            rules: value.rules?,
        })
    }
}

impl ::std::convert::From<super::VpcFirewallRules> for VpcFirewallRules {
    fn from(value: super::VpcFirewallRules) -> Self {
        Self {
            rules: Ok(value.rules),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::Vpc>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for VpcResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl VpcResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::Vpc>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcResultsPage> for super::VpcResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::VpcResultsPage> for VpcResultsPage {
    fn from(value: super::VpcResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcRouter {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    kind: ::std::result::Result<super::VpcRouterKind, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    vpc_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for VpcRouter {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            kind: Err("no value supplied for kind".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            vpc_id: Err("no value supplied for vpc_id".to_string()),
        }
    }
}

impl VpcRouter {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn kind<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::VpcRouterKind>,
        T::Error: ::std::fmt::Display,
    {
        self.kind = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for kind: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn vpc_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.vpc_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vpc_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcRouter> for super::VpcRouter {
    type Error = super::error::ConversionError;
    fn try_from(value: VpcRouter) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            id: value.id?,
            kind: value.kind?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            vpc_id: value.vpc_id?,
        })
    }
}

impl ::std::convert::From<super::VpcRouter> for VpcRouter {
    fn from(value: super::VpcRouter) -> Self {
        Self {
            description: Ok(value.description),
            id: Ok(value.id),
            kind: Ok(value.kind),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            vpc_id: Ok(value.vpc_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcRouterCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for VpcRouterCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl VpcRouterCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcRouterCreate> for super::VpcRouterCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcRouterCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::VpcRouterCreate> for VpcRouterCreate {
    fn from(value: super::VpcRouterCreate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcRouterResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::VpcRouter>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for VpcRouterResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl VpcRouterResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::VpcRouter>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcRouterResultsPage> for super::VpcRouterResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcRouterResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::VpcRouterResultsPage> for VpcRouterResultsPage {
    fn from(value: super::VpcRouterResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcRouterUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for VpcRouterUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl VpcRouterUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcRouterUpdate> for super::VpcRouterUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcRouterUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::VpcRouterUpdate> for VpcRouterUpdate {
    fn from(value: super::VpcRouterUpdate) -> Self {
        Self {
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcSubnet {
    custom_router_id:
        ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    ipv4_block: ::std::result::Result<super::Ipv4Net, ::std::string::String>,
    ipv6_block: ::std::result::Result<super::Ipv6Net, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    vpc_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
}

impl ::std::default::Default for VpcSubnet {
    fn default() -> Self {
        Self {
            custom_router_id: Ok(Default::default()),
            description: Err("no value supplied for description".to_string()),
            id: Err("no value supplied for id".to_string()),
            ipv4_block: Err("no value supplied for ipv4_block".to_string()),
            ipv6_block: Err("no value supplied for ipv6_block".to_string()),
            name: Err("no value supplied for name".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
            vpc_id: Err("no value supplied for vpc_id".to_string()),
        }
    }
}

impl VpcSubnet {
    pub fn custom_router_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
        T::Error: ::std::fmt::Display,
    {
        self.custom_router_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for custom_router_id: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn ipv4_block<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Ipv4Net>,
        T::Error: ::std::fmt::Display,
    {
        self.ipv4_block = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ipv4_block: {e}"));
        self
    }

    pub fn ipv6_block<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Ipv6Net>,
        T::Error: ::std::fmt::Display,
    {
        self.ipv6_block = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ipv6_block: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }

    pub fn vpc_id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.vpc_id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for vpc_id: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcSubnet> for super::VpcSubnet {
    type Error = super::error::ConversionError;
    fn try_from(value: VpcSubnet) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            custom_router_id: value.custom_router_id?,
            description: value.description?,
            id: value.id?,
            ipv4_block: value.ipv4_block?,
            ipv6_block: value.ipv6_block?,
            name: value.name?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
            vpc_id: value.vpc_id?,
        })
    }
}

impl ::std::convert::From<super::VpcSubnet> for VpcSubnet {
    fn from(value: super::VpcSubnet) -> Self {
        Self {
            custom_router_id: Ok(value.custom_router_id),
            description: Ok(value.description),
            id: Ok(value.id),
            ipv4_block: Ok(value.ipv4_block),
            ipv6_block: Ok(value.ipv6_block),
            name: Ok(value.name),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
            vpc_id: Ok(value.vpc_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcSubnetCreate {
    custom_router:
        ::std::result::Result<::std::option::Option<super::NameOrId>, ::std::string::String>,
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    ipv4_block: ::std::result::Result<super::Ipv4Net, ::std::string::String>,
    ipv6_block: ::std::result::Result<::std::option::Option<super::Ipv6Net>, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
}

impl ::std::default::Default for VpcSubnetCreate {
    fn default() -> Self {
        Self {
            custom_router: Ok(Default::default()),
            description: Err("no value supplied for description".to_string()),
            ipv4_block: Err("no value supplied for ipv4_block".to_string()),
            ipv6_block: Ok(Default::default()),
            name: Err("no value supplied for name".to_string()),
        }
    }
}

impl VpcSubnetCreate {
    pub fn custom_router<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::NameOrId>>,
        T::Error: ::std::fmt::Display,
    {
        self.custom_router = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for custom_router: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn ipv4_block<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Ipv4Net>,
        T::Error: ::std::fmt::Display,
    {
        self.ipv4_block = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ipv4_block: {e}"));
        self
    }

    pub fn ipv6_block<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Ipv6Net>>,
        T::Error: ::std::fmt::Display,
    {
        self.ipv6_block = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for ipv6_block: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcSubnetCreate> for super::VpcSubnetCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcSubnetCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            custom_router: value.custom_router?,
            description: value.description?,
            ipv4_block: value.ipv4_block?,
            ipv6_block: value.ipv6_block?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::VpcSubnetCreate> for VpcSubnetCreate {
    fn from(value: super::VpcSubnetCreate) -> Self {
        Self {
            custom_router: Ok(value.custom_router),
            description: Ok(value.description),
            ipv4_block: Ok(value.ipv4_block),
            ipv6_block: Ok(value.ipv6_block),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcSubnetResultsPage {
    items: ::std::result::Result<::std::vec::Vec<super::VpcSubnet>, ::std::string::String>,
    next_page:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
}

impl ::std::default::Default for VpcSubnetResultsPage {
    fn default() -> Self {
        Self {
            items: Err("no value supplied for items".to_string()),
            next_page: Ok(Default::default()),
        }
    }
}

impl VpcSubnetResultsPage {
    pub fn items<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::VpcSubnet>>,
        T::Error: ::std::fmt::Display,
    {
        self.items = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for items: {e}"));
        self
    }

    pub fn next_page<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.next_page = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for next_page: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcSubnetResultsPage> for super::VpcSubnetResultsPage {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcSubnetResultsPage,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            items: value.items?,
            next_page: value.next_page?,
        })
    }
}

impl ::std::convert::From<super::VpcSubnetResultsPage> for VpcSubnetResultsPage {
    fn from(value: super::VpcSubnetResultsPage) -> Self {
        Self {
            items: Ok(value.items),
            next_page: Ok(value.next_page),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcSubnetUpdate {
    custom_router:
        ::std::result::Result<::std::option::Option<super::NameOrId>, ::std::string::String>,
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for VpcSubnetUpdate {
    fn default() -> Self {
        Self {
            custom_router: Ok(Default::default()),
            description: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl VpcSubnetUpdate {
    pub fn custom_router<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::NameOrId>>,
        T::Error: ::std::fmt::Display,
    {
        self.custom_router = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for custom_router: {e}"));
        self
    }

    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcSubnetUpdate> for super::VpcSubnetUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: VpcSubnetUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            custom_router: value.custom_router?,
            description: value.description?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::VpcSubnetUpdate> for VpcSubnetUpdate {
    fn from(value: super::VpcSubnetUpdate) -> Self {
        Self {
            custom_router: Ok(value.custom_router),
            description: Ok(value.description),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VpcUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    dns_name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for VpcUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            dns_name: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl VpcUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn dns_name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.dns_name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for dns_name: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<VpcUpdate> for super::VpcUpdate {
    type Error = super::error::ConversionError;
    fn try_from(value: VpcUpdate) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            dns_name: value.dns_name?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::VpcUpdate> for VpcUpdate {
    fn from(value: super::VpcUpdate) -> Self {
        Self {
            description: Ok(value.description),
            dns_name: Ok(value.dns_name),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebhookCreate {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    secrets: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    subscriptions:
        ::std::result::Result<::std::vec::Vec<super::AlertSubscription>, ::std::string::String>,
}

impl ::std::default::Default for WebhookCreate {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            endpoint: Err("no value supplied for endpoint".to_string()),
            name: Err("no value supplied for name".to_string()),
            secrets: Err("no value supplied for secrets".to_string()),
            subscriptions: Ok(Default::default()),
        }
    }
}

impl WebhookCreate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn endpoint<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.endpoint = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for endpoint: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn secrets<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.secrets = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for secrets: {e}"));
        self
    }

    pub fn subscriptions<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AlertSubscription>>,
        T::Error: ::std::fmt::Display,
    {
        self.subscriptions = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subscriptions: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<WebhookCreate> for super::WebhookCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: WebhookCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            endpoint: value.endpoint?,
            name: value.name?,
            secrets: value.secrets?,
            subscriptions: value.subscriptions?,
        })
    }
}

impl ::std::convert::From<super::WebhookCreate> for WebhookCreate {
    fn from(value: super::WebhookCreate) -> Self {
        Self {
            description: Ok(value.description),
            endpoint: Ok(value.endpoint),
            name: Ok(value.name),
            secrets: Ok(value.secrets),
            subscriptions: Ok(value.subscriptions),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebhookDeliveryAttempt {
    attempt: ::std::result::Result<u32, ::std::string::String>,
    response: ::std::result::Result<
        ::std::option::Option<super::WebhookDeliveryResponse>,
        ::std::string::String,
    >,
    result: ::std::result::Result<super::WebhookDeliveryAttemptResult, ::std::string::String>,
    time_sent:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for WebhookDeliveryAttempt {
    fn default() -> Self {
        Self {
            attempt: Err("no value supplied for attempt".to_string()),
            response: Ok(Default::default()),
            result: Err("no value supplied for result".to_string()),
            time_sent: Err("no value supplied for time_sent".to_string()),
        }
    }
}

impl WebhookDeliveryAttempt {
    pub fn attempt<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.attempt = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for attempt: {e}"));
        self
    }

    pub fn response<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::WebhookDeliveryResponse>>,
        T::Error: ::std::fmt::Display,
    {
        self.response = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for response: {e}"));
        self
    }

    pub fn result<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::WebhookDeliveryAttemptResult>,
        T::Error: ::std::fmt::Display,
    {
        self.result = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for result: {e}"));
        self
    }

    pub fn time_sent<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_sent = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_sent: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<WebhookDeliveryAttempt> for super::WebhookDeliveryAttempt {
    type Error = super::error::ConversionError;
    fn try_from(
        value: WebhookDeliveryAttempt,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            attempt: value.attempt?,
            response: value.response?,
            result: value.result?,
            time_sent: value.time_sent?,
        })
    }
}

impl ::std::convert::From<super::WebhookDeliveryAttempt> for WebhookDeliveryAttempt {
    fn from(value: super::WebhookDeliveryAttempt) -> Self {
        Self {
            attempt: Ok(value.attempt),
            response: Ok(value.response),
            result: Ok(value.result),
            time_sent: Ok(value.time_sent),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebhookDeliveryResponse {
    duration_ms: ::std::result::Result<u32, ::std::string::String>,
    status: ::std::result::Result<u16, ::std::string::String>,
}

impl ::std::default::Default for WebhookDeliveryResponse {
    fn default() -> Self {
        Self {
            duration_ms: Err("no value supplied for duration_ms".to_string()),
            status: Err("no value supplied for status".to_string()),
        }
    }
}

impl WebhookDeliveryResponse {
    pub fn duration_ms<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u32>,
        T::Error: ::std::fmt::Display,
    {
        self.duration_ms = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for duration_ms: {e}"));
        self
    }

    pub fn status<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<u16>,
        T::Error: ::std::fmt::Display,
    {
        self.status = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for status: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<WebhookDeliveryResponse> for super::WebhookDeliveryResponse {
    type Error = super::error::ConversionError;
    fn try_from(
        value: WebhookDeliveryResponse,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            duration_ms: value.duration_ms?,
            status: value.status?,
        })
    }
}

impl ::std::convert::From<super::WebhookDeliveryResponse> for WebhookDeliveryResponse {
    fn from(value: super::WebhookDeliveryResponse) -> Self {
        Self {
            duration_ms: Ok(value.duration_ms),
            status: Ok(value.status),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebhookReceiver {
    description: ::std::result::Result<::std::string::String, ::std::string::String>,
    endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    name: ::std::result::Result<super::Name, ::std::string::String>,
    secrets: ::std::result::Result<::std::vec::Vec<super::WebhookSecret>, ::std::string::String>,
    subscriptions:
        ::std::result::Result<::std::vec::Vec<super::AlertSubscription>, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    time_modified:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for WebhookReceiver {
    fn default() -> Self {
        Self {
            description: Err("no value supplied for description".to_string()),
            endpoint: Err("no value supplied for endpoint".to_string()),
            id: Err("no value supplied for id".to_string()),
            name: Err("no value supplied for name".to_string()),
            secrets: Err("no value supplied for secrets".to_string()),
            subscriptions: Err("no value supplied for subscriptions".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
            time_modified: Err("no value supplied for time_modified".to_string()),
        }
    }
}

impl WebhookReceiver {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn endpoint<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.endpoint = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for endpoint: {e}"));
        self
    }

    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<super::Name>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }

    pub fn secrets<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::WebhookSecret>>,
        T::Error: ::std::fmt::Display,
    {
        self.secrets = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for secrets: {e}"));
        self
    }

    pub fn subscriptions<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::AlertSubscription>>,
        T::Error: ::std::fmt::Display,
    {
        self.subscriptions = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for subscriptions: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }

    pub fn time_modified<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_modified = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_modified: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<WebhookReceiver> for super::WebhookReceiver {
    type Error = super::error::ConversionError;
    fn try_from(
        value: WebhookReceiver,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            endpoint: value.endpoint?,
            id: value.id?,
            name: value.name?,
            secrets: value.secrets?,
            subscriptions: value.subscriptions?,
            time_created: value.time_created?,
            time_modified: value.time_modified?,
        })
    }
}

impl ::std::convert::From<super::WebhookReceiver> for WebhookReceiver {
    fn from(value: super::WebhookReceiver) -> Self {
        Self {
            description: Ok(value.description),
            endpoint: Ok(value.endpoint),
            id: Ok(value.id),
            name: Ok(value.name),
            secrets: Ok(value.secrets),
            subscriptions: Ok(value.subscriptions),
            time_created: Ok(value.time_created),
            time_modified: Ok(value.time_modified),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebhookReceiverUpdate {
    description:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    endpoint:
        ::std::result::Result<::std::option::Option<::std::string::String>, ::std::string::String>,
    name: ::std::result::Result<::std::option::Option<super::Name>, ::std::string::String>,
}

impl ::std::default::Default for WebhookReceiverUpdate {
    fn default() -> Self {
        Self {
            description: Ok(Default::default()),
            endpoint: Ok(Default::default()),
            name: Ok(Default::default()),
        }
    }
}

impl WebhookReceiverUpdate {
    pub fn description<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.description = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for description: {e}"));
        self
    }

    pub fn endpoint<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
        T::Error: ::std::fmt::Display,
    {
        self.endpoint = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for endpoint: {e}"));
        self
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::option::Option<super::Name>>,
        T::Error: ::std::fmt::Display,
    {
        self.name = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for name: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<WebhookReceiverUpdate> for super::WebhookReceiverUpdate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: WebhookReceiverUpdate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            description: value.description?,
            endpoint: value.endpoint?,
            name: value.name?,
        })
    }
}

impl ::std::convert::From<super::WebhookReceiverUpdate> for WebhookReceiverUpdate {
    fn from(value: super::WebhookReceiverUpdate) -> Self {
        Self {
            description: Ok(value.description),
            endpoint: Ok(value.endpoint),
            name: Ok(value.name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebhookSecret {
    id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    time_created:
        ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
}

impl ::std::default::Default for WebhookSecret {
    fn default() -> Self {
        Self {
            id: Err("no value supplied for id".to_string()),
            time_created: Err("no value supplied for time_created".to_string()),
        }
    }
}

impl WebhookSecret {
    pub fn id<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::uuid::Uuid>,
        T::Error: ::std::fmt::Display,
    {
        self.id = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for id: {e}"));
        self
    }

    pub fn time_created<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
        T::Error: ::std::fmt::Display,
    {
        self.time_created = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for time_created: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<WebhookSecret> for super::WebhookSecret {
    type Error = super::error::ConversionError;
    fn try_from(
        value: WebhookSecret,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            id: value.id?,
            time_created: value.time_created?,
        })
    }
}

impl ::std::convert::From<super::WebhookSecret> for WebhookSecret {
    fn from(value: super::WebhookSecret) -> Self {
        Self {
            id: Ok(value.id),
            time_created: Ok(value.time_created),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebhookSecretCreate {
    secret: ::std::result::Result<::std::string::String, ::std::string::String>,
}

impl ::std::default::Default for WebhookSecretCreate {
    fn default() -> Self {
        Self {
            secret: Err("no value supplied for secret".to_string()),
        }
    }
}

impl WebhookSecretCreate {
    pub fn secret<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::string::String>,
        T::Error: ::std::fmt::Display,
    {
        self.secret = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for secret: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<WebhookSecretCreate> for super::WebhookSecretCreate {
    type Error = super::error::ConversionError;
    fn try_from(
        value: WebhookSecretCreate,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            secret: value.secret?,
        })
    }
}

impl ::std::convert::From<super::WebhookSecretCreate> for WebhookSecretCreate {
    fn from(value: super::WebhookSecretCreate) -> Self {
        Self {
            secret: Ok(value.secret),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebhookSecrets {
    secrets: ::std::result::Result<::std::vec::Vec<super::WebhookSecret>, ::std::string::String>,
}

impl ::std::default::Default for WebhookSecrets {
    fn default() -> Self {
        Self {
            secrets: Err("no value supplied for secrets".to_string()),
        }
    }
}

impl WebhookSecrets {
    pub fn secrets<T>(mut self, value: T) -> Self
    where
        T: ::std::convert::TryInto<::std::vec::Vec<super::WebhookSecret>>,
        T::Error: ::std::fmt::Display,
    {
        self.secrets = value
            .try_into()
            .map_err(|e| format!("error converting supplied value for secrets: {e}"));
        self
    }
}

impl ::std::convert::TryFrom<WebhookSecrets> for super::WebhookSecrets {
    type Error = super::error::ConversionError;
    fn try_from(
        value: WebhookSecrets,
    ) -> ::std::result::Result<Self, super::error::ConversionError> {
        Ok(Self {
            secrets: value.secrets?,
        })
    }
}

impl ::std::convert::From<super::WebhookSecrets> for WebhookSecrets {
    fn from(value: super::WebhookSecrets) -> Self {
        Self {
            secrets: Ok(value.secrets),
        }
    }
}
