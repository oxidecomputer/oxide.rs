// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::watch;

pub fn start_progress_bar(
    mut progress_rx: watch::Receiver<u64>,
    total: u64,
    msg: &str,
) -> Result<ProgressBar> {
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::default_bar().template(
        "[{elapsed_precise}] [{wide_bar:.green}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta}",
    )?);
    pb.set_position(0);
    pb.println(msg);
    let pb_updater = pb.clone();

    tokio::spawn(async move {
        loop {
            match progress_rx.changed().await {
                Ok(_) => {
                    let p = *progress_rx.borrow();
                    pb_updater.set_position(p);

                    if p >= total {
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
