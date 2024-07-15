// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use clap_complete::{generate, Shell};
use std::io;

/// Generate shell completion scripts for Oxide CLI commands.
///
/// This command generates scripts for various shells that can be used to
/// enable completion.
///
/// ## Installation
///
/// ### Bash
///
/// Add this to your `~/.bash_profile`:
///
/// ```sh
/// eval "$(oxide completion -s bash)"
/// ```
///
/// ### Zsh
///
/// Generate an `_oxide` completion script and put it somewhere in your
/// `$fpath`, for example:
///
/// ```sh
/// oxide completion -s zsh > ~/.zfunc/_oxide
/// ```
///
/// and check that you have the following lines in your `~/.zshrc`:
///
/// ```sh
/// autoload -U compinit
/// compinit -i
/// ```
///
/// ### Fish
///
/// Generate an `oxide.fish` completion script:
///
/// ```sh
/// oxide completion -s fish > ~/.config/fish/completions/oxide.fish
/// ```
///
/// ### PowerShell
///
/// Open your profile script with:
///
/// ```sh
/// mkdir -Path (Split-Path -Parent $profile)
/// notepad $profile
/// ```
///
/// Add the following line and save the file:
///
/// ```powershell
/// Invoke-Expression -Command $(oxide completion -s powershell | Out-String)
/// ```
///
/// ### Elvish
///
/// Generate an `oxide.elv` completion script and put it in a module search
/// directory, for example:
///
/// ```sh
/// oxide completion -s elvish > ~/.local/share/elvish/lib/oxide.elv
/// ```
///
/// and import this by adding the following to `~/.config/elvish/rc.elv`
///
/// ```
/// use oxide
/// ```
#[derive(Parser, Debug, Clone)]
#[command(verbatim_doc_comment)]
#[command(name = "generate-completions")]
pub struct CmdCompletion {
    /// Shell type
    #[clap(short, long)]
    shell: Shell,
}

#[async_trait]
impl crate::RunnableCmd for CmdCompletion {
    async fn run(&self, _: &crate::Context) -> Result<()> {
        let cli = crate::make_cli();
        let mut cmd = cli.command_take();
        let name = cmd.get_name().to_string();
        generate(self.shell, &mut cmd, name, &mut io::stdout());

        Ok(())
    }
}
