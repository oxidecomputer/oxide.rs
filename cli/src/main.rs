// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

#![forbid(unsafe_code)]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::print_stderr))]

use std::io;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use base64::Engine;
use cli_builder::NewCli;
use context::Context;
use generated_cli::CliConfig;
use oxide::{
    types::{
        AllowedSourceIps, DerEncodedKeyPair, IdpMetadataSource, IpRange, Ipv4Range, Ipv6Range,
    },
    Client,
};
use url::Url;

mod cmd_api;
mod cmd_auth;
mod cmd_completion;
mod cmd_disk;
mod cmd_docs;
mod cmd_instance;
mod cmd_net;
mod cmd_support_bundle;
mod cmd_timeseries;
mod cmd_update;
mod cmd_version;

mod cli_builder;
mod context;
#[macro_use]
mod print;
mod util;

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

    fn maybe_long() -> bool {
        false
    }
}

#[async_trait]
pub trait AuthenticatedCmd: Send + Sync {
    async fn run(&self, client: &Client) -> Result<()>;

    fn is_subtree() -> bool {
        true
    }

    fn maybe_long() -> bool {
        false
    }
}

#[async_trait]
impl<T: AuthenticatedCmd> RunnableCmd for T {
    async fn run(&self, ctx: &Context) -> Result<()> {
        let client = Client::new_authenticated_config(ctx.client_config())?;
        self.run(&client).await
    }
    fn is_subtree() -> bool {
        <T as AuthenticatedCmd>::is_subtree()
    }
    fn maybe_long() -> bool {
        <T as AuthenticatedCmd>::maybe_long()
    }
}

pub fn make_cli() -> NewCli<'static> {
    NewCli::default()
        .add_custom::<cmd_auth::CmdAuth>("auth")
        // Informational commands that don't require server access
        .add_custom::<cmd_docs::CmdDocs>("docs")
        .add_custom::<cmd_completion::CmdCompletion>("completion")
        .add_custom::<cmd_version::CmdVersion>("version")
        // Custom--often compound--client commands
        .add_custom::<cmd_api::CmdApi>("api")
        .add_custom::<cmd_disk::CmdDiskImport>("disk import")
        .add_custom::<cmd_instance::CmdInstanceSerial>("instance serial")
        .add_custom::<cmd_instance::CmdInstanceFromImage>("instance from-image")
        .add_custom::<cmd_timeseries::CmdTimeseriesDashboard>("experimental timeseries dashboard")
        .add_custom::<cmd_net::CmdAddr>("system networking addr")
        .add_custom::<cmd_net::CmdLink>("system networking link")
        .add_custom::<cmd_net::CmdPortConfig>("system networking switch-port-settings show")
        .add_custom::<cmd_net::CmdPortStatus>("system hardware switch-port show-status")
        .add_custom::<cmd_net::CmdBgpStatus>("system networking bgp show-status")
        .add_custom::<cmd_net::CmdBgpPeer>("system networking bgp peer")
        .add_custom::<cmd_net::CmdBgpAnnounce>("system networking bgp announce")
        .add_custom::<cmd_net::CmdBgpWithdraw>("system networking bgp withdraw")
        .add_custom::<cmd_net::CmdBgpFilter>("system networking bgp filter")
        .add_custom::<cmd_net::CmdBgpAuth>("system networking bgp auth")
        .add_custom::<cmd_net::CmdBgpLocalPref>("system networking bgp pref")
        .add_custom::<cmd_net::CmdStaticRoute>("system networking route")
        .add_custom::<cmd_support_bundle::CmdDownload>("bundle download")
        .add_custom::<cmd_support_bundle::CmdInspect>("bundle inspect")
        .add_custom::<cmd_update::CmdUpload>("system update repo upload")
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let new_cli = make_cli();

    if let Err(e) = new_cli.run().await {
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            if io_err.kind() == io::ErrorKind::BrokenPipe {
                return;
            }
        }

        eprintln_nopipe!("{e:#}");
        std::process::exit(1)
    }
}

#[derive(Default)]
struct OxideOverride {
    needs_comma: AtomicBool,
}

impl OxideOverride {
    fn ip_range(matches: &clap::ArgMatches) -> anyhow::Result<IpRange> {
        let first = matches.get_one::<IpAddr>("first").unwrap();
        let last = matches.get_one::<IpAddr>("last").unwrap();

        match (first, last) {
            (IpAddr::V4(first), IpAddr::V4(last)) => {
                let range = Ipv4Range::try_from(Ipv4Range::builder().first(*first).last(*last))?;
                Ok(range.into())
            }
            (IpAddr::V6(first), IpAddr::V6(last)) => {
                let range = Ipv6Range::try_from(Ipv6Range::builder().first(*first).last(*last))?;
                Ok(range.into())
            }
            _ => anyhow::bail!(
                "first and last must either both be ipv4 or ipv6 addresses".to_string()
            ),
        }
    }
}

impl CliConfig for OxideOverride {
    fn success_item<T>(&self, value: &oxide::ResponseValue<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        let s = serde_json::to_string_pretty(std::ops::Deref::deref(value))
            .expect("failed to serialize return to json");
        println_nopipe!("{}", s);
    }

    fn success_no_item(&self, _: &oxide::ResponseValue<()>) {}

    fn error<T>(&self, _value: &oxide::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        eprintln_nopipe!("error");
    }

    fn list_start<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        self.needs_comma
            .store(false, std::sync::atomic::Ordering::Relaxed);
        print_nopipe!("[");
    }

    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        let s = serde_json::to_string_pretty(&[value]).expect("failed to serialize result to json");
        if self.needs_comma.load(std::sync::atomic::Ordering::Relaxed) {
            print_nopipe!(", {}", &s[4..s.len() - 2]);
        } else {
            print_nopipe!("\n{}", &s[2..s.len() - 2]);
        };
        self.needs_comma
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        if self.needs_comma.load(std::sync::atomic::Ordering::Relaxed) {
            println_nopipe!("\n]");
        } else {
            println_nopipe!("]");
        }
    }

    fn list_end_error<T>(&self, _value: &oxide::Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        self.list_end_success::<T>()
    }

    // Deal with all the operations that require an `IpPool` as input
    fn execute_ip_pool_range_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::IpPoolRangeAdd,
    ) -> anyhow::Result<()> {
        *request = request.to_owned().body(Self::ip_range(matches)?);
        Ok(())
    }
    fn execute_ip_pool_range_remove(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::IpPoolRangeRemove,
    ) -> anyhow::Result<()> {
        *request = request.to_owned().body(Self::ip_range(matches)?);
        Ok(())
    }
    fn execute_ip_pool_service_range_add(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::IpPoolServiceRangeAdd,
    ) -> anyhow::Result<()> {
        *request = request.to_owned().body(Self::ip_range(matches)?);
        Ok(())
    }
    fn execute_ip_pool_service_range_remove(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::IpPoolServiceRangeRemove,
    ) -> anyhow::Result<()> {
        *request = request.to_owned().body(Self::ip_range(matches)?);
        Ok(())
    }

    fn execute_saml_identity_provider_create(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::SamlIdentityProviderCreate,
    ) -> anyhow::Result<()> {
        match matches
            .get_one::<clap::Id>("idp_metadata_source")
            .map(clap::Id::as_str)
        {
            Some("metadata-url") => {
                let value = matches.get_one::<String>("metadata-url").unwrap();
                *request = request.to_owned().body_map(|body| {
                    body.idp_metadata_source(IdpMetadataSource::Url { url: value.clone() })
                });
                Ok::<_, anyhow::Error>(())
            }
            Some("metadata-value") => {
                let xml_path = matches.get_one::<PathBuf>("metadata-value").unwrap();
                let xml_bytes = std::fs::read(xml_path).with_context(|| {
                    format!("failed to read metadata XML file {}", xml_path.display())
                })?;
                let encoded_xml = base64::engine::general_purpose::STANDARD.encode(xml_bytes);
                *request = request.to_owned().body_map(|body| {
                    body.idp_metadata_source(IdpMetadataSource::Base64EncodedXml {
                        data: encoded_xml,
                    })
                });
                Ok(())
            }
            _ => unreachable!("invalid value for idp_metadata_source group"),
        }?;

        if matches.get_one::<clap::Id>("signing_keypair").is_some() {
            let privkey_path = matches.get_one::<PathBuf>("private-key").unwrap();
            let privkey_bytes = std::fs::read(privkey_path).with_context(|| {
                format!("failed to read private key file {}", privkey_path.display())
            })?;
            let encoded_privkey = base64::engine::general_purpose::STANDARD.encode(&privkey_bytes);

            let cert_path = matches.get_one::<PathBuf>("public-cert").unwrap();
            let cert_bytes = std::fs::read(cert_path).with_context(|| {
                format!("failed to read public cert file {}", cert_path.display())
            })?;
            let encoded_cert = base64::engine::general_purpose::STANDARD.encode(&cert_bytes);

            *request = request.to_owned().body_map(|body| {
                body.signing_keypair(DerEncodedKeyPair {
                    private_key: encoded_privkey,
                    public_cert: encoded_cert,
                })
            });
        }
        Ok(())
    }

    fn execute_networking_allow_list_update(
        &self,
        matches: &clap::ArgMatches,
        request: &mut oxide::builder::NetworkingAllowListUpdate,
    ) -> anyhow::Result<()> {
        match matches
            .get_one::<clap::Id>("allow-list")
            .map(clap::Id::as_str)
        {
            Some("any") => {
                let value = matches.get_one::<bool>("any").unwrap();
                assert!(value);
                *request = request
                    .to_owned()
                    .body_map(|body| body.allowed_ips(AllowedSourceIps::Any));
            }
            Some("ips") => {
                let values: Vec<IpOrNet> = matches.get_many("ips").unwrap().cloned().collect();
                *request = request.to_owned().body_map(|body| {
                    body.allowed_ips(AllowedSourceIps::List(
                        values.into_iter().map(IpOrNet::into_ip_net).collect(),
                    ))
                });
            }
            _ => unreachable!("invalid value for allow-list group"),
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

#[derive(Debug, Clone)]
pub(crate) enum IpOrNet {
    Ip(std::net::IpAddr),
    Net(oxide::types::IpNet),
}

#[derive(Clone, Debug)]
pub(crate) struct IpOrNetParser;
impl clap::builder::TypedValueParser for IpOrNetParser {
    type Value = IpOrNet;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> std::prelude::v1::Result<Self::Value, clap::Error> {
        fn parse(value: &str) -> Result<IpOrNet, String> {
            if let Ok(ip) = value.parse() {
                Ok(IpOrNet::Ip(ip))
            } else if let Ok(net) = value.parse() {
                Ok(IpOrNet::Net(net))
            } else {
                Err("value must be an IP address or subnet".to_string())
            }
        }

        parse.parse_ref(cmd, arg, value)
    }
}

impl clap::builder::ValueParserFactory for IpOrNet {
    type Parser = IpOrNetParser;

    fn value_parser() -> Self::Parser {
        IpOrNetParser
    }
}

impl IpOrNet {
    pub fn into_ip_net(self) -> oxide::types::IpNet {
        match self {
            IpOrNet::Ip(std::net::IpAddr::V4(v4)) => format!("{}/32", v4).parse().unwrap(),
            IpOrNet::Ip(std::net::IpAddr::V6(v6)) => format!("{}/128", v6).parse().unwrap(),
            IpOrNet::Net(net) => net,
        }
    }
}

pub(crate) trait AsHost {
    fn as_host(&self) -> &str;
}

impl AsHost for Url {
    fn as_host(&self) -> &str {
        assert_eq!(self.path(), "/");
        let s = self.as_str();
        &s[..s.len() - 1]
    }
}
