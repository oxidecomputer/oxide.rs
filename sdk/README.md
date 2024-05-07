# The Oxide Rust SDK

Generated bindings for the Oxide API.

## Installation

The `oxide` crate is available on crates.io. You'll probably want to use `tokio` as well. Add them to your `Cargo.toml` file
or use `cargo add`:

```console
$ cargo add oxide
$ cargo add tokio
```

## Example

Create a new `oxide::Client` like this:

```rust
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