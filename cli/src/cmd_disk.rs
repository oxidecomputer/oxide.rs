// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use std::{io::Write, time::Duration};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
#[clap(name = "import")]
pub struct CmdDiskImport {}

impl CmdDiskImport {
    pub async fn run(&self, _ctx: &mut crate::context::Context) -> Result<()> {
        print!("importing disk .");
        std::io::stdout().flush()?;
        for _ in 0..20 {
            tokio::time::sleep(Duration::new(0, 200_000_000)).await;
            print!(".");
            std::io::stdout().flush()?;
        }

        println!(" done!");

        Ok(())
    }
}
