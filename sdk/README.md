# The Oxide Rust SDK

SDK for the Oxide API.

## Installation

The `oxide` crate is available on crates.io. You'll probably want to use
`tokio` as well. Add them to your `Cargo.toml` file or use `cargo add`:

```console
$ cargo add oxide
$ cargo add tokio
```

## Authentication

To connect to the Oxide API, the SDK needs a host URL and a token. There are
several ways to specify these:

* Configuration files: the CLI's `oxide auth login` command generates
  `config.toml` and `credentials.toml` in `$HOME/.config/oxide/`. The
  credentials file contains sensitive information such as tokens and user IDs.
* Environment variables: You can set the `OXIDE_HOST` and `OXIDE_TOKEN`
  environment variables.
* Explicit host URL and token.

The simplest way to create an authenticated client is to use
`oxide::Client::new_authenticated()`, which uses the same credentials and
authentication logic as the CLI. By default, it reads data from configuration
files in `$HOME/.config/oxide`.

## Example

Create a new `oxide::Client` like this:

```rust ,no_run
use futures::StreamExt;
use oxide::{Client, prelude::*};

#[tokio::main]
async fn main() {
    // Make a client from the on-disk configuration.
    let client = Client::new_authenticated()
        .expect("unable to create an authenticated client");

    // Start using the client!

    // For example we can look at the projects in our silo:
    let mut projects = client.project_list().stream();
    loop {
        match projects.next().await {
            // No more items.
            None => break,
            // Print the name of a project.
            Some(Ok(project)) => println!("project {}", *project.name),
            // Something went wrong
            Some(Err(err)) => println!("error {}", err),
        }
    }
}
```
