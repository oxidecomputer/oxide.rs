// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use crate::RunnableCmd;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use oxide::composite::ClientCompositeExt;
use oxide::types::BlockSize;
use oxide::types::ByteCount;
use oxide::types::Name;
use oxide::types::NameOrId;
use std::path::PathBuf;

/// Create a disk from a file upload
///
/// Create a new disk and upload the contents of a file to that disk. Use
/// `--snapshot` to optionally create a snapshot of the disk after the upload is
/// complete. If creating a snapshot, optionally use the `--image` options to
/// create an image from that snapshot.
///
/// The start, write, stop, and finalize subcommands can be used for lower-
/// level operations.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(args_conflicts_with_subcommands(true))]
pub struct CmdDiskImport {
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

#[async_trait]
impl RunnableCmd for CmdDiskImport {
    fn is_subtree() -> bool {
        false
    }
    async fn run(&self, ctx: &oxide::context::Context) -> Result<()> {
        let client = ctx.client()?;
        client
            .disk_import()
            .project(&self.project)
            .description(&self.description)
            .path(&self.path)
            .disk(&self.disk)
            .disk_size(self.disk_size.clone())
            .disk_block_size(self.disk_block_size.clone())
            .snapshot(self.snapshot.clone())
            .image(self.image.clone())
            .image_description(self.image_description.clone())
            .image_os(self.image_os.clone())
            .image_version(self.image_version.clone())
            .extra_output(true)
            .send()
            .await?;

        Ok(())
    }
}
