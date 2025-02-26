// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::path::PathBuf;

use thiserror::Error;

mod auth;
#[cfg(feature = "clap")]
mod clap_feature;
pub mod extras;
mod generated_sdk;
mod tracing;

pub use auth::*;
pub use generated_sdk::*;

/// Errors for interfaces related to authentication
#[derive(Error, Debug)]
pub enum OxideAuthError {
    #[error(r"$OXIDE_TOKEN is set but $OXIDE_HOST is not")]
    MissingHost,
    #[error(
        r"$OXIDE_HOST is set, but {0} has no corresponding token.\n
                Login without $OXIDE_HOST set or set $OXIDE_TOKEN."
    )]
    MissingToken(String),
    #[error("Parse error for {0}: {1}")]
    TomlError(PathBuf, toml::de::Error),
    #[error("IO Error: {0}")]
    IoError(std::io::Error),
    #[error("No profile specified and no default profile")]
    NoDefaultProfile,
    #[error("Profile information not present in {0} for {1}")]
    NoProfile(PathBuf, String),
    #[error("no authenticated hosts; use oxide auth login to authenticate")]
    NoAuthenticatedHosts,
}
