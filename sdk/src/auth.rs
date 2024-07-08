// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

//! TODO
//! What do we need to do in here?
//! - create an authenticated client based on a profile
//!
//! What do we want to keep in the CLI
//! - the new / refresh authentication workflow
//!
//! Does this imply that the config.toml is purely the domain of the CLI? I
//! think so.
//!
//! Do we want a ClientBuilder type to let users do customization? It would
//! probably have a fn map() that takes a reqwest::ClientBuilder
//!
//! How do we want to deal with the concept of a "default" profile?
//! 1. configuration within the creds file--don't love it because now config
//!    is split between config and creds
//! 2. have the SDK load up a config file--kind of lousy because now we always
//!    need two files. Maybe that's fine... and
//!
//! Also... what if the user specifies a profile explicitly? Then we don't need
//! a config file. Maybe an enum that's like "profile-name-or-config-file"?

use std::{
    collections::BTreeMap,
    fmt::Display,
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
};

use crate::Client;
use reqwest::ClientBuilder;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProfileCredentials {
    pub token: String,
    pub host: String,
    pub user: String,
}

// TODO: do we want a way to easily change the port number? It would need to be
// shoved into the baseurl string
pub struct ResolveValue {
    pub domain: String,
    pub addr: IpAddr,
}

/// Configuration for creating a [Client]
pub struct ClientConfig {
    config_dir: PathBuf,
    auth_method: AuthMethod,
    resolve: Option<ResolveValue>,
    cert: Option<reqwest::Certificate>,
    insecure: bool,
    timeout: Option<u64>,
}

enum AuthMethod {
    DefaultProfile,
    Profile(String),
    HostToken { host: String, token: String },
}

impl Default for ClientConfig {
    fn default() -> Self {
        let mut config_dir = dirs::home_dir().unwrap();
        config_dir.push(".config");
        config_dir.push("oxide");
        Self {
            config_dir,
            auth_method: AuthMethod::DefaultProfile,
            resolve: None,
            cert: None,
            insecure: false,
            timeout: None,
        }
    }
}

impl ClientConfig {
    pub fn with_config_dir(mut self, config_dir: impl Into<PathBuf>) -> Self {
        self.config_dir = config_dir.into();
        self
    }

    pub fn with_profile(mut self, profile: impl ToString) -> Self {
        self.auth_method = AuthMethod::Profile(profile.to_string());
        self
    }

    pub fn with_host_and_token(mut self, host: impl ToString, token: impl ToString) -> Self {
        self.auth_method = AuthMethod::HostToken {
            host: host.to_string(),
            token: token.to_string(),
        };
        self
    }

    pub fn with_resolve(mut self, domain: impl ToString, addr: IpAddr) -> Self {
        self.resolve = Some(ResolveValue {
            domain: domain.to_string(),
            addr,
        });
        self
    }

    pub fn with_cert(mut self, cert: reqwest::Certificate) -> Self {
        self.cert = Some(cert);
        self
    }

    pub fn with_insecure(mut self, insecure: bool) -> Self {
        self.insecure = insecure;
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn profile(&self) -> Option<&str> {
        match &self.auth_method {
            AuthMethod::Profile(profile) => Some(profile.as_ref()),
            _ => None,
        }
    }
}

// TODO do I want a version = 1 line in there?
#[derive(Deserialize, Debug, Default)]
pub struct CredentialsFile {
    pub profile: BTreeMap<String, ProfileCredentials>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct BasicConfigFile {
    pub default_profile: Option<String>,
}

// TODO thiserror
#[derive(Debug)]
pub struct ClientAuthError;

impl Display for ClientAuthError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for ClientAuthError {}

impl Client {
    pub fn new_authenticated() -> Result<Self, ClientAuthError> {
        Self::new_authenticated_config(&ClientConfig::default())
    }

    // TODO remove unwraps
    // TODO maybe we have a version that returns a reqwest::ClientBuilder?
    pub fn new_authenticated_config(config: &ClientConfig) -> Result<Self, ClientAuthError> {
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
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", &token).as_str()).unwrap();
        bearer.set_sensitive(true);
        client_builder = client_builder.default_headers(
            [(reqwest::header::AUTHORIZATION, bearer)]
                .into_iter()
                .collect(),
        );

        Ok(Self::new_with_client(
            &host,
            client_builder.build().unwrap(),
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
            ..
        } = self;
        let dur = std::time::Duration::from_secs(timeout.unwrap_or(15));
        let mut client_builder = ClientBuilder::new().connect_timeout(dur).timeout(dur);

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

        client_builder
    }
}

fn get_profile_auth(
    config_dir: &Path,
    profile: Option<&String>,
) -> Result<(String, String), ClientAuthError> {
    if let (None, Ok(env_token)) = (profile, std::env::var("OXIDE_TOKEN")) {
        let env_host = std::env::var("OXIDE_HOST").map_err(|_| ClientAuthError)?;
        Ok((env_host, env_token))
    } else {
        let credentials_path = config_dir.join("credentials.toml");
        let contents = std::fs::read_to_string(credentials_path).unwrap();
        let creds = toml::from_str::<CredentialsFile>(&contents).unwrap();

        // TODO
        // For backward compatibility, allow users to specify a profile by
        // naming its host in OXIDE_HOST
        assert!(std::env::var("OXIDE_HOST").is_err());

        let profile_name = if let Some(profile_name) = profile {
            profile_name.clone()
        } else {
            let config_path = config_dir.join("config.toml");
            let contents = std::fs::read_to_string(config_path).unwrap();
            let config = toml::from_str::<BasicConfigFile>(&contents).unwrap();
            config.default_profile.clone().unwrap()
        };
        // TODO unwrap
        let profile = creds.profile.get(&profile_name).unwrap();
        Ok((profile.host.clone(), profile.token.clone()))
    }
}

#[cfg(test)]
mod tests {

    use crate::auth::CredentialsFile;

    #[test]
    fn xxx_playing_with_files() {
        let contents = std::fs::read_to_string("tests/data/test-credentials.toml").unwrap();
        println!("{}", contents);

        let creds = toml::from_str::<CredentialsFile>(&contents).unwrap();
        println!("{:#?}", creds);
    }
}
