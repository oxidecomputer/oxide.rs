// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::{net::SocketAddr, time::Duration};

use reqwest::ClientBuilder;

use crate::{
    config::{Config, ResolveValue},
    Client, OxideError,
};

pub struct Context {
    client: Option<Client>,
    config: Config,
}

impl Context {
    pub fn new(config: Config) -> Result<Self, OxideError> {
        let client = get_client(&config)?;
        Ok(Self { client, config })
    }

    pub fn client(&self) -> Result<&Client, OxideError> {
        self.client
            .as_ref()
            .ok_or_else(|| OxideError::NoAuthenticatedHosts)
    }

    pub fn config(&self) -> &Config {
        &self.config
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
