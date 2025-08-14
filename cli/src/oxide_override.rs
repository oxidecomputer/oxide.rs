// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use std::collections::HashSet;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

use crate::generated_cli::CliConfig;
use crate::{eprintln_nopipe, print_nopipe, println_nopipe, IpOrNet};
use anyhow::Context as _;
use base64::Engine;
use comfy_table::{ContentArrangement, Table};
use indexmap::IndexSet;
use oxide::types::{
    AllowedSourceIps, DerEncodedKeyPair, IdpMetadataSource, IpRange, Ipv4Range, Ipv6Range,
};
use schemars::schema::{RootSchema, Schema, SingleOrVec};

const TABLE_NOT_SUPPORTED: &str = "table formatting is not supported for this command";

pub enum OxideOverride {
    Json { needs_comma: AtomicBool },
    Table { table: Box<Mutex<TableFormatter>> },
}

/// Format response values into a table.
///
/// This works as follows:
///
/// 1. Get the `schemars` schema of the endpoint's return type. For the endpoints we support
///    tabular output for, this will always be a JSON object.
///
/// 2. Parse the schema and determine the parameters of the object being returned. The column
///    names displayed will be the parameters of the return object.
///
/// 3. Handle different return types:
///     - **Non-paginated scalar objects**: Use directly.
///     - **Non-paginated array object**: Extract the inner type of the array.
///     - **Paginated output**: Extract the inner type of the collection.
///     - **Enums**: Take the union of parameter names across all variants. Fields not present
///       for a given variant will be blank.
///
/// 4. Apply field filtering if requested by the user. Take the intersection of their list
///    and the available fields.
///
/// 5. Format values for display:
///     - Non-scalar objects are printed as compact JSON.
///     - Strings without any spaces have their quotes stripped.
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
            // Downcase user-requested fields to better match the schema.
            requested_fields: requested_fields.iter().map(|f| f.to_lowercase()).collect(),
            fields_to_print: Vec::new(),
            table,
        }
    }

    fn set_header_fields(&mut self, schema_fields: Vec<String>) {
        let fields_to_print = if !self.requested_fields.is_empty() {
            let requested: HashSet<_> = self.requested_fields.iter().collect();
            let available: HashSet<_> = schema_fields.iter().collect();
            let invalid = requested.difference(&available);

            for field in invalid {
                eprintln_nopipe!("WARNING: '{field}' is not a valid field");
            }

            let mut fields = self.requested_fields.to_vec();
            fields.retain(|f| available.contains(f));
            fields
        } else {
            schema_fields
        };

        let upcased: Vec<_> = fields_to_print.iter().map(|f| f.to_uppercase()).collect();

        self.table.set_header(upcased);
        self.fields_to_print = fields_to_print;
    }

    fn add_row(&mut self, obj: &serde_json::Map<String, serde_json::Value>) {
        let mut row = Vec::with_capacity(self.fields_to_print.len());

        for field in &self.fields_to_print {
            let s = obj.get(field).map(|f| f.to_string()).unwrap_or_default();

            // `to_string` encloses values in double quotes.
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
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        match &self {
            OxideOverride::Json { needs_comma: _ } => {
                let s = serde_json::to_string_pretty(std::ops::Deref::deref(value))
                    .expect("failed to serialize return to json");
                println_nopipe!("{}", s);
            }
            OxideOverride::Table { table: t } => {
                let mut t = t.lock().unwrap();

                let root_schema = schemars::schema_for!(T);
                let (available_fields, obj_type) = success_item_fields(&root_schema);

                if available_fields.is_empty() {
                    println_nopipe!("{TABLE_NOT_SUPPORTED}");
                    return;
                }

                t.set_header_fields(available_fields);

                let serde_json::Value::Object(obj) = serde_json::to_value(value.as_ref())
                    .expect("failed to serialize result to json")
                else {
                    unreachable!("result was not a JSON object");
                };

                match obj_type {
                    ReturnType::Array => {
                        let Some(serde_json::Value::Array(arr)) = obj.values().next() else {
                            unreachable!("object was not an array");
                        };

                        for entry in arr {
                            let serde_json::Value::Object(obj) = entry else {
                                let s = serde_json::to_string_pretty(std::ops::Deref::deref(value))
                                    .expect("failed to serialize result array member to json");
                                println_nopipe!("{}", s);
                                return;
                            };

                            t.add_row(obj);
                        }
                    }
                    ReturnType::Object => {
                        t.add_row(&obj);
                    }
                }

                println_nopipe!("{}", t.table);
            }
        }
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
        match &self {
            OxideOverride::Json { needs_comma } => {
                needs_comma.store(false, std::sync::atomic::Ordering::Relaxed);
                print_nopipe!("[");
            }
            OxideOverride::Table { table: t } => {
                let mut t = t.lock().unwrap();

                let root_schema = schemars::schema_for!(T);
                let available_fields = list_start_fields(&root_schema);

                t.set_header_fields(available_fields);

                if t.fields_to_print.is_empty() {
                    println_nopipe!("{TABLE_NOT_SUPPORTED}");
                }
            }
        }
    }

    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
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
                let s = serde_json::to_value(value).expect("failed to serialize result to json");
                if let serde_json::Value::Object(obj) = s {
                    let mut t = t.lock().unwrap();

                    t.add_row(&obj);
                }
            }
        }
    }

    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
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

enum ReturnType {
    Array,
    Object,
}

/// Find the fields present on a the object returned by `success_item`.
fn success_item_fields(root_schema: &RootSchema) -> (Vec<String>, ReturnType) {
    let props = match (
        root_schema.schema.object.as_ref().map(|o| &o.properties),
        root_schema
            .schema
            .subschemas
            .as_ref()
            .and_then(|s| s.one_of.as_ref()),
    ) {
        // Generally the array items are a single type.
        (Some(props), _) => props,
        // Some endpoints return an enum, e.g.
        // `/v1/anti-affinity-groups/{anti_affinity_group}/members/instance`.
        // Collect the fields from all of the variants.
        (_, Some(variants)) => {
            return (collect_variant_fields(variants), ReturnType::Object);
        }
        // Some endpoints, e.g. `/v1/system/hardware/switch-port/{port}/status` will
        // return an untyped JSON value. Just bail out when we hit these.
        _ => {
            return (Vec::new(), ReturnType::Object);
        }
    };

    // Check if we're receiving an array.
    let arr = match props
        .iter()
        .next()
        .and_then(|(_k, v)| v.clone().into_object().array)
    {
        Some(arr) => arr,
        None => {
            let available_fields = root_schema
                .schema
                .object
                .as_ref()
                .expect("schema missing object")
                .properties
                .keys()
                .cloned()
                .collect();
            return (available_fields, ReturnType::Object);
        }
    };

    // We have an array, determine the type of object it contains.
    let item_type = match arr.items.unwrap() {
        SingleOrVec::Single(s) => {
            let Some(item_name) = s.into_object().reference else {
                // `/v1/instances/{instance}/serial-console` is unique in returning a byte stream
                // instead of a referenced type. We can't display this, so bail out.
                return (Vec::new(), ReturnType::Object);
            };
            let Some(name) = item_name.strip_prefix("#/definitions/") else {
                unimplemented!("returned array type was not a reference");
            };
            name.to_string()
        }
        SingleOrVec::Vec(_) => {
            unimplemented!("nested array of items")
        }
    };

    let ref_fields = get_referenced_type_fields(root_schema, &item_type);
    (ref_fields, ReturnType::Array)
}

/// Find the fields present on individual items in the array type of a `list_start`.
fn list_start_fields(root_schema: &RootSchema) -> Vec<String> {
    let props = &root_schema.schema.object.as_ref().unwrap().properties;

    let items = props
        .get("items")
        .expect("list response type does not have items")
        .clone()
        .into_object()
        .array
        .unwrap()
        .items
        .clone()
        .unwrap();

    let item_type = match items {
        schemars::schema::SingleOrVec::Single(s) => {
            let item_name = s.into_object().reference.clone().unwrap();
            item_name
                .strip_prefix("#/definitions/")
                .unwrap()
                .to_string()
        }
        schemars::schema::SingleOrVec::Vec(_) => {
            unimplemented!("nested array of items")
        }
    };

    get_referenced_type_fields(root_schema, &item_type)
}

/// Get the field names of a type using its name.
fn get_referenced_type_fields(root_schema: &RootSchema, type_name: &str) -> Vec<String> {
    let item_schema = root_schema
        .definitions
        .get(type_name)
        .unwrap()
        .clone()
        .into_object();

    // Generally child items are a single type, but a few APIs return an enum, e.g.
    // `/v1/anti-affinity-groups/{anti_affinity_group}/members`. In this case we collect all of the
    // fields present on the variants.
    match (
        item_schema.object.as_ref(),
        item_schema.subschemas.and_then(|s| s.one_of),
    ) {
        (Some(obj), _) => obj.properties.keys().cloned().collect(),
        (_, Some(variants)) => collect_variant_fields(&variants),
        _ => unimplemented!("item was neither an object nor one_of subschema"),
    }
}

/// Gather the fields present on each variant, removing duplicates and retaining their original order.
fn collect_variant_fields(variants: &[Schema]) -> Vec<String> {
    let mut fields = IndexSet::new();
    for variant in variants {
        fields.extend(
            variant
                .clone()
                .into_object()
                .object
                .as_ref()
                .expect("variant was not an object")
                .properties
                .keys()
                .cloned(),
        );
    }
    fields.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_schema_parsing() {
        // The return types that we don't attempt to print in tabular form.
        let expected_empty = HashSet::from([
            // Unstructured binary data.
            "oxide::types::InstanceSerialConsoleData",
            // No defined return type in the schema, and it contains a number of deeply nested
            // fields.
            "oxide::types::SwitchLinkState",
        ]);

        // We want to confirm that all API return types can be succesfully parsed, but to do this
        // we need their schemas, which requires knowing the type names at compile time. The `xtask
        // --sdk` task will write the file below, which contains the type names and their
        // associated schema.
        mod schemas {
            include!("../tests/data/api_return_types.rs");
        }
        for (type_name, schema) in schemas::schemas() {
            let fields = if type_name.ends_with("Page") {
                super::list_start_fields(&schema)
            } else {
                let (fields, _) = super::success_item_fields(&schema);
                fields
            };

            if !expected_empty.contains(type_name) {
                // Any successfully parsed schema will return at least one field name.
                assert!(!fields.is_empty());
            }
        }
    }
}
