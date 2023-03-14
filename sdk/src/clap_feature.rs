// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

//! Enabled with the "clap" feature. This is to support clap consumers such as
//! the `oxide` CLI.

use crate::*;

use std::os::unix::prelude::OsStrExt;

impl clap::builder::ValueParserFactory for types::ByteCount {
    type Parser = ByteCountParser;
    /// Add KiB, MiB, GiB, TiB parsing.
    fn value_parser() -> Self::Parser {
        ByteCountParser
    }
}

#[derive(Clone, Debug)]
pub struct ByteCountParser;
impl clap::builder::TypedValueParser for ByteCountParser {
    type Value = types::ByteCount;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let bytes = value.as_bytes();
        let ii = bytes.partition_point(|c| (*c as char).is_ascii_digit());
        let number = std::ffi::OsStr::from_bytes(&bytes[..ii]);
        let suffix = std::ffi::OsStr::from_bytes(&bytes[ii..])
            .to_str()
            .map(str::to_lowercase);

        let multiple = match suffix.as_deref() {
            Some("kib") | Some("k") => 1024,
            Some("mib") | Some("m") => 1024 * 1024,
            Some("gib") | Some("g") => 1024 * 1024 * 1024,
            Some("tib") | Some("t") => 1024 * 1024 * 1024 * 1024,
            Some("") => 1,
            _ => Err(clap::Error::new(clap::error::ErrorKind::InvalidValue))?,
        };

        let inner = clap::value_parser!(u64);
        let val = inner.parse_ref(cmd, arg, number)? * multiple;
        Ok(types::ByteCount(val))
    }
}
