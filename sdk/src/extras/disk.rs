// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use super::ClientExtraDiskExt;
use crate::Client;

impl ClientExtraDiskExt for Client {
    fn disk_import(&self) -> builder::DiskImport {
        builder::DiskImport::new(self)
    }
}

pub mod builder {
    use crate::extras::disk::types::{DiskImportError, DiskImportHandle, DiskInfo, ImageInfo};
    use crate::types::{Name, NameOrId};
    use crate::{Client, Error};

    use std::future::Future;
    use std::num::NonZeroUsize;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::{oneshot, watch};

    /// Builder for [`ClientExtraDiskExt::disk_import`]
    ///
    /// [`ClientExtraDiskExt::disk_import`]: super::ClientExtraDiskExt::disk_import
    pub struct DiskImport<'a> {
        client: &'a Client,
        project: Result<NameOrId, String>,
        description: Result<String, String>,
        upload_task_ct: Result<NonZeroUsize, String>,
        disk: Result<Name, String>,
        disk_info: Result<DiskInfo, String>,
        image_info: Result<Option<ImageInfo>, String>,
    }

    impl<'a> DiskImport<'a> {
        pub fn new(client: &'a Client) -> Self {
            Self {
                client,
                project: Err("project was not initialized".to_string()),
                description: Err("description was not initialized".to_string()),
                upload_task_ct: Err("upload_task_ct was not initialized".to_string()),
                disk: Err("disk was not initialized".to_string()),
                disk_info: Err("disk_info was not initialized".to_string()),
                image_info: Ok(None),
            }
        }

        pub fn project<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<NameOrId>,
        {
            self.project = value
                .try_into()
                .map_err(|_| "conversion to `NameOrId` for project failed".to_string());
            self
        }

        pub fn description<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.description = value
                .try_into()
                .map_err(|_| "conversion to `String` for description failed".to_string());
            self
        }

        pub fn upload_task_ct<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<NonZeroUsize>,
        {
            self.upload_task_ct = value.try_into().map_err(|_| {
                "conversion to `non-zero usize` for upload_task_ct failed".to_string()
            });
            self
        }

        pub fn disk<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<Name>,
        {
            self.disk = value
                .try_into()
                .map_err(|_| "conversion to `Name` for disk failed".to_string());
            self
        }

        pub fn disk_info(mut self, value: DiskInfo) -> Self {
            self.disk_info = Ok(value);
            self
        }

        pub fn image_info(mut self, value: ImageInfo) -> Self {
            self.image_info = Ok(Some(value));
            self
        }

        /// Return a `Future` for the disk creation.
        pub fn execute(
            self,
        ) -> Result<
            impl Future<Output = Result<(), DiskImportError>> + 'a,
            Error<crate::types::Error>,
        > {
            let (progress_tx, _progress_rx) = watch::channel(0);
            let importer = super::types::DiskImport::try_from((self, progress_tx))?;

            Ok(importer.run())
        }

        /// Return a `Future` for the disk creation and a `DiskImportHandle` which
        /// can be used to track upload progress and cancel the import.
        pub fn execute_with_control(
            self,
        ) -> Result<
            (
                impl Future<Output = Result<(), DiskImportError>> + 'a,
                DiskImportHandle,
            ),
            Error<crate::types::Error>,
        > {
            let (progress_tx, progress_rx) = watch::channel(0);
            let (cancel_tx, cancel_rx) = oneshot::channel();

            let importer = super::types::DiskImport::try_from((self, progress_tx))?;
            let handle = DiskImportHandle {
                progress_rx,
                cancel_tx,
            };

            Ok((importer.run_with_cancel(cancel_rx), handle))
        }
    }

    impl<'a> TryFrom<(DiskImport<'a>, watch::Sender<u64>)> for super::types::DiskImport<'a> {
        type Error = Error<crate::types::Error>;

        fn try_from(input: (DiskImport<'a>, watch::Sender<u64>)) -> Result<Self, Self::Error> {
            let (builder, progress_tx) = input;

            let project = builder.project.map_err(Error::InvalidRequest)?;
            let description = builder.description.map_err(Error::InvalidRequest)?;
            let upload_task_ct = builder.upload_task_ct.map_err(Error::InvalidRequest)?;
            let disk = builder.disk.map_err(Error::InvalidRequest)?;
            let disk_info = builder.disk_info.map_err(Error::InvalidRequest)?;
            let image_info = builder.image_info.map_err(Error::InvalidRequest)?;

            Ok(Self {
                client: builder.client,
                project,
                description,
                upload_task_ct,
                disk,
                disk_info,
                image_info,
                progress_tx,
                cleanup_started: AtomicBool::new(false),
            })
        }
    }
}

pub mod types {
    use crate::types::{
        self, BlockSize, ByteCount, DiskCreate, DiskSource, DiskState, FinalizeDisk, ImageCreate,
        ImageSource, ImportBlocksBulkWrite, Name, NameOrId,
    };
    use crate::{
        Client, ClientDisksExt, ClientImagesExt, ClientSnapshotsExt, Error, ResponseValue,
    };

    use base64::Engine;
    use std::collections::HashSet;
    use std::num::NonZeroUsize;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    use tokio::fs::File;
    use tokio::io::{self, AsyncReadExt};
    use tokio::sync::{oneshot, watch};

    // Upload to Nexus in 512k byte chunks
    const CHUNK_SIZE: u64 = 512 * 1024;

    #[derive(thiserror::Error, Debug)]
    pub enum DiskImportError {
        #[error("{err}")]
        Wrapped {
            err: Box<dyn std::error::Error + Send + Sync>,
            source: Box<DiskImportError>,
        },
        #[error("{0}")]
        Other(Box<dyn std::error::Error + Send + Sync>),
        #[error(transparent)]
        Api(#[from] crate::Error<types::Error>),
        #[error(transparent)]
        Conversion(#[from] types::error::ConversionError),
        #[error(transparent)]
        Io(#[from] io::Error),
    }

    impl DiskImportError {
        pub fn context(
            err: impl Into<Box<dyn std::error::Error + Send + Sync>>,
            source: impl Into<DiskImportError>,
        ) -> Self {
            DiskImportError::Wrapped {
                err: err.into(),
                source: Box::new(source.into()),
            }
        }

        pub fn other(err: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
            DiskImportError::Other(err.into())
        }
    }

    /// Provides access to upload progress and cancellation for
    /// a disk import.
    #[derive(Debug)]
    pub struct DiskImportHandle {
        pub(super) progress_rx: watch::Receiver<u64>,
        pub(super) cancel_tx: oneshot::Sender<()>,
    }

    impl DiskImportHandle {
        /// Returns `tokio::sync::watch::Receiver` that reports the current number of bytes uploaded.
        /// For use with progress reports.
        pub fn progress(&self) -> watch::Receiver<u64> {
            self.progress_rx.clone()
        }

        /// Attempt to cancel the disk upload. If the upload has completed and the disk is
        /// finalized then this will not remove the disk.
        pub fn cancel(self) {
            // Err indicates DiskImport has dropped and request has completed, nothing to do.
            let _ = self.cancel_tx.send(());
        }
    }

    #[derive(Debug)]
    pub(super) struct DiskImport<'a> {
        pub client: &'a Client,
        pub project: NameOrId,
        pub description: String,
        pub upload_task_ct: NonZeroUsize,
        pub disk: Name,
        pub disk_info: DiskInfo,
        pub image_info: Option<ImageInfo>,
        pub progress_tx: watch::Sender<u64>,
        pub cleanup_started: AtomicBool,
    }

    impl<'a> DiskImport<'a> {
        pub async fn run_with_cancel(
            self,
            cancel_rx: oneshot::Receiver<()>,
        ) -> Result<(), DiskImportError> {
            let result = tokio::select! {
                biased; // Prioritize cancellation.
                _ = cancel_rx => {
                    Err(DiskImportError::other("Disk import canceled"))
                }
                result = self.do_disk_import() => result,
            };

            if let Err(e) = result {
                if let Err(cleanup_err) = self.cleanup().await {
                    return Err(DiskImportError::Wrapped {
                        err: cleanup_err.into(),
                        source: e.into(),
                    });
                }
                return Err(e);
            }

            Ok(())
        }

        pub async fn run(self) -> Result<(), DiskImportError> {
            if let Err(e) = self.do_disk_import().await {
                if let Err(cleanup_err) = self.cleanup().await {
                    return Err(DiskImportError::Wrapped {
                        err: cleanup_err.into(),
                        source: e.into(),
                    });
                }
                return Err(e);
            }

            Ok(())
        }

        async fn do_disk_import(&self) -> Result<(), DiskImportError> {
            self.check_for_existing_disk().await?;

            // Create the disk in state "importing blocks"
            self.client
                .disk_create()
                .project(self.project.clone())
                .body(DiskCreate {
                    name: self.disk.clone(),
                    description: self.description.clone(),
                    disk_source: DiskSource::ImportingBlocks {
                        block_size: self.disk_info.disk_block_size.clone(),
                    },
                    size: self.disk_info.disk_size.clone(),
                })
                .send()
                .await
                .map_err(|e| DiskImportError::context("creating the disk failed", e))?;

            // Start the bulk write process
            self.client
                .disk_bulk_write_import_start()
                .project(self.project.clone())
                .disk(self.disk.clone())
                .send()
                .await
                .map_err(|e| {
                    DiskImportError::context("starting the build write process failed", e)
                })?;

            let mut handles: Vec<tokio::task::JoinHandle<Result<(), DiskImportError>>> =
                Vec::with_capacity(self.upload_task_ct.get());
            let (tx, rx) = flume::bounded(64);
            let (failed_tx, failed_rx) = flume::bounded(self.upload_task_ct.get());
            let (resubmit_tx, resubmit_rx) = flume::bounded(self.upload_task_ct.get());

            for _ in 0..self.upload_task_ct.get() {
                let mut worker = UploadWorker {
                    client: self.client.clone(),
                    disk: self.disk.clone(),
                    project: self.project.clone(),
                    progress_tx: self.progress_tx.clone(),
                };

                let rx = rx.clone();
                let failed_tx = failed_tx.clone();
                let resubmit_rx = resubmit_rx.clone();

                handles.push(tokio::spawn(async move {
                    while let Ok(chunk) = rx.recv_async().await {
                        if let Err(e) = worker.upload_chunk(&chunk).await {
                            let _ = failed_tx.send_async(chunk).await;
                            return Err(e);
                        }
                    }

                    // Signal the main task that the worker has sent all chunks successfully and
                    // none remain to be sent.
                    drop(failed_tx);

                    while let Ok(chunk) = resubmit_rx.recv_async().await {
                        worker.upload_chunk(&chunk).await?;
                    }

                    Ok(())
                }));
            }
            // Only worker tasks use this sender.
            drop(failed_tx);

            let mut buf = Vec::with_capacity(CHUNK_SIZE as usize);
            let mut file = File::open(&self.disk_info.file_path).await?;
            let mut offset = 0;

            let read_result: Result<(), DiskImportError> = loop {
                let n = match (&mut file).take(CHUNK_SIZE).read_to_end(&mut buf).await {
                    Ok(n) => n,
                    Err(e) => {
                        return Err(DiskImportError::context(
                            format!("reading from {} failed", self.disk_info.file_path.display()),
                            e,
                        ));
                    }
                };

                if n == 0 {
                    break Ok(());
                }

                // If the chunk we just read is all zeroes, don't POST it.
                let data = &buf[..n];
                if !data.iter().all(|x| *x == 0) {
                    // Failure to send indicates that all upload tasks exited early
                    // due to errors on their end. We will return those errors below.
                    if tx
                        .send_async(Chunk {
                            offset,
                            data: data.to_vec(),
                        })
                        .await
                        .is_err()
                    {
                        break Ok(());
                    }
                } else {
                    // Bump the progress bar here to make it consistent.
                    self.progress_tx.send_modify(|offset| *offset += n as u64);
                }

                offset += n as u64;
                buf.clear();
            };
            drop(tx);

            // Resubmit any failed chunks back to the remaining workers to retry.
            while let Ok(failed_chunk) = failed_rx.recv_async().await {
                if resubmit_tx.send_async(failed_chunk).await.is_err() {
                    // All worker tasks have failed.
                    break;
                }
            }
            drop(resubmit_tx);

            let mut errors = Vec::new();
            if let Err(e) = read_result {
                errors.push(e);
            }
            for handle in handles {
                let result = handle.await.map_err(DiskImportError::other)?;
                if let Err(err) = result {
                    errors.push(err);
                }
            }

            // Only return an error if all worker tasks failed.
            if errors.len() == self.upload_task_ct.get() {
                // Dedupe error messages.
                let mut err_set = HashSet::new();
                for err in errors {
                    err_set.insert(format!("\n * {err}"));
                }

                let mut msg = match err_set.len() {
                    1 => String::from("Error while uploading the disk image:"),
                    2.. => String::from("Errors while uploading the disk image:"),
                    0 => unreachable!("error count was zero"),
                };

                for err in err_set {
                    msg += &err;
                }
                return Err(DiskImportError::Other(msg.into()));
            }

            // Stop the bulk write process
            self.client
                .disk_bulk_write_import_stop()
                .project(self.project.clone())
                .disk(self.disk.clone())
                .send()
                .await
                .map_err(|e| {
                    DiskImportError::context("stopping the bulk write process failed", e)
                })?;

            // Finalize the disk, optionally making a snapshot
            self.client
                .disk_finalize_import()
                .project(self.project.clone())
                .disk(self.disk.clone())
                .body(FinalizeDisk {
                    snapshot_name: self.image_info.as_ref().map(|ii| ii.snapshot.clone()),
                })
                .send()
                .await
                .map_err(|e| DiskImportError::context("finalizing the disk failed", e))?;

            if self.image_info.is_some() {
                self.create_image().await?;
            }

            Ok(())
        }

        async fn create_image(&self) -> Result<(), DiskImportError> {
            let Some(image_info) = &self.image_info else {
                return Err(DiskImportError::other("no snapshot provided"));
            };

            // At this point, unwinding is not as important as before. the user
            // has uploaded their file to a disk and finalized that disk, making
            // a snapshot out of it. if this step fails, they can always
            // manually make an image out of the snapshot later and be sure that
            // the snapshot's contents are the same.
            let snapshot_id = self
                .client
                .snapshot_view()
                .project(&self.project)
                .snapshot(image_info.snapshot.clone())
                .send()
                .await
                .map_err(|e| DiskImportError::context("failed to fetch snapshot", e))?;

            self.client
                .image_create()
                .project(&self.project)
                .body(ImageCreate {
                    name: image_info.image.clone(),
                    description: image_info.image_description.clone(),
                    os: image_info.image_os.clone(),
                    version: image_info.image_version.clone(),
                    source: ImageSource::Snapshot(snapshot_id.id),
                })
                .send()
                .await
                .map_err(|e| DiskImportError::context("failed to create image", e))?;

            Ok(())
        }

        async fn cleanup(&self) -> Result<(), DiskImportError> {
            if self
                .cleanup_started
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_err()
            {
                return Ok(());
            }

            let disk_state = match self.wait_for_disk().await {
                Ok(state) => state,
                Err(e) => match e {
                    // No disk, no problem.
                    Error::ErrorResponse(rv) if rv.status() == 404 => return Ok(()),
                    _ => Err(e)?,
                },
            };

            // Disk is not partially imported, no cleanup needed.
            if !matches!(
                disk_state,
                DiskState::ImportReady
                    | DiskState::ImportingFromBulkWrites
                    | DiskState::ImportingFromUrl
                    | DiskState::Finalizing
            ) {
                return Ok(());
            }

            if matches!(disk_state, DiskState::ImportingFromBulkWrites) {
                self.unwind_disk_bulk_write_stop().await?;
            }

            self.unwind_disk_finalize().await?;
            self.unwind_disk_delete().await
        }

        async fn check_for_existing_disk(&self) -> Result<(), DiskImportError> {
            err_if_object_exists(
                format!("disk \"{}\" exists already", &*self.disk),
                self.client
                    .disk_view()
                    .project(&self.project)
                    .disk(self.disk.clone())
                    .send()
                    .await,
            )?;

            if let Some(image_info) = &self.image_info {
                err_if_object_exists(
                    format!("snapshot \"{}\" exists already", &*image_info.snapshot),
                    self.client
                        .snapshot_view()
                        .project(&self.project)
                        .snapshot(image_info.snapshot.clone())
                        .send()
                        .await,
                )?;

                err_if_object_exists(
                    format!("image \"{}\" exists already", &*image_info.image),
                    self.client
                        .image_view()
                        .project(&self.project)
                        .image(image_info.image.clone())
                        .send()
                        .await,
                )?;
            }

            Ok(())
        }

        async fn get_disk_state(&self) -> Result<DiskState, Error<types::Error>> {
            let response = self
                .client
                .disk_view()
                .project(&self.project)
                .disk(self.disk.clone())
                .send()
                .await?;

            Ok(response.into_inner().state)
        }

        /// A shutdown may race with the newly created disk transitioning from Created
        /// to ImportReady state. Wait until the disk has moved past Created before
        /// attempting tear it down.
        async fn wait_for_disk(&self) -> Result<DiskState, Error<types::Error>> {
            const RETRY_CT: usize = 10;
            const RETRY_DELAY: Duration = Duration::from_millis(500);

            let mut disk_state = self.get_disk_state().await?;

            for _ in 0..RETRY_CT {
                if !matches!(disk_state, DiskState::Creating) {
                    return Ok(disk_state);
                }

                tokio::time::sleep(RETRY_DELAY).await;
                disk_state = self.get_disk_state().await?;
            }

            Err(Error::InvalidRequest(
                "disk remained in \"Creating\" state for more than 5 seconds".to_string(),
            ))
        }

        async fn unwind_disk_delete(&self) -> Result<(), DiskImportError> {
            self.client
                .disk_delete()
                .project(&self.project)
                .disk(self.disk.clone())
                .send()
                .await
                .map_err(|e| {
                    DiskImportError::context(
                        format!("trying to unwind, deleting \"{}\" failed", &*self.disk),
                        e,
                    )
                })?;

            Ok(())
        }

        async fn unwind_disk_finalize(&self) -> Result<(), DiskImportError> {
            self.client
                .disk_finalize_import()
                .project(&self.project)
                .disk(self.disk.clone())
                .send()
                .await
                .map_err(|e| {
                    DiskImportError::context(
                        format!("trying to unwind, finalizing \"{}\" failed", &*self.disk),
                        e,
                    )
                })?;

            Ok(())
        }

        async fn unwind_disk_bulk_write_stop(&self) -> Result<(), DiskImportError> {
            self.client
                .disk_bulk_write_import_stop()
                .project(&self.project)
                .disk(self.disk.clone())
                .send()
                .await
                .map_err(|e| {
                    DiskImportError::context(
                        format!(
                            "trying to unwind, stopping the bulk write process for \"{}\"",
                            &*self.disk
                        ),
                        e,
                    )
                })?;

            Ok(())
        }
    }

    /// The information required to create a `Disk`.
    #[derive(Clone, Debug)]
    pub struct DiskInfo {
        /// The path to the file
        pub file_path: PathBuf,

        /// The size of the file in bytes
        pub file_size: u64,

        /// The size of the disk in bytes
        pub disk_size: ByteCount,

        /// The block size of the disk
        pub disk_block_size: BlockSize,
    }

    impl DiskInfo {
        /// Determine the `DiskInfo` for an image file.
        pub fn calculate(
            file_path: PathBuf,
            requested_disk_size: Option<&ByteCount>,
            requested_disk_block_size: Option<&BlockSize>,
        ) -> Result<DiskInfo, DiskImportError> {
            if !Path::new(&file_path).exists() {
                return Err(DiskImportError::other(format!(
                    "path {} does not exist",
                    file_path.display()
                )));
            }

            let file_size = std::fs::metadata(&file_path)?.len();
            let disk_size = Self::get_disk_size(file_size, requested_disk_size.map(|x| **x)).into();

            let disk_block_size = match requested_disk_block_size {
                Some(v) => v.clone(),
                None => BlockSize::try_from(512)?,
            };

            if (file_size % *disk_block_size as u64) != 0 {
                return Err(DiskImportError::other(format!(
                    "file size {file_size} is not divisible by block size {}!",
                    *disk_block_size
                )));
            }

            Ok(DiskInfo {
                file_path,
                file_size,
                disk_size,
                disk_block_size,
            })
        }

        /// Return a disk size that Nexus will accept for the path and size arguments
        fn get_disk_size(file_size: u64, size: Option<u64>) -> u64 {
            const ONE_GB: u64 = 1024 * 1024 * 1024;

            let disk_size = if let Some(size) = size {
                size
            } else {
                file_size
            };

            // Nexus' disk size minimum is 1 GB, and Nexus only supports disks whose
            // size is a multiple of 1 GB
            if disk_size % ONE_GB != 0 {
                let rounded_down_gb: u64 = disk_size - disk_size % ONE_GB;
                assert_eq!(rounded_down_gb % ONE_GB, 0);
                rounded_down_gb + ONE_GB
            } else {
                disk_size
            }
        }
    }

    fn err_if_object_exists<T>(
        error_msg: String,
        send_response: Result<ResponseValue<T>, Error<types::Error>>,
    ) -> Result<(), crate::Error<crate::types::Error>> {
        match send_response {
            Ok(_) => {
                return Err(Error::InvalidRequest(error_msg));
            }

            Err(e) => {
                match &e {
                    // Match against 404
                    crate::Error::ErrorResponse(response_value) => {
                        if response_value.status() == 404 {
                            // ok
                        } else {
                            Err(e)?
                        }
                    }

                    // Bail on any other error
                    _ => Err(e)?,
                }
            }
        }
        Ok(())
    }

    /// The information required to create an `Image`.
    #[derive(Clone, Debug)]
    pub struct ImageInfo {
        /// The name of the snapshot
        pub snapshot: Name,

        /// The name of the image
        pub image: Name,

        /// Human-readable free-form text about the image
        pub image_description: String,

        /// The name of the image's operating system
        pub image_os: String,

        /// The version of the image's operating system
        pub image_version: String,
    }

    struct Chunk {
        offset: u64,
        data: Vec<u8>,
    }

    struct UploadWorker {
        client: Client,
        disk: Name,
        project: NameOrId,
        progress_tx: watch::Sender<u64>,
    }

    impl UploadWorker {
        async fn upload_chunk(&mut self, chunk: &Chunk) -> Result<(), DiskImportError> {
            let base64_encoded_data = base64::engine::general_purpose::STANDARD.encode(&chunk.data);
            self.client
                .disk_bulk_write_import()
                .disk(&*self.disk)
                .project(&self.project)
                .body(ImportBlocksBulkWrite {
                    offset: chunk.offset,
                    base64_encoded_data,
                })
                .send()
                .await?;

            self.progress_tx
                .send_modify(|offset| *offset += chunk.data.len() as u64);

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_disk_size() {
            let file_size = 1; //"not relevant because we're supplying a size.

            // test rounding up
            assert_eq!(
                DiskInfo::get_disk_size(file_size, Some(1)),
                1024 * 1024 * 1024,
            );

            assert_eq!(
                DiskInfo::get_disk_size(file_size, Some(1024 * 1024 * 1024 - 1)),
                1024 * 1024 * 1024,
            );

            // test even multiples of GB
            assert_eq!(
                DiskInfo::get_disk_size(file_size, Some(1024 * 1024 * 1024)),
                1024 * 1024 * 1024,
            );

            assert_eq!(
                DiskInfo::get_disk_size(file_size, Some(2 * 1024 * 1024 * 1024)),
                2 * 1024 * 1024 * 1024,
            );

            // test non-even multiples of GB
            assert_eq!(
                DiskInfo::get_disk_size(file_size, Some(2 * 1024 * 1024 * 1024 + 1)),
                3 * 1024 * 1024 * 1024,
            );

            assert_eq!(
                DiskInfo::get_disk_size(file_size, Some(3 * 1024 * 1024 * 1024 - 1)),
                3 * 1024 * 1024 * 1024,
            );
        }
    }
}
