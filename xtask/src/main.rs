// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use std::{fs::File, path::PathBuf, time::Instant};

use progenitor::{GenerationSettings, Generator, TagStyle};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 || args.get(1).map(String::as_str) != Some("generate") {
        usage();
    }

    generate();
}

fn usage() -> ! {
    todo!()
}

// TODO flag to --check the generated file that we can use in CI to keep people
// from modifying the generated file by hand or forgetting to update it.
fn generate() {
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

    // TODO I'd like to generate a hash as well to have a way to check if the
    // spec has changed since the last generation.

    let code = generator.generate_text(&spec).unwrap();

    let contents = format!(
        "// The contents of this file are generated; do not modify them.\n\n{}",
        code
    );

    let mut out_path = root_path.clone();
    out_path.push("sdk");
    out_path.push("src");
    out_path.push("generated_sdk.rs");

    std::fs::write(out_path, contents).unwrap();

    // CLI
    let code = generator.cli_text(&spec, "oxide_api").unwrap();

    let contents = format!(
        "{}\n\n{}\n\n{}",
        "// The contents of this file are generated; do not modify them.",
        "use oxide_api::*;",
        code,
    );

    let mut out_path = root_path;
    out_path.push("cli");
    out_path.push("src");
    out_path.push("generated_cli.rs");

    let loc = contents.matches('\n').count();
    std::fs::write(out_path, contents).unwrap();
    let duration = Instant::now().duration_since(start).as_millis();
    println!(
        "generation took {}ms ({:.3}ms per line)",
        duration,
        duration as f64 / loc as f64,
    );
}
