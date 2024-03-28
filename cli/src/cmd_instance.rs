// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use crate::RunnableCmd;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use oxide::types::{
    ByteCount, DiskSource, ExternalIpCreate, InstanceCpuCount, InstanceDiskAttachment, Name,
    NameOrId,
};

use futures::{SinkExt, StreamExt};
use oxide::ClientImagesExt;
use oxide::ClientInstancesExt;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::tungstenite::protocol::{frame::coding::CloseCode, CloseFrame, Role};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

/// Connect to or retrieve data from the instance's serial console.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "serial")]
pub struct CmdInstanceSerial {
    #[clap(subcommand)]
    subcmd: SerialSubCommand,
}

#[async_trait]
impl RunnableCmd for CmdInstanceSerial {
    async fn run(&self, ctx: &oxide::context::Context) -> Result<()> {
        match &self.subcmd {
            SerialSubCommand::Console(cmd) => cmd.run(ctx).await,
            SerialSubCommand::History(cmd) => cmd.run(ctx).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum SerialSubCommand {
    Console(CmdInstanceSerialConsole),
    History(CmdInstanceSerialHistory),
}

/// Connect to an instance's serial console interactively.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "console")]
pub struct CmdInstanceSerialConsole {
    /// Name or ID of the instance
    #[clap(long, short)]
    instance: NameOrId,

    /// Name or ID of the project
    #[clap(long, short)]
    project: Option<NameOrId>,

    /// The number of bytes from the most recent output to retrieve as context
    /// before connecting to the interactive session directly.
    #[clap(long, short, default_value = "262144")]
    most_recent: u64,

    /// If this sequence of bytes is typed, the client will exit.
    /// Defaults to "^]^C" (Ctrl+], Ctrl+C). Note that the string passed
    /// for this argument must be valid UTF-8, and is used verbatim without
    /// any parsing; in most shells, if you wish to include a special
    /// character (such as Enter or a Ctrl+letter combo), you can insert
    /// the character by preceding it with Ctrl+V at the command line.
    /// To disable the escape string altogether, provide an empty string to
    /// this flag (and to exit in such a case, use pkill or similar).
    #[clap(long, short, default_value = "\x1d\x03")]
    escape_string: String,

    /// The number of bytes from the beginning of the escape string to pass
    /// to the VM before beginning to buffer inputs until a mismatch.
    /// Defaults to 0, such that input matching the escape string does not
    /// get sent to the VM at all until a non-matching character is typed.
    /// For example, to mimic the escape sequence for exiting SSH ("\n~."),
    /// you may pass `-e '^M~.' --escape-prefix-length=1` such that newline
    /// gets sent to the VM immediately while still continuing to match the
    /// rest of the sequence.
    #[clap(long, default_value = "0")]
    escape_prefix_length: usize,

    /// Use a specified tty device (e.g. /dev/ttyUSB0) rather than the current
    /// terminal's stdin/stdout.
    #[clap(long, short)]
    tty: Option<PathBuf>,
}

#[async_trait]
impl RunnableCmd for CmdInstanceSerialConsole {
    // cli process becomes an interactive remote shell.
    async fn run(&self, ctx: &oxide::context::Context) -> Result<()> {
        let mut req = ctx
            .client()?
            .instance_serial_console_stream()
            .instance(self.instance.clone())
            .most_recent(self.most_recent);

        if let Some(value) = &self.project {
            req = req.project(value.clone());
        } else if let NameOrId::Name(_) = &self.instance {
            // on the server end, the connection is upgraded by the server
            // before the worker thread attempts to look up the instance.
            anyhow::bail!("Must provide --project when specifying instance by name rather than ID");
        }

        let upgraded = req.send().await.map_err(|e| e.into_untyped())?.into_inner();

        let esc_bytes = self.escape_string.clone().into_bytes();
        let escape = if esc_bytes.is_empty() {
            None
        } else {
            Some(thouart::EscapeSequence::new(
                esc_bytes,
                self.escape_prefix_length,
            )?)
        };
        if let Some(path) = &self.tty {
            let f_in = tokio::fs::File::open(path).await?;
            let f_out = tokio::fs::OpenOptions::new().write(true).open(path).await?;
            let mut tty = thouart::Console::new(f_in, f_out, escape).await?;
            tty.attach_to_websocket(upgraded).await?;
        } else {
            let mut tty = thouart::Console::new_stdio(escape).await?;
            tty.attach_to_websocket(upgraded).await?;
        }
        Ok(())
    }
}

/// Fetch an instance's serial console output.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "history")]
pub struct CmdInstanceSerialHistory {
    /// Name or ID of the instance
    #[clap(long, short)]
    instance: NameOrId,

    /// Name or ID of the project
    #[clap(long, short)]
    project: Option<NameOrId>,

    /// The offset since boot (or if negative, the current end of the
    /// buffered data) from which to retrieve output history.
    /// Defaults to the instance's first output from boot.
    #[clap(long, short, default_value = "0")]
    byte_offset: Option<i64>,

    /// Maximum number of bytes of buffered serial console contents to return.
    /// If the requested range (starting at --byte-offset) runs to the end of
    /// the available buffer, the data returned will be shorter (and if --json
    /// is provided, the actual final offset will be provided).
    #[clap(long, short)]
    max_bytes: Option<u64>,

    /// Output a JSON payload of the requested bytes, and the absolute
    /// byte-offset-since-boot of the last byte retrieved, rather than
    /// formatting the output to the terminal directly.
    #[clap(long, short)]
    json: bool,
}

#[async_trait]
impl RunnableCmd for CmdInstanceSerialHistory {
    async fn run(&self, ctx: &oxide::context::Context) -> Result<()> {
        let mut req = ctx
            .client()?
            .instance_serial_console()
            .instance(self.instance.clone());

        if let Some(value) = self.max_bytes {
            req = req.max_bytes(value);
        }

        if let Some(value) = &self.project {
            req = req.project(value.clone());
        }

        match self.byte_offset {
            Some(x) if x >= 0 => req = req.from_start(x as u64),
            Some(x) => req = req.most_recent(-x as u64),
            None => {}
        }

        let data = req.send().await.map_err(|e| e.into_untyped())?.into_inner();

        if self.json {
            println!("{}", serde_json::to_string(&data)?);
        } else {
            let mut tty = thouart::Console::new_stdio(None).await?;
            tty.write_stdout(&data.data).await?;
        }
        Ok(())
    }
}

/// Connect to the instance's framebuffer and input with a local VNC client.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "serial")]
pub struct CmdInstanceVnc {
    /// Name or ID of the instance
    #[clap(long, short)]
    instance: NameOrId,

    /// Name or ID of the project
    #[clap(long, short)]
    project: Option<NameOrId>,
    // TODO: vncviewer executable, or flag that says not to
}

#[async_trait]
impl RunnableCmd for CmdInstanceVnc {
    async fn run(&self, ctx: &oxide::context::Context) -> Result<()> {
        let mut req = ctx.client()?.instance_vnc().instance(self.instance.clone());

        if let Some(value) = &self.project {
            req = req.project(value.clone());
        } else if let NameOrId::Name(_) = &self.instance {
            // on the server end, the connection is upgraded by the server
            // before the worker thread attempts to look up the instance.
            anyhow::bail!("Must provide --project when specifying instance by name rather than ID");
        }

        // TODO: custom listen address
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        // yes, two ':' between IP and port. otherwise VNC adds 5900 to it!
        let vncviewer_arg = format!("{ip}::{port}", ip = addr.ip(), port = addr.port());

        // TODO: custom args etc.
        let mut cmd = std::process::Command::new("vncviewer");
        cmd.arg(&vncviewer_arg);
        let child_res = cmd.spawn();
        if child_res.is_err() {
            eprintln!(
                "Please connect a VNC client to {ip} on TCP port {port}.\nFor example: vncviewer {vncviewer_arg}",
                ip = addr.ip(),
                port = addr.port(),
                vncviewer_arg = vncviewer_arg,
            );
        }

        // TODO: clearer error case communication
        let Ok((tcp_stream, _addr)) = listener.accept().await else {
            anyhow::bail!("Failed to accept connection from local VNC client");
        };

        // okay, we have a local client, now actually start requesting I/O through nexus
        let upgraded = req.send().await.map_err(|e| e.into_untyped())?.into_inner();

        let ws = WebSocketStream::from_raw_socket(upgraded, Role::Client, None).await;

        let (mut ws_sink, mut ws_stream) = ws.split();
        let (mut tcp_reader, mut tcp_writer) = tcp_stream.into_split();
        let (closed_tx, mut closed_rx) = tokio::sync::oneshot::channel::<()>();

        let mut jh = tokio::spawn(async move {
            // medium-sized websocket payload
            let mut tcp_read_buf = vec![0u8; 65535];
            loop {
                tokio::select! {
                    _ = &mut closed_rx => break,
                    num_bytes_res = tcp_reader.read(&mut tcp_read_buf) => {
                        match num_bytes_res {
                            Ok(num_bytes) => {
                                ws_sink
                                    .send(Message::Binary(Vec::from(&tcp_read_buf[..num_bytes])))
                                    .await?;
                            }
                            Err(e) => {
                                ws_sink.send(Message::Close(None)).await?;
                                anyhow::bail!("Local client disconnected: {}", e);
                            }
                        }
                    }
                }
            }
            Ok(ws_sink)
        });

        let mut close_frame = None;
        loop {
            tokio::select! {
                _ = &mut jh => break,
                msg = ws_stream.next() => {
                    match msg {
                        Some(Ok(Message::Binary(data))) => {
                            let mut start = 0;
                            while start < data.len() {
                                match tcp_writer.write(&data[start..]).await {
                                    Ok(num_bytes) => {
                                        start += num_bytes;
                                    }
                                    Err(e) => {
                                        close_frame = Some(CloseFrame {
                                            code: CloseCode::Error,
                                            reason: e.to_string().into(),
                                        });
                                        break;
                                    }
                                }
                            }
                        }
                        Some(Ok(Message::Close(Some(CloseFrame {code, reason})))) => {
                            match code {
                                CloseCode::Abnormal
                                | CloseCode::Error
                                | CloseCode::Extension
                                | CloseCode::Invalid
                                | CloseCode::Policy
                                | CloseCode::Protocol
                                | CloseCode::Size
                                | CloseCode::Unsupported => {
                                    anyhow::bail!("Server disconnected: {}", reason.to_string());
                                }
                                _ => break,
                            }
                        }
                        Some(Ok(Message::Close(None))) => {
                            eprintln!("Connection closed.");
                            break;
                        }
                        None => {
                            eprintln!("Connection lost.");
                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }

        // let _: the connection may have already been dropped at this point.
        let _ = closed_tx.send(()).is_ok();
        if let Ok(Ok(mut ws_sink)) = jh.await {
            let _ = ws_sink.send(Message::Close(close_frame)).await.is_ok();
        }

        Ok(())
    }
}

/// Launch an instance from a disk image.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "from-image")]
pub struct CmdInstanceFromImage {
    /// Name of the instance to create
    #[clap(long)]
    name: Name,

    /// Project for image and instance
    #[clap(long)]
    project: NameOrId,

    /// Description of the instance to create
    #[clap(long)]
    description: String,

    /// Hostname of the instance to create
    #[clap(long)]
    hostname: String,

    /// Instance memory e.g 32M. Suffix can be k,m,g,t
    #[clap(long)]
    memory: ByteCount,

    /// Instance CPU count
    #[clap(long)]
    ncpus: InstanceCpuCount,

    /// Source image
    #[clap(long)]
    image: NameOrId,

    /// Boot disk size e.g. 512G. Suffix can be k,m,g,t
    #[clap(long)]
    size: ByteCount,

    /// Start the instance immediately
    #[clap(long)]
    start: bool,
}

#[async_trait]
impl RunnableCmd for CmdInstanceFromImage {
    async fn run(&self, ctx: &oxide::context::Context) -> Result<()> {
        // Validate the image and get its ID (if specified by name).
        let mut image_request = ctx.client()?.image_view().image(&self.image);
        // We only need the project if the image is specified by name.
        if let NameOrId::Name(_) = &self.image {
            image_request = image_request.project(&self.project);
        };
        let image_view = image_request.send().await?;

        let instance = ctx
            .client()?
            .instance_create()
            .project(&self.project)
            .body_map(|body| {
                body.name(self.name.clone())
                    .description(self.description.clone())
                    .disks(vec![InstanceDiskAttachment::Create {
                        description: format!("{} disk", *self.name),
                        disk_source: DiskSource::Image {
                            image_id: image_view.id,
                        },
                        name: format!("{}-disk", *self.name)
                            .parse()
                            .expect("valid disk name"),
                        size: self.size.clone(),
                    }])
                    .external_ips(vec![ExternalIpCreate::Ephemeral { pool: None }])
                    .hostname(self.hostname.clone())
                    .memory(self.memory.clone())
                    .ncpus(self.ncpus.clone())
                    .start(self.start)
            })
            .send()
            .await?;

        println!("instance {} created", instance.id);

        Ok(())
    }
}
