// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use crate::shutdown::{Cleanup, GracefulShutdown};
use crate::{eprintln_nopipe, println_nopipe};

use anyhow::{bail, Result};
use async_trait::async_trait;
use base64::Engine;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use oxide::types::BlockSize;
use oxide::types::ByteCount;
use oxide::types::DiskCreate;
use oxide::types::DiskSource;
use oxide::types::DiskState;
use oxide::types::FinalizeDisk;
use oxide::types::ImageCreate;
use oxide::types::ImageSource;
use oxide::types::ImportBlocksBulkWrite;
use oxide::types::Name;
use oxide::types::NameOrId;
use oxide::Client;
use oxide::ClientDisksExt;
use oxide::ClientImagesExt;
use oxide::ClientSnapshotsExt;
use oxide::ResponseValue;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

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

/// Return a disk size that Nexus will accept for the path and size arguments
fn get_disk_size(path: &PathBuf, size: Option<u64>) -> Result<u64> {
    const ONE_GB: u64 = 1024 * 1024 * 1024;

    let disk_size = if let Some(size) = size {
        size
    } else {
        std::fs::metadata(path)?.len()
    };

    // Nexus' disk size minimum is 1 GB, and Nexus only supports disks whose
    // size is a multiple of 1 GB
    let disk_size = if disk_size % ONE_GB != 0 {
        let rounded_down_gb: u64 = disk_size - disk_size % ONE_GB;
        assert_eq!(rounded_down_gb % ONE_GB, 0);
        rounded_down_gb + ONE_GB
    } else {
        disk_size
    };

    Ok(disk_size)
}

fn err_if_object_exists<T>(
    error_msg: String,
    send_response: Result<ResponseValue<T>, oxide::Error<oxide::types::Error>>,
) -> Result<()> {
    match send_response {
        Ok(_) => {
            bail!(error_msg);
        }

        Err(e) => {
            match &e {
                // Match against 404
                oxide::Error::ErrorResponse(response_value) => {
                    if response_value.status() == 404 {
                        // ok
                    } else {
                        bail!(e);
                    }
                }

                // Bail on any other error
                _ => bail!(e),
            }
        }
    }
    Ok(())
}

// Upload to Nexus in 512k byte chunks
const CHUNK_SIZE: u64 = 512 * 1024;

impl CmdDiskImport {
    async fn unwind_disk_delete(&self, client: &Client) -> Result<()> {
        let response = client
            .disk_delete()
            .project(&self.project)
            .disk(self.disk.clone())
            .send()
            .await;

        if let Err(e) = response {
            eprintln_nopipe!(
                "trying to unwind, deleting {:?} failed with {:?}",
                self.disk,
                e
            );
            return Err(e.into());
        }

        Ok(())
    }

    async fn unwind_disk_finalize(&self, client: &Client) -> Result<()> {
        let response = client
            .disk_finalize_import()
            .project(&self.project)
            .disk(self.disk.clone())
            .send()
            .await;

        // If this fails, then the disk will remain in state "import-ready"
        if let Err(e) = response {
            eprintln_nopipe!(
                "trying to unwind, finalizing {:?} failed with {:?}",
                self.disk,
                e
            );
            return Err(e.into());
        }

        Ok(())
    }

    async fn unwind_disk_bulk_write_stop(&self, client: &Client) -> Result<()> {
        let response = client
            .disk_bulk_write_import_stop()
            .project(&self.project)
            .disk(self.disk.clone())
            .send()
            .await;

        // If this fails, then the disk will remain in state
        // "importing-from-bulk-writes"
        if let Err(e) = response {
            eprintln_nopipe!(
                "trying to unwind, stopping the bulk write process for {:?} failed with {:?}",
                self.disk,
                e
            );
            return Err(e.into());
        }

        Ok(())
    }

    async fn get_disk_state(&self, client: &Client) -> Result<DiskState> {
        let response = client
            .disk_view()
            .project(&self.project)
            .disk(NameOrId::Name(self.disk.clone()))
            .send()
            .await?;

        Ok(response.into_inner().state)
    }

    // A shutdown may race with the newly created disk transitioning from Created
    // to ImportReady state. Wait until the disk has moved past Created before
    // attempting tear it down.
    async fn wait_for_disk(&self, client: &Client) -> Result<DiskState> {
        const RETRY_CT: usize = 10;
        const RETRY_DELAY: Duration = Duration::from_millis(500);

        let mut disk_state = self.get_disk_state(client).await?;

        for _ in 0..RETRY_CT {
            if !matches!(disk_state, DiskState::Creating) {
                return Ok(disk_state);
            }

            time::sleep(RETRY_DELAY).await;
            disk_state = self.get_disk_state(client).await?;
        }

        bail!("disk remained in Creating state for more than 5 seconds")
    }

    async fn remove_failed_disk(&self, client: &Client) -> Result<()> {
        let disk_state = self.wait_for_disk(client).await?;

        // TODO hook this into the progress bar.
        if matches!(disk_state, DiskState::ImportingFromBulkWrites) {
            self.unwind_disk_bulk_write_stop(client).await?;
        }

        self.unwind_disk_finalize(client).await?;
        self.unwind_disk_delete(client).await
    }
}

#[async_trait]
impl crate::AuthenticatedCmd for CmdDiskImport {
    fn is_subtree() -> bool {
        false
    }
    async fn run(&self, client: &Client) -> Result<()> {
        if !Path::new(&self.path).exists() {
            bail!("path {} does not exist", self.path.to_string_lossy());
        }

        // validate that objects don't exist already
        err_if_object_exists(
            format!("disk {:?} exists already", &self.disk),
            client
                .disk_view()
                .project(&self.project)
                .disk(NameOrId::Name(self.disk.clone()))
                .send()
                .await,
        )?;

        // snapshot
        if let Some(snapshot_name) = &self.snapshot {
            err_if_object_exists(
                format!("snapshot {:?} exists already", &snapshot_name),
                client
                    .snapshot_view()
                    .project(&self.project)
                    .snapshot(NameOrId::Name(snapshot_name.clone()))
                    .send()
                    .await,
            )?;
        }

        // image
        if let Some(image_name) = &self.image {
            err_if_object_exists(
                format!("image {:?} exists already", &image_name),
                client
                    .image_view()
                    .project(&self.project)
                    .image(NameOrId::Name(image_name.clone()))
                    .send()
                    .await,
            )?;
        }

        let file_size = std::fs::metadata(&self.path)?.len();
        let disk_size = get_disk_size(&self.path, self.disk_size.as_ref().map(|x| **x))?;

        let disk_block_size = match &self.disk_block_size {
            Some(v) => v.clone(),
            None => BlockSize::try_from(512).unwrap(),
        };

        if (file_size % *disk_block_size as u64) != 0 {
            bail!(
                "file size {} is not divisible by block size{}!",
                file_size,
                *disk_block_size
            );
        }

        // Use 8 upload tasks - this evenly divides all block sizes, and we know
        // that the file size is evenly divided by the selected block size due
        // to the above check.
        const UPLOAD_TASKS: usize = 8;

        let mut shutdown = GracefulShutdown::new(
            "Cleaning up partially imported disk",
            "See https://docs.oxide.computer/guides/troubleshooting#_cant_delete_disk_after_canceled_image_import for instructions on removing the disk",
            );

        // Create the disk in state "importing blocks"
        shutdown
            .run_with_cleanup(
                client
                    .disk_create()
                    .project(&self.project)
                    .body(DiskCreate {
                        name: self.disk.clone(),
                        description: self.description.clone(),
                        disk_source: DiskSource::ImportingBlocks {
                            block_size: disk_block_size.clone(),
                        },
                        size: disk_size.into(),
                    })
                    .send(),
                Cleanup {
                    future: self.remove_failed_disk(client),
                    context: "creating new disk failed",
                },
            )
            .await?;

        // Start the bulk write process
        shutdown
            .run_with_cleanup(
                client
                    .disk_bulk_write_import_start()
                    .project(&self.project)
                    .disk(self.disk.clone())
                    .send(),
                Cleanup {
                    future: self.remove_failed_disk(client),
                    context: "starting the bulk write process failed",
                },
            )
            .await?;

        // Create one tokio task for each thread that will upload file chunks
        let mut handles: Vec<tokio::task::JoinHandle<Result<()>>> =
            Vec::with_capacity(UPLOAD_TASKS);
        let mut senders = Vec::with_capacity(UPLOAD_TASKS);

        let pb = Arc::new(ProgressBar::new(file_size));
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.green}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        );
        pb.set_position(0);

        for _i in 0..UPLOAD_TASKS {
            let (tx, mut rx) = mpsc::channel(100);

            let client = client.clone();
            let disk_name = self.disk.clone();
            let project = self.project.clone();

            let pb = pb.clone();

            handles.push(tokio::spawn(async move {
                while let Some((offset, base64_encoded_data, data_len)) = rx.recv().await {
                    client
                        .disk_bulk_write_import()
                        .disk(disk_name.clone())
                        .project(project.clone())
                        .body(ImportBlocksBulkWrite {
                            offset,
                            base64_encoded_data,
                        })
                        .send()
                        .await?;

                    pb.inc(data_len as u64);
                }

                Ok(())
            }));

            senders.push(tx);
        }

        // Read chunks from the file in the file system and send them to the
        // upload threads.
        let mut file = File::open(&self.path)?;
        let mut i = 0;
        let mut offset = 0;

        let read_result: Result<()> = loop {
            let mut chunk = Vec::with_capacity(CHUNK_SIZE as usize);

            let n = match file.by_ref().take(CHUNK_SIZE).read_to_end(&mut chunk) {
                Ok(n) => n,
                Err(e) => {
                    eprintln_nopipe!(
                        "reading from {} failed with {:?}",
                        self.path.to_string_lossy(),
                        e,
                    );
                    break Err(e.into());
                }
            };

            if n == 0 {
                break Ok(());
            }

            // If the chunk we just read is all zeroes, don't POST it.
            if !chunk.iter().all(|x| *x == 0) {
                let encoded = base64::engine::general_purpose::STANDARD.encode(&chunk[0..n]);

                if let Err(e) = senders[i % UPLOAD_TASKS].send((offset, encoded, n)).await {
                    eprintln_nopipe!("sending chunk to thread failed with {:?}", e);
                    break Err(e.into());
                }
            } else {
                // Bump the progress bar here to make it consistent
                pb.inc(n as u64);
            }

            offset += CHUNK_SIZE;
            i += 1;

            if shutdown.shutdown_requested()? {
                self.remove_failed_disk(client).await?;
                bail!("user canceled request");
            }
        };

        for tx in senders {
            drop(tx);
        }

        if let Err(e) = read_result {
            // some part of reading from the disk and sending to the upload
            // threads failed, so unwind. stop the bulk write process, finalize
            // the disk, then delete it.
            self.remove_failed_disk(client).await?;

            // return the original error
            return Err(e);
        }

        let mut results = Vec::with_capacity(handles.len());
        for handle in handles {
            let result = handle.await?;
            results.push(result);
        }

        if results.iter().any(|x| x.is_err()) {
            // If any of the upload threads returned an error, unwind the disk.
            eprintln_nopipe!("one of the upload threads failed");
            self.remove_failed_disk(client).await?;
            bail!("one of the upload threads failed");
        }

        // wait for upload threads to complete, then finish the progress bar
        pb.finish();

        // Stop the bulk write process
        shutdown
            .run_with_cleanup(
                client
                    .disk_bulk_write_import_stop()
                    .project(&self.project)
                    .disk(self.disk.clone())
                    .send(),
                Cleanup {
                    // Attempt to unwind the disk, although it will probably fail - the
                    // first step is to stop the bulk write process!
                    future: self.remove_failed_disk(client),
                    context: "stopping the bulk write process failed",
                },
            )
            .await?;

        shutdown
            .run_with_cleanup(
                client
                    .disk_finalize_import()
                    .project(&self.project)
                    .disk(self.disk.clone())
                    .body(FinalizeDisk {
                        snapshot_name: self.snapshot.clone(),
                    })
                    .send(),
                Cleanup {
                    // Attempt to unwind the disk, although it will probably fail.
                    // first step is to finalize the disk!
                    future: self.remove_failed_disk(client),
                    context: "finalizing the disk failed",
                },
            )
            .await?;

        // optionally, make an image out of that snapshot
        if let Some(image_name) = &self.image {
            let snapshot_name = self.snapshot.as_ref().unwrap();
            let image_description = self.image_description.as_ref().unwrap();
            let image_os = self.image_os.as_ref().unwrap();
            let image_version = self.image_version.as_ref().unwrap();

            // at this point, unwinding is not as important as before. the user
            // has uploaded their file to a disk and finalized that disk, making
            // a snapshot out of it. if this step fails, they can always
            // manually make an image out of the snapshot later and be sure that
            // the snapshot's contents are the same.
            let snapshot = client
                .snapshot_view()
                .project(&self.project)
                .snapshot(NameOrId::Name(snapshot_name.clone()))
                .send()
                .await?;

            client
                .image_create()
                .project(&self.project)
                .body(ImageCreate {
                    name: image_name.clone(),
                    description: image_description.clone(),
                    os: image_os.clone(),
                    version: image_version.clone(),
                    source: ImageSource::Snapshot(snapshot.id),
                })
                .send()
                .await?;
        }

        println_nopipe!("done!");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_disk_size() {
        let path = PathBuf::from("not relevant because we're supplying a size");

        // test rounding up
        assert_eq!(get_disk_size(&path, Some(1)).unwrap(), 1024 * 1024 * 1024,);

        assert_eq!(
            get_disk_size(&path, Some(1024 * 1024 * 1024 - 1)).unwrap(),
            1024 * 1024 * 1024,
        );

        // test even multiples of GB
        assert_eq!(
            get_disk_size(&path, Some(1024 * 1024 * 1024)).unwrap(),
            1024 * 1024 * 1024,
        );

        assert_eq!(
            get_disk_size(&path, Some(2 * 1024 * 1024 * 1024)).unwrap(),
            2 * 1024 * 1024 * 1024,
        );

        // test non-even multiples of GB
        assert_eq!(
            get_disk_size(&path, Some(2 * 1024 * 1024 * 1024 + 1)).unwrap(),
            3 * 1024 * 1024 * 1024,
        );

        assert_eq!(
            get_disk_size(&path, Some(3 * 1024 * 1024 * 1024 - 1)).unwrap(),
            3 * 1024 * 1024 * 1024,
        );
    }
}
