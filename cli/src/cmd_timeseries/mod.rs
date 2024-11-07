// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

mod dashboard;

use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use anyhow::{bail, Context as _, Result};
use async_trait::async_trait;
use clap::{ArgGroup, Parser};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use oxide::{
    types::{FieldValue, OxqlQueryResult, Table, TimeseriesQuery, ValueArray},
    Client, ClientMetricsExt, ResponseValue,
};

use chrono::{DateTime, TimeDelta, Utc};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::str::FromStr;

use ratatui::{prelude::CrosstermBackend, Terminal};
use statrs::statistics::Statistics;

use self::dashboard::Dashboard;

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
impl crate::AuthenticatedCmd for CmdTimeseriesDashboard {
    async fn run(&self, client: &Client) -> Result<()> {
        anyhow::ensure!(
            self.interval > 0 && self.interval < 1_000,
            "Please provide a reasonable update interval"
        );
        let interval = Duration::from_secs(self.interval);
        let client = client.clone();
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

#[derive(Copy, Clone, Debug)]
struct RawTime(DateTime<Utc>);

impl FromStr for RawTime {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let err = match DateTime::parse_from_rfc3339(s) {
            Ok(val) => {
                return Ok(Self(val.into()));
            }
            Err(err) => err,
        };

        if !s.ends_with('Z') {
            if let Ok(val) = DateTime::parse_from_rfc3339(&format!("{s}Z")) {
                return Ok(Self(val.into()));
            }
        }

        match s.parse::<f64>() {
            Ok(val) => {
                let secs: i64 = val as i64;
                let nsecs: u32 = (val.fract() * 1_000_000_000.0) as u32;

                match DateTime::from_timestamp(secs, nsecs) {
                    Some(val) => Ok(Self(val)),
                    None => {
                        bail!("illegal timestamp {s}");
                    }
                }
            }
            Err(_) => {
                bail!("could not parse {s}: {err:?}");
            }
        }
    }
}

/// Pull a timeseries for a single sensor.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[clap(
    name = "timeseries raw",
    group = ArgGroup::new("which").multiple(false),
    group = ArgGroup::new("duration").multiple(false),
)]
pub struct CmdTimeseriesRaw {
    /// Kind of sensor
    #[clap(long, default_value = "voltage")]
    kind: String,

    /// Kind of chassis
    #[clap(long, default_value = "sled")]
    chassis_kind: String,

    /// The slot to use
    #[clap(long, group = "which")]
    slot: Option<u32>,

    /// The serial number to use
    #[clap(long, group = "which")]
    serial: Option<String>,

    /// Start time of data
    #[clap(long)]
    start: Option<RawTime>,

    /// End time of data
    #[clap(long)]
    end: Option<RawTime>,

    /// Seconds of data to gather
    #[clap(long, group = "duration")]
    seconds: Option<u32>,

    /// Minuts of data to gather
    #[clap(long, group = "duration")]
    minutes: Option<u32>,

    /// Hours of data to gather
    #[clap(long, group = "duration")]
    hours: Option<u32>,

    /// Seconds of data to gather
    #[clap(long, group = "duration")]
    days: Option<u32>,

    /// Show query
    #[clap(long, short)]
    verbose: bool,

    /// Granulatity at which to rollup, in minutes
    #[clap(long)]
    rollup: Option<i64>,

    /// Number of hours of samples to batch
    #[clap(long, default_value = "12")]
    batchsize: u32,

    /// Get all available samples
    #[clap(long, conflicts_with_all = &["start", "end", "seconds"])]
    all: bool,

    /// List all available sensors for a given slot and kind
    #[clap(long, conflicts_with_all = &["start", "end", "duration", "all"])]
    list: bool,

    #[clap(conflicts_with = "list")]
    sensor: Option<String>,
}

#[derive(Debug, Default)]
struct State {
    header: bool,
    rollup: Option<i64>,
    epoch: Option<i64>,
    data: Vec<f64>,
}

impl State {
    fn new(rollup: Option<i64>) -> Self {
        State {
            header: false,
            rollup: match rollup {
                Some(rollup) => Some(rollup * 60),
                None => None,
            },
            epoch: None,
            data: Vec::new(),
        }
    }
}

fn earliest(table: &Table) -> Result<DateTime<Utc>> {
    let mut earliest = BTreeSet::new();

    for ts in table.timeseries.values() {
        earliest.insert(ts.points.timestamps.first().unwrap().clone());
    }

    match earliest.first() {
        None => {
            bail!("empty timeseries?");
        }
        Some(e) => Ok(*e),
    }
}

impl CmdTimeseriesRaw {
    fn process(&self, state: &mut State, table: &Table) -> Result<()> {
        let mut aggregated = BTreeMap::new();

        for ts in table.timeseries.values() {
            if !state.header {
                println!(
                    "# chassis={:?} slot={:?} sensor={:?} kind={}",
                    ts.fields.get("chassis_kind").unwrap(),
                    ts.fields.get("slot").unwrap(),
                    ts.fields.get("sensor").unwrap(),
                    self.kind
                );

                state.header = true;
            }

            let p = &ts.points;

            let vals = match ts.points.values[0].values {
                ValueArray::Double(ref vals) => vals,
                _ => {
                    bail!("unknown type {:?}", p.values[0].values);
                }
            };

            for (t, value) in ts.points.timestamps.iter().zip(vals.iter()) {
                if let Some(value) = value {
                    aggregated.insert(t, value);
                }
            }
        }

        for (ts, value) in aggregated.iter() {
            if let Some(rollup) = state.rollup {
                let epoch = ts.timestamp() / rollup;

                match state.epoch {
                    None => {
                        state.epoch = Some(epoch);
                    }
                    Some(e) if e != epoch => {
                        let start = DateTime::from_timestamp(e * rollup, 0).unwrap();

                        print!("{} ", start.format("%s"));
                        print!("min={:.5} ", Statistics::min(&state.data));
                        print!("mean={:.5} ", Statistics::mean(&state.data));
                        print!("max={:.5} ", Statistics::max(&state.data));
                        print!("stddev={:.5} ", Statistics::std_dev(&state.data));
                        println!();

                        state.epoch = Some(epoch);
                        state.data = vec![**value];
                    }
                    _ => {
                        state.data.push(**value);
                    }
                }
            } else {
                println!("{} {value}", ts.format("%s%.9f"));
            }
        }

        Ok(())
    }

    fn list(&self, table: &Table) -> Result<()> {
        let mut sensors = BTreeMap::new();

        let get = |f: &HashMap<String, FieldValue>, name| {
            if let Some(FieldValue::String(str)) = f.get(name) {
                str.clone()
            } else {
                "???".to_string()
            }
        };

        for ts in table.timeseries.values() {
            sensors.insert(get(&ts.fields, "sensor"), &ts.fields);
        }

        println!("{:22} {:13} {}", "SENSOR", "DEVICE", "DESCRIPTION");

        for (sensor, fields) in sensors {
            println!(
                "{:22} {:13} {}",
                sensor,
                get(fields, "component_kind"),
                get(fields, "description"),
            );
        }

        Ok(())
    }

    #[rustfmt::skip]
    async fn query(
        &self,
        client: &Client,
        query: &str
    ) -> Result<ResponseValue<OxqlQueryResult>> {
        if self.verbose {
            eprintln!("running query: {query}")
        }

        loop {
            let request = client.timeseries_query().body(TimeseriesQuery {
                query: query.to_string(),
            });

            match request.send().await {
                Ok(r) => break Ok(r),
                Err(err) => {
                    let str = format!("{err}");

                    if str.contains("operation timed out") {
                        eprintln!("request failed ({str}); retrying...")
                    } else {
                        bail!("request failed: {err:?}");
                    }
                }
            }
        }
    }
}

#[async_trait]
impl crate::AuthenticatedCmd for CmdTimeseriesRaw {
    async fn run(&self, client: &Client) -> Result<()> {
        let formatstr = "%Y-%m-%dT%H:%M:%S";

        let which = if let Some(slot) = self.slot {
            format!(
                "slot == {slot} && chassis_kind == \"{}\"",
                self.chassis_kind
            )
        } else if let Some(ref serial) = self.serial {
            format!("chassis_serial == \"{serial}\"")
        } else {
            bail!("must specify slot or serial number");
        };

        if self.list {
            let time = Utc::now() - TimeDelta::seconds(60);

            let q = format!(
                "get hardware_component:{} | \
                filter {which} && timestamp >= @{} | last 1",
                self.kind,
                time.format(formatstr)
            );

            let result = self.query(client, &q).await?;
            self.list(&result.tables[0])?;

            return Ok(());
        }

        let sensor = format!("sensor == \"{}\"", self.sensor.as_ref().unwrap());
        let default = TimeDelta::hours(24);

        let seconds = if let Some(seconds) = self.seconds {
            Some(seconds)
        } else if let Some(minutes) = self.minutes {
            Some(minutes * 60)
        } else if let Some(hours) = self.hours {
            Some(hours * 60 * 60)
        } else if let Some(days) = self.days {
            Some(days * 60 * 60 * 24)
        } else {
            None
        };

        let (start, end) = match (self.start, self.end, seconds) {
            (None, None, None) => {
                let now = Utc::now();

                if self.all {
                    let q = format!(
                        "get hardware_component:{} | \
                         filter {which} && {sensor} | first 1",
                        self.kind
                    );

                    let result = self.query(client, &q).await?;
                    let earliest = earliest(&result.tables[0])?;

                    (earliest, now)
                } else {
                    (now - default, now)
                }
            }

            (None, Some(RawTime(end)), None) => (end - default, end),
            (Some(RawTime(start)), None, None) => (start, start + default),
            (Some(RawTime(start)), Some(RawTime(end)), None) => (start, end),

            (None, Some(RawTime(end)), Some(seconds)) => {
                (end - TimeDelta::seconds(seconds.into()), end)
            }

            (Some(RawTime(start)), None, Some(seconds)) => {
                (start, start + TimeDelta::seconds(seconds.into()))
            }

            (None, None, Some(seconds)) => {
                let now = Utc::now();
                (now - TimeDelta::seconds(seconds.into()), now)
            }

            (Some(_), Some(_), Some(_)) => {
                bail!("cannot specify start time, end time AND duration");
            }
        };

        let mut s = start;

        let batchsize = TimeDelta::hours(self.batchsize.into());

        let mut state = State::new(self.rollup);

        loop {
            let e = if end - s > batchsize {
                s + batchsize
            } else {
                end
            };

            let time = format!(
                "(timestamp >= @{} && timestamp < @{})",
                s.format(formatstr),
                e.format(formatstr)
            );

            let q = format!(
                "get hardware_component:{} | \
                filter {time} && {which} && {sensor}",
                self.kind
            );

            let result = self.query(client, &q).await?;
            self.process(&mut state, &result.tables[0])?;

            if e >= end {
                break;
            }

            s += batchsize;
        }

        Ok(())
    }
}
