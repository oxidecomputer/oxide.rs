// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use thiserror::Error;

mod auth;
mod generated_sdk;

#[cfg(feature = "clap")]
mod clap_feature;

pub mod config;

pub use generated_sdk::*;

#[derive(Error, Debug)]
/// Errors for interfaces related to configuration and authentication
pub enum OxideError {
    #[error(
        r"$OXIDE_HOST is set, but {0} has no corresponding token.\n
                Login without $OXIDE_HOST set or set $OXIDE_TOKEN."
    )]
    MissingToken(String),
    #[error("no authenticated hosts")]
    NoAuthenticatedHosts,
    #[error("TomlError: {0}")]
    TomlError(toml_edit::TomlError),
    #[error("IO Error: {0}")]
    IoError(std::io::Error),
}
