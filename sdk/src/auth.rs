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
    net::{IpAddr, SocketAddr},
    path::PathBuf,
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
    domain: String,
    addr: IpAddr,
}

/// Configuration for creating a [Client]
pub struct ClientConfig {
    profile_or_config: ProfileOrConfig,
    credential_path: PathBuf,
    resolve: Option<ResolveValue>,
    cert: Option<reqwest::Certificate>,
    insecure: bool,
    timeout: Option<u64>,
}
enum ProfileOrConfig {
    Profile(String),
    Config(PathBuf),
}

impl Default for ClientConfig {
    fn default() -> Self {
        let mut dir = dirs::home_dir().unwrap();
        dir.push(".config");
        dir.push("oxide");
        Self {
            profile_or_config: ProfileOrConfig::Config(dir.join("config.toml")),
            credential_path: dir.join("credentials.toml"),
            resolve: None,
            cert: None,
            insecure: false,
            timeout: None,
        }
    }
}

impl ClientConfig {
    pub fn with_profile(mut self, profile: impl ToString) -> Self {
        self.profile_or_config = ProfileOrConfig::Profile(profile.to_string());
        self
    }

    pub fn with_config(mut self, config_path: impl Into<PathBuf>) -> Self {
        if matches!(&self.profile_or_config, ProfileOrConfig::Config(_)) {
            self.profile_or_config = ProfileOrConfig::Config(config_path.into());
        }
        self
    }

    pub fn with_resolve(mut self, domain: impl ToString, addr: IpAddr) -> Self {
        self.resolve = Some(ResolveValue {
            domain: domain.to_string(),
            addr,
        });
        self
    }

    // TODO
    // with_cert
    // with_insecure
    // with_timeout
}

// TODO do I want a version = 1 line in there?
#[derive(Deserialize, Debug)]
struct BasicCredentialsFile {
    profile: BTreeMap<String, ProfileCredentials>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct BasicConfigFile {
    default_profile: String,
}

pub struct ClientAuthError;

impl Client {
    pub fn new_authenticated() -> Result<Self, ClientAuthError> {
        Self::new_authenticated_config(ClientConfig::default())
    }

    // TODO remove unwraps
    // TODO maybe we have a version that returns a reqwest::ClientBuilder?
    pub fn new_authenticated_config(config: ClientConfig) -> Result<Self, ClientAuthError> {
        let ClientConfig {
            profile_or_config,
            credential_path,
            resolve,
            cert,
            insecure,
            timeout,
        } = config;
        let profile_name = match profile_or_config {
            ProfileOrConfig::Profile(profile) => profile.clone(),
            ProfileOrConfig::Config(config_path) => {
                let contents = std::fs::read_to_string(config_path).unwrap();
                let config = toml::from_str::<BasicConfigFile>(&contents).unwrap();
                config.default_profile
            }
        };

        let contents = std::fs::read_to_string(credential_path).unwrap();
        let creds = toml::from_str::<BasicCredentialsFile>(&contents).unwrap();

        let profile = creds.profile.get(&profile_name).unwrap();

        let dur = std::time::Duration::from_secs(timeout.unwrap_or(15));
        let mut bearer =
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", &profile.token).as_str())
                .unwrap();
        bearer.set_sensitive(true);
        let mut client_builder = ClientBuilder::new()
            .connect_timeout(dur)
            .timeout(dur)
            .default_headers(
                [(reqwest::header::AUTHORIZATION, bearer)]
                    .into_iter()
                    .collect(),
            );

        if let Some(ResolveValue { domain, addr }) = resolve {
            client_builder = client_builder.resolve(&domain, SocketAddr::new(addr, 0));
        }
        if let Some(cert) = cert {
            client_builder = client_builder.add_root_certificate(cert.clone());
        }

        if insecure {
            client_builder = client_builder
                .danger_accept_invalid_hostnames(true)
                .danger_accept_invalid_certs(true);
        }

        Ok(Self::new_with_client(
            &profile.host,
            client_builder.build().unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::auth::BasicCredentialsFile;

    #[test]
    fn xxx_playing_with_files() {
        let contents = std::fs::read_to_string("tests/data/test-credentials.toml").unwrap();
        println!("{}", contents);

        let creds = toml::from_str::<BasicCredentialsFile>(&contents).unwrap();
        println!("{:#?}", creds);
    }
}
