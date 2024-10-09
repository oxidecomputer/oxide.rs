// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::path::{Path, PathBuf};

use anyhow::Result;
use oxide::{BasicConfigFile, ClientConfig, CredentialsFile};
use serde::{de::DeserializeOwned, Deserialize};

/// The Context is what we use to carry globally relevant information around
/// to subcommands. This includes configuration information and top-level
/// command-line options. This may be used to construct an authenticated
/// client if the subcommand requires it.
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

fn read_or_default<T: DeserializeOwned + Default>(path: PathBuf) -> Result<T> {
    match std::fs::read_to_string(path) {
        Ok(contents) => Ok(toml::from_str(&contents)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(T::default()),
        Err(e) => Err(e.into()),
    }
}

impl Context {
    pub fn new(client_config: ClientConfig) -> Result<Self> {
        let config_dir = client_config.config_dir();
        let cred_file = read_or_default(config_dir.join("credentials.toml"))?;
        let config_file = read_or_default(config_dir.join("config.toml"))?;

        validate_credentials_permissions(&config_dir.join("credentials.toml"));

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

fn validate_credentials_permissions(path: &Path) {
    #[cfg(unix)]
    {
        use crate::eprintln_nopipe;
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        if let Ok(stat) = fs::metadata(path) {
            let mode = stat.permissions().mode();
            if mode & 0o077 != 0 {
                eprintln_nopipe!("WARNING: {:o} permissions on \"{}\" may allow other users to access your login credentials.", mode & 0o777, path.display());
            }
        }
    }
}
