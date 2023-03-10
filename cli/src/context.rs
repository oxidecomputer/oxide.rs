use anyhow::{anyhow, Result};
use oxide_api::Client;

use crate::config::Config;

pub struct Context {
    pub client: Client,
    pub config: Config,
}

impl Context {
    pub fn new(config: Config) -> Result<Self> {
        let Some((hostname, host)) = config.hosts.hosts.iter().next() else {
            return Err(anyhow!("no authenticated hosts"));
        };

        let auth = format!("Bearer {}", &host.token);
        let dur = std::time::Duration::from_secs(15);
        let rclient = reqwest::Client::builder()
            .connect_timeout(dur)
            .timeout(dur)
            .default_headers(
                [(http::header::AUTHORIZATION, auth.try_into().unwrap())]
                    .into_iter()
                    .collect(),
            )
            .build()
            .unwrap();
        let client = oxide_api::Client::new_with_client(hostname, rclient);

        println!("{:#?}:", client);

        Ok(Self { client, config })
    }
}
