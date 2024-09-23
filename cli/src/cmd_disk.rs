// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use crate::{eprintln_nopipe, println_nopipe};

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use oxide::extras::disk::types::{DiskImportHandle, DiskInfo, ImageInfo};
use oxide::extras::ClientExtraDiskExt;
use oxide::types::{BlockSize, ByteCount, Name, NameOrId};
use oxide::Client;
use std::path::PathBuf;
use std::process;
use tokio::signal;
use tokio::sync::watch;

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
impl crate::AuthenticatedCmd for CmdDiskImport {
    fn is_subtree() -> bool {
        false
    }

    async fn run(&self, client: &Client) -> Result<()> {
        let disk_info = DiskInfo::calculate(
            self.path.clone(),
            self.disk_size.as_ref(),
            self.disk_block_size.as_ref(),
        )?;

        // Default to 8 upload tasks - this evenly divides all block sizes, and we know
        // that the file size is evenly divided by the selected block size.
        let mut builder = client
            .disk_import()
            .project(self.project.clone())
            .description(self.description.clone())
            .upload_thread_ct(8)
            .disk(self.disk.clone())
            .disk_info(disk_info.clone());

        if self.snapshot.is_some() {
            // Clap enforces that all of these are present when snapshot is.
            builder = builder.image_info(ImageInfo {
                snapshot: self.snapshot.clone().unwrap(),
                image: self.image.clone().unwrap(),
                image_description: self.image_description.clone().unwrap(),
                image_os: self.image_os.clone().unwrap(),
                image_version: self.image_version.clone().unwrap(),
            });
        }

        let (import_future, handle) = builder.execute_with_control()?;

        let pb = ProgressBar::new(disk_info.file_size);
        pb.set_style(ProgressStyle::default_bar().template(
            "[{elapsed_precise}] [{wide_bar:.green}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta}",
        )?);
        pb.println(format!("Creating disk \"{}\"", &*self.disk));
        let pb = start_progress_bar(handle.progress(), disk_info.file_size, &self.disk)?;
        watch_for_ctrl_c(handle, pb);

        import_future.await?;

        println_nopipe!("Done!");
        Ok(())
    }
}

fn start_progress_bar(
    mut progress_rx: watch::Receiver<u64>,
    file_size: u64,
    disk_name: &str,
) -> Result<ProgressBar> {
    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar().template(
        "[{elapsed_precise}] [{wide_bar:.green}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta}",
    )?);
    pb.set_position(0);
    pb.println(format!("Creating disk \"{disk_name}\""));
    let pb_updater = pb.clone();

    tokio::spawn(async move {
        loop {
            match progress_rx.changed().await {
                Ok(_) => {
                    let p = *progress_rx.borrow();
                    pb_updater.set_position(p);

                    if p >= file_size {
                        pb_updater.finish();
                        return;
                    }
                }
                Err(_) => return, // Sender has dropped.
            }
        }
    });

    Ok(pb)
}

fn watch_for_ctrl_c(handle: DiskImportHandle, pb: ProgressBar) {
    tokio::spawn(async move {
        let mut force_shutdown = false;
        let mut handle = Some(handle);

        loop {
            signal::ctrl_c().await.expect("Failed to listen for CTRL+C");

            if force_shutdown {
                eprintln_nopipe!("Shutting down immediately.\nSee https://docs.oxide.computer/guides/troubleshooting#_cant_delete_disk_after_canceled_image_import for instructions on removing the disk.");

                // Match exit code used by Bash on Unix, 128 + 2 (SIGINT).
                process::exit(130);
            }
            force_shutdown = true;

            pb.finish_and_clear();
            pb.println("Cleaning up disk. Press CTRL+C again to exit immediately.");

            if let Some(handle) = handle.take() {
                handle.cancel();
            }
        }
    });
}
