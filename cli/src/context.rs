// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::{net::SocketAddr, path::PathBuf, time::Duration};

use reqwest::ClientBuilder;

use oxide::{
    config::{Config, ResolveValue},
    BasicConfigFile, Client, ClientConfig, CredentialsFile, OxideError,
};
use serde::{de::DeserializeOwned, Deserialize};

// TODO
// I think we're going to want a few things here:
// The parameters necessary for creating the client. Most of that is going to
// be in ClientConfig.
// The other big chunk is going to be more customization: output format (text,
// json), ... other shit? default project? I don't know.

pub struct Context {
    client: Option<Client>,
    config: Config,
    client_config: ClientConfig,
    cred_file: CredentialsFile,
    config_file: ConfigFile,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigFile {
    #[serde(flatten)]
    pub basics: BasicConfigFile,
}

fn read_or_default<T: DeserializeOwned + Default>(path: PathBuf) -> T {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|contents| toml::from_str(&contents).ok())
        .unwrap_or_default()
}

impl Context {
    pub fn new(config: Config, client_config: ClientConfig) -> Result<Self, OxideError> {
        let client = get_client(&config)?;

        let cred_file = read_or_default(client_config.config_dir.join("credentials.toml"));
        let config_file = read_or_default(client_config.config_dir.join("config.toml"));

        Ok(Self {
            client,
            config,
            client_config,
            cred_file,
            config_file,
        })
    }

    pub fn client(&self) -> Result<&Client, OxideError> {
        self.client
            .as_ref()
            .ok_or_else(|| OxideError::NoAuthenticatedHosts)
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn client_config(&self) -> &ClientConfig {
        &self.client_config
    }

    pub fn cred_file(&self) -> &CredentialsFile {
        &self.cred_file
    }

    pub fn config_file(&self) -> &ConfigFile {
        &self.config_file
    }
}

fn get_client(config: &Config) -> Result<Option<Client>, OxideError> {
    let (host, token) = match (std::env::var("OXIDE_HOST"), std::env::var("OXIDE_TOKEN")) {
        (Ok(host), Ok(token)) => (host, token),
        (Ok(host), Err(_)) => {
            let Some(host_entry) = config.hosts.get(&host) else {
                return Err(OxideError::MissingToken(host));
            };
            (host, host_entry.token.clone())
        }
        (Err(_), Ok(token)) => {
            let Some((host, _)) = config.hosts.hosts.iter().next() else {
                return Ok(None);
            };
            (host.clone(), token)
        }
        (Err(_), Err(_)) => {
            let Some((host, host_entry)) = config.hosts.hosts.iter().next() else {
                return Ok(None);
            };
            (host.clone(), host_entry.token.clone())
        }
    };

    Ok(Some(make_client(&host, token, config)))
}

pub fn make_client(host: &str, token: String, config: &Config) -> Client {
    Client::new_with_client(host, make_rclient(Some(token), config).build().unwrap())
}

pub fn make_rclient(token: Option<String>, config: &Config) -> reqwest::ClientBuilder {
    let mut client_builder = ClientBuilder::new().connect_timeout(Duration::from_secs(15));

    if let Some(token) = token {
        let mut bearer =
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap();
        bearer.set_sensitive(true);
        client_builder = client_builder.default_headers(
            [(reqwest::header::AUTHORIZATION, bearer)]
                .into_iter()
                .collect(),
        );
    }

    if let Some(ResolveValue { host, port, addr }) = &config.resolve {
        client_builder = client_builder.resolve(host, SocketAddr::new(*addr, *port));
    }
    if let Some(cert) = &config.cert {
        client_builder = client_builder.add_root_certificate(cert.clone());
    }
    if let Some(timeout) = &config.timeout {
        client_builder = client_builder.timeout(Duration::from_secs(*timeout));
    }

    if config.insecure {
        client_builder = client_builder
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true);
    }

    client_builder
}
