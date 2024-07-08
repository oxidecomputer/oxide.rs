// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::{fs::File, io::Read};

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use clap::Parser;
use oauth2::{
    basic::BasicClient, devicecode::StandardDeviceAuthorizationResponse, AuthType, AuthUrl,
    ClientId, DeviceAuthorizationUrl, TokenResponse, TokenUrl,
};
use oxide::{Client, ClientConfig, ClientSessionExt};
use toml_edit::{Item, Table};
use uuid::Uuid;

use crate::context::Context;
use crate::{AsHost, RunnableCmd};

/// Login, logout, and get the status of your authentication.
///
/// Manage `oxide`'s authentication state.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "auth")]
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

#[async_trait]
impl RunnableCmd for CmdAuth {
    async fn run(&self, ctx: &Context) -> Result<()> {
        match &self.subcmd {
            SubCommand::Login(cmd) => cmd.run(ctx).await,
            SubCommand::Logout(cmd) => cmd.run(ctx).await,
            SubCommand::Status(cmd) => cmd.run(ctx).await,
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

fn yes(prompt: impl Into<String>) -> Result<bool> {
    match dialoguer::Confirm::new().with_prompt(prompt).interact() {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        Err(err) => Err(anyhow!("prompt failed: {}", err)),
    }
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

/// Authenticate with an Oxide Silo
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
#[command(verbatim_doc_comment)]
pub struct CmdAuthLogin {
    // TODO seems pointless
    /// Read token from standard input.
    #[clap(long)]
    with_token: bool,

    /// The host of the Oxide instance to authenticate with.
    /// This assumes http; specify an `http://` prefix if needed.
    #[clap(short = 'H', long, value_parser = parse_host)]
    host: url::Url,

    /// Override the default browser when opening the authentication URL.
    #[clap(long)]
    browser: Option<String>,

    /// Print the authentication URL rather than opening a browser window.
    #[clap(long)]
    no_browser: bool,
}

impl CmdAuthLogin {
    // TODO There are a few cases to consider
    // - If the user specifies a profile, we use that profile name and prompt
    //   if there's already a token/user
    // - If there is no profile, we use the silo name to make a new profile
    //   (if there's a conflict I guess we prompt: overwrite / new name)
    // - If there's no default profile, we make the newly authenticated profile
    //   the new default
    pub async fn run(&self, ctx: &Context) -> Result<()> {
        // if !ctx.io.can_prompt() && !self.with_token {
        //     return Err(anyhow!(
        //         "--with-token required when not running interactively"
        //     ));
        // }

        let profile = ctx
            .client_config()
            .profile()
            .or_else(|| ctx.config_file().basics.default_profile.as_deref());

        if let Some(profile_name) = profile {
            // If the profile already has a token, alert the user.
            if ctx.cred_file().profile.contains_key(profile_name)
                && !yes(format!(
                    "The profile \"{}\" is already authenticated. {}",
                    profile_name, "Do you want to re-authenticate?",
                ))?
            {
                return Ok(());
            }
        } else if let Some(existing_profile) = ctx
            .cred_file()
            .profile
            .iter()
            .filter_map(|(name, info)| (info.host == self.host.as_str()).then_some(name))
            .next()
        {
            // If the host is already present in a profile, alert the user.
            if !yes(format!(
                "The profile \"{}\" {} {}. {}",
                existing_profile,
                "is already authenticated with host",
                &self.host,
                "Do you want to proceed?"
            ))? {
                return Ok(());
            }
        }

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
            // Make the client for use by oauth2
            let client = &ctx
                .client_config()
                .make_unauthenticated_client_builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()?;
            // Copied from oauth2::async_http_client to customize the
            // reqwest::Client with the top-level command-line options.
            let async_http_client_custom = |request: oauth2::HttpRequest| async move {
                let mut request_builder = client
                    .request(request.method, request.url.as_str())
                    .body(request.body);
                for (name, value) in &request.headers {
                    request_builder = request_builder.header(name.as_str(), value.as_bytes());
                }
                let request = request_builder.build()?;

                let response = client.execute(request).await?;

                let status_code = response.status();
                let headers = response.headers().to_owned();
                let chunks = response.bytes().await?;
                std::result::Result::<_, reqwest::Error>::Ok(oauth2::HttpResponse {
                    status_code,
                    headers,
                    body: chunks.to_vec(),
                })
            };

            // Do an OAuth 2.0 Device Authorization Grant dance to get a token.
            let device_auth_url =
                DeviceAuthorizationUrl::new(format!("{}device/auth", &self.host))?;
            // TODO what's the point of this?
            let client_id = Uuid::new_v4();
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
                .request_async(async_http_client_custom)
                .await?;

            let uri = details.verification_uri().to_string();

            let opened = match (&self.browser, self.no_browser) {
                (None, false) => open::that(&uri).is_ok(),
                (Some(app), false) => open::with(&uri, app).is_ok(),
                (None, true) => false,
                (Some(_), true) => unreachable!(),
            };

            if opened {
                println!("Opened this URL in your browser:\n  {}", uri);
            } else {
                println!("Open this URL in your browser:\n  {}", uri);
            }

            println!("\nEnter the code: {}\n", details.user_code().secret());

            auth_client
                .exchange_device_access_token(&details)
                .request_async(async_http_client_custom, tokio::time::sleep, None)
                .await?
                .access_token()
                .secret()
                .to_string()
        };

        let host = self.host.as_host();

        // Construct a one-off API client, authenticated with the token
        // returned in the previous step, so that we can extract and save the
        // identity of the authenticated user.
        let client = Client::new_authenticated_config(
            &ClientConfig::default().with_host_and_token(host, &token),
        )
        .unwrap();

        let user = client.current_user_view().send().await?;

        // TODO format this more reasonably
        println!("{:#?}", user);

        let profile_name = profile.unwrap_or_else(|| &user.silo_name);

        // Set the user.
        // TODO: This should instead store the email, or some username or something
        // that is human knowable.
        // TODO ahl: not sure what even we're shooting for here. Maybe just a
        // way to understand the config files?
        let uid = user.id;

        println!("Logged in as {} under profile {}", uid, profile_name);

        // Read / modify / write the credentials file.
        let credentials_path = ctx.client_config().config_dir().join("credentials.toml");
        let mut credentials =
            if let Ok(contents) = std::fs::read_to_string(credentials_path.clone()) {
                contents.parse::<toml_edit::DocumentMut>().unwrap()
            } else {
                Default::default()
            };

        let profile_table = credentials
            .entry("profile")
            .or_insert_with(|| {
                let mut table = Table::default();
                // Avoid a bare [profile] table.
                table.set_implicit(true);
                Item::Table(table)
            })
            .as_table_mut()
            .unwrap_or_else(|| {
                panic!(
                    "\"profile\" in {} is not a table",
                    credentials_path.to_string_lossy()
                )
            });

        let profile = profile_table
            .entry(profile_name)
            .or_insert_with(|| Item::Table(Table::default()))
            .as_table_mut()
            .unwrap_or_else(|| {
                panic!(
                    "\"profile.{}\" in {} is not a table",
                    profile_name,
                    credentials_path.to_string_lossy()
                )
            });

        profile.insert("host", toml_edit::value(self.host.as_host().to_string()));
        profile.insert("token", toml_edit::value(token));
        profile.insert("user", toml_edit::value(uid.to_string()));

        std::fs::write(credentials_path, credentials.to_string())
            .expect("unable to write credentials.toml");

        println!("{}", credentials);

        // If there is no default profile then we'll set this new profile to be
        // the default.
        if ctx.config_file().basics.default_profile.is_none() {
            let config_path = ctx.client_config().config_dir().join("config.toml");
            let mut config = if let Ok(contents) = std::fs::read_to_string(config_path.clone()) {
                contents.parse::<toml_edit::DocumentMut>().unwrap()
            } else {
                Default::default()
            };

            config.insert("default-profile", Item::Value(profile_name.into()));

            std::fs::write(config_path, config.to_string()).expect("unable to write config.toml");
        }

        Ok(())
    }
}

/// Removes saved authentication information from profiles.
///
/// This command does not invalidate any tokens.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
pub struct CmdAuthLogout {
    /// If set, authentication information from all profiles will be deleted.
    #[clap(short = 'a', long)]
    pub all: bool,
    /// Skip confirmation prompt.
    #[clap(short = 'f', long)]
    pub force: bool,
}

impl CmdAuthLogout {
    pub async fn run(&self, ctx: &Context) -> Result<()> {
        if !self.force && !yes("Confirm authentication information deletion:")? {
            return Ok(());
        }

        let credentials_path = ctx.client_config().config_dir().join("credentials.toml");

        if self.all {
            // Clear the entire file for users who want to reset their known hosts.
            let _ = File::create(credentials_path)?;
            println!("Removed all authentication information");
        } else {
            let profile = ctx
                .client_config()
                .profile()
                .or_else(|| ctx.config_file().basics.default_profile.as_deref());
            let Some(profile_name) = profile else {
                bail!("No profile specified and no default profile");
            };
            let Ok(credentials_contents) = std::fs::read_to_string(credentials_path.clone()) else {
                // No file; no credentials.
                return Ok(());
            };
            let mut credentials = credentials_contents
                .parse::<toml_edit::DocumentMut>()
                .unwrap();
            if let Some(profiles) = credentials.get_mut("profile") {
                let profiles = profiles.as_table_mut().unwrap();
                profiles.remove(profile_name);
            }
            std::fs::write(credentials_path, credentials.to_string())
                .expect("unable to write credentials.toml");
            println!(
                "Removed authentication information for profile \"{}\"",
                profile_name,
            );
        }

        Ok(())
    }
}

/// Verifies and displays information about your authentication state.
///
/// This command validates the authentication state for each profile in the
/// current configuration.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
pub struct CmdAuthStatus {}

impl CmdAuthStatus {
    pub async fn run(&self, ctx: &Context) -> Result<()> {
        for (profile_name, profile_info) in &ctx.cred_file().profile {
            let client = Client::new_authenticated_config(
                &ClientConfig::default()
                    .with_host_and_token(&profile_info.host, &profile_info.token),
            )
            .unwrap();

            let result = client.current_user_view().send().await;
            let status = match result {
                Ok(_) => "Authenticated".to_string(),
                Err(e) => e.to_string(),
            };

            println!(
                "Profile \"{}\" ({}) status: {}",
                profile_name, profile_info.host, status
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use assert_cmd::Command;
    use expectorate::assert_contents;
    use httpmock::{
        Method::{GET, POST},
        MockServer,
    };
    use oxide::types::CurrentUser;
    use oxide_httpmock::MockServerExt;
    use predicates::str;
    use serde_json::json;

    #[test]
    fn test_parse_host() {
        use super::parse_host;

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
        assert!(parse_host("ftp://localhost")
            .map(|host| host.to_string())
            .is_err());

        // Strip out any extraneous pieces we don't need
        assert!(matches!(
            parse_host("http://user:pass@example.com:8888/random/path/?k=v&t=s#fragment=33").map(|host| host.to_string()),
            Ok(host) if host == "http://example.com:8888/"
        ));
    }

    // TODO redo this test to be less shitty
    #[ignore]
    #[test]
    fn test_cmd_auth_status() {
        let server = MockServer::start();
        // TODO redo this to use the sdk-mock
        let oxide_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v1/me")
                .header("authorization", "Bearer oxide-token-1111");
            then.status(200).json_body_obj(&oxide::types::CurrentUser {
                display_name: "privileged".to_string(),
                id: "001de000-05e4-4000-8000-000000004007".parse().unwrap(),
                silo_id: "d1bb398f-872c-438c-a4c6-2211e2042526".parse().unwrap(),
                silo_name: "funky-town".parse().unwrap(),
            });
        });

        // Validate authenticated credentials.
        Command::cargo_bin("oxide")
            .unwrap()
            .arg("auth")
            .arg("status")
            .env("OXIDE_HOST", server.url(""))
            .env("OXIDE_TOKEN", "oxide-token-1111")
            .assert()
            .success()
            .stdout(str::contains(format!(
                "{}: [{}, {}]",
                server.url(""),
                format_args!(
                    "\"Logged in to {} as 001de000-05e4-4000-8000-000000004007\"",
                    server.url("")
                ),
                "\"Token: *******************\"",
            )));

        // Validate authenticated credentials with the token displayed in plain text.
        Command::cargo_bin("oxide")
            .unwrap()
            .arg("auth")
            .arg("status")
            .env("OXIDE_HOST", server.url(""))
            .env("OXIDE_TOKEN", "oxide-token-1111")
            .arg("-t")
            .assert()
            .success()
            .stdout(str::contains(format!(
                "{}: [{}, {}]",
                server.url(""),
                format_args!(
                    "\"Logged in to {} as 001de000-05e4-4000-8000-000000004007\"",
                    server.url("")
                ),
                "\"Token: oxide-token-1111\"",
            )));

        // Assert that both commands hit the mock.
        oxide_mock.assert_hits(2);

        let oxide_mock = server.mock(|when, then| {
            when.header("authorization", "Bearer oxide-token-1112");
            then.status(401).json_body_obj(&oxide::types::Error {
                error_code: None,
                message: "oops".to_string(),
                request_id: "42".to_string(),
            });
        });

        // Try invalid credentials.
        Command::cargo_bin("oxide")
            .unwrap()
            .arg("auth")
            .arg("status")
            .env("OXIDE_HOST", server.url(""))
            .env("OXIDE_TOKEN", "oxide-token-1112")
            .assert()
            .success()
            .stdout(str::contains(format!(
                "{}: [\"Not authenticated. Host/token combination invalid\"]",
                server.url("")
            )));

        // Make sure the command only returns information about the specified host.
        Command::cargo_bin("oxide")
            .unwrap()
            .arg("auth")
            .arg("status")
            .env("OXIDE_HOST", server.url("/"))
            .env("OXIDE_TOKEN", "oxide-token-1112")
            .arg("-H")
            .arg(server.url(""))
            .assert()
            .success()
            .stdout(format!(
                "{}: [\"Not authenticated. Host/token combination invalid\"]\n",
                server.url("/")
            ));
        // Assert that both commands hit the mock.
        oxide_mock.assert_hits(2);
    }

    #[test]
    fn test_auth_login_first() {
        env_logger::init();

        let server = MockServer::start();

        // Respond to the initial OAuth2 request.
        let device_auth = server.mock(|when, then| {
            let body = json!({
                "device_code": "DEV-CODE",
                "user_code": "0X1-D3C",
                "verification_uri": "http://go.here.to/verify",
                "expires_in": 10,
            });
            when.method(POST).path("/device/auth");
            then.status(200)
                .json_body(body)
                .header("content-type", "application/json");
        });

        // This is where we'd poll, but let's just wave them through.
        let device_token = server.mock(|when, then| {
            let body = json!({
                "access_token": "123-456-789",
                "token_type": "Bearer",
            });
            when.method(POST).path("/device/token");
            then.delay(std::time::Duration::from_secs(1))
                .status(200)
                .json_body(body)
                .header("content-type", "application/json");
        });

        // User and silo identity now that we're "authenticated".
        let me = server.current_user_view(|when, then| {
            when.into_inner().any_request();
            then.ok(&CurrentUser {
                display_name: "falken".to_string(),
                id: "831dedf4-0a66-4b04-a232-b610f9f8924c".parse().unwrap(),
                silo_id: "12e8c7a4-399f-41e2-985e-7b120ecbcc1a".parse().unwrap(),
                silo_name: "crystal-palace".try_into().unwrap(),
            });
        });

        let temp_dir = tempfile::tempdir().unwrap().into_path();

        println!("SERVER {}", server.url(""));

        let x = Command::cargo_bin("oxide")
            .unwrap()
            .env("RUST_BACKTRACE", "1")
            .arg("--config-dir")
            .arg(temp_dir.as_os_str())
            .arg("auth")
            .arg("login")
            .arg("--no-browser")
            .arg("--host")
            .arg(server.url(""))
            .assert()
            .success();
        let x = x.get_output();

        println!("{}", String::from_utf8_lossy(&x.stdout));

        device_auth.assert();
        device_token.assert();
        me.assert();

        assert_contents(
            "tests/data/test_auth_login_first_credentials.toml",
            &scrub_server(
                read_to_string(temp_dir.join("credentials.toml")).unwrap(),
                server.url(""),
            ),
        );

        assert_contents(
            "tests/data/test_auth_login_first_config.toml",
            &read_to_string(temp_dir.join("config.toml")).unwrap(),
        );
    }

    fn scrub_server(raw: String, server: String) -> String {
        raw.replace(&server, "<TEST-SERVER>")
    }
}
