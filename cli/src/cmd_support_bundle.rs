// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use anyhow::bail;
use anyhow::Context as _;
use anyhow::Result;
use async_trait::async_trait;
use bytes::Buf;
use bytes::Bytes;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use clap::Parser;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use oxide::Client;
use oxide::ClientExperimentalExt;
use std::io;
use std::num::NonZeroU64;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use support_bundle_viewer::BoxedFileAccessor;
use support_bundle_viewer::LocalFileAccess;
use support_bundle_viewer::SupportBundleAccessor;
use support_bundle_viewer::SupportBundleIndex;
use tokio::io::AsyncRead;
use tokio::io::AsyncWriteExt;
use tokio::io::ReadBuf;
use tokio::sync::watch;
use uuid::Uuid;

/// Downloads a support bundle
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
pub struct CmdDownload {
    /// ID of the bundle
    #[clap(long)]
    id: Uuid,

    /// Path where the bundle should be downloaded
    #[clap(short, long)]
    output: Utf8PathBuf,

    /// The size, in bytes, of each range request to use when downloading bundles
    #[clap(short, long, default_value_t = NonZeroU64::new(100 * (1 << 20)).unwrap())]
    chunk_size: NonZeroU64,
}

// Downloads a portion of a support bundle using range requests.
//
// "range" is in bytes, and is inclusive on both sides.
//
// Returns: The observed content length of the stream, and the stream.
async fn support_bundle_download_range(
    client: &Client,
    id: Uuid,
    range: (u64, u64),
) -> anyhow::Result<(
    u64,
    impl futures::Stream<Item = anyhow::Result<bytes::Bytes>>,
)> {
    let range_str = format!("bytes={}-{}", range.0, range.1);
    let response = client
        .support_bundle_download()
        .bundle_id(id)
        .range(&range_str)
        .send()
        .await
        .with_context(|| format!("downloading support bundle {}", id))?;

    let Some(len) = response.content_length() else {
        bail!("No content length observed when downloading bundle");
    };

    Ok((
        len,
        response
            .into_inner_stream()
            .map(|r| r.map_err(|err| anyhow::anyhow!(err))),
    ))
}

// Downloads all ranges of a support bundle, and combines them into a single
// stream.
//
// Starts the download at "start" bytes (inclusive) and continues up to "end"
// bytes (exclusive).
fn support_bundle_download_ranges(
    client: &Client,
    id: Uuid,
    chunk_size: NonZeroU64,
    start: u64,
    end: u64,
) -> impl futures::Stream<Item = anyhow::Result<bytes::Bytes>> + use<'_> {
    futures::stream::try_unfold(
        (start, start + chunk_size.get() - 1),
        move |range| async move {
            if end <= range.0 {
                return Ok(None);
            }

            let (actual_len, stream) = support_bundle_download_range(client, id, range).await?;
            let next_range = (range.0 + actual_len, range.1 + actual_len);
            Ok::<_, anyhow::Error>(Some((stream, next_range)))
        },
    )
    .try_flatten()
}

#[async_trait]
impl crate::AuthenticatedCmd for CmdDownload {
    async fn run(&self, client: &Client) -> Result<()> {
        let mut output = tokio::fs::File::create(&self.output)
            .await
            .with_context(|| format!("Cannot create output file: {}", self.output))?;

        let (progress_tx, progress_rx) = watch::channel(0);

        let total_length = client
            .support_bundle_head()
            .bundle_id(self.id)
            .send()
            .await?
            .content_length()
            .ok_or_else(|| anyhow::anyhow!("No content length"))?;

        start_progress_bar(progress_rx, total_length, self.id)?;

        let mut stream =
            support_bundle_download_ranges(client, self.id, self.chunk_size, 0, total_length);
        let mut stream = std::pin::pin!(stream);
        while let Some(data) = stream.next().await {
            match data {
                Err(err) => bail!(err),
                Ok(data) => {
                    output.write_all(&data).await?;
                    progress_tx.send_modify(|l| *l += data.len() as u64);
                }
            }
        }
        Ok(())
    }

    fn maybe_long() -> bool {
        true
    }
}

fn start_progress_bar(
    mut progress_rx: watch::Receiver<u64>,
    file_size: u64,
    bundle_id: Uuid,
) -> Result<ProgressBar> {
    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar().template(
        "[{elapsed_precise}] [{wide_bar:.green}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta}",
    )?);
    pb.set_position(0);
    pb.println(format!("Downloading bundle \"{bundle_id}\""));
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

/// Inspects a support bundle
///
/// Support bundles may be inspected before they are downloaded (via
/// smaller HTTP requests), or after the entire zip file has been
/// downloaded.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(args_conflicts_with_subcommands(true))]
pub struct CmdInspect {
    /// ID of the bundle to be inspected, if accessing via API
    #[clap(long)]
    id: Option<Uuid>,

    /// Path of the bundle to inspect, if downloaded
    #[clap(short, long)]
    path: Option<Utf8PathBuf>,
}

#[async_trait]
impl crate::AuthenticatedCmd for CmdInspect {
    async fn run(&self, client: &Client) -> Result<()> {
        let accessor: Box<dyn SupportBundleAccessor> = match (self.id, &self.path) {
            (None, Some(path)) => Box::new(LocalFileAccess::new(path)?),
            (Some(id), None) => Box::new(ApiAccess::new(client, id)),
            (None, None) => {
                bail!("Must specify at least one of --id or --path");
            }
            (Some(_), Some(_)) => {
                bail!("Cannot specify both --id and --path");
            }
        };

        support_bundle_viewer::run_dashboard(accessor).await
    }

    fn maybe_long() -> bool {
        true
    }
}

struct StreamedFile<'a> {
    client: &'a Client,
    id: Uuid,
    path: Utf8PathBuf,
    stream: Option<Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>> + Send>>>,
    buffer: Bytes,
}

impl<'a> StreamedFile<'a> {
    fn new(client: &'a Client, id: Uuid, path: Utf8PathBuf) -> Self {
        Self {
            client,
            id,
            path,
            stream: None,
            buffer: Bytes::new(),
        }
    }

    // NOTE: This is a distinct method from "new", because ideally some day we could
    // use range requests to stream out portions of the file.
    //
    // This means that we would potentially want to restart the stream with a different position.
    async fn start_stream(&mut self) -> anyhow::Result<()> {
        // TODO: Add range headers, for range requests? Though this
        // will require adding support to Progenitor + Nexus too.
        let stream = self
            .client
            .support_bundle_download_file()
            .bundle_id(self.id)
            .file(self.path.as_str())
            .send()
            .await
            .with_context(|| format!("downloading support bundle file {}: {}", self.id, self.path))?
            .into_inner_stream();

        self.stream = Some(Box::pin(stream));
        Ok(())
    }
}

impl AsyncRead for StreamedFile<'_> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        while self.buffer.is_empty() {
            match futures::ready!(self
                .stream
                .as_mut()
                .expect("Stream must be initialized before polling")
                .as_mut()
                .poll_next(cx))
            {
                Some(Ok(bytes)) => {
                    self.buffer = bytes;
                }
                Some(Err(e)) => {
                    return Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, e)));
                }
                None => return Poll::Ready(Ok(())), // EOF
            }
        }

        let to_copy = std::cmp::min(self.buffer.len(), buf.remaining());
        buf.put_slice(&self.buffer[..to_copy]);
        self.buffer.advance(to_copy);

        Poll::Ready(Ok(()))
    }
}

/// Access to a support bundle from the external API
struct ApiAccess<'a> {
    client: &'a Client,
    id: Uuid,
}

impl<'a> ApiAccess<'a> {
    fn new(client: &'a Client, id: Uuid) -> Self {
        Self { client, id }
    }
}

async fn utf8_stream_to_string(
    mut stream: impl futures::Stream<Item = reqwest::Result<bytes::Bytes>> + std::marker::Unpin,
) -> anyhow::Result<String> {
    let mut bytes = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        bytes.extend_from_slice(&chunk);
    }
    Ok(String::from_utf8(bytes)?)
}

#[async_trait]
impl<'c> SupportBundleAccessor for ApiAccess<'c> {
    async fn get_index(&self) -> anyhow::Result<SupportBundleIndex> {
        let stream = self
            .client
            .support_bundle_index()
            .bundle_id(self.id)
            .send()
            .await
            .with_context(|| format!("downloading support bundle index {}", self.id))?
            .into_inner_stream();
        let s = utf8_stream_to_string(stream).await?;

        Ok(SupportBundleIndex::new(&s))
    }

    async fn get_file<'a>(&mut self, path: &Utf8Path) -> anyhow::Result<BoxedFileAccessor<'a>>
    where
        'c: 'a,
    {
        let mut file = StreamedFile::new(self.client, self.id, path.to_path_buf());
        file.start_stream()
            .await
            .with_context(|| "failed to start stream in get_file")?;
        Ok(Box::new(file))
    }
}
