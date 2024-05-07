# The Oxide Rust SDK

Generated bindings for the Oxide API.

## Installation

The `oxide` crate is available on crates.io. You'll probably want to use `tokio` as well. Add them to your `Cargo.toml` file
or use `cargo add`:

```console
$ cargo add oxide
$ cargo add tokio
```

## Authentication

To connect to the Oxide API, the SDK needs a host URL and a token. There are
two ways to specify these:

* Environment variables: You can set the OXIDE_HOST and OXIDE_TOKEN environment
  variables.
* Configuration file: When you run oxide auth login in the CLI, a
  $HOME/.config/oxide/hosts.toml file is generated. This file contains
  sensitive information like your token and user ID.

The SDK will first look for the environment variables, and if they are not
defined, it will look for the config file.

## Example

Create a new `oxide::Client` like this:

```rust ,no_run
use futures::StreamExt;
use oxide::{config::Config, context::Context, prelude::*};

#[tokio::main]
async fn main() {
    // Get a client from the on-disk configuration.
    let context = Context::new(Config::default()).expect("unabled to create context");
    let client: &Client = context.client().expect("unable to get client");

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
