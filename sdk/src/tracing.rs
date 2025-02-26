// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use http::Extensions;
use reqwest::{Request, Response};
use reqwest_tracing::ReqwestOtelSpanBackend;
use tracing::{field::Empty, Level, Span};

pub(super) struct RequestSpan;

impl ReqwestOtelSpanBackend for RequestSpan {
    fn on_request_start(request: &Request, _extensions: &mut Extensions) -> Span {
        let url = request.url();

        let span = tracing::debug_span!(
            "Oxide API Request",
            http.request.method = %request.method(),
            url = %url,
            host = %url.host_str().unwrap_or("unknown"),
            http.request.body = Empty,
            http.response.status_code = Empty,
            http.response.content_length = Empty,
            oxide.request_id = Empty,
            error.message = Empty,
            error.cause_chain = Empty,
        );

        // Log up to the first KiB of the request body. Avoid performing this relatively
        // expensive operation unless the log level is DEBUG or above.
        if tracing::span_enabled!(target: "oxide", Level::DEBUG) {
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
                span.record("http.request.body", b);
            }
        }

        span
    }

    fn on_request_end(
        span: &Span,
        outcome: &Result<Response, reqwest_middleware::Error>,
        _extensions: &mut Extensions,
    ) {
        reqwest_tracing::default_on_request_end(span, outcome);
        if let Ok(resp) = outcome {
            if let Some(req_id) = resp
                .headers()
                .get("x-request-id")
                .and_then(|id| id.to_str().ok())
            {
                span.record("oxide.request_id", req_id.trim_matches('"'));
            }

            if let Some(content_length) = resp.content_length() {
                span.record("http.response.content_length", content_length);
            }
        }
    }
}
