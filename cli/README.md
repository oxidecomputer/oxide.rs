# The Oxide CLI

## Installation

You can find pre-built binaries for various platforms
[here](https://github.com/oxidecomputer/oxide.rs/releases). Look for the most
recent release whose version suffix matches the version of Oxide software
currently deployed. For example, if you're running the `20240502.0` release,
use `0.5.0+20240502.0`.

You can build the CLI yourself using `cargo build --bin oxide`; then add the
executable in `target/debug` to your `PATH` or copy it to another location.

## Authentication

To authenticate, use `oxide auth login --host my.oxide.host`. This will add a
new profile to `$HOME/.config/oxide/credentials.toml` (and create the file if
necessary). The profile will derive its name from the name of the Oxide Silo.

The first time you authenticate, the `default-profile` will be set in
`$HOME/.config/oxide/config.toml`. Edit that file to change the default later.

If you have a token value, you may set the `OXIDE_HOST` and `OXIDE_TOKEN`
environment variables. This method can be useful for service accounts.
