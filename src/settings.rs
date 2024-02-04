use config::Config;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    database: Database,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let run_mode: String = env::var("SERVICE_PROFILE")
            .unwrap_or_else(|_| "development".into());

        let settings: Config = Config::builder()
            .add_source(config::File::with_name("src/config/default"))
            .add_source(config::File::with_name(&format!("src/config/{}", run_mode)).required(false))
            .add_source(config::Environment::with_prefix("refinery"))
            .build()?;

        settings.try_deserialize()
    }
}