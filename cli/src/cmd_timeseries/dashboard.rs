// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

//! Simple TUI dashboard app for display timeseries.

use super::Tui;
use anyhow::{Context as _, Result};
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use oxide::{
    types::{MetricType, OxqlTable, TimeseriesQuery, ValueArray},
    Client, ClientSystemMetricsExt,
};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols::Marker,
    widgets::{
        Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Row, StatefulWidget, Table,
        TableState, Widget,
    },
};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct Dashboard<'a> {
    // CLI-supplied query string.
    query: &'a str,
    // Timeseries update interval.
    interval: Duration,
    // Flag to exit the app.
    done: bool,
    // The timeseries data state.
    //
    // As we fetch timeseries, we'll convert data into the representation for
    // plotting, which often includes modifying the data values for graphing,
    // computing limits, etc.
    graph_state: GraphState,
}

impl<'a> Dashboard<'a> {
    pub fn new(query: &'a str, interval: Duration) -> Self {
        Self {
            query,
            interval,
            done: false,
            graph_state: GraphState::new(),
        }
    }

    pub async fn run(mut self, terminal: &'a mut Tui, client: Client) -> Result<()> {
        // Oneshot used to tell the client task to exit, if we get interrupted.
        let (done_tx, done_rx) = oneshot::channel();

        // Channel through which the client task sends us tables or error
        // messages.
        let (tx, mut rx) = mpsc::channel(1);

        // Spawn client task for actually running query, sending us the
        // results.
        let query = self.query.to_string();
        let interval = self.interval;
        let task = tokio::task::spawn(client_query_loop(done_rx, tx, client, query, interval));

        // Main application loop.
        //
        // We'll handle events; render the application; and wait for new
        // messages from the client task. If it sends us an error, we'll exit.
        // We also tell the client to exit if we're interrupted.
        let mut tick = tokio::time::interval(Duration::from_millis(50));
        let mut event_stream = EventStream::new();
        while !self.done {
            let events = event_stream.next().fuse();
            tokio::select! {
                Some(message) = rx.recv() => {
                    match message {
                        Message::Table(mut tables) => {
                            if let Err(e) = sanity_check_tables(&tables) {
                                let _ = done_tx.send(());
                                let _ = task.await;
                                anyhow::bail!(e);
                            }
                            self.graph_state.handle_new_table(tables.pop());
                        }
                        Message::Error(e) => return Err(e),
                    }
                }
                _ = tick.tick() => {},
                maybe_event = events => {
                    match maybe_event {
                        Some(Ok(event)) => self.handle_event(event)?,
                        Some(Err(e)) => anyhow::bail!(e),
                        None => todo!(),
                    }
                }
            }

            // Always draw the frame on each tick.
            terminal.draw(|frame| self.render_frame(frame))?;
        }
        if done_tx.send(()).is_err() {
            anyhow::bail!("Failed to notify client task to exit");
        }
        task.await.context("failed to await client task")
    }

    fn render_frame(&mut self, frame: &mut ratatui::Frame<'_>) {
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(60),
                Constraint::Percentage(30),
                Constraint::Percentage(10),
            ],
        )
        .split(frame.area());

        // Render the timeseries data itself.
        let graph = TimeseriesGraph {
            graph_state: &self.graph_state,
        };
        frame.render_widget(graph, layout[0]);

        // Render a table with the fields of each timeseries.
        //
        // This displays the schema and values, and also is effectively a
        // legend.
        let table = TimeseriesSchemaTable {
            graph_state: &mut self.graph_state,
        };
        frame.render_widget(table, layout[1]);

        // Render the query string display.
        let q = QueryString { query: self.query };
        frame.render_widget(q, layout[2]);
    }

    fn handle_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_event(key),
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL
            || key.code == KeyCode::Esc
        {
            self.done = true;
        } else if key.code == KeyCode::Up {
            if let Some(ix) = self.graph_state.table_state.selected_mut() {
                *ix = ix.saturating_sub(1);
            } else {
                self.graph_state.table_state.select(Some(1));
            }
        } else if key.code == KeyCode::Down {
            if let Some(ix) = self.graph_state.table_state.selected_mut() {
                // Limit to the number of timeseries we actually have.
                let n_timeseries = self.graph_state.data_arrays.len();
                *ix = (*ix + 1).min(n_timeseries - 1);
            } else {
                self.graph_state.table_state.select(Some(1));
            }
        }
    }
}

fn sanity_check_tables(tables: &[OxqlTable]) -> Result<()> {
    anyhow::ensure!(
        tables.len() == 1,
        "Timeseries queries must return a single table"
    );
    let table = &tables[0];
    anyhow::ensure!(
        table
            .timeseries
            .iter()
            .all(|ts| ts.points.values.len() == 1),
        "Graphing multidimensional timeseries is not yet supported"
    );
    anyhow::ensure!(
        table.timeseries.iter().all(|ts| matches!(
            ts.points.values[0].metric_type,
            MetricType::Delta | MetricType::Gauge
        )),
        "Timeseries must produce delta or gauge data to graph",
    );
    anyhow::ensure!(
        table.timeseries.iter().all(|ts| matches!(
            ts.points.values[0].values,
            ValueArray::Double(_) | ValueArray::Integer(_)
        )),
        "Timeseries must produce numeric data to graph",
    );
    Ok(())
}

enum Message {
    Table(Vec<OxqlTable>),
    Error(anyhow::Error),
}

async fn client_query_loop(
    mut done: oneshot::Receiver<()>,
    tx: mpsc::Sender<Message>,
    client: Client,
    query: String,
    interval: Duration,
) {
    let mut interval = tokio::time::interval(interval);
    loop {
        tokio::select! {
            _ = &mut done => break,
            _ = interval.tick() => {
                let request = client
                    .system_timeseries_query()
                    .body(TimeseriesQuery {
                        query: query.clone(),
                        include_summaries: false,
                    });
                match request.send().await {
                    Ok(response) => {
                        tx.send(Message::Table(response.into_inner().tables))
                            .await
                            .expect("Failed to send response to main task");
                    }
                    Err(e) => {
                        tx.send(Message::Error(anyhow::anyhow!(e)))
                            .await
                            .expect("Failed to send error to main task");
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct GraphState {
    // The table we've most recently received.
    table: Option<OxqlTable>,
    // The data arrays to be plotted.
    //
    // These are computed every time we receive a table.
    data_arrays: Vec<Vec<(f64, f64)>>,
    // The axis limits for each axis.
    t_limits: [f64; 2],
    t_labels: [String; 3],
    y_limits: [f64; 2],
    y_labels: [String; 3],
    // The state of the table used to display the schema and legend.
    table_state: TableState,
}

impl GraphState {
    fn new() -> Self {
        Self {
            table_state: TableState::default().with_offset(1),
            ..Default::default()
        }
    }

    fn handle_new_table(&mut self, table: Option<OxqlTable>) {
        self.table = table;
        let Some(table) = self.table.as_ref() else {
            // No table at all, just clear everything.
            self.data_arrays.clear();
            self.t_limits = [0.0, 0.0];
            self.y_limits = [0.0, 0.0];
            self.table_state.select(None);
            return;
        };

        // Reset the data arrays and limits in self.
        self.data_arrays.clear();
        self.data_arrays.reserve(table.timeseries.len());
        self.t_limits[1] = 0.0;
        self.y_limits = [f64::INFINITY, 0.0];

        // Construct the plotting arrays from the received data.
        for timeseries in &table.timeseries {
            if timeseries.points.timestamps.is_empty() {
                self.data_arrays.push(vec![]);
            }

            // For either gauge or delta timeseries, we convert the
            // timestamps into seconds from the first timestamp. We'll plot
            // them relative to that, and adjust the axis labels.
            //
            // For delta timeseries, we'd also like to plot everything relative
            // to the first value. There are a few ways to do that, but the
            // simplest is to add the first value to every other value after
            // that.
            //
            // In both cases, note that we filter out `None` values.
            let timestamps = timeseries.points.timestamps.iter().map(|ts| {
                (*ts - timeseries.points.timestamps[0])
                    .to_std()
                    .unwrap()
                    .as_secs_f64()
            });

            let is_delta = matches!(timeseries.points.values[0].metric_type, MetricType::Delta);
            let data: Vec<_> = match &timeseries.points.values[0].values {
                ValueArray::Double(values) => {
                    let mut data = Vec::with_capacity(values.len());
                    let mut offset = None;
                    for (t, y) in timestamps
                        .zip(values.iter())
                        .filter_map(|(t, y)| y.map(|y| (t, y)))
                    {
                        if is_delta {
                            match offset {
                                None => {
                                    // The first value, no modification needed.
                                    data.push((t, y));
                                    offset.replace(y);
                                }
                                Some(offset) => {
                                    // A later value, we'll offset it by the first.
                                    data.push((t, y + offset));
                                }
                            }
                        } else {
                            // Deltas need no modification.
                            data.push((t, y));
                        }

                        // Update the axis limits, if needed.
                        self.y_limits[0] = self.y_limits[0].min(data.last().as_ref().unwrap().1);
                        self.y_limits[1] = self.y_limits[1].max(data.last().as_ref().unwrap().1);
                    }

                    // The time axis should always be sorted, so we can update
                    // it ouside the above loop.
                    self.t_limits[1] = self.t_limits[1].max(data.last().as_ref().unwrap().0);
                    data
                }
                ValueArray::Integer(values) => {
                    let mut data = Vec::with_capacity(values.len());
                    let mut offset = None;
                    for (t, y) in timestamps
                        .zip(values.iter())
                        .filter_map(|(t, y)| y.map(|y| (t, y as f64)))
                    {
                        if is_delta {
                            match offset {
                                None => {
                                    // The first value, no modification needed.
                                    data.push((t, y));
                                    offset.replace(y);
                                }
                                Some(offset) => {
                                    // A later value, we'll offset it by the first.
                                    data.push((t, y + offset));
                                }
                            }
                        } else {
                            // Deltas need no modification
                            data.push((t, y))
                        }

                        // Update the y-axis limits, if needed.
                        self.y_limits[0] = self.y_limits[0].min(data.last().as_ref().unwrap().1);
                        self.y_limits[1] = self.y_limits[1].max(data.last().as_ref().unwrap().1);
                    }

                    // The time axis should always be sorted, so we can update
                    // it ouside the above loop.
                    self.t_limits[1] = self.t_limits[1].max(data.last().as_ref().unwrap().0);
                    data
                }
                _ => unreachable!(),
            };
            self.data_arrays.push(data);
        }

        // If there was not previously a timeseries selected, do so now.
        if self.table_state.selected().is_none() {
            self.table_state.select(Some(0));
        }

        // We need a bit of play in the y-axis, to show the values reasonably.
        let p2p = (self.y_limits[1] - self.y_limits[0]).abs();
        const ROOM: f64 = 0.1;
        self.y_limits = [self.y_limits[0] - ROOM * p2p, self.y_limits[1] + ROOM * p2p];
        self.update_axis_labels();
    }

    fn update_axis_labels(&mut self) {
        let Some(timeseries) = self.table.as_ref().unwrap().timeseries.first() else {
            self.t_labels.fill(String::new());
            self.y_labels.fill(String::new());
            return;
        };
        let start_time = timeseries.points.timestamps[0];
        let end_time = *timeseries.points.timestamps.last().unwrap();
        let mid = start_time + (end_time - start_time) / 2;

        // The time axis should include a few labels along its extent, but how
        // we format them depends on their extent.
        let extent = Duration::from_secs_f64(self.t_limits[1] - self.t_limits[0]);
        const WEEK: Duration = Duration::from_secs(60 * 60 * 24 * 7);
        const DAY: Duration = Duration::from_secs(60 * 60 * 24);
        const HOUR: Duration = Duration::from_secs(60 * 60);
        const MINUTE: Duration = Duration::from_secs(60);
        let formats = if extent > WEEK {
            // Everything will be labeled by the day.
            ["%Y-%m-%d"; 3]
        } else if extent > DAY {
            // Label some hours as well.
            ["%Y-%m-%d %H:%M"; 3]
        } else if extent > HOUR {
            // Label the first day, then just hours
            ["%Y-%m-%d %H:%M:%S", "%H:%M", "%H:%M"]
        } else if extent > MINUTE {
            // Full, then to the second.
            ["%Y-%m-%d %H:%M:%S", "%H:%M:%S", "%H:%M:%S"]
        } else {
            // Full, then to the millisecond.
            ["%Y-%m-%d %H:%M:%S.%3f", "%H:%M:%S.%3f", "%H:%M:%S.%3f"]
        };

        for ((fmt, time), label) in formats
            .iter()
            .zip(&[start_time, mid, end_time])
            .zip(self.t_labels.iter_mut())
        {
            *label = time.format(fmt).to_string();
        }

        // For the y-axis labels, we'll try to format a bit nicely as well.
        //
        // Find a nice power of 10, such that the value is less than 1000 in
        // that unit, and print 2 decimal places.
        let mut y_max = self.y_limits[1];
        const UNITS: [&str; 5] = ["", "k", "M", "B", "T"];
        let mut factor = 1.0;
        let mut unit = "";
        for each in UNITS.iter() {
            if y_max < 1000.0 {
                break;
            }
            unit = each;
            y_max /= 1000.0;
            factor *= 1000.0;
        }
        let midpoint = self.y_limits[0] + ((self.y_limits[1] - self.y_limits[0]) / 2.0);
        self.y_labels[0] = format!("{:0.2} {}", self.y_limits[0] / factor, unit);
        self.y_labels[1] = format!("{:0.2} {}", midpoint / factor, unit);
        self.y_labels[2] = format!("{:0.2} {}", self.y_limits[1] / factor, unit);
    }
}

#[derive(Debug)]
struct TimeseriesGraph<'a> {
    graph_state: &'a GraphState,
}

impl Widget for TimeseriesGraph<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let datasets = self
            .graph_state
            .data_arrays
            .iter()
            .enumerate()
            .map(|(i, data)| {
                let ds = Dataset::default()
                    .data(data)
                    .marker(Marker::Dot)
                    .graph_type(GraphType::Scatter);
                // If this is the highlighted dataset, make it obvious.
                if let Some(index) = self.graph_state.table_state.selected() {
                    if index == i {
                        return ds.white();
                    }
                }
                ds.style(color_from_index(i))
            })
            .collect();
        let t_axis = Axis::default()
            .title("Time")
            .style(Style::default())
            .bounds(self.graph_state.t_limits)
            .labels(self.graph_state.t_labels.clone())
            .labels_alignment(Alignment::Right);
        let y_axis = Axis::default()
            .title("Value")
            .style(Style::default())
            .bounds(self.graph_state.y_limits)
            .labels(self.graph_state.y_labels.clone())
            .labels_alignment(Alignment::Right);
        Chart::new(datasets)
            .x_axis(t_axis)
            .y_axis(y_axis)
            .block(
                Block::default()
                    .title("Timeseries data")
                    .borders(Borders::ALL),
            )
            .render(area, buf);
    }
}

// Return an indexed `Color` from a linear index in a list of timeseries
fn color_from_index(i: usize) -> Color {
    Color::Indexed((20..=231).step_by(8).cycle().nth(i).unwrap())
}

#[derive(Debug)]
struct TimeseriesSchemaTable<'a> {
    graph_state: &'a mut GraphState,
}

impl Widget for TimeseriesSchemaTable<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut empty_table = |state: &mut TableState| {
            // No tables, just render an empty table.
            let table = Table::default().block(
                Block::default()
                    .title("Timeseries fields")
                    .borders(Borders::ALL),
            );
            state.select(None);
            StatefulWidget::render(table, area, buf, state);
        };

        let Some(table) = self.graph_state.table.as_ref() else {
            empty_table(&mut self.graph_state.table_state);
            return;
        };
        if table.timeseries.is_empty() {
            empty_table(&mut self.graph_state.table_state);
            return;
        }

        let mut rows = Vec::with_capacity(table.timeseries.len());
        let mut header = Row::default();
        let mut first = true;
        let mut n_columns;
        let mut widths = Vec::new();

        for timeseries in &table.timeseries {
            // We also need to sort the columns of each table, again because the
            // hash maps generated by progenitor are not stable.
            let mut items = timeseries
                .fields
                .iter()
                .map(|(k, v)| (k.as_str(), format!("{v:?}")))
                .collect::<Vec<_>>();
            items.sort();
            let (field_names, field_values): (Vec<_>, Vec<_>) = items.into_iter().unzip();
            n_columns = field_names.len();

            // Update the set of widths for each column, setting a minimum width
            // based on the field name or value, whichever is larger.
            if widths.len() != n_columns {
                widths.resize(n_columns, 0);
            }
            for (i, (name, value)) in field_names.iter().zip(field_values.iter()).enumerate() {
                let new_len = name.len().max(value.len());
                widths[i] = widths[i].max(new_len as u16);
            }

            // Create a header row.
            if first {
                header = Row::new(field_names).style(Style::new().bold());
                first = false;
            }

            // Add the values as a data row.
            rows.push(Row::new(field_values));
        }

        let constraints = widths.into_iter().map(Constraint::Min).collect::<Vec<_>>();
        let table = Table::new(rows, constraints)
            .header(header)
            .row_highlight_style(Style::new().reversed())
            .block(
                Block::default()
                    .title("Timeseries fields")
                    .borders(Borders::ALL),
            );
        StatefulWidget::render(table, area, buf, &mut self.graph_state.table_state)
    }
}

#[derive(Debug)]
struct QueryString<'a> {
    query: &'a str,
}

impl Widget for QueryString<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(self.query)
            .block(Block::default().title("Query").borders(Borders::ALL))
            .render(area, buf)
    }
}
