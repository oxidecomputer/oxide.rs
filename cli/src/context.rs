// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::path::PathBuf;

use oxide::{BasicConfigFile, ClientConfig, CredentialsFile, OxideError};
use serde::{de::DeserializeOwned, Deserialize};

// TODO
// I think we're going to want a few things here:
// The parameters necessary for creating the client. Most of that is going to
// be in ClientConfig.
// The other big chunk is going to be more customization: output format (text,
// json), ... other shit? default project? I don't know.

pub struct Context {
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
    pub fn new(client_config: ClientConfig) -> Result<Self, OxideError> {
        let config_dir = client_config.config_dir();
        let cred_file = read_or_default(config_dir.join("credentials.toml"));
        let config_file = read_or_default(config_dir.join("config.toml"));

        Ok(Self {
            client_config,
            cred_file,
            config_file,
        })
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
