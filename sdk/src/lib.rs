// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

#![forbid(unsafe_code)]

#[allow(clippy::clone_on_copy)]
#[allow(clippy::len_zero)]
#[allow(clippy::uninlined_format_args)]
#[allow(clippy::vec_init_then_push)]
mod generated_sdk;

#[cfg(feature = "clap")]
mod clap_feature;

pub use generated_sdk::*;
