# The Oxide CLI

## Installation

Build with `cargo build --bin oxide` and add the executable in `target/debug` to your `PATH`.

## Authentication

There are two ways to authenticate against the Oxide rack using the CLI:

- Environment variables: You can set the `OXIDE_HOST` and `OXIDE_TOKEN` environment variables. This method is useful for service accounts.

- Configuration file: When running the `oxide auth login` command, a `$HOME/.config/oxide/hosts.toml` file is generated. This file contains sensitive information like your token and user ID.
