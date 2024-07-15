// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::{
    collections::HashMap,
    io::{Read, Write},
};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use clap::Parser;
use futures::{StreamExt, TryStreamExt};
use oxide::Client;
use serde::Deserialize;

/// Makes an authenticated HTTP request to the Oxide API and prints the response.
///
/// The endpoint argument should be a path of a Oxide API endpoint.
///
/// The default HTTP request method is "GET" normally and "POST" if any
/// parameters were added. Override the method with `--method`.
///
/// Pass one or more `-f/--raw-field` values in "key=value" format to add
/// static string parameters to the request payload. To add non-string or
/// otherwise dynamic values, see `--field` below. Note that adding request
/// parameters will automatically switch the request method to POST. To send
/// the parameters as a GET query string instead, use `--method GET`.
///
/// The `-F/--field` flag has magic type conversion based on the format of the
/// value:
///
/// - literal values "true", "false", "null", and integer/float numbers get
///   converted to appropriate JSON types;
/// - if the value starts with "@", the rest of the value is interpreted as a
///   filename to read the value from. Pass "-" to read from standard input.
///
/// Raw request body may be passed from the outside via a file specified by
/// `--input`. Pass "-" to read from standard input. In this mode, parameters
/// specified via `--field` flags are serialized into URL query parameters.
///
/// In `--paginate` mode, all pages of results will sequentially be requested
/// until there are no more pages of results.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "api")]
pub struct CmdApi {
    /// The endpoint to request.
    #[clap(name = "endpoint", required = true)]
    pub endpoint: String,

    /// The HTTP method for the request.
    #[clap(short = 'X', long)]
    pub method: Option<reqwest::Method>,

    /// Make additional HTTP requests to fetch all pages of results.
    #[clap(long, conflicts_with = "input")]
    pub paginate: bool,

    /// Limit the number of paginated items retrieved.
    #[clap(long, requires = "paginate")]
    pub limit: Option<std::num::NonZeroU32>,

    /// Add a typed parameter in key=value format.
    #[clap(short = 'F', long)]
    pub field: Vec<String>,

    /// Add a string parameter in key=value format.
    #[clap(short = 'f', long)]
    pub raw_field: Vec<String>,

    /// The file to use as body for the HTTP request (use "-" to read from
    /// standard input).
    #[clap(long, default_value = "", conflicts_with = "paginate")]
    pub input: String,

    /// Include HTTP response headers in the output.
    #[clap(short, long)]
    pub include: bool,

    /// Add a HTTP request header in `key:value` format.
    #[clap(short = 'H', long)]
    pub header: Vec<String>,
}

/// The JSON type for a paginated response.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse {
    /// The items in the response.
    pub items: Vec<serde_json::Value>,
    /// The pagination information for the response.
    #[serde(default)]
    pub next_page: Option<String>,
}

#[async_trait]
impl crate::AuthenticatedCmd for CmdApi {
    async fn run(&self, client: &Client) -> Result<()> {
        // Make sure the endpoint starts with a slash.
        let endpoint = if self.endpoint.starts_with('/') {
            self.endpoint.clone()
        } else {
            format!("/{}", self.endpoint)
        };

        // Parse the fields.
        let params = self.parse_fields()?;

        // Set them as our body if they exist.
        let mut b = String::new();
        if !params.is_empty() {
            b = serde_json::to_string(&params)?;
        }

        let mut bytes = b.as_bytes().to_vec();

        // If no method is specified and we have parameters, we'll assume the
        // user wants to use POST.
        let method = if let Some(m) = &self.method {
            m.clone()
        } else if !params.is_empty() {
            reqwest::Method::POST
        } else {
            reqwest::Method::GET
        };

        if self.paginate && method != reqwest::Method::GET {
            return Err(anyhow!(
                "the `--paginate` option is only compatible with GET requests",
            ));
        }

        let mut endpoint_with_query = endpoint.clone();

        // Parse the input file.
        if !self.input.is_empty() {
            let mut buf = Vec::new();
            if self.input == "-" {
                // Read from stdin.
                // TODO
                std::io::stdin().read_to_end(&mut buf)?;
            } else {
                let mut input_file = std::fs::File::open(&self.input)?;
                input_file.read_to_end(&mut buf)?;
            }

            // Set this as our body.
            bytes = buf.clone();

            // Set our params to the query string.
            if !params.is_empty() {
                let mut query_string = String::new();
                for (key, value) in params {
                    if !query_string.is_empty() {
                        query_string.push('&');
                    }
                    query_string.push_str(&format!("{}={}", key, value));
                }

                endpoint_with_query = add_query_string(&endpoint_with_query, &query_string);
            }
        }

        let rclient = client.client();
        let uri = format!("{}{}", client.baseurl(), endpoint_with_query);

        // Make the request.
        let mut req = rclient.request(method.clone(), uri);

        if !bytes.is_empty() {
            req = req.body(bytes.clone())
        }

        let headers = self.parse_headers()?;
        for (key, value) in headers.clone() {
            req = req.header(key, value);
        }

        let resp = req.send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let v = resp.json::<serde_json::Value>().await?;
            println!(
                "error; status code: {} {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or_default()
            );
            println!("{}", serde_json::to_string_pretty(&v).unwrap());
            return Err(anyhow!("error"));
        }

        // Print the response headers if requested.
        if self.include {
            println!("{:?} {}", resp.version(), resp.status());
            print_headers(resp.headers())?;
        }

        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(());
        }

        if !self.paginate {
            // Print the response body.
            let result = resp.json::<serde_json::Value>().await?;
            println!("{}", serde_json::to_string_pretty(&result)?);

            Ok(())
        } else {
            // If this is a paginated request, wrap the output in brackets, and
            // print out the contents of each page to make a unified json array
            // as the output.

            // Remove any query parameters for subsequent requests.
            let endpoint = match endpoint.split_once('?') {
                Some((prefix, _)) => prefix,
                None => endpoint.as_str(),
            };

            let PaginatedResponse { items, next_page } = resp.json::<PaginatedResponse>().await?;
            let first = futures::stream::once(futures::future::ready(Ok(items)));
            let rest = futures::stream::try_unfold(next_page, |maybe_page_token| async {
                let Some(page_token) = maybe_page_token else {
                    return Result::<Option<(Vec<serde_json::Value>, Option<String>)>>::Ok(None);
                };
                let uri = format!("{}{}?page_token={}", client.baseurl(), endpoint, page_token,);

                let mut req = rclient.request(method.clone(), uri);
                for (key, value) in headers.clone() {
                    req = req.header(key, value);
                }

                let resp = req.send().await?.error_for_status()?;
                let PaginatedResponse { items, next_page } =
                    resp.json::<PaginatedResponse>().await?;

                Result::Ok(Some((items, next_page)))
            });

            print!("[");

            let result = first
                .chain(rest)
                .try_fold(false, |comma_needed, items| async move {
                    if !items.is_empty() {
                        let value_out = serde_json::to_string_pretty(&items)?;
                        let len = value_out.len();
                        assert_eq!(&value_out[0..2], "[\n");
                        assert_eq!(&value_out[len - 2..], "\n]");
                        let items_core = &value_out[2..len - 2];

                        if comma_needed {
                            print!(",");
                        }
                        println!();
                        print!("{}", items_core);
                    }
                    Ok(true)
                })
                .await;
            println!();
            println!("]");

            if let Err(e) = &result {
                println!("An error occurred during a paginated query:\n{}", e);
            }
            result.map(|_| ())
        }
    }
}

impl CmdApi {
    fn parse_headers(&self) -> Result<HashMap<String, String>> {
        let mut headers: HashMap<String, String> = HashMap::new();

        for h in self.header.iter() {
            let mut parts = h.splitn(2, ':');
            let key = parts
                .next()
                .ok_or_else(|| anyhow!("missing key in --header"))?;
            let value = parts
                .next()
                .ok_or_else(|| anyhow!("missing value in --header"))?;

            headers.insert(key.to_string(), value.to_string());
        }

        Ok(headers)
    }

    fn parse_fields(&self) -> Result<HashMap<String, serde_json::Value>> {
        let mut params: HashMap<String, serde_json::Value> = HashMap::new();

        // Parse the raw fields.
        // These are always added as strings.
        for f in self.raw_field.iter() {
            let mut parts = f.splitn(2, '=');
            let key = parts
                .next()
                .ok_or_else(|| anyhow!("missing key in --raw-field"))?;
            let value = parts
                .next()
                .ok_or_else(|| anyhow!("missing value in --raw-field"))?;

            params.insert(
                key.to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }

        // Parse the typed fields.
        for t in self.field.iter() {
            let mut parts = t.splitn(2, '=');
            let key = parts
                .next()
                .ok_or_else(|| anyhow!("missing key in --field"))?;
            let value = parts
                .next()
                .ok_or_else(|| anyhow!("missing value in --field"))?;

            // See if value parses as an integer.
            if let Ok(i) = value.parse::<i64>() {
                params.insert(key.to_string(), serde_json::Value::Number(i.into()));
                continue;
            }

            // See if value parses as a float.
            if let Ok(f) = value.parse::<f64>() {
                let num = serde_json::Number::from_f64(f)
                    .ok_or_else(|| anyhow!("invalid float {}", f))?;
                params.insert(key.to_string(), serde_json::Value::Number(num));
                continue;
            }

            // Check the rest.
            let value = match value {
                "true" => serde_json::Value::Bool(true),
                "false" => serde_json::Value::Bool(false),
                "null" => serde_json::Value::Null,
                _ => {
                    // Check if we have a file.
                    if value.starts_with('@') {
                        let filename = value.trim_start_matches('@');
                        let mut file = std::fs::File::open(filename)?;
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)?;
                        serde_json::Value::String(contents)
                    } else if value == "-" {
                        // Read from stdin.
                        let mut contents = String::new();
                        // ctx.io.stdin.read_to_string(&mut contents)?;
                        std::io::stdin().read_to_string(&mut contents)?;
                        serde_json::Value::String(contents)
                    } else {
                        serde_json::Value::String(value.to_string())
                    }
                }
            };

            params.insert(key.to_string(), value);
        }

        Ok(params)
    }
}

fn print_headers(headers: &reqwest::header::HeaderMap) -> Result<()> {
    let mut names: Vec<String> = headers.keys().map(|k| k.as_str().to_string()).collect();
    names.sort_by_key(|a| a.to_lowercase());

    // let cs = ctx.io.color_scheme();

    let mut tw = tabwriter::TabWriter::new(vec![]);
    for name in names {
        if name.to_lowercase() == "status" {
            continue;
        }

        let value = headers.get(name.as_str()).unwrap();

        // writeln!(tw, "{}:\t{:?}", cs.cyan(&name), value)?;
        writeln!(tw, "{}:\t{:?}", name, value)?;
    }

    tw.flush()?;

    let table = String::from_utf8(tw.into_inner()?)?;
    println!("{}", table);

    Ok(())
}

fn add_query_string(endpoint: &str, query_string: &str) -> String {
    if endpoint.contains('?') {
        format!("{}&{}", endpoint, query_string)
    } else {
        format!("{}?{}", endpoint, query_string)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_add_query_string() {
        let mut endpoint = "https://api.github.com/users/octocat/repos";
        let mut query_string = "page=2&per_page=100";

        let mut result = add_query_string(endpoint, query_string);
        let mut expected = "https://api.github.com/users/octocat/repos?page=2&per_page=100";
        assert_eq!(result, expected);

        endpoint = "https://api.github.com/users/octocat/repos?page=2&per_page=100";
        query_string = "foo=bar";

        result = add_query_string(endpoint, query_string);
        expected = "https://api.github.com/users/octocat/repos?page=2&per_page=100&foo=bar";
        assert_eq!(result, expected);
    }
}
