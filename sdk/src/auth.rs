// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use std::{
    collections::BTreeMap,
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
};

use crate::{Client, OxideAuthError};
use reqwest::ClientBuilder;
use serde::Deserialize;

/// Credentials for a particular profile.
#[derive(Deserialize, Debug)]
pub struct ProfileCredentials {
    /// Secret API token (DO NOT SHARE)
    pub token: String,
    /// Token ID for accessing token properties in the API
    pub token_id: Option<String>,
    /// API host
    pub host: String,
    /// User id
    pub user: String,
    /// Expiration time of the token if any
    pub time_expires: Option<String>,
}

// TODO: do we want a way to easily change the port number? It would need to be
// shoved into the baseurl string
#[derive(Clone)]
struct ResolveValue {
    pub domain: String,
    pub addr: IpAddr,
}

/// Configuration for creating an authenticated [Client]
#[derive(Clone)]
pub struct ClientConfig {
    config_dir: PathBuf,
    auth_method: AuthMethod,
    user_agent: String,
    resolve: Option<ResolveValue>,
    cert: Option<reqwest::Certificate>,
    insecure: bool,
    timeout: Option<u64>,
    connect_timeout: Option<u64>,
    read_timeout: Option<u64>,
}

#[derive(Clone)]
enum AuthMethod {
    DefaultProfile,
    Profile(String),
    HostToken { host: String, token: String },
}

impl Default for ClientConfig {
    fn default() -> Self {
        let mut config_dir = dirs::home_dir().expect("unable to determine the home directory");
        config_dir.push(".config");
        config_dir.push("oxide");
        Self {
            config_dir,
            auth_method: AuthMethod::DefaultProfile,
            user_agent: format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            resolve: None,
            cert: None,
            insecure: false,
            timeout: None,
            connect_timeout: None,
            read_timeout: None,
        }
    }
}

impl ClientConfig {
    /// Specify the configuration directory where [Client] authentication will
    /// look for the `credentials.toml` and `config.toml` files.
    pub fn with_config_dir(mut self, config_dir: impl Into<PathBuf>) -> Self {
        self.config_dir = config_dir.into();
        self
    }

    /// Specify the profile name that will be looked up in the
    /// `credentials.toml` file.
    pub fn with_profile(mut self, profile: impl ToString) -> Self {
        self.auth_method = AuthMethod::Profile(profile.to_string());
        self
    }

    /// Authenticate with an explicit host and token.
    pub fn with_host_and_token(mut self, host: impl AsRef<str>, token: impl AsRef<str>) -> Self {
        self.auth_method = AuthMethod::HostToken {
            host: host.as_ref().to_string(),
            token: token.as_ref().to_string(),
        };
        self
    }

    /// Override hostname resolution with a particular address.
    pub fn with_resolve(mut self, domain: impl ToString, addr: IpAddr) -> Self {
        self.resolve = Some(ResolveValue {
            domain: domain.to_string(),
            addr,
        });
        self
    }

    /// Use the specified certificate when establishing a secure connection
    /// with the host.
    pub fn with_cert(mut self, cert: reqwest::Certificate) -> Self {
        self.cert = Some(cert);
        self
    }

    /// Allow insecure connections.
    pub fn with_insecure(mut self, insecure: bool) -> Self {
        self.insecure = insecure;
        self
    }

    /// Specify the desired client timeout in seconds.
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Specify the desired client connect_timeout in seconds.
    pub fn with_connect_timeout(mut self, connect_timeout: u64) -> Self {
        self.connect_timeout = Some(connect_timeout);
        self
    }

    /// Specify the desired client read_timeout in seconds.
    pub fn with_read_timeout(mut self, read_timeout: u64) -> Self {
        self.read_timeout = Some(read_timeout);
        self
    }

    /// Specify the user_agent header to be sent by the client.
    pub fn with_user_agent(mut self, user_agent: impl ToString) -> Self {
        self.user_agent = user_agent.to_string();
        self
    }

    /// Retrieve the configuration directory.
    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    /// Retrieve the specified profile (if any).
    pub fn profile(&self) -> Option<&str> {
        match &self.auth_method {
            AuthMethod::Profile(profile) => Some(profile.as_ref()),
            _ => None,
        }
    }
}

// Structure of the credentials file.
#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct CredentialsFile {
    pub profile: BTreeMap<String, ProfileCredentials>,
}

/// Clients such as the CLI may specify additional configuration information;
/// authentication only relies on the value of `default-profile`.
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct BasicConfigFile {
    pub default_profile: Option<String>,
}

impl Client {
    pub fn new_authenticated() -> Result<Self, OxideAuthError> {
        Self::new_authenticated_config(&ClientConfig::default())
    }

    pub fn new_authenticated_config(config: &ClientConfig) -> Result<Self, OxideAuthError> {
        let ClientConfig {
            config_dir,
            auth_method,
            ..
        } = config;

        let (host, token) = match auth_method {
            AuthMethod::DefaultProfile => get_profile_auth(config_dir, None)?,
            AuthMethod::Profile(profile) => get_profile_auth(config_dir, Some(profile))?,
            AuthMethod::HostToken { host, token } => (host.clone(), token.clone()),
        };

        let mut client_builder = config.make_unauthenticated_client_builder();

        let mut bearer =
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", &token).as_str())
                .expect("failed to construct the auth header");
        bearer.set_sensitive(true);
        client_builder = client_builder.default_headers(
            [(reqwest::header::AUTHORIZATION, bearer)]
                .into_iter()
                .collect(),
        );

        Ok(Self::new_with_client(
            &host,
            client_builder
                .build()
                .expect("failure to construct underlying client object"),
        ))
    }
}

impl ClientConfig {
    pub fn make_unauthenticated_client_builder(&self) -> ClientBuilder {
        let ClientConfig {
            resolve,
            cert,
            insecure,
            timeout,
            connect_timeout,
            read_timeout,
            user_agent,
            ..
        } = self;
        const DEFAULT_TIMEOUT: u64 = 15;

        let dur = std::time::Duration::from_secs(timeout.unwrap_or(DEFAULT_TIMEOUT));
        let mut client_builder = ClientBuilder::new().timeout(dur);

        // Use an explicit connect_timeout if provided, otherwise fallback to
        // timeout or default.
        let connect_timeout = match (connect_timeout, timeout) {
            (Some(ct), _) => std::time::Duration::from_secs(*ct),
            (None, Some(t)) => std::time::Duration::from_secs(*t),
            (None, None) => std::time::Duration::from_secs(DEFAULT_TIMEOUT),
        };
        client_builder = client_builder.connect_timeout(connect_timeout);

        if let Some(rt) = read_timeout {
            let read_dur = std::time::Duration::from_secs(*rt);
            client_builder = client_builder.read_timeout(read_dur);
        }

        if let Some(ResolveValue { domain, addr }) = resolve {
            client_builder = client_builder.resolve(domain, SocketAddr::new(*addr, 0));
        }
        if let Some(cert) = cert {
            client_builder = client_builder.add_root_certificate(cert.clone());
        }

        if *insecure {
            client_builder = client_builder
                .danger_accept_invalid_hostnames(true)
                .danger_accept_invalid_certs(true);
        }

        if let Some(user_agent) = user_agent {
            client_builder = client_builder.user_agent(user_agent);
        }

        client_builder
    }
}

fn get_profile_auth(
    config_dir: &Path,
    profile: Option<&String>,
) -> Result<(String, String), OxideAuthError> {
    if let (None, Ok(env_token)) = (profile, std::env::var("OXIDE_TOKEN")) {
        let env_host = std::env::var("OXIDE_HOST").map_err(|_| OxideAuthError::MissingHost)?;
        Ok((env_host, env_token))
    } else {
        let credentials_path = config_dir.join("credentials.toml");
        let contents = std::fs::read_to_string(&credentials_path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                OxideAuthError::NoAuthenticatedHosts
            } else {
                OxideAuthError::IoError(e)
            }
        })?;
        let creds = toml::from_str::<CredentialsFile>(&contents)
            .map_err(|e| OxideAuthError::TomlError(credentials_path.clone(), e))?;

        let profile_name = if let Some(profile_name) = profile {
            profile_name.clone()
        } else if let Ok(env_host) = std::env::var("OXIDE_HOST") {
            // For backward compatibility, allow users to specify a profile by
            // naming its host in OXIDE_HOST
            creds
                .profile
                .iter()
                .filter_map(|(profile_name, profile_info)| {
                    (profile_info.host == env_host).then_some(profile_name)
                })
                .next()
                .ok_or(OxideAuthError::MissingToken(env_host))?
                .clone()
        } else {
            let config_path = config_dir.join("config.toml");
            let contents = std::fs::read_to_string(&config_path).map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    OxideAuthError::NoDefaultProfile
                } else {
                    OxideAuthError::IoError(e)
                }
            })?;
            let config = toml::from_str::<BasicConfigFile>(&contents)
                .map_err(|e| OxideAuthError::TomlError(config_path, e))?;
            match config.default_profile {
                Some(p) => p,
                None => return Err(OxideAuthError::NoDefaultProfile),
            }
        };
        let profile = creds
            .profile
            .get(&profile_name)
            .ok_or(OxideAuthError::NoProfile(credentials_path, profile_name))?;
        Ok((profile.host.clone(), profile.token.clone()))
    }
}
