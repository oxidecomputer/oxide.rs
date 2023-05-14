// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

#![forbid(unsafe_code)]

use std::{fs::File, io::Write, path::PathBuf, time::Instant};

use clap::Parser;
use newline_converter::dos2unix;
use progenitor::{GenerationSettings, Generator, TagStyle};
use similar::{Algorithm, ChangeTag, TextDiff};

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "build tasks")]
enum Xtask {
    #[command(about = "generate CLI and SDK code from oxide.json")]
    Generate {
        #[clap(long)]
        check: bool,
        #[clap(short = 'v', long)]
        verbose: bool,

        #[clap(long)]
        sdk: bool,
        #[clap(long)]
        cli: bool,
        #[clap(long)]
        httpmock: bool,
    },
}

fn main() -> Result<(), String> {
    let xtask = Xtask::parse();

    match xtask {
        Xtask::Generate {
            check,
            verbose,
            sdk,
            cli,
            httpmock,
        } => generate(check, verbose, sdk, cli, httpmock),
    }
}

// TODO flag to --check the generated file that we can use in CI to keep people
// from modifying the generated file by hand or forgetting to update it.
fn generate(
    check: bool,
    verbose: bool,
    mut sdk: bool,
    mut cli: bool,
    mut httpmock: bool,
) -> Result<(), String> {
    if !(sdk || cli || httpmock) {
        (sdk, cli, httpmock) = (true, true, true);
    }

    let start = Instant::now();
    let xtask_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_path = xtask_path.parent().unwrap().to_path_buf();
    let mut spec_path = root_path.clone();
    spec_path.push("oxide.json");

    let file = File::open(spec_path).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(progenitor::InterfaceStyle::Builder)
            .with_tag(TagStyle::Separate)
            .with_derive("schemars::JsonSchema"),
    );

    let mut error = false;
    let mut loc = 0;

    // TODO I'd like to generate a hash as well to have a way to check if the
    // spec has changed since the last generation.

    // SDK
    if sdk {
        print!("generating sdk ... ");
        std::io::stdout().flush().unwrap();

        let code = generator.generate_tokens(&spec).unwrap();
        let contents = format_code(code.to_string());
        loc += contents.matches('\n').count();

        let mut out_path = root_path.clone();
        out_path.push("sdk");
        out_path.push("src");
        out_path.push("generated_sdk.rs");

        error |= output_contents(check, out_path, contents, verbose).is_err();
    }

    // SDK httpmock library
    if httpmock {
        print!("generating httpmock ... ");
        std::io::stdout().flush().unwrap();
        let code = generator.httpmock(&spec, "oxide_api").unwrap().to_string();
        let contents = format_code(code);
        loc += contents.matches('\n').count();

        let mut out_path = root_path.clone();
        out_path.push("sdk-httpmock");
        out_path.push("src");
        out_path.push("generated_httpmock.rs");

        error |= output_contents(check, out_path, contents, verbose).is_err();
    }

    // CLI
    if cli {
        print!("generating cli ... ");
        std::io::stdout().flush().unwrap();
        let code = generator.cli(&spec, "oxide_api").unwrap().to_string();
        let contents = format_code(format!("{}\n\n{}", "use oxide_api::*;", code));
        loc += contents.matches('\n').count();

        let mut out_path = root_path;
        out_path.push("cli");
        out_path.push("src");
        out_path.push("generated_cli.rs");

        error |= output_contents(check, out_path, contents, verbose).is_err();
    }

    let duration = Instant::now().duration_since(start).as_micros();
    println!(
        "generation took {:.3}s ({}us per line)",
        duration as f64 / 1_000_000_f64,
        duration / loc as u128,
    );

    if error {
        Err("Generated code is out of date relative to oxide.json".to_string())
    } else {
        Ok(())
    }
}

fn format_code(code: String) -> String {
    let contents = format!(
        "// The contents of this file are generated; do not modify them.\n\n{}",
        code,
    );
    let contents = rustfmt_wrapper::rustfmt_config(
        rustfmt_wrapper::config::Config {
            format_strings: Some(true),
            normalize_doc_attributes: Some(true),
            wrap_comments: Some(true),
            ..Default::default()
        },
        contents,
    )
    .unwrap();
    let contents = dos2unix(&contents);

    // Add newlines after end-braces at <= two levels of indentation. Rustfmt's
    // `blank_lines_lower_bound` is broken.
    let regex = regex::Regex::new(r#"(})(\n\s{0,8}[^} ])"#).unwrap();
    let contents = regex.replace_all(&contents, "$1\n$2");

    let regex = regex::Regex::new(r#"(\n\s*///)(\S)"#).unwrap();
    regex.replace_all(&contents, "$1 $2").to_string()
}

fn output_contents(
    check: bool,
    out_path: PathBuf,
    contents: String,
    verbose: bool,
) -> Result<(), ()> {
    if check {
        let checked_in = std::fs::read_to_string(out_path).unwrap();
        let checked_in = dos2unix(&checked_in);
        if checked_in != contents {
            println!("❌");
            if verbose {
                show_diff(&checked_in, &contents);
            }
            return Err(());
        } else {
            println!("👍");
        }
    } else {
        std::fs::write(out_path, &contents).unwrap();
        println!("done.");
    }

    Ok(())
}

fn show_diff(expected: &str, actual: &str) {
    for hunk in TextDiff::configure()
        .algorithm(Algorithm::Myers)
        .diff_lines(expected, actual)
        .unified_diff()
        .context_radius(5)
        .iter_hunks()
    {
        println!("{}", hunk.header());
        for change in hunk.iter_changes() {
            let marker = match change.tag() {
                ChangeTag::Delete => '-',
                ChangeTag::Insert => '+',
                ChangeTag::Equal => ' ',
            };
            print!("{}{}", marker, change);
            if change.missing_newline() {
                println!();
            }
        }
    }
}
