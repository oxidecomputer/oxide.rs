// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

#![forbid(unsafe_code)]

use std::net::IpAddr;

use anyhow::Result;
use async_trait::async_trait;
use cli_builder::NewCli;
use generated_cli::CliOverride;
use oxide::context::Context;
use oxide::types::{IdpMetadataSource, IpRange, Ipv4Range, Ipv6Range};

mod cli_builder;
mod cmd_api;
mod cmd_auth;
mod cmd_disk;
mod cmd_docs;
mod cmd_instance;

mod cmd_version;
#[allow(unused_mut)]
#[allow(unused)] // TODO
#[allow(unused_must_use)] // TODO
#[allow(clippy::clone_on_copy)]
mod generated_cli;

#[async_trait]
pub trait RunnableCmd: Send + Sync {
    async fn run(&self, ctx: &Context) -> Result<()>;
    fn is_subtree() -> bool {
        true
    }
}

pub fn make_cli() -> NewCli<'static> {
    NewCli::default()
        .add_custom::<cmd_auth::CmdAuth>("auth")
        .add_custom::<cmd_api::CmdApi>("api")
        .add_custom::<cmd_docs::CmdDocs>("docs")
        .add_custom::<cmd_version::CmdVersion>("version")
        .add_custom::<cmd_disk::CmdDiskImport>("disk import")
        .add_custom::<cmd_instance::CmdInstanceSerial>("instance serial")
        .add_custom::<cmd_instance::CmdInstanceFromImage>("instance from-image")
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let new_cli = make_cli();

    // Spawn a task so we get this potentially chunky Future off the
    // main thread's stack.
    let result = tokio::spawn(async move { new_cli.run().await })
        .await
        .unwrap();
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1)
    }
}

struct OxideOverride;

impl OxideOverride {
    fn ip_range(matches: &clap::ArgMatches) -> Result<IpRange, String> {
        let first = matches.get_one::<IpAddr>("first").unwrap();
        let last = matches.get_one::<IpAddr>("last").unwrap();

        match (first, last) {
            (IpAddr::V4(first), IpAddr::V4(last)) => {
                let range = Ipv4Range::try_from(Ipv4Range::builder().first(*first).last(*last))
                    .map_err(|e| e.to_string())?;
                Ok(range.into())
            }
            (IpAddr::V6(first), IpAddr::V6(last)) => {
                let range = Ipv6Range::try_from(Ipv6Range::builder().first(*first).last(*last))
                    .map_err(|e| e.to_string())?;
                Ok(range.into())
            }
            _ => Err("first and last must either both be ipv4 or ipv6 addresses".to_string()),
        }
    }
}

impl CliOverride for OxideOverride {
    // Deal with all the operations that require an `IpPool` as input
    fn execute_ip_pool_range_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::IpPoolRangeAdd,
    ) -> Result<(), String> {
        *request = request.to_owned().body(Self::ip_range(matches)?);
        Ok(())
    }
    fn execute_ip_pool_range_remove(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::IpPoolRangeRemove,
    ) -> Result<(), String> {
        *request = request.to_owned().body(Self::ip_range(matches)?);
        Ok(())
    }
    fn execute_ip_pool_service_range_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::IpPoolServiceRangeAdd,
    ) -> Result<(), String> {
        *request = request.to_owned().body(Self::ip_range(matches)?);
        Ok(())
    }
    fn execute_ip_pool_service_range_remove(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::IpPoolServiceRangeRemove,
    ) -> Result<(), String> {
        *request = request.to_owned().body(Self::ip_range(matches)?);
        Ok(())
    }

    fn execute_saml_identity_provider_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::SamlIdentityProviderCreate,
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
    use clap::Command;
    use expectorate::assert_contents;
    use oxide::types::ByteCount;

    use crate::make_cli;

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
    impl MyFromStr for oxide::types::ByteCount {
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

    #[test]
    fn test_json_body_required() {
        fn find_json_body_required(path: String, cmd: &Command) -> Vec<String> {
            let mut ret = cmd
                .get_subcommands()
                .flat_map(|subcmd| {
                    find_json_body_required(format!("{} {}", path, subcmd.get_name()), subcmd)
                })
                .collect::<Vec<_>>();

            if cmd
                .get_arguments()
                .any(|arg| arg.get_long() == Some("json-body") && arg.is_required_set())
            {
                ret.push(path);
            }

            ret
        }

        let cli = make_cli();
        let cmd = cli.command();
        let out = find_json_body_required("oxide".to_string(), cmd).join("\n");

        // We want this list to shrink, not grow.
        assert_contents("tests/data/json-body-required.txt", &out);
    }
}
