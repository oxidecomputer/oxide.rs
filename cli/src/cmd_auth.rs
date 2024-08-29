// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::fs::File;
use std::io::{self, Write};

use anyhow::{anyhow, bail, Context as _, Result};
use async_trait::async_trait;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use oauth2::{
    basic::BasicClient, devicecode::StandardDeviceAuthorizationResponse, AuthType, AuthUrl,
    ClientId, DeviceAuthorizationUrl, TokenResponse, TokenUrl,
};
use oxide::types::CurrentUser;
use oxide::{Client, ClientConfig, ClientSessionExt};
use std::path::Path;
use std::time::Duration;
use toml_edit::{Item, Table};
use uuid::Uuid;

use crate::context::Context;
use crate::{println_nopipe, AsHost, RunnableCmd};

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
    RenameProfile(CmdAuthRenameProfile),
    Status(CmdAuthStatus),
}

#[async_trait]
impl RunnableCmd for CmdAuth {
    async fn run(&self, ctx: &Context) -> Result<()> {
        match &self.subcmd {
            SubCommand::Login(cmd) => cmd.login(ctx).await,
            SubCommand::Logout(cmd) => cmd.logout(ctx).await,
            SubCommand::RenameProfile(cmd) => cmd.rename_profile(ctx).await,
            SubCommand::Status(cmd) => cmd.status(ctx).await,
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
///     # authenticate with a specific Oxide silo
///     $ oxide auth login --host oxide.internal
///
///     # authenticate with an insecure Oxide silo (not recommended)
///     $ oxide auth login --host http://oxide.internal
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
pub struct CmdAuthLogin {
    /// The host of the Oxide instance to authenticate with.
    /// This assumes https; specify an `http://` prefix if needed.
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
    pub async fn login(&self, ctx: &Context) -> Result<()> {
        let profile = ctx.client_config().profile();

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
        let device_auth_url = DeviceAuthorizationUrl::new(format!("{}device/auth", &self.host))?;
        // The client ID is intended to be an identifier issued to clients;
        // since we're not doing that and this ID would be public if it were
        // static, we just generate a random one each time we authenticate.
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
            writeln!(io::stdout(), "Opened this URL in your browser:\n  {}", uri)?;
        } else {
            writeln!(io::stdout(), "Open this URL in your browser:\n  {}", uri)?;
        }

        writeln!(
            io::stdout(),
            "\nEnter the code: {}\n",
            details.user_code().secret()
        )?;

        let token = auth_client
            .exchange_device_access_token(&details)
            .request_async(async_http_client_custom, tokio::time::sleep, None)
            .await?
            .access_token()
            .secret()
            .to_string();

        let host = self.host.as_host();

        // Construct a one-off API client, authenticated with the token
        // returned in the previous step, so that we can extract and save the
        // identity of the authenticated user. We clone the existing configuration
        // here to ensure that any command level configuration provided by the
        // user is maintained
        let client = Client::new_authenticated_config(
            &ctx.client_config()
                .clone()
                .with_host_and_token(host, &token),
        )?;

        let user = client.current_user_view().send().await?.into_inner();

        // If there's no specified profile, we'll use the name of the silo as
        // the profile name... and deal with conflicting names.
        let profile_name = profile.map(String::from).unwrap_or_else(|| {
            let silo_name = user.silo_name.to_string();
            let mut name = silo_name.clone();
            let mut ii = 2;
            while ctx.cred_file().profile.contains_key(&name) {
                name = format!("{}{}", silo_name, ii);
                ii += 1;
            }
            name
        });

        // Set the user.
        // TODO: This should instead store the email, or some username or something
        // that is human knowable.
        // TODO ahl: not sure what even we're shooting for here. Maybe just a
        // way to understand the config files?
        let uid = user.id;

        // Read / modify / write the credentials file.
        let config_dir = ctx.client_config().config_dir();
        let credentials_path = config_dir.join("credentials.toml");
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
            .entry(&profile_name)
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

        write_configuration_file(&credentials_path, &credentials.to_string())?;

        // If there is no default profile then we'll set this new profile to be
        // the default.
        if ctx.config_file().basics.default_profile.is_none() {
            let config_path = ctx.client_config().config_dir().join("config.toml");
            let mut config = if let Ok(contents) = std::fs::read_to_string(config_path.clone()) {
                contents.parse::<toml_edit::DocumentMut>().unwrap()
            } else {
                Default::default()
            };

            config.insert("default-profile", Item::Value(profile_name.clone().into()));

            std::fs::write(config_path, config.to_string()).expect("unable to write config.toml");
        }

        let CurrentUser {
            display_name,
            id,
            silo_id,
            silo_name,
        } = &user;

        writeln!(io::stdout(), "Login successful")?;
        writeln!(io::stdout(), "  silo: {} ({})", **silo_name, silo_id)?;
        writeln!(io::stdout(), "  user: {} ({})", display_name, id)?;
        if ctx.config_file().basics.default_profile.is_none() {
            writeln!(
                io::stdout(),
                "Profile '{}' set as the default",
                profile_name
            )?;
        } else {
            writeln!(io::stdout(), "Use --profile '{}'", profile_name)?;
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
    pub async fn logout(&self, ctx: &Context) -> Result<()> {
        if !self.force && !yes("Confirm authentication information deletion:")? {
            return Ok(());
        }

        let config_dir = ctx.client_config().config_dir();
        let credentials_path = config_dir.join("credentials.toml");

        if self.all {
            // Clear the entire file for users who want to reset their known hosts.
            let _ = File::create(credentials_path)?;
            writeln!(io::stdout(), "Removed all authentication information")?;
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
            write_configuration_file(&credentials_path, &credentials.to_string())?;
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
    pub async fn status(&self, ctx: &Context) -> Result<()> {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")
                .expect("Failed to set spinner template"),
        );

        // For backward compatibility, we'll check OXIDE_HOST and OXIDE_TOKEN
        // first.
        if let (Ok(host_env), Ok(token_env)) =
            (std::env::var("OXIDE_HOST"), std::env::var("OXIDE_TOKEN"))
        {
            let client = Client::new_authenticated_config(
                &ClientConfig::default().with_host_and_token(&host_env, &token_env),
            )
            .expect("client authentication from host/token failed");

            spinner.set_message(format!("Checking {}...", host_env));
            spinner.enable_steady_tick(Duration::from_millis(100));

            let result = client.current_user_view().send().await;
            spinner.finish();

            match result {
                Ok(user) => {
                    log::debug!("success response for {} (env): {:?}", host_env, user);
                    println_nopipe!("Logged in to {} as {}", host_env, user.id)
                }
                Err(e) => {
                    log::debug!("error response for {} (env): {}", host_env, e);
                    println_nopipe!("{}: {}", host_env, Self::error_msg(&e))
                }
            };
        } else {
            for (profile_name, profile_info) in &ctx.cred_file().profile {
                let client = Client::new_authenticated_config(
                    &ClientConfig::default()
                        .with_host_and_token(&profile_info.host, &profile_info.token),
                )
                .expect("client authentication from host/token failed");

                spinner.reset();
                spinner.set_message(format!("Checking {}...", &profile_info.host));
                spinner.enable_steady_tick(Duration::from_millis(100));

                let result = client.current_user_view().send().await;
                spinner.finish();

                let status = match result {
                    Ok(v) => {
                        log::debug!("success response for {}: {:?}", profile_info.host, v);
                        "Authenticated".to_string()
                    }
                    Err(e) => {
                        log::debug!("error response for {}: {}", profile_info.host, e);
                        Self::error_msg(&e)
                    }
                };

                println_nopipe!(
                    "Profile \"{}\" ({}) status: {}",
                    profile_name,
                    profile_info.host,
                    status
                );
            }
        }
        Ok(())
    }

    fn error_msg(e: &oxide::Error<oxide::types::Error>) -> String {
        match e {
            oxide::Error::ErrorResponse(ee) => format!("Error Response: {}", ee.message),
            ee => ee.to_string(),
        }
    }
}

/// Rename a profile.
///
/// This command does not modify the profile credentials. If
/// the profile being renamed is your `default-profile` that
/// will be updated as well.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
pub struct CmdAuthRenameProfile {
    /// The current name of the profile to be renamed.
    current_profile_name: String,

    /// The new name of the profile to be renamed.
    new_profile_name: String,
}

impl CmdAuthRenameProfile {
    pub async fn rename_profile(&self, ctx: &Context) -> Result<()> {
        let config_dir = ctx.client_config().config_dir();
        let credentials_path = config_dir.join("credentials.toml");
        if let Ok(credentials_contents) = std::fs::read_to_string(&credentials_path) {
            let mut credentials = credentials_contents
                .parse::<toml_edit::DocumentMut>()
                .unwrap();
            if let Some(profiles) = credentials.get_mut("profile") {
                let profiles = profiles.as_table_mut().unwrap();
                let Some(profile) = profiles.remove(&self.current_profile_name) else {
                    bail!(
                        "No profile named \"{}\" found in {}",
                        self.current_profile_name,
                        credentials_path.display()
                    );
                };
                profiles.insert(&self.new_profile_name, profile);
            }

            write_configuration_file(&credentials_path, &credentials.to_string())?;
        };

        let config_path = config_dir.join("config.toml");
        if let Ok(config_contents) = std::fs::read_to_string(&config_path) {
            let mut config = config_contents.parse::<toml_edit::DocumentMut>()?;

            if let Some(old_default) = config.remove("default-profile") {
                if Some(self.current_profile_name.as_str()) == old_default.as_str() {
                    config.insert(
                        "default-profile",
                        Item::Value(self.new_profile_name.clone().into()),
                    );
                    write_configuration_file(&config_path, &config.to_string())?;
                }
            }
        }

        writeln!(
            io::stdout(),
            "Renamed profile \"{}\" to \"{}\"",
            self.current_profile_name,
            self.new_profile_name,
        )?;
        Ok(())
    }
}

fn write_configuration_file(cfg_path: &Path, contents: &str) -> Result<()> {
    let Some(cfg_dir) = cfg_path.parent() else {
        bail!("no parent directory for config file {}", cfg_path.display());
    };

    std::fs::create_dir_all(cfg_dir)
        .with_context(|| format!("creating config directory \"{}\"", cfg_dir.display()))?;
    std::fs::write(cfg_path, contents)
        .with_context(|| format!("writing configuration to \"{}\"", cfg_path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cmd_auth_login() {
        use assert_cmd::Command;
        use predicates::str;

        let bad_url = "sys.oxide.invalid";

        // Validate connection error details are printed
        Command::cargo_bin("oxide")
            .unwrap()
            .arg("auth")
            .arg("login")
            .arg("--host")
            .arg(bad_url)
            .assert()
            .failure()
            .stderr(str::starts_with(format!(
                "Request failed: error sending request for url (https://{bad_url}/device/auth):"
            )));
    }

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
}
