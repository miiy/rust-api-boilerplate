use config::{ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub app: App,
    pub server: Server,
    pub database: Database,
    pub redis: Redis,
    pub jwt: JWT,
}

#[derive(Clone, Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub url: String,
    pub key: String,
    pub debug: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub addrs: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Redis {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JWT {
    pub secret: String,
    pub expires_in: i64,
    pub encryption_key: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = config::Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
