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
struct Mqtt {
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    database: Database,
    mqtt: Mqtt,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let run_mode: String = env::var("SERVICE_PROFILE")
            .unwrap_or_else(|_| "development".into());

        let settings: Config = Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(config::Environment::with_prefix("refinery"))
            .build()?;

        settings.try_deserialize()
    }
}
