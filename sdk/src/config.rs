// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use core::str::FromStr;
use std::fs::create_dir_all;
use std::net::IpAddr;
use std::{collections::HashMap, path::PathBuf};

use crate::OxideError;
use serde::{Deserialize, Serialize};
use toml_edit::{Item, Table};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveValue {
    pub host: String,
    pub port: u16,
    pub addr: IpAddr,
}

impl FromStr for ResolveValue {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let values = s.splitn(3, ':').collect::<Vec<_>>();
        let [host, port, addr] = values.as_slice() else {
            return Err(r#"value must be "host:port:addr"#.to_string());
        };

        let host = host.to_string();
        let port = port
            .parse()
            .map_err(|_| format!("error parsing port '{}'", port))?;

        // `IpAddr::parse()` does not accept enclosing brackets on IPv6
        // addresses; strip them off if they exist.
        let addr = addr
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap_or(addr);
        let addr = addr
            .parse()
            .map_err(|_| format!("error parsing address '{}'", addr))?;

        Ok(Self { host, port, addr })
    }
}

pub struct Config {
    pub client_id: Uuid,
    pub hosts: Hosts,
    pub resolve: Option<ResolveValue>,
    pub cert: Option<reqwest::Certificate>,
    pub insecure: bool,
    pub timeout: Option<u64>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Hosts {
    #[serde(flatten)]
    pub hosts: HashMap<String, Host>,
}

impl Hosts {
    pub fn get<S: AsRef<str>>(&self, hostname: S) -> Option<&Host> {
        self.hosts.get(hostname.as_ref())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Host {
    pub token: String,
    pub user: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub default: bool,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &T::default()
}

impl Default for Config {
    fn default() -> Self {
        let mut dir = dirs::home_dir().unwrap();
        dir.push(".config");
        dir.push("oxide");
        create_dir_all(&dir).unwrap();
        Self::new_with_config_dir(dir)
    }
}

impl Config {
    pub fn new_with_config_dir(dir: PathBuf) -> Self {
        let hosts_path = dir.join("hosts.toml");
        let hosts = if let Ok(contents) = std::fs::read_to_string(hosts_path) {
            toml::from_str(&contents).unwrap()
        } else {
            Default::default()
        };

        Self {
            client_id: Default::default(),
            hosts,
            resolve: None,
            cert: None,
            insecure: false,
            timeout: None,
        }
    }

    pub fn update_host(&self, hostname: String, host_entry: Host) -> Result<(), OxideError> {
        let mut dir = dirs::home_dir().unwrap();
        dir.push(".config");
        dir.push("oxide");
        create_dir_all(&dir).unwrap();

        let hosts_path = dir.join("hosts.toml");
        let mut hosts = if let Ok(contents) = std::fs::read_to_string(hosts_path.clone()) {
            contents
                .parse::<toml_edit::Document>()
                .map_err(OxideError::TomlError)?
        } else {
            Default::default()
        };

        let table = hosts
            .entry(&hostname)
            .or_insert_with(|| Item::Table(Table::default()))
            .as_table_mut()
            .unwrap(); // TODO

        let Host {
            token,
            user,
            default,
        } = host_entry;

        table.insert("token", toml_edit::value(token));
        table.insert("user", toml_edit::value(user));

        if default || table.contains_key("default") {
            table.insert("default", toml_edit::value(default));
        }

        std::fs::write(hosts_path, hosts.to_string()).map_err(OxideError::IoError)?;

        Ok(())
    }

    pub fn with_resolve(mut self, resolve: ResolveValue) -> Self {
        self.resolve = Some(resolve);
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
}
