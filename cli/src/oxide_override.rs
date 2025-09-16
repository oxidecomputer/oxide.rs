// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use crate::generated_cli::CliConfig;
use crate::{eprintln_nopipe, print_nopipe, println_nopipe, IpOrNet};
use anyhow::Context as _;
use base64::Engine;
use oxide::types::{
    AllowedSourceIps, DerEncodedKeyPair, IdpMetadataSource, IpRange, Ipv4Range, Ipv6Range,
};

#[derive(Default)]
pub struct OxideOverride {
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
