// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

//! Enabled with the "clap" feature. This is to support clap consumers such as
//! the `oxide` CLI.

use std::ffi::OsString;

use crate::*;

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
        fn parse(
            cmd: &clap::Command,
            arg: Option<&clap::Arg>,
            value: &std::ffi::OsStr,
        ) -> Option<types::ByteCount> {
            let bytes = value.to_str()?.as_bytes();
            let ii = bytes.partition_point(|c| (*c as char).is_ascii_digit());
            let number = OsString::from(std::str::from_utf8(&bytes[..ii]).ok()?);
            let suffix = std::str::from_utf8(&bytes[ii..]).ok()?.to_lowercase();

            let multiple = match suffix.as_str() {
                "kib" | "k" => 1024,
                "mib" | "m" => 1024 * 1024,
                "gib" | "g" => 1024 * 1024 * 1024,
                "tib" | "t" => 1024 * 1024 * 1024 * 1024,
                "" => 1,
                _ => None?,
            };

            let inner = clap::value_parser!(u64);
            let val = inner.parse_ref(cmd, arg, number.as_os_str()).ok()? * multiple;

            Some(types::ByteCount(val))
        }

        parse(cmd, arg, value).ok_or_else(|| clap::Error::new(clap::error::ErrorKind::InvalidValue))
    }
}
