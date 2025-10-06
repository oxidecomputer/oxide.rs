// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use std::time::Duration;

use chrono::DateTime;
use chrono::Utc;
use dropshot::{
    endpoint, ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, HttpError,
    HttpResponseOk, Query, RequestContext, ServerBuilder, StreamingBody,
};
use futures::{pin_mut, StreamExt};
use oxide::types::{TufRepo, TufRepoUpload, TufRepoUploadStatus};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Instant};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct UpdatesPutRepositoryParams {
    pub file_name: String,
}

const MIB: usize = 1024 * 1024;
const GIB: usize = 1024 * MIB;
const PUT_UPDATE_REPOSITORY_MAX_BYTES: usize = 4 * GIB;

#[endpoint {
    method = PUT,
    path = "/v1/system/update/repository",
    request_body_max_bytes = PUT_UPDATE_REPOSITORY_MAX_BYTES,
}]
async fn system_update_put_repository(
    _rqctx: RequestContext<()>,
    query: Query<UpdatesPutRepositoryParams>,
    body: StreamingBody,
) -> Result<HttpResponseOk<TufRepoUpload>, HttpError> {
    const BYTES_PER_SEC: usize = 200 * MIB;

    let start = Instant::now();
    let mut bytes_read: usize = 0;
    let stream = body.into_stream();
    pin_mut!(stream);

    while let Some(result) = stream.next().await {
        let bytes = result?;
        bytes_read += bytes.len();

        let target_millis = 1_000 * bytes_read / BYTES_PER_SEC;

        let target_elapsed = Duration::from_millis(target_millis as u64);

        let actual_elapsed = start.elapsed();
        if let Some(ahead) = target_elapsed.checked_sub(actual_elapsed) {
            if ahead >= Duration::from_millis(1) {
                sleep(ahead).await;
            }
        }
    }

    Ok(HttpResponseOk(TufRepoUpload {
        repo: TufRepo {
            file_name: query.into_inner().file_name,
            hash: String::new(),
            system_version: "1.2.3".parse().unwrap(),
            time_created: DateTime::<Utc>::from_timestamp(0x8000_0000, 0).unwrap(),
        },
        status: TufRepoUploadStatus::Inserted,
    }))
}

#[tokio::main]
async fn main() {
    let mut api = ApiDescription::new();

    api.register(system_update_put_repository).unwrap();

    let config_logging = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    };
    let log = config_logging
        .to_logger("example-basic")
        .map_err(|error| format!("failed to create logger: {}", error))
        .unwrap();

    let server = ServerBuilder::new(api, (), log).config(ConfigDropshot {
        bind_address: "127.0.0.1:8080".parse().unwrap(),
        ..Default::default()
    });

    let _ = server.start().unwrap().await;
}
