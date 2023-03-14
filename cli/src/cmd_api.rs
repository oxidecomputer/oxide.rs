use std::{
    collections::HashMap,
    io::{Read, Write},
};

use anyhow::{anyhow, Result};
use clap::Parser;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

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
#[clap(verbatim_doc_comment)]
#[clap(name = "api")]
pub struct CmdApi {
    /// The endpoint to request.
    #[clap(name = "endpoint", required = true)]
    pub endpoint: String,

    /// The HTTP method for the request.
    #[clap(short = 'X', long)]
    pub method: Option<http::method::Method>,

    /// Make additional HTTP requests to fetch all pages of results.
    #[clap(long, conflicts_with = "input")]
    pub paginate: bool,

    /// Add a typed parameter in key=value format.
    #[clap(short = 'F', long)]
    pub field: Vec<String>,

    /// Add a string parameter in key=value format.
    #[clap(short = 'f', long)]
    pub raw_field: Vec<String>,

    /// The file to use as body for the HTTP request (use "-" to read from standard input).
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse {
    /// The items in the response.
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    /// The pagination information for the response.
    pub next_page: Option<String>,
}

impl CmdApi {
    pub async fn run(&self, ctx: &mut crate::context::Context) -> Result<()> {
        // Let's get the raw client.
        let client = ctx.client.client();

        // Make sure the endpoint starts with a slash.
        let mut endpoint = self.endpoint.to_string();
        if !self.endpoint.starts_with('/') {
            endpoint = format!("/{}", endpoint);
        }

        // Parse the fields.
        let params = self.parse_fields(ctx)?;

        // Set them as our body if they exist.
        let mut b = String::new();
        if !params.is_empty() {
            b = serde_json::to_string(&params)?;
        }

        let mut bytes = b.as_bytes().to_vec();

        // If they didn't specify the method and we have parameters, we'll
        // assume they want to use POST.
        let method = if let Some(m) = &self.method {
            m.clone()
        } else if !params.is_empty() {
            http::method::Method::POST
        } else {
            http::method::Method::GET
        };

        if self.paginate && method != http::method::Method::GET {
            return Err(anyhow!(
                "the `--paginate` option only compatible with GET requests",
            ));
        }

        // Parse the input file.
        if !self.input.is_empty() {
            // Read the input file.

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

                endpoint = add_query_string(&endpoint, &query_string);
            }
        }

        // TODO
        endpoint = add_query_string(&endpoint, "limit=1");

        // Make the request.
        let mut req = client.request(
            method.clone(),
            format!("{}{}", ctx.client.baseurl(), endpoint),
        );

        if !bytes.is_empty() {
            req = req.body(bytes.clone())
        }

        for (key, value) in self.parse_headers()? {
            req = req.header(key, value);
        }

        let resp = req.send().await?;

        // Print the response headers if requested.
        if self.include {
            println!("{:?} {}", resp.version(), resp.status());
            print_headers(ctx, resp.headers())?;
        }

        if resp.status() == 204 {
            return Ok(());
        }

        if !resp.status().is_success() {
            return Err(anyhow!(
                "{} {}",
                resp.status(),
                resp.status().canonical_reason().unwrap_or("")
            ));
        }

        if !self.paginate {
            // Read the response body.
            let result = resp.json::<serde_json::Value>().await?;

            println!("{}", serde_json::to_string_pretty(&result)?);

            Ok(())
        } else {
            println!("[");

            let result = futures::stream::try_unfold(resp, |resp| async {
                let page = resp.json::<PaginatedResponse>().await?;

                if !page.items.is_empty() {
                    let items_out = serde_json::to_string_pretty(&page.items)?;
                    let len = items_out.len();
                    assert_eq!(&items_out[0..2], "[\n");
                    assert_eq!(&items_out[len - 2..], "\n]");
                    let items_core = &items_out[2..len - 2];

                    println!("page");
                    println!("{}", items_core);
                }

                match page.next_page {
                    None => Ok(None),
                    Some(next_page) => {
                        // TODO deal with limit
                        let uri = format!(
                            "{}{}?page_token={}",
                            ctx.client.baseurl(),
                            self.endpoint,
                            next_page,
                        );

                        let mut req = client.request(method.clone(), uri);
                        for (key, value) in self.parse_headers()? {
                            req = req.header(key, value);
                        }

                        match req.send().await {
                            Ok(r) => Ok(Some(((), r))),
                            Err(e) => Err(anyhow::Error::from(e)),
                        }
                    }
                }
            })
            .try_collect::<()>()
            .await;
            // let mut resp = resp;

            // let xxx = loop {
            //     let page = resp.json::<PaginatedResponse>().await?;

            //     if !page.items.is_empty() {
            //         let items_out = serde_json::to_string_pretty(&page.items)?;
            //         let len = items_out.len();
            //         assert_eq!(&items_out[0..2], "[\n");
            //         assert_eq!(&items_out[len - 2..], "\n]");
            //         let items_core = &items_out[2..len - 2];

            //         println!("page");
            //         println!("{}", items_core);
            //     }

            //     let Some(next_page) = page.next_page else {
            //         break Ok(());
            //     };

            //     // TODO deal with limit
            //     let uri = format!(
            //         "{}{}?page_token={}",
            //         ctx.client.baseurl(),
            //         self.endpoint,
            //         next_page,
            //     );

            //     let mut req = client.request(method.clone(), uri);
            //     for (key, value) in self.parse_headers()? {
            //         req = req.header(key, value);
            //     }

            //     match req.send().await {
            //         Ok(r) => {
            //             resp = r;
            //         }
            //         Err(e) => break Err(e),
            //     }
            // };

            // let page = resp.json::<PaginatedResponse>().await?;

            // let xxx = serde_json::to_string_pretty(&page.items)?;
            // assert_eq!(&xxx[0..2], "[\n");
            // let len = xxx.len();
            // assert_eq!(&xxx[len - 2..], "\n]");
            // let items = &xxx[2..len - 2];

            // println!("{items}");

            // let mut maybe_next_page = page.next_page;

            // while let Some(next_page) = maybe_next_page {
            //     println!("page");
            //     let uri = format!(
            //         "{}{}?page_token={}",
            //         ctx.client.baseurl(),
            //         self.endpoint,
            //         next_page
            //     );

            //     let mut req = client.request(method.clone(), uri);
            //     for (key, value) in self.parse_headers()? {
            //         req = req.header(key, value);
            //     }

            //     match req.send().await {
            //         Ok(resp) => {
            //             let page = resp.json::<PaginatedResponse>().await?;

            //             let xxx = serde_json::to_string_pretty(&page.items)?;
            //             assert_eq!(&xxx[0..2], "[\n");
            //             let len = xxx.len();
            //             assert_eq!(&xxx[len - 2..], "\n]");
            //             let items = &xxx[2..len - 2];

            //             println!("{items}");

            //             maybe_next_page = page.next_page;
            //         }
            //         Err(err) => todo!(),
            //     }
            // }

            println!("]");

            //         if !page.items.is_empty() {
            //             page_results.append(&mut page.items);
            //         }

            //         match page.next_page {
            //             Some(next_page) => {
            //                 endpoint =
            //                     add_query_string(&endpoint, &format!("page_token={}", next_page));
            //             }
            //             None => {
            //                 has_next_page = false;
            //             }
            //         }
            //     } else {
            //         // Read the response body.
            //         result = resp.json().await?;
            //         has_next_page = false;
            //     }
            // }

            // if self.paginate {
            //     result = serde_json::Value::Array(page_results);
            // }

            Ok(())
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

    fn parse_fields(
        &self,
        ctx: &crate::context::Context,
    ) -> Result<HashMap<String, serde_json::Value>> {
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

fn print_headers(
    ctx: &crate::context::Context,
    headers: &reqwest::header::HeaderMap,
) -> Result<()> {
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
mod test {
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
