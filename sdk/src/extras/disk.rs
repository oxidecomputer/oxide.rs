// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use std::path::PathBuf;

use crate::types::{BlockSize, ByteCount, Name, NameOrId};

// TODO
// Maybe I have this struct and then have a proc macro that emits
// disk_import_builder::DiskImport which is basically the same as the builder
// structs that progenitor creates. It could then have some crate-private
// conversion method that would be like
// -> Result<(Client, DiskImport), progenitor_client::Error<T>>
// then I do a custom impl for send or execute
// meh.. maybe too much work for this

// TODO
// another wrinkle --we do need some custom output etc. progress bars
// we could have some other bits of the builder like
// with_error_output or with_upload_progress

mod disk_import_builder {
    use std::path::PathBuf;

    use crate::{
        types::{BlockSize, ByteCount, Name, NameOrId},
        Client,
    };

    pub struct DiskImport<'a> {
        client: &'a Client,
        project: Result<NameOrId, String>,
        description: Result<String, String>,
        path: Result<PathBuf, String>,
        disk: Result<Name, String>,
        disk_size: Result<Option<ByteCount>, String>,
        disk_block_size: Result<Option<BlockSize>, String>,
        snapshot: Result<Option<Name>, String>,
        image: Result<Option<Name>, String>,
        image_description: Result<Option<String>, String>,
        image_os: Result<Option<String>, String>,
        image_version: Result<Option<String>, String>,
    }
    impl<'a> DiskImport<'a> {
        pub fn foo() {}
        pub async fn execute() {}
    }

    impl<'a> TryFrom<DiskImport<'a>> for (&'a Client, super::DiskImport) {
        type Error = progenitor_client::Error;

        fn try_from(value: DiskImport) -> Result<Self, Self::Error> {
            let DiskImport {
                client,
                project,
                description,
                path,
                disk,
                disk_size,
                disk_block_size,
                snapshot,
                image,
                image_description,
                image_os,
                image_version,
            } = value;

            Ok((
                client,
                || -> Result<super::DiskImport, String> {
                    Ok(super::DiskImport {
                        project: project?,
                        description: description?,
                        path: path?,
                        disk: disk?,
                        disk_size: disk_size?,
                        disk_block_size: disk_block_size?,
                        snapshot: snapshot?,
                        image: image?,
                        image_description: image_description?,
                        image_os: image_os?,
                        image_version: image_version?,
                    })
                }()
                .map_err(progenitor_client::Error::InvalidRequest)?,
            ))
        }
    }
}

/// Create a disk from a file upload
///
/// Create a new disk and upload the contents of a file to that disk. Use
/// `--snapshot` to optionally create a snapshot of the disk after the upload is
/// complete. If creating a snapshot, optionally use the `--image` options to
/// create an image from that snapshot.
///
/// The start, write, stop, and finalize subcommands can be used for lower-
/// level operations.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(feature = "clap", command(verbatim_doc_comment))]
#[cfg_attr(feature = "clap", command(args_conflicts_with_subcommands(true)))]
pub(crate) struct DiskImport {
    /// Name or ID of the project
    #[clap(long)]
    project: NameOrId,

    #[clap(long)]
    description: String,

    /// Path to the file to import
    #[clap(long)]
    path: PathBuf,

    /// The name of the disk to create
    #[clap(long)]
    disk: Name,

    /// The size of the disk to create. If unspecified, the size of the file
    /// will be used, rounded up to the nearest GB.
    #[clap(long)]
    disk_size: Option<ByteCount>,

    /// The desired size of the disk's blocks.
    #[clap(long)]
    disk_block_size: Option<BlockSize>,

    /// If supplied, create a snapshot with the given name.
    #[clap(long)]
    snapshot: Option<Name>,

    /// If supplied, create an image with the given name. Requires the creation
    /// of a snapshot.
    #[clap(long, requires_all = ["snapshot", "image_description", "image_os", "image_version"])]
    image: Option<Name>,

    /// The description for the image created out of the snapshot of this disk.
    #[clap(long, requires_all = ["snapshot", "image", "image_os", "image_version"])]
    image_description: Option<String>,

    /// The OS of this image (e.g. Debian)
    #[clap(long, requires_all = ["snapshot", "image", "image_description", "image_version"])]
    image_os: Option<String>,

    /// The version of this image (e.g. 11, focal, a9e77e3a, 2023-04-06T14:23:34Z)
    #[clap(long, requires_all = ["snapshot", "image", "image_description", "image_os"])]
    image_version: Option<String>,
}
