// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

//! The `extra` feature adds additional, non-generated, compound interfaces to
//! the [`Client`](super::generated_sdk::Client). It is intended for methods
//! that are functional rather than for those that offer enhanced output or a
//! simplified interface. (This is why the the CLI uses a disk import interface
//! from here, but has a number of custom network subcommands that pretty-print
//! or provide a simpler user interface for common use cases.)
//!
//! These interfaces operate very similarly to the generated interfaces.

pub mod disk;

pub trait ClientExtraDiskExt {
    /// A convenience wrapper around the SDK to simplify disk creation.
    /// Create a disk, optionally with a snapshot and image. The standard
    /// disk import methods in the SDK will frequently leave the partially
    /// imported disks when interrupted or an error occurs. This extension
    /// will attempt to cleanup the disk.
    ///
    /// Arguments:
    /// - `project`: Name or ID of the project
    /// - `description`: Human-readable free-form text about the disk
    /// - `upload_thread_ct`: The number of threads used to upload the disk
    /// - `disk`: Name of the disk
    /// - `disk_info`: Information needed to construct the disk
    /// - `image_info`: Information needed to construct a snapshot and image, optional
    ///
    /// The [`execute`](disk::builder::DiskImport::execute) method is equivalent to the
    /// `send` method used in standard SDK actions.
    /// ```ignore
    /// client.disk_import()
    ///    .project(project)
    ///    .description(description)
    ///    .upload_thread_ct(upload_thread_ct)
    ///    .disk(disk)
    ///    .disk_info(disk_info)
    ///    .image_info(image_info)
    ///    .execute()?.await?;
    /// ```
    ///
    /// The [`execute_with_control`](disk::builder::DiskImport::execute_with_control)
    /// method returns a future for the import and a [`DiskImportHandle`](disk::types::DiskImportHandle).
    /// The handle exposes a [`progress`](disk::types::DiskImportHandle::progress) method
    /// to check the current number of bytes uploaded, and a
    /// [`cancel`](disk::types::DiskImportHandle::cancel) method that will stop the
    /// import and remove the new disk.
    /// ```ignore
    /// let (import_future, handle) = client.disk_import()
    ///    .project(project)
    ///    .description(description)
    ///    .upload_thread_ct(upload_thread_ct)
    ///    .disk(disk)
    ///    .disk_info(disk_info)
    ///    .image_info(image_info)
    ///    .execute_with_control()?;
    ///
    /// let pb = ProgressBar::new();
    /// let progress_rx = handler.progress();
    /// tokio::spawn(async move {
    ///     loop {
    ///         _ = tokio::signal::ctrl_c() => {
    ///            handle.cancel();
    ///         }
    ///         _ = progress_rx.changed() => {
    ///             pb.set_position(*progress_rs.borrow());
    ///         }
    ///     }
    /// });
    ///
    /// import_future.await?;
    /// ```
    fn disk_import(&self) -> disk::builder::DiskImport;
}
