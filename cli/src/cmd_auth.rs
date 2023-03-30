// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use std::{collections::HashMap, io::Read, time::Duration};

use anyhow::{anyhow, bail, Result};
use clap::Parser;
use httpmock::Method::GET;
use oauth2::{
    basic::BasicClient, devicecode::StandardDeviceAuthorizationResponse,
    reqwest::async_http_client, AuthType, AuthUrl, ClientId, DeviceAuthorizationUrl, TokenResponse,
    TokenUrl,
};
use oxide_api::{Client, ClientHiddenExt, ClientSessionExt};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    ClientBuilder,
};

use crate::{
    config::{Config, Host},
    context::Context,
};

/// Login, logout, and get the status of your authentication.
///
/// Manage `oxide`'s authentication state.
#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
#[clap(name = "auth")]
pub struct CmdAuth {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug, Clone)]
enum SubCommand {
    Login(CmdAuthLogin),
    Logout(CmdAuthLogout),
    Status(CmdAuthStatus),
}

impl CmdAuth {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        match &self.subcmd {
            SubCommand::Login(cmd) => cmd.run(ctx).await,
            SubCommand::Logout(cmd) => cmd.run().await,
            SubCommand::Status(cmd) => cmd.run().await,
        }
    }
}

/// Parse and normalize a given host string as a valid URL.
///
/// http(s) are the only supported schemas. If no schema is specified then
/// https is assumed. The returned URL if successful will be stripped of any
/// path, username, password,
/// fragment or query.
pub fn parse_host(input: &str) -> Result<url::Url> {
    match url::Url::parse(input) {
        Ok(mut url) => {
            if !url.has_host() {
                // We've successfully parsed a URL with no host. This can
                // happen if input was something like `localhost:8080`
                // where `localhost:` is treated as the scheme (`8080` would be
                // the path). Let's try again by prefixing with `https://`.
                return parse_host(&format!("https://{input}"));
            }

            // Make sure scheme is http(s).
            let scheme = url.scheme();
            if scheme != "http" && scheme != "https" {
                anyhow::bail!("non-http(s) scheme given")
            }

            // We're only interested in the scheme, host & port. Clear any
            // other component that was set.
            url.set_path("");
            let _ = url.set_username("");
            let _ = url.set_password(None);
            url.set_fragment(None);
            url.set_query(None);

            Ok(url)
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            // The input is being interpreted as a relative path meaning the
            // input didn't include a scheme mostly likely. Let's try again by
            // prefixing with `https://`
            parse_host(&format!("https://{input}"))
        }
        Err(err) => anyhow::bail!(err),
    }
}

fn custom_authentication_client(host: &str, token: String) -> Client {
    let mut bearer = HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap();
    bearer.set_sensitive(true);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, bearer);

    let rclient = ClientBuilder::new()
        .connect_timeout(Duration::from_secs(15))
        .default_headers(headers)
        .build()
        .unwrap();
    Client::new_with_client(host, rclient)
}

// fn parse_host_interactively(ctx: &mut crate::context::Context) -> Result<url::Url> {
//     loop {
//         match dialoguer::Input::<String>::new()
//             .with_prompt("Oxide instance host (this assumes https:// unless http:// is given as a part of the URL)")
//             .interact_text()
//         {
//             Ok(input) => match parse_host(&input) {
//                 Ok(url) => return Ok(url),
//                 Err(err) => {
//                     writeln!(ctx.io.err_out, "Invalid host specified ({err}). Try again.")?;
//                     continue;
//                 }
//             },
//             Err(err) => anyhow::bail!("host prompt failed: {err}"),
//         }
//     }
// }

/// Authenticate with an Oxide host.
///
/// Alternatively, pass in a token on standard input by using `--with-token`.
///
///     # start interactive setup
///     $ oxide auth login
///
///     # authenticate against a specific Oxide instance by reading the token
///     # from a file
///     $ oxide auth login --with-token --host oxide.internal < mytoken.txt
///
///     # authenticate with a specific Oxide instance
///     $ oxide auth login --host oxide.internal
///
///     # authenticate with an insecure Oxide instance (not recommended)
///     $ oxide auth login --host http://oxide.internal
#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
pub struct CmdAuthLogin {
    /// Read token from standard input.
    #[clap(long)]
    pub with_token: bool,

    /// The host of the Oxide instance to authenticate with.
    /// This assumes http; specify an `http://` prefix if needed.
    #[clap(short = 'H', long, env = "OXIDE_HOST", value_parser = parse_host)]
    pub host: url::Url,
}

// #[async_trait::async_trait]
// impl crate::cmd::Command for CmdAuthLogin {
//     async fn run(&self, ctx: &mut crate::context::Context) -> Result<()> {
impl CmdAuthLogin {
    pub async fn run(&self, ctx: &mut Context) -> Result<()> {
        // if !ctx.io.can_prompt() && !self.with_token {
        //     return Err(anyhow!(
        //         "--with-token required when not running interactively"
        //     ));
        // }

        let token = if self.with_token {
            // Read the token from stdin.
            let mut token = String::new();
            std::io::stdin().read_to_string(&mut token)?;
            token = token.trim_end_matches('\n').to_string();

            if token.is_empty() {
                bail!("token cannot be empty");
            }

            token
        } else {
            let existing_token = ctx
                .config
                .hosts
                .get(&self.host)
                .map(|host_entry| &host_entry.token);

            if existing_token.is_some() {
                match dialoguer::Confirm::new()
                    .with_prompt(format!(
                        "You're already logged into {}\nDo you want to re-authenticate?",
                        &self.host,
                    ))
                    .interact()
                {
                    Ok(true) => {}
                    Ok(false) => {
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(anyhow!("prompt failed: {}", err));
                    }
                }
            }

            // Do an OAuth 2.0 Device Authorization Grant dance to get a token.
            let device_auth_url =
                DeviceAuthorizationUrl::new(format!("{}device/auth", &self.host))?;
            let client_id = &ctx.config.client_id;
            let auth_client = BasicClient::new(
                ClientId::new(client_id.to_string()),
                None,
                AuthUrl::new(format!("{}authorize", &self.host))?,
                Some(TokenUrl::new(format!("{}device/token", &self.host))?),
            )
            .set_auth_type(AuthType::RequestBody)
            .set_device_authorization_url(device_auth_url);

            let details: StandardDeviceAuthorizationResponse = auth_client
                .exchange_device_code()?
                .request_async(async_http_client)
                .await?;

            // TODO ahl kludge
            if let Some(uri) = details.verification_uri_complete() {
                open::that(uri.secret())?;
            }

            // if let Some(uri) = details.verification_uri_complete() {
            //     writeln!(
            //         ctx.io.out,
            //         "Opening {} in your browser.\n\
            //          Please verify user code: {}\n",
            //         uri.secret(),
            //         details.user_code().secret()
            //     )?;
            //     let _ = ctx.browser(host, uri.secret());
            // } else {
            //     writeln!(
            //         ctx.io.out,
            //         "Open this URL in your browser:\n{}\n\
            //          And enter the code: {}\n",
            //         **details.verification_uri(),
            //         details.user_code().secret()
            //     )?;
            // }

            auth_client
                .exchange_device_access_token(&details)
                .request_async(async_http_client, tokio::time::sleep, None)
                .await?
                .access_token()
                .secret()
                .to_string()
        };

        // if let Err(err) = ctx.config.check_writable(host, "token") {
        //     if let Some(crate::config_from_env::ReadOnlyEnvVarError::Variable(var)) =
        //         err.downcast_ref()
        //     {
        //         writeln!(
        //             ctx.io.err_out,
        //             "The value of the {} environment variable is being used for authentication.",
        //             var
        //         )?;
        //         writeln!(
        //             ctx.io.err_out,
        //             "To have Oxide CLI store credentials instead, first clear the value from the environment."
        //         )?;
        //         return Err(anyhow!(""));
        //     }

        //     return Err(err);
        // }

        // Set the token in the config file.
        // ctx.config.set(host, "token", &token)?;

        // let client = ctx.api_client(host)?;

        // Get the session for the token.
        // let session = client.hidden().session_me().await?;

        // Construct a one-off API client, authenticated with the token
        // returned in the previous step, so that we can extract and save the
        // identity of the authenticated user.
        let client = custom_authentication_client(self.host.as_ref(), token.clone());

        let user = client.current_user_view().send().await?;

        println!("{:#?}", user);

        // Set the user.
        // TODO: This should instead store the email, or some username or something
        // that is human knowable.
        // TODO ahl: not sure what even we're shooting for here. Maybe just a
        // way to understand the config files?
        let uid = user.id;
        // ctx.config.set(host, "user", &email)?;

        // Save the config.
        // ctx.config.write()?;

        println!("Logged in as {}", uid);

        let host_entry = Host {
            token,
            user: uid.to_string(),
            default: false,
        };

        ctx.config.update_host(self.host.to_string(), host_entry)?;

        Ok(())
    }
}

/// Log out of an Oxide host.
///
/// This command removes the authentication configuration for a host either specified
/// interactively or via `--host`.
///
///     $ oxide auth logout
///     # => select what host to log out of via a prompt
///
///     $ oxide auth logout --host oxide.internal
///     # => log out of specified host
#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
pub struct CmdAuthLogout {
    /// The hostname of the Oxide instance to log out of.
    #[clap(short = 'H', long, env = "OXIDE_HOST", value_parser = parse_host)]
    pub host: Option<url::Url>,
}

// #[async_trait::async_trait]
// impl crate::cmd::Command for CmdAuthLogout {
impl CmdAuthLogout {
    // async fn run(&self, ctx: &mut crate::context::Context) -> Result<()> {
    pub async fn run(&self) -> Result<()> {
        // if self.host.is_none() && !ctx.io.can_prompt() {
        //     return Err(anyhow!("--host required when not running interactively"));
        // }

        // let candidates = ctx.config.hosts()?;
        // if candidates.is_empty() {
        //     return Err(anyhow!("not logged in to any hosts"));
        // }

        let hostname = if self.host.is_none() {
            // if candidates.len() == 1 {
            //     candidates[0].to_string()
            // } else {
            //     let index =
            //         dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            //             .with_prompt("What account do you want to log out of?")
            //             .default(0)
            //             .items(&candidates[..])
            //             .interact();

            //     match index {
            //         Ok(i) => candidates[i].to_string(),
            //         Err(err) => {
            //             return Err(anyhow!("prompt failed: {}", err));
            //         }
            //     }
            // }
            todo!()
        } else {
            let hostname = self.host.as_ref().unwrap().to_string();
            let mut found = false;
            // for c in candidates {
            //     if c == hostname {
            //         found = true;
            //         break;
            //     }
            // }

            if !found {
                return Err(anyhow!("not logged into {}", hostname));
            }

            hostname
        };

        // if let Err(err) = ctx.config.check_writable(&hostname, "token") {
        //     if let Some(crate::config_from_env::ReadOnlyEnvVarError::Variable(var)) =
        //         err.downcast_ref()
        //     {
        //         writeln!(
        //             ctx.io.err_out,
        //             "The value of the {} environment variable is being used for authentication.",
        //             var
        //         )?;
        //         writeln!(
        //             ctx.io.err_out,
        //             "To erase credentials stored in Oxide CLI, first clear the value from the environment."
        //         )?;
        //         return Err(anyhow!(""));
        //     }

        //     return Err(err);
        // }

        // let client = ctx.api_client(&hostname)?;

        // Get the current user.
        // let session = client.hidden().session_me().await?;

        // TODO: this should be the users email or something better.
        // make it consistent with login.
        // let email = session.id;

        // if ctx.io.can_prompt() {
        //     match dialoguer::Confirm::new()
        //         .with_prompt(format!(
        //             "Are you sure you want to log out of {}{}?",
        //             hostname, email
        //         ))
        //         .interact()
        //     {
        //         Ok(true) => {}
        //         Ok(false) => {
        //             return Ok(());
        //         }
        //         Err(err) => {
        //             return Err(anyhow!("prompt failed: {}", err));
        //         }
        //     }
        // }

        // // Unset the host.
        // ctx.config.unset_host(&hostname)?;

        // // Write the changes to the config.
        // ctx.config.write()?;

        // let cs = ctx.io.color_scheme();
        // writeln!(
        //     ctx.io.out,
        //     "{} Logged out of {} {}",
        //     cs.success_icon(),
        //     cs.bold(&hostname),
        //     email
        // )?;

        Ok(())
    }
}

/// Verifies and displays information about your authentication state.
/// This command validates the authentication state for each Oxide environment in the current configuration. These hosts may be from your hosts.toml file and/or
/// $OXIDE_HOST environment variable.
#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
pub struct CmdAuthStatus {
    /// Display the auth token.
    #[clap(short = 't', long)]
    pub show_token: bool,

    /// Specific hostname to validate.
    #[clap(short = 'H', long, value_parser = parse_host)]
    pub host: Option<url::Url>,
}

// #[async_trait::async_trait]
// impl crate::cmd::Command for CmdAuthStatus {
impl CmdAuthStatus {
    pub async fn run(&self) -> Result<()> {
        let mut status_info: HashMap<String, Vec<String>> = HashMap::new();

        // Initialising a new Config here instead of taking ctx (&Context) because
        // ctx already has an initialised oxide_api::Client. This would give the CLI
        // a chance to return an error before the status checks have even started.
        //
        // For example: the user has the OXIDE_HOST env var set but hasn't set OXIDE_TOKEN
        // nor do they have a corresponding token for that host on the hosts.toml file,
        // the CLI would return an error even if other host/token combinations on the hosts.toml file are valid.
        let config = Config::default();
        let mut host_list = config.hosts.hosts;

        // Include login information from environment variables if set
        if let Some(h) = std::env::var_os("OXIDE_HOST") {
            let env_token;
            match std::env::var_os("OXIDE_TOKEN") {
                Some(t) => env_token = t.into_string().unwrap(),
                None => env_token = String::from(""),
            };
            let info = Host {
                token: env_token,
                user: String::from(""),
                default: false,
            };
            host_list.insert(h.into_string().unwrap(), info);
        }

        if host_list.is_empty() {
            return Err(anyhow!("You are not logged into any Oxide hosts."));
        }

        // Return a single result if the --host flag has been set
        if let Some(url) = &self.host {
            if host_list.contains_key(url.as_ref()) {
                host_list.retain(|k, _| k == url.as_str())
            } else {
                return Err(anyhow!(
                    "Host {} Not found in hosts.toml file or environment variables.",
                    url.as_str()
                ));
            }
        }

        for (host, info) in host_list.iter() {
            // Construct a client with each host/token combination
            let client = custom_authentication_client(host, info.token.clone());

            let mut host_status: Vec<String> = vec![];

            let result = client.current_user_view().send().await;
            let user = match result {
                Ok(usr) => usr,
                Err(_) => {
                    host_status.push(String::from(
                        "Not authenticated. Host/token combination invalid",
                    ));
                    status_info.insert(host.to_string(), host_status);
                    continue;
                }
            };

            // TODO: this should be the users email or something consistent with login
            // and logout.
            let email = user.id.to_string();

            // TODO: Once tokens have expiry dates, report expired tokens.

            host_status.push(format!("Logged in to {} as {}", host, &email,));
            let token_display = if self.show_token {
                info.token.to_string()
            } else {
                "*******************".to_string()
            };
            host_status.push(format!("Token: {}", token_display));

            status_info.insert(host.to_string(), host_status);
        }

        for (key, value) in &status_info {
            println!("{}: {:?}", key, value);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    // use crate::cmd::Command;

    pub struct TestItem {
        name: String,
        cmd: crate::cmd_auth::SubCommand,
        stdin: String,
        want_out: String,
        want_err: String,
    }

    // TODO: Auth is shaky with current docker container CI implementation.
    // remove ignore tag once tests run against mock API server
    // #[ignore]
    // #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    // #[serial_test::serial]
    // async fn test_cmd_auth() {
    //     let test_host = std::env::var("OXIDE_TEST_HOST")
    //         .expect("you need to set OXIDE_TEST_HOST to where the api is running");
    //     let test_host = crate::cmd_auth::parse_host(&test_host).expect("invalid OXIDE_TEST_HOST");

    //     let test_token = std::env::var("OXIDE_TEST_TOKEN").expect("OXIDE_TEST_TOKEN is required");

    //     let tests: Vec<TestItem> = vec![
    //         TestItem {
    //             name: "status".to_string(),
    //             cmd: crate::cmd_auth::SubCommand::Status(crate::cmd_auth::CmdAuthStatus {
    //                 show_token: false,
    //                 host: None,
    //             }),
    //             stdin: "".to_string(),
    //             want_out: "You are not logged into any Oxide hosts. Run oxide auth login to authenticate.\n"
    //                 .to_string(),
    //             want_err: "".to_string(),
    //         },
    //         TestItem {
    //             name: "login --with-token=false".to_string(),
    //             cmd: crate::cmd_auth::SubCommand::Login(crate::cmd_auth::CmdAuthLogin {
    //                 host: Some(test_host.clone()),
    //                 with_token: false,
    //             }),
    //             stdin: test_token.to_string(),
    //             want_out: "".to_string(),
    //             want_err: "--with-token required when not running interactively".to_string(),
    //         },
    //         TestItem {
    //             name: "login --with-token=true".to_string(),
    //             cmd: crate::cmd_auth::SubCommand::Login(crate::cmd_auth::CmdAuthLogin {
    //                 host: Some(test_host.clone()),
    //                 with_token: true,
    //             }),
    //             stdin: test_token.to_string(),
    //             want_out: "✔ Logged in as ".to_string(),
    //             want_err: "".to_string(),
    //         },
    //         TestItem {
    //             name: "status".to_string(),
    //             cmd: crate::cmd_auth::SubCommand::Status(crate::cmd_auth::CmdAuthStatus {
    //                 show_token: false,
    //                 host: Some(test_host.clone()),
    //             }),
    //             stdin: "".to_string(),
    //             want_out: format!("{}\n✔ Logged in to {} as", test_host, test_host),
    //             want_err: "".to_string(),
    //         },
    //         TestItem {
    //             name: "logout no prompt no host".to_string(),
    //             cmd: crate::cmd_auth::SubCommand::Logout(crate::cmd_auth::CmdAuthLogout { host: None }),
    //             stdin: "".to_string(),
    //             want_out: "".to_string(),
    //             want_err: "--host required when not running interactively".to_string(),
    //         },
    //         TestItem {
    //             name: "logout no prompt with host".to_string(),
    //             cmd: crate::cmd_auth::SubCommand::Logout(crate::cmd_auth::CmdAuthLogout {
    //                 host: Some(test_host.clone()),
    //             }),
    //             stdin: "".to_string(),
    //             want_out: format!("✔ Logged out of {}", test_host),
    //             want_err: "".to_string(),
    //         },
    //     ];

    //     let mut config = crate::config::new_blank_config().unwrap();
    //     let mut c = crate::config_from_env::EnvConfig::inherit_env(&mut config);

    //     for t in tests {
    //         let (mut io, stdout_path, stderr_path) = crate::iostreams::IoStreams::test();
    //         if !t.stdin.is_empty() {
    //             io.stdin = Box::new(std::io::Cursor::new(t.stdin));
    //         }
    //         // We need to also turn off the fancy terminal colors.
    //         // This ensures it also works in GitHub actions/any CI.
    //         io.set_color_enabled(false);
    //         // TODO: we should figure out how to test the prompts.
    //         io.set_never_prompt(true);
    //         let mut ctx = crate::context::Context {
    //             config: &mut c,
    //             io,
    //             debug: false,
    //         };

    //         let cmd_auth = crate::cmd_auth::CmdAuth { subcmd: t.cmd };
    //         match cmd_auth.run(&mut ctx).await {
    //             Ok(()) => {
    //                 let stdout = std::fs::read_to_string(stdout_path).unwrap();
    //                 let stderr = std::fs::read_to_string(stderr_path).unwrap();
    //                 assert!(stderr.is_empty(), "test {}: {}", t.name, stderr);
    //                 if !stdout.contains(&t.want_out) {
    //                     assert_eq!(stdout, t.want_out, "test {}: stdout mismatch", t.name);
    //                 }
    //             }
    //             Err(err) => {
    //                 let stdout = std::fs::read_to_string(stdout_path).unwrap();
    //                 let stderr = std::fs::read_to_string(stderr_path).unwrap();
    //                 assert_eq!(stdout, t.want_out, "test {}", t.name);
    //                 if !err.to_string().contains(&t.want_err) {
    //                     assert_eq!(err.to_string(), t.want_err, "test {}: err mismatch", t.name);
    //                 }
    //                 assert!(stderr.is_empty(), "test {}: {}", t.name, stderr);
    //             }
    //         }
    //     }
    // }

    #[test]
    fn test_parse_host() {
        use super::parse_host;

        // TODO: Replace with assert_matches when stable

        // The simple cases where only the host name or IP is passed
        assert!(matches!(
            parse_host("example.com").map(|host| host.to_string()),
            Ok(host) if host == "https://example.com/"
        ));
        assert!(matches!(
            parse_host("localhost").map(|host| host.to_string()),
            Ok(host) if host == "https://localhost/"
        ));
        assert!(matches!(
            parse_host("127.0.0.1").map(|host| host.to_string()),
            Ok(host) if host == "https://127.0.0.1/"
        ));
        assert!(matches!(
            parse_host("[::1]").map(|host| host.to_string()),
            Ok(host) if host == "https://[::1]/"
        ));

        // Explicit port
        assert!(matches!(
            parse_host("example.com:8888").map(|host| host.to_string()),
            Ok(host) if host == "https://example.com:8888/"
        ));
        assert!(matches!(
            parse_host("localhost:8888").map(|host| host.to_string()),
            Ok(host) if host == "https://localhost:8888/"
        ));
        assert!(matches!(
            parse_host("127.0.0.1:8888").map(|host| host.to_string()),
            Ok(host) if host == "https://127.0.0.1:8888/"
        ));
        assert!(matches!(
            parse_host("[::1]:8888").map(|host| host.to_string()),
            Ok(host) if host == "https://[::1]:8888/"
        ));

        // Explicit scheme
        assert!(matches!(
            parse_host("http://example.com:8888").map(|host| host.to_string()),
            Ok(host) if host == "http://example.com:8888/"
        ));
        assert!(matches!(
            parse_host("http://localhost").map(|host| host.to_string()),
            Ok(host) if host == "http://localhost/"
        ));

        // Nonsense scheme
        assert!(matches!(
            parse_host("ftp://localhost").map(|host| host.to_string()),
            Err(_)
        ));

        // Strip out any extraneous pieces we don't need
        assert!(matches!(
            parse_host("http://user:pass@example.com:8888/random/path/?k=v&t=s#fragment=33").map(|host| host.to_string()),
            Ok(host) if host == "http://example.com:8888/"
        ));
    }
}

#[test]
fn test_cmd_auth_status() {
    use assert_cmd::Command;
    use httpmock::MockServer;
    use predicates::str;
    use serde_json::json;

    let server = MockServer::start();
    let oxide_mock = server.mock(|when, then| {
        when.method(GET).path("/v1/me");
        then.status(200).json_body(json!({"id":"001de000-05e4-4000-8000-000000004007","display_name":"privileged","silo_id":"001de000-5110-4000-8000-000000000000"}));
    });

    // Validate a successful login
    let mut cmd = Command::cargo_bin("oxide").unwrap();
    cmd.arg("auth")
        .arg("status")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "oxide-token-1111");
    cmd.assert().success().stdout(str::contains(format!(
        "{}: [\"Logged in to {} as 001de000-05e4-4000-8000-000000004007\", \"Token: *******************\"]",
        server.url(""), server.url("")
    )));

    // Validate a successful login with token displayed in plain text
    cmd.arg("-t");
    cmd.assert().success().stdout(str::contains(format!(
    "{}: [\"Logged in to {} as 001de000-05e4-4000-8000-000000004007\", \"Token: oxide-token-1111\"]",
    server.url(""), server.url("")
    )));

    // Assert the mock received the provided number of requests which matched all the request requirements
    oxide_mock.assert_hits(2);

    // Set auth information through environment variables and check they are included in the results
    let mut cmd = Command::cargo_bin("oxide").unwrap();
    cmd.arg("auth")
        .arg("status")
        .env("OXIDE_HOST", "http://111.1.1.1/")
        .env("OXIDE_TOKEN", "oxide-token-1111");
    cmd.assert().success().stdout(str::contains(
        "http://111.1.1.1/: [\"Not authenticated. Host/token combination invalid\"]",
    ));

    // Make sure the command only returns information about the specified host
    cmd.arg("-H").arg("http://111.1.1.1/");
    cmd.assert()
        .success()
        .stdout("http://111.1.1.1/: [\"Not authenticated. Host/token combination invalid\"]\n");
}
