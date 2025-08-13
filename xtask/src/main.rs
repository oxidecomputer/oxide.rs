// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2025 Oxide Computer Company

#![forbid(unsafe_code)]

use std::{collections::HashSet, fs::File, io::Write, path::PathBuf, time::Instant};

use clap::Parser;
use newline_converter::dos2unix;
use proc_macro2::TokenStream;
use progenitor::{GenerationSettings, Generator, TagStyle};
use quote::{quote, ToTokens};
use similar::{Algorithm, ChangeTag, TextDiff};
use syn::{
    GenericArgument, ImplItem, Item, ItemImpl, PathArguments, PathSegment, ReturnType, Type,
};

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
        let contents = format_code(code.clone().to_string());
        loc += contents.matches('\n').count();

        let mut out_path = root_path.clone();
        out_path.push("sdk");
        out_path.push("src");
        out_path.push("generated_sdk.rs");

        error |= output_contents(check, out_path, contents, verbose).is_err();

        let ret_types = generate_return_types(code);
        let ret_contents = format_code(ret_types.to_string());
        loc += ret_contents.matches('\n').count();

        let mut ret_out_path = root_path.clone();
        ret_out_path.push("cli");
        ret_out_path.push("tests");
        ret_out_path.push("data");
        ret_out_path.push("api_return_types.rs");

        error |= output_contents(check, ret_out_path, ret_contents, verbose).is_err();
    }

    // SDK httpmock library
    if httpmock {
        print!("generating httpmock ... ");
        std::io::stdout().flush().unwrap();
        let code = generator.httpmock(&spec, "oxide").unwrap().to_string();
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
        let code = generator.cli(&spec, "oxide").unwrap().to_string();
        let contents = format_code(code);
        loc += contents.matches('\n').count();

        let mut out_path = root_path.clone();
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

/// For testing purposes, generate the schemas for all unique types returned by the
/// `send` method.
fn generate_return_types(tokens: TokenStream) -> TokenStream {
    let file: syn::File = syn::parse2(tokens).unwrap();

    // Get the `builder` mod.
    let builder = file
        .items
        .iter()
        .filter_map(|i| match i {
            Item::Mod(module) if module.ident == "builder" => Some(module),
            _ => None,
        })
        .next()
        .unwrap();

    let Some((_, builder_content)) = &builder.content else {
        unreachable!("no items in mod 'builder'");
    };

    let mut return_types = Vec::new();
    let mut types_as_string = HashSet::new();

    // Find inherent impl blocks for structs, then find the return types from `fn send`.
    for impl_item in builder_content.iter().filter_map(|i| match i {
        Item::Impl(ItemImpl {
            items,
            trait_: None,
            ..
        }) => Some(items),
        _ => None,
    }) {
        for method in impl_item.iter().filter_map(|i| match i {
            ImplItem::Fn(method) if method.sig.ident == "send" => Some(method),
            _ => None,
        }) {
            if let ReturnType::Type(_, ty) = &method.sig.output {
                if let Some(return_type) = extract_response_value_inner_type(ty) {
                    // We want to remove duplcates, but `TokenStream` cannot be inserted
                    // into a `HashSet`. Convert to a `String`.
                    if types_as_string.insert(return_type.to_string()) {
                        return_types.push(return_type);
                    }
                }
            }
        }
    }

    let schemas = return_types.into_iter().map(|ty| {
        quote! {
            (stringify!(oxide::#ty), schemars::schema_for!(oxide::#ty))
        }
    });

    quote! {
        pub fn schemas() -> Vec<(&'static str, schemars::schema::RootSchema)> {
            vec![
                #(#schemas),*
            ]
        }
    }
}

/// Extract the Oxide success type returned by `send`.
/// For example, `types::Vpc` in `Result<ResponseValue<types::Vpc>, Error>`.
fn extract_response_value_inner_type(ty: &Type) -> Option<TokenStream> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    // Extract `Result<ResponseValue<T>, _>`.
    let result_args = type_path
        .path
        .segments
        .last()
        .filter(|s| s.ident == "Result")
        .and_then(|s| match &s.arguments {
            PathArguments::AngleBracketed(args) => Some(args),
            _ => None,
        })?;

    // Extract `ResponseValue<T>`.
    let GenericArgument::Type(Type::Path(ok_path)) = result_args.args.first()? else {
        return None;
    };
    let response_args = ok_path
        .path
        .segments
        .last()
        .filter(|s| s.ident == "ResponseValue")
        .and_then(|s| match &s.arguments {
            PathArguments::AngleBracketed(args) => Some(args),
            _ => None,
        })?;

    // Extract inner type `T`.
    let GenericArgument::Type(Type::Path(inner_path)) = response_args.args.first()? else {
        return None;
    };

    // Handle container types (e.g., `Vec<types::Item>`) or direct `types::Item`.
    match inner_path.path.segments.last()? {
        PathSegment {
            arguments: PathArguments::AngleBracketed(args),
            ..
        } => {
            // Container type - extract the inner type.
            let GenericArgument::Type(Type::Path(contained)) = args.args.first()? else {
                return None;
            };
            // Verify it starts with "types::".
            contained
                .path
                .segments
                .first()
                .filter(|s| s.ident == "types")
                .map(|_| contained.path.to_token_stream())
        }
        PathSegment {
            arguments: PathArguments::None,
            ..
        } => {
            // Direct type - verify it starts with "types::".
            inner_path
                .path
                .segments
                .first()
                .filter(|s| s.ident == "types")
                .map(|_| inner_path.path.to_token_stream())
        }
        _ => None,
    }
}

fn format_code(code: String) -> String {
    let contents = format!(
        "// The contents of this file are generated; do not modify them.\n\n{}",
        code,
    );
    let contents = match rustfmt_wrapper::rustfmt_config(
        rustfmt_wrapper::config::Config {
            format_strings: Some(true),
            normalize_doc_attributes: Some(true),
            wrap_comments: Some(true),
            ..Default::default()
        },
        contents,
    ) {
        Ok(contents) => contents,
        Err(rustfmt_wrapper::Error::Unstable(msg)) => {
            // rustfmt_wrapper uses `toolchain_find` to get a recent rustfmt.
            // `toolchain_find`, then, picks the latest version (by date
            // reported in `<component> -V`) as the path to return. This means
            // that not only does `cargo xtask generate` require *a* nightly to
            // be installed, it requires the installed nightly to be more recent
            // than the latest stable toolchain.
            //
            // This is subtle, but it means that upgrading the stable toolchain
            // used in oxide.rs can break some developers' environments by
            // moving stable to a more recent version than their latest nightly.
            //
            // Even if that is not the case, one may reasonably use a nightly
            // toolchain to `cargo +nightly xtask generate`, only for the most
            // recent toolchain to be stable and for us to error below.
            //
            // Hopefully this is enough information to help users figure out
            // what to investigate with their local toolchains, or if a nightly
            // is not installed at all.
            let rustc_version = rustc_version::version_meta().expect("can get rustc version meta");
            panic!(
                "\"generate\" xtask requires a nightly rustfmt more recent \
                than {} to be installed, as it uses the following features: {}",
                rustc_version
                    .commit_date
                    .expect("version has a commit date"),
                msg
            );
        }
        Err(other) => {
            panic!("unexpected issue formatting generated code: {}", other);
        }
    };
    let contents = dos2unix(&contents);

    // Add newlines after end-braces at <= two levels of indentation. Rustfmt's
    // `blank_lines_lower_bound` is broken.
    let regex = regex::Regex::new(r"(\n\s*})(\n\s{0,8}[^} ])").unwrap();
    let contents = regex.replace_all(&contents, "$1\n$2");

    let regex = regex::Regex::new(r"(\n\s*///)(\S)").unwrap();
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
