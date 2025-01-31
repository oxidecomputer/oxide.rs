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
    use std::sync::atomic::{AtomicBool, AtomicU64};
    use std::sync::Arc;
    use tokio::sync::{oneshot, watch};

    /// Builder for [`ClientExtraDiskExt::disk_import`]
    ///
    /// [`ClientExtraDiskExt::disk_import`]: super::ClientExtraDiskExt::disk_import
    pub struct DiskImport<'a> {
        client: &'a Client,
        project: Result<NameOrId, String>,
        description: Result<String, String>,
        upload_thread_ct: Result<usize, String>,
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
                upload_thread_ct: Err("upload_thread_ct was not initialized".to_string()),
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

        pub fn upload_thread_ct<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<usize>,
        {
            self.upload_thread_ct = value
                .try_into()
                .map_err(|_| "conversion to `usize` for upload_thread_ct failed".to_string());
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
            let upload_thread_ct = builder.upload_thread_ct.map_err(Error::InvalidRequest)?;
            let disk = builder.disk.map_err(Error::InvalidRequest)?;
            let disk_info = builder.disk_info.map_err(Error::InvalidRequest)?;
            let image_info = builder.image_info.map_err(Error::InvalidRequest)?;

            let upload_progress = Arc::new(AtomicU64::new(0));

            Ok(Self {
                client: builder.client,
                project,
                description,
                upload_thread_ct,
                disk,
                disk_info,
                image_info,
                upload_progress,
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
    use std::fs::{self, File};
    use std::io::{self, Read};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::{mpsc, oneshot, watch};

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
        pub upload_thread_ct: usize,
        pub disk: Name,
        pub disk_info: DiskInfo,
        pub image_info: Option<ImageInfo>,
        pub upload_progress: Arc<AtomicU64>,
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

            // Create one tokio task for each thread that will upload file chunks
            let mut handles: Vec<tokio::task::JoinHandle<Result<(), DiskImportError>>> =
                Vec::with_capacity(self.upload_thread_ct);
            let mut senders = Vec::with_capacity(self.upload_thread_ct);

            for _ in 0..self.upload_thread_ct {
                let (tx, mut rx) = mpsc::channel(100);

                let client = self.client.clone();
                let project = self.project.clone();
                let disk = self.disk.clone();
                let upload_progress = self.upload_progress.clone();
                let progress_tx = self.progress_tx.clone();

                handles.push(tokio::spawn(async move {
                    while let Some((offset, base64_encoded_data, data_len)) = rx.recv().await {
                        client
                            .disk_bulk_write_import()
                            .disk(disk.clone())
                            .project(project.clone())
                            .body(ImportBlocksBulkWrite {
                                offset,
                                base64_encoded_data,
                            })
                            .send()
                            .await?;

                        upload_progress.fetch_add(data_len, Ordering::Relaxed);
                        progress_tx.send_replace(upload_progress.load(Ordering::Relaxed));
                    }

                    Ok(())
                }));

                senders.push(tx);
            }

            // Read chunks from the file in the file system and send them to the
            // upload threads.
            let mut file = File::open(&self.disk_info.file_path)?;
            let mut i = 0;
            let mut offset = 0;

            let read_result: Result<(), DiskImportError> = loop {
                let mut chunk = Vec::with_capacity(CHUNK_SIZE as usize);

                let n = match file.by_ref().take(CHUNK_SIZE).read_to_end(&mut chunk) {
                    Ok(n) => n,
                    Err(e) => {
                        break Err(DiskImportError::context(
                            format!("reading from {} failed", self.disk_info.file_path.display()),
                            e,
                        ));
                    }
                };

                if n == 0 {
                    break Ok(());
                }

                // If the chunk we just read is all zeroes, don't POST it.
                if !chunk.iter().all(|x| *x == 0) {
                    let encoded = base64::engine::general_purpose::STANDARD.encode(&chunk[0..n]);

                    if senders[i % self.upload_thread_ct]
                        .send((offset, encoded, n as u64))
                        .await
                        .is_err()
                    {
                        // Failure to send indicates that the upload task exited early
                        // due to an error on its end. We will return that error below.
                        break Ok(());
                    }
                } else {
                    // Bump the progress bar here to make it consistent
                    self.upload_progress.fetch_add(n as u64, Ordering::Relaxed);
                    self.progress_tx
                        .send_replace(self.upload_progress.load(Ordering::Relaxed));
                }

                offset += CHUNK_SIZE;
                i += 1;
            };

            for tx in senders {
                drop(tx);
            }

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

            match errors.len() {
                1 => {
                    return Err(DiskImportError::context(
                        "Error while uploading the disk image",
                        errors.remove(0),
                    ))
                }
                2.. => {
                    let mut msg = String::from("Errors while uploading the disk image:");
                    for err in errors {
                        msg += &format!("\n * {err}");
                    }
                    return Err(DiskImportError::Other(msg.into()));
                }
                0 => {}
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

            let file_size = fs::metadata(&file_path)?.len();
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
