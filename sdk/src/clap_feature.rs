// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

//! Enabled with the "clap" feature. This is to support clap consumers such as
//! the `oxide` CLI.

use std::str::FromStr;

use crate::*;

// Note that we make use of clap's impl of TypedValueParser for Fn(&str) ->
// Result<..>.

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
        fn parse(value: &str) -> Result<types::ByteCount, String> {
            let ii = value
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(value.len());

            let number = &value[..ii];
            let suffix = &value[ii..];

            let multiple = match suffix {
                "kib" | "k" => 1024,
                "mib" | "m" => 1024 * 1024,
                "gib" | "g" => 1024 * 1024 * 1024,
                "tib" | "t" => 1024 * 1024 * 1024 * 1024,
                "" => 1,
                _ => return Err(format!("unknown suffix '{}'", suffix)),
            };

            number
                .parse()
                .map(|n: u64| types::ByteCount(n * multiple))
                .map_err(|e| e.to_string())
        }

        parse.parse_ref(cmd, arg, value)
    }
}

impl clap::builder::ValueParserFactory for types::BlockSize {
    type Parser = BlockSizeParser;
    fn value_parser() -> Self::Parser {
        BlockSizeParser
    }
}

#[derive(Clone, Debug)]
pub struct BlockSizeParser;
impl clap::builder::TypedValueParser for BlockSizeParser {
    type Value = types::BlockSize;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        fn parse(value: &str) -> Result<types::BlockSize, String> {
            i64::from_str(value)
                .map_err(|e| e.to_string())?
                .try_into()
                .map_err(|_| "block size must be 512, 2048, or 4096".to_string())
        }

        parse.parse_ref(cmd, arg, value)
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::builder::PossibleValue> + '_>> {
        Some(Box::new(
            [
                clap::builder::PossibleValue::new("512"),
                clap::builder::PossibleValue::new("2048"),
                clap::builder::PossibleValue::new("4096"),
            ]
            .into_iter(),
        ))
    }
}

// It would be nice if progenitor were able to give a nice, specific error
// message, but since it can't we'll craft one for the CLI.
impl clap::builder::ValueParserFactory for types::NameOrId {
    type Parser = NameOrIdParser;
    fn value_parser() -> Self::Parser {
        NameOrIdParser
    }
}

#[derive(Clone, Debug)]
pub struct NameOrIdParser;
impl clap::builder::TypedValueParser for NameOrIdParser {
    type Value = types::NameOrId;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        fn parse(value: &str) -> Result<types::NameOrId, String> {
            value
                .parse()
                .map_err(|_| "value must be a UUID or name".to_string())
        }

        parse.parse_ref(cmd, arg, value)
    }
}

#[cfg(test)]
mod tests {
    use clap::{builder::TypedValueParser, value_parser};

    use crate::types::{BlockSize, ByteCount, NameOrId};

    #[test]
    fn test_byte_count() {
        let parser = value_parser!(ByteCount);
        let cmd = clap::Command::new("cmd");
        let arg = None;
        let value = std::ffi::OsStr::new("1.21jiggabytes");
        let Err(err) = parser.parse_ref(&cmd, arg, value) else {
            panic!()
        };
        assert!(err.to_string().contains("unknown suffix"), "{err}",);
    }

    #[test]
    fn test_block_size() {
        let parser = value_parser!(BlockSize);
        let cmd = clap::Command::new("cmd");
        let arg = None;
        let value = std::ffi::OsStr::new("123");
        let Err(err) = parser.parse_ref(&cmd, arg, value) else {
            panic!()
        };
        assert!(
            err.to_string()
                .contains("block size must be 512, 2048, or 4096"),
            "{err}",
        );
    }

    #[test]
    fn test_name_or_id() {
        let parser = value_parser!(NameOrId);
        let cmd = clap::Command::new("cmd");
        let arg = None;
        let value = std::ffi::OsStr::new("123");
        let Err(err) = parser.parse_ref(&cmd, arg, value) else {
            panic!()
        };
        assert!(
            err.to_string().contains("value must be a UUID or name"),
            "{err}",
        );
    }
}
