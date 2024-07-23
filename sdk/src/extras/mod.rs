// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

//! The `extra` feature adds additional, non-generated, compound interfaces to
//! the [Client]. It is intended for methods that are functional rather than
//! for those that offer enhanced output or a simplified interface. (This is
//! why the the CLI uses a disk import interface from here, but has a number of
//! custom network subcommands that pretty-print or provide a simpler user
//! interface for common use cases.)
//!
//! These interfaces operate very similarly to the generated interfaces.

mod disk;

pub mod builder {
    pub use super::disk::DiskImport;
}

pub trait ClientExtraExt {
    fn disk_import(&self) -> builder::DiskImport;
}
