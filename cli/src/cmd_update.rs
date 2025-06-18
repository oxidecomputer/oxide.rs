// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use std::{path::PathBuf, process::exit};

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use clap::Parser;
use futures::StreamExt;
use oxide::{Client, ClientExperimentalExt};
use tokio::{fs::File, sync::watch};
use tokio_util::io::ReaderStream;

use crate::{generated_cli::CliConfig, util::start_progress_bar, AuthenticatedCmd, OxideOverride};

#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
pub struct CmdUpload {
    /// Path to the TUF repository
    #[clap(short, long)]
    path: PathBuf,
}

#[async_trait]
impl AuthenticatedCmd for CmdUpload {
    async fn run(&self, client: &Client) -> Result<()> {
        let file_name = self.path.file_name().unwrap().to_str().unwrap();

        let file = File::open(&self.path).await?;

        let len = file.metadata().await?.len();

        let file_stream = ReaderStream::new(file).map(|res| res.map(Bytes::from));

        let (progress_tx, progress_rx) = watch::channel(0);

        let xxx = start_progress_bar(progress_rx, len, "xxx");
        exit(1);

        let body = reqwest::Body::wrap_stream(file_stream);

        let response = client
            .system_update_put_repository()
            .file_name(file_name)
            .body(body)
            .send()
            .await;

        // TODO probably we want to figure out the output modality for all
        // commands, but let's figure that out once we have multiple output
        // modes.
        let out = OxideOverride::default();

        match response {
            Ok(r) => {
                out.success_item(&r);
                Ok(())
            }
            Err(err) => {
                out.error(&err);
                Err(anyhow::Error::new(err))
            }
        }
    }

    fn maybe_long() -> bool {
        true
    }
}
