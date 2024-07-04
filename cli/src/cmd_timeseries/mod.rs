// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

mod dashboard;

use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crate::context::Context;
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use self::dashboard::Dashboard;
use crate::RunnableCmd;

/// Graph the results of an OxQL timeseries query.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "timeseries dashboard")]
pub struct CmdTimeseriesDashboard {
    /// The timeseries query to display in the dashboard.
    query: String,
    /// The interval on which to update the dashboard display, in seconds.
    #[clap(short, long, default_value_t = 10)]
    interval: u64,
}

// Type alias for our TUI application.
type Tui = Terminal<CrosstermBackend<Stdout>>;

#[async_trait]
impl RunnableCmd for CmdTimeseriesDashboard {
    async fn run(&self, ctx: &Context) -> Result<()> {
        anyhow::ensure!(
            self.interval > 0 && self.interval < 1_000,
            "Please provide a reasonable update interval"
        );
        let interval = Duration::from_secs(self.interval);
        let client = ctx.client()?.clone();
        let mut terminal = init_tui()?;
        let res = Dashboard::new(&self.query, interval)
            .run(&mut terminal, client)
            .await;
        restore_tui()?;
        res
    }
}

fn init_tui() -> Result<Tui> {
    // Setup panic handler to replace screen.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore_tui().unwrap();
        hook(panic_info);
    }));

    // Enter TUI world.
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Tui::new(CrosstermBackend::new(stdout())).context("creating TUI")
}

fn restore_tui() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
