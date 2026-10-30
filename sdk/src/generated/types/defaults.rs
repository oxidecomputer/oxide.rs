// The contents of this file are generated; do not modify them.

pub(super) fn default_bool<const V: bool>() -> bool {
    V
}

pub(super) fn address_allocator_auto_pool_selector() -> super::PoolSelector {
    super::PoolSelector::Auto {
        ip_version: ::std::option::Option::None,
    }
}

pub(super) fn bgp_config_create_max_paths() -> super::MaxPathConfig {
    super::MaxPathConfig(::std::num::NonZeroU8::new(1).unwrap())
}

pub(super) fn ephemeral_ip_create_pool_selector() -> super::PoolSelector {
    super::PoolSelector::Auto {
        ip_version: ::std::option::Option::None,
    }
}

pub(super) fn external_ip_create_ephemeral_pool_selector() -> super::PoolSelector {
    super::PoolSelector::Auto {
        ip_version: ::std::option::Option::None,
    }
}

pub(super) fn external_subnet_allocator_auto_pool_selector() -> super::PoolSelector {
    super::PoolSelector::Auto {
        ip_version: ::std::option::Option::None,
    }
}

pub(super) fn floating_ip_create_address_allocator() -> super::AddressAllocator {
    super::AddressAllocator::Auto {
        pool_selector: super::PoolSelector::Auto {
            ip_version: ::std::option::Option::None,
        },
    }
}

pub(super) fn instance_create_network_interfaces() -> super::InstanceNetworkInterfaceAttachment {
    super::InstanceNetworkInterfaceAttachment::DefaultDualStack
}

pub(super) fn instance_network_interface_create_ip_config() -> super::PrivateIpStackCreate {
    super::PrivateIpStackCreate::DualStack {
        v4: super::PrivateIpv4StackCreate {
            ip: super::Ipv4Assignment::Auto,
            transit_ips: vec![],
        },
        v6: super::PrivateIpv6StackCreate {
            ip: super::Ipv6Assignment::Auto,
            transit_ips: vec![],
        },
    }
}

pub(super) fn ip_pool_create_assignment() -> super::IpPoolAssignment {
    super::IpPoolAssignment::Silos
}

pub(super) fn ip_pool_create_ip_version() -> super::IpVersion {
    super::IpVersion::V4
}

pub(super) fn ip_pool_create_pool_type() -> super::IpPoolType {
    super::IpPoolType::Unicast
}

pub(super) fn probe_create_pool_selector() -> super::PoolSelector {
    super::PoolSelector::Auto {
        ip_version: ::std::option::Option::None,
    }
}
