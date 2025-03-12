// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use chrono::{DateTime, Utc};
use reqwest::{Method, Request, Response, Url};
use tokio::task::Id;
use tracing::{Level, Span};

use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug)]
struct StartDetails {
    url: Url,
    method: Method,
    start_time: SystemTime,
    body: Option<String>,
    span: Span,
}

/// Store Oxide API request metadata during execution so a unified log entry can
/// be generated.
#[derive(Clone, Debug)]
pub struct LogCtx(Arc<Mutex<HashMap<Id, StartDetails>>>);

impl LogCtx {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
}

impl Default for LogCtx {
    fn default() -> Self {
        Self::new()
    }
}

pub fn on_request_start(ctx: &LogCtx, request: &Request) {
    let url = request.url();
    let span = tracing::debug_span!("oxide", request = format!("{} {}", request.method(), url));

    let mut details = StartDetails {
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

    // Try to store the request details for retrieval when the requests finishes.
    // If we're not in a tokio Task this is not possible, instead log separate
    // events for start and finish.
    let Some(task_id) = tokio::task::try_id() else {
        tracing::debug!(
            url = %url,
            path = url.path(),
            http.request.method = %request.method(),
            http.request.body = details.body,
        );
        return;
    };

    let Ok(mut ctx_map) = ctx.0.lock() else {
        tracing::warn!(task_id = ?task_id, "failed to take LogCtx lock");
        return;
    };

    ctx_map.insert(task_id, details);
}

pub fn on_request_end(ctx: &LogCtx, outcome: &Result<Response, reqwest::Error>) {
    let Some(task_id) = tokio::task::try_id() else {
        log_uncorrelated_response(outcome);
        return;
    };

    let Ok(mut ctx_map) = ctx.0.lock() else {
        tracing::warn!(task_id = ?task_id, "failed to take LogCtx lock");
        return;
    };

    let Some(details) = ctx_map.remove(&task_id) else {
        tracing::warn!(task_id = ?task_id, response = ?outcome, "no LogCtx entry found for task");
        log_uncorrelated_response(outcome);
        return;
    };

    // Don't hold the lock any longer than necessary.
    drop(ctx_map);

    // Convert to a u64 so this can be logged as a JSON number.
    let duration_ms: u64 = SystemTime::now()
        .duration_since(details.start_time)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX);

    let _enter = details.span.enter();
    match outcome {
        Ok(resp) => {
            tracing::debug!(
                url = %details.url,
                path = details.url.path(),
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
            tracing::debug!(
                url = %details.url,
                path = details.url.path(),
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
}

fn log_uncorrelated_response(outcome: &Result<Response, reqwest::Error>) {
    match outcome {
        Ok(resp) => {
            let url = resp.url();
            tracing::debug!(
                url = %url,
                path = url.path(),
                remote_addr = resp.remote_addr().map(|a| a.to_string()),
                http.response.content_length = resp.content_length(),
                http.response.status_code = resp.status().as_u16(),
                oxide.request_id = get_request_id(resp),
                "request succeeded",
            );
        }
        Err(e) => {
            tracing::debug!(
                url = ?e.url(),
                path = ?e.url().map(|u| u.path()),
                http.response.status_code = ?e.status().map(|s| s.as_u16()),
                error.message = e.to_string(),
                error.cause = ?e.source(),
                "request failed",
            );
        }
    }
}

fn get_request_id(response: &Response) -> Option<&str> {
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
