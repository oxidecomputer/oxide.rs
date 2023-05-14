// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

#![forbid(unsafe_code)]

use std::{collections::BTreeMap, net::IpAddr};

use anyhow::Result;
use async_trait::async_trait;
use cli_builder::NewCli;
use context::Context;
use generated_cli::CliOverride;
use oxide_api::types::{IdpMetadataSource, IpRange, Ipv4Range, Ipv6Range};

mod cli_builder;
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
pub trait RunnableCmd: Send + Sync {
    async fn run(&self, ctx: &Context) -> Result<()>;
}

pub fn make_cli() -> NewCli<'static> {
    NewCli::default()
        .add_custom::<cmd_auth::CmdAuth>("auth")
        .add_custom::<cmd_api::CmdApi>("api")
        .add_custom::<cmd_docs::CmdDocs>("docs")
        .add_custom::<cmd_version::CmdVersion>("version")
        .add_custom::<cmd_disk::CmdDiskImport>("disk import")
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let new_cli = make_cli();

    let result = new_cli.run().await;
    if let Err(e) = result {
        println!("error: {}", e);
        std::process::exit(1)
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

    fn execute_saml_identity_provider_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide_api::builder::SamlIdentityProviderCreate,
    ) -> Result<(), String> {
        match matches
            .get_one::<clap::Id>("idp_metadata_source")
            .map(clap::Id::as_str)
        {
            Some("metadata-url") => {
                let value = matches.get_one::<String>("metadata-url").unwrap();
                *request = request.to_owned().body_map(|body| {
                    body.idp_metadata_source(IdpMetadataSource::Url { url: value.clone() })
                });
            }
            Some("metadata-value") => {
                let value = matches.get_one::<String>("metadata-value").unwrap();
                *request = request.to_owned().body_map(|body| {
                    body.idp_metadata_source(IdpMetadataSource::Base64EncodedXml {
                        data: value.clone(),
                    })
                });
            }
            _ => unreachable!("invalid value for idp_metadata_source group"),
        }
        Ok(())
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
