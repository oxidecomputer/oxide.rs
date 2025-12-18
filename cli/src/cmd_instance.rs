// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use oxide::types::{
    ByteCount, DiskBackend, DiskSource, ExternalIpCreate, InstanceCpuCount, InstanceDiskAttachment,
    Name, NameOrId,
};

use oxide::ClientInstancesExt;
use oxide::{Client, ClientImagesExt};
use std::io::{self, Write};
use std::path::PathBuf;

use crate::println_nopipe;

/// Connect to or retrieve data from the instance's serial console.
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "serial")]
pub struct CmdInstanceSerial {
    #[clap(subcommand)]
    subcmd: SerialSubCommand,
}

#[async_trait]
impl crate::AuthenticatedCmd for CmdInstanceSerial {
    async fn run(&self, client: &Client) -> Result<()> {
        match &self.subcmd {
            SerialSubCommand::Console(cmd) => cmd.run(client).await,
            SerialSubCommand::History(cmd) => cmd.run(client).await,
        }
    }
}

#[derive(Parser, Debug, Clone)]
enum SerialSubCommand {
    Console(CmdInstanceSerialConsole),
    History(CmdInstanceSerialHistory),
}

/// Connect to an instance's serial console interactively.
///
/// (To pull output non-interactively, try `oxide instance serial history`)
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

    #[clap(
        long,
        short,
        default_value = "\x1d\x03",
        hide_default_value = true,
        help = "If this sequence of bytes is typed, the client will exit. \
Note that the string passed for this argument must be valid UTF-8, \
and is used verbatim without any parsing; in most shells, if you wish \
to include a special character (such as Enter or a Ctrl+letter combo), \
you can insert the character by preceding it with Ctrl+V at the command line. \
To disable the escape string altogether, provide an empty string to \
this flag (and to exit in such a case, use pkill or similar).

[default: { Ctrl+], Ctrl+C }]
-- which would appear in your shell as ^]^C if you provided it manually \
by typing { Ctrl+V, Ctrl+], Ctrl+V, Ctrl+C } at the command line."
    )]
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

impl CmdInstanceSerialConsole {
    // cli process becomes an interactive remote shell.
    async fn run(&self, client: &Client) -> Result<()> {
        let mut req = client
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
///
/// (To connect interactively and follow live output, try `oxide instance serial console`)
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

impl CmdInstanceSerialHistory {
    // cli process becomes an interactive remote shell.
    async fn run(&self, client: &Client) -> Result<()> {
        let mut req = client
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
            writeln!(io::stdout(), "{}", serde_json::to_string(&data)?)?;
        } else {
            let mut tty = thouart::Console::new_stdio(None).await?;
            tty.write_stdout(&data.data).await?;
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

    /// Description of the instance
    #[clap(long)]
    description: String,

    /// The hostname to be assigned to the instance
    #[clap(long)]
    hostname: String,

    /// Amount of RAM to be allocated to the instance
    #[clap(long)]
    memory: ByteCount,

    /// Number of vCPUs to be allocated to the instance
    #[clap(long)]
    ncpus: InstanceCpuCount,

    /// Source image
    #[clap(long)]
    image: NameOrId,

    /// Boot disk size
    #[clap(long)]
    size: ByteCount,

    /// Start the instance immediately
    #[clap(long)]
    start: bool,
}

#[async_trait]
impl crate::AuthenticatedCmd for CmdInstanceFromImage {
    async fn run(&self, client: &Client) -> Result<()> {
        // Validate the image and get its ID (if specified by name).
        let mut image_request = client.image_view().image(&self.image);
        // We only need the project if the image is specified by name.
        if let NameOrId::Name(_) = &self.image {
            image_request = image_request.project(&self.project);
        };
        let image_view = image_request.send().await?;

        let instance = client
            .instance_create()
            .project(&self.project)
            .body_map(|body| {
                body.name(self.name.clone())
                    .description(self.description.clone())
                    .boot_disk(InstanceDiskAttachment::Create {
                        description: format!("{} disk", *self.name),
                        disk_backend: DiskBackend::Distributed(DiskSource::Image {
                            image_id: image_view.id,
                        }),
                        name: format!("{}-disk", *self.name)
                            .parse()
                            .expect("valid disk name"),
                        size: self.size.clone(),
                    })
                    .external_ips(vec![ExternalIpCreate::Ephemeral { pool: None }])
                    .hostname(self.hostname.clone())
                    .memory(self.memory.clone())
                    .ncpus(self.ncpus.clone())
                    .start(self.start)
            })
            .send()
            .await?;

        println_nopipe!("instance {} created", instance.id);

        Ok(())
    }
}
