// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use std::collections::HashSet;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

use crate::generated_cli::{CliConfig, ResponseFields};
use crate::{eprintln_nopipe, print_nopipe, println_nopipe, IpOrNet};
use anyhow::Context as _;
use base64::Engine;
use comfy_table::{ContentArrangement, Table};
use oxide::types::{
    AllowedSourceIps, DerEncodedKeyPair, IdpMetadataSource, IpRange, Ipv4Range, Ipv6Range,
};

const NO_USER_REQUESTED_FIELDS: &str =
    "ERROR: None of the requested '--format' fields are present in this command's output";

pub enum OxideOverride {
    Json { needs_comma: AtomicBool },
    Table { table: Box<Mutex<TableFormatter>> },
}

/// Format response values into a table.
pub struct TableFormatter {
    requested_fields: Vec<String>,
    fields_to_print: Vec<String>,
    table: Table,
}

impl TableFormatter {
    fn new(requested_fields: &[String]) -> Self {
        let mut table = Table::new();

        table
            .load_preset(comfy_table::presets::NOTHING)
            .set_content_arrangement(ContentArrangement::Disabled);

        Self {
            // Downcase user-requested fields to better match the return type.
            requested_fields: requested_fields.iter().map(|f| f.to_lowercase()).collect(),
            fields_to_print: Vec::new(),
            table,
        }
    }

    fn set_header_fields(&mut self, available_fields: &[&str]) -> Result<(), String> {
        let fields_to_print = if !self.requested_fields.is_empty() {
            let requested: HashSet<_> = self.requested_fields.iter().map(|s| s.as_str()).collect();
            let available: HashSet<_> = available_fields.iter().copied().collect();
            let invalid = requested.difference(&available);

            for field in invalid {
                eprintln_nopipe!("WARNING: '{field}' is not a valid field");
            }

            let mut fields = self.requested_fields.to_vec();
            fields.retain(|f| available.contains(f.as_str()));

            if fields.is_empty() {
                // Show list of the available fields to the user.
                let field_list = available_fields
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join("\n ");

                return Err(format!(
                    "{NO_USER_REQUESTED_FIELDS}\n\nAvailable fields:\n {field_list}"
                ));
            }

            fields
        } else {
            available_fields.iter().map(|s| s.to_string()).collect()
        };

        let upcased: Vec<_> = fields_to_print.iter().map(|f| f.to_uppercase()).collect();

        self.table.set_header(upcased);
        self.fields_to_print = fields_to_print;

        Ok(())
    }

    fn add_row<T: ResponseFields>(&mut self, obj: &T) {
        let mut row = Vec::with_capacity(self.fields_to_print.len());

        for field in &self.fields_to_print {
            let s = obj
                .get_field(field)
                .filter(|v| !v.is_null()) // Convert Null to None
                .map(|v| v.to_string())
                .unwrap_or_default();

            // `to_string` encloses values in double quotes. Remove these unless we're
            // writing a multi-word field.
            if s.contains(' ') {
                row.push(s);
            } else {
                row.push(s.trim_matches('"').to_string());
            }
        }
        self.table.add_row(row);
    }
}

impl Default for OxideOverride {
    fn default() -> Self {
        Self::new_json()
    }
}

impl CliConfig for OxideOverride {
    fn success_item<T>(&self, value: &oxide::ResponseValue<T>)
    where
        T: ResponseFields + schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        match &self {
            OxideOverride::Json { needs_comma: _ } => {
                let s = serde_json::to_string_pretty(std::ops::Deref::deref(value))
                    .expect("failed to serialize return to json");
                println_nopipe!("{}", s);
            }
            OxideOverride::Table { table: t } => {
                let mut t = t.lock().unwrap();

                if let Err(e) = t.set_header_fields(T::field_names()) {
                    eprintln_nopipe!("{e}");
                    std::process::exit(1);
                }

                for item in value.items() {
                    t.add_row(item);
                }

                println_nopipe!("{}", t.table);
            }
        }
    }

    fn success_no_item(&self, _: &oxide::ResponseValue<()>) {}

    fn error<T>(&self, _value: &oxide::Error<T>)
    where
        T: ResponseFields + schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        eprintln_nopipe!("error");
    }

    fn list_start<T>(&self)
    where
        T: ResponseFields + schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        match &self {
            OxideOverride::Json { needs_comma } => {
                needs_comma.store(false, std::sync::atomic::Ordering::Relaxed);
                print_nopipe!("[");
            }
            OxideOverride::Table { table: t } => {
                let mut t = t.lock().unwrap();

                if let Err(e) = t.set_header_fields(T::field_names()) {
                    eprintln_nopipe!("{e}");
                    std::process::exit(1);
                }
            }
        }
    }

    fn list_item<T>(&self, value: &T)
    where
        T: ResponseFields + schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        match &self {
            OxideOverride::Json { needs_comma } => {
                let s = serde_json::to_string_pretty(&[value])
                    .expect("failed to serialize result to json");
                if needs_comma.load(std::sync::atomic::Ordering::Relaxed) {
                    print_nopipe!(", {}", &s[4..s.len() - 2]);
                } else {
                    print_nopipe!("\n{}", &s[2..s.len() - 2]);
                };
                needs_comma.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            OxideOverride::Table { table: t } => {
                let mut t = t.lock().unwrap();
                t.add_row(value);
            }
        }
    }

    fn list_end_success<T>(&self)
    where
        T: ResponseFields + schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        match &self {
            OxideOverride::Json { needs_comma } => {
                if needs_comma.load(std::sync::atomic::Ordering::Relaxed) {
                    println_nopipe!("\n]");
                } else {
                    println_nopipe!("]");
                }
            }
            OxideOverride::Table { table: t } => {
                let t = t.lock().unwrap();
                println_nopipe!("{}", t.table);
            }
        }
    }

    fn list_end_error<T>(&self, _value: &oxide::Error<T>)
    where
        T: ResponseFields + schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
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

impl OxideOverride {
    /// Construct a new OxideOverride for JSON output.
    pub fn new_json() -> Self {
        OxideOverride::Json {
            needs_comma: AtomicBool::new(false),
        }
    }

    /// Construct a new OxideOverride for tabular output.
    pub fn new_table(fields: &[String]) -> Self {
        OxideOverride::Table {
            table: Box::new(Mutex::new(TableFormatter::new(fields))),
        }
    }

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
