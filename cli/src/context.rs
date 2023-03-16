// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use anyhow::{anyhow, Result};
use oxide_api::Client;

use crate::config::Config;

pub struct Context {
    pub client: Client,
    pub config: Config,
}

impl Context {
    pub fn new(config: Config) -> Result<Self> {
        let (host, token) = match (std::env::var("OXIDE_HOST"), std::env::var("OXIDE_TOKEN")) {
            (Ok(host), Ok(token)) => (host, token),
            (Ok(host), Err(_)) => {
                let Some(host_entry) = config.hosts.get(&host) else {
                    return Err(anyhow!("host {} not found", host));
                };
                (host, host_entry.token.clone())
            }
            (Err(_), Ok(token)) => {
                let Some((host, _)) = config.hosts.hosts.iter().next() else {
                    return Err(anyhow!("no authenticated hosts"));
                };
                (host.clone(), token)
            }
            (Err(_), Err(_)) => {
                let Some((host, host_entry)) = config.hosts.hosts.iter().next() else {
                    return Err(anyhow!("no authenticated hosts"));
                };
                (host.clone(), host_entry.token.clone())
            }
        };

        let auth = format!("Bearer {}", token);
        let mut auth_value = reqwest::header::HeaderValue::from_str(&auth)?;
        auth_value.set_sensitive(true);

        let dur = std::time::Duration::from_secs(15);
        let rclient = reqwest::Client::builder()
            .connect_timeout(dur)
            .timeout(dur)
            .default_headers(
                [(http::header::AUTHORIZATION, auth_value)]
                    .into_iter()
                    .collect(),
            )
            .build()
            .unwrap();
        let client = oxide_api::Client::new_with_client(&host, rclient);

        Ok(Self { client, config })
    }
}
