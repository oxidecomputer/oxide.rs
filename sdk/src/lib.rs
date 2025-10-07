// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::{
    path::PathBuf,
    time::{Duration, SystemTime},
};

use chrono::{DateTime, Utc};
use reqwest::{Method, Url};
use thiserror::Error;
use tracing::{Level, Span};

mod auth;
#[cfg(feature = "clap")]
mod clap_feature;
#[cfg(feature = "extras")]
pub mod extras;
mod generated_sdk;

pub use auth::*;
pub use generated_sdk::*;

/// Errors for interfaces related to authentication
#[derive(Error, Debug)]
pub enum OxideAuthError {
    #[error(r"$OXIDE_TOKEN is set but $OXIDE_HOST is not")]
    MissingHost,
    #[error(
        r"$OXIDE_HOST is set, but {0} has no corresponding token.\n
                Login without $OXIDE_HOST set or set $OXIDE_TOKEN."
    )]
    MissingToken(String),
    #[error("Both $OXIDE_HOST and $OXIDE_PROFILE are set, only one may be used")]
    HostProfileConflict,
    #[error("Parse error for {0}: {1}")]
    TomlError(PathBuf, toml::de::Error),
    #[error("IO Error: {0}")]
    IoError(std::io::Error),
    #[error("No profile specified and no default profile")]
    NoDefaultProfile,
    #[error("Profile information not present in {0} for {1}")]
    NoProfile(PathBuf, String),
    #[error("no authenticated hosts; use oxide auth login to authenticate")]
    NoAuthenticatedHosts,
}

// Hook into the generated API client to capture and log request metadata.
impl progenitor_client::ClientHooks for Client {
    async fn exec(
        &self,
        request: reqwest::Request,
        op_info: &progenitor_client::OperationInfo,
    ) -> reqwest::Result<reqwest::Response> {
        let url = request.url();
        let span = tracing::debug_span!("oxide", request = format!("{} {}", request.method(), url));

        #[derive(Clone, Debug)]
        struct StartDetails {
            op_id: &'static str,
            url: Url,
            method: Method,
            start_time: SystemTime,
            body: Option<String>,
            span: Span,
        }

        let mut details = StartDetails {
            op_id: op_info.operation_id,
            url: url.clone(),
            method: request.method().clone(),
            body: None,
            start_time: SystemTime::now(),
            span,
        };

        // Log up to the first KiB of the request body. Avoid performing this relatively
        // expensive operation unless the log level is DEBUG or above.
        if tracing::enabled!(target: "oxide", Level::DEBUG) {
            let body_bytes = request.body().and_then(|b| b.as_bytes());
            let body = body_bytes.map(|b| {
                let len = b.len().min(1024);
                let mut out = String::from_utf8_lossy(&b[..len]).into_owned();
                if b.len() > 1024 {
                    out.push_str("...");
                }
                out
            });

            if let Some(b) = body {
                details.body = Some(b);
            }
        }

        let result = self.client().execute(request).await;

        let duration_ms: u64 = SystemTime::now()
            .duration_since(details.start_time)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX);

        let _enter = details.span.enter();
        match &result {
            Ok(resp) => {
                tracing::debug!(
                    url = %details.url,
                    path = details.url.path(),
                    operation_id = %details.op_id,
                    remote_addr = resp.remote_addr().map(|a| a.to_string()),
                    http.request.method = %details.method,
                    http.request.body = details.body,
                    http.response.content_length = resp.content_length(),
                    http.response.status_code = resp.status().as_u16(),
                    start_time = format_time(details.start_time),
                    duration_ms,
                    oxide.request_id = get_request_id(resp),
                    "request succeeded",
                );
            }
            Err(e) => {
                use std::error::Error;
                tracing::debug!(
                    url = %details.url,
                    path = details.url.path(),
                    operation_id = %details.op_id,
                    http.request.method = %details.method,
                    http.request.body = details.body,
                    http.response.status_code = ?e.status(),
                    start_time = format_time(details.start_time),
                    duration_ms,
                    error.message = e.to_string(),
                    error.cause = ?e.source(),
                    "request failed",
                );
            }
        }
        result
    }
}

fn get_request_id(response: &reqwest::Response) -> Option<&str> {
    response
        .headers()
        .get("x-request-id")
        .and_then(|id| id.to_str().ok())
        .map(|id| id.trim_matches('"'))
}

fn format_time(time: SystemTime) -> String {
    let datetime = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| DateTime::from_timestamp(d.as_secs() as i64, d.subsec_nanos()))
        .ok()
        .flatten()
        .unwrap_or_else(Utc::now);

    datetime.format("%Y-%m-%dT%H:%M:%S.%6fZ").to_string()
}
