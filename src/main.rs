use std::sync::OnceLock;

mod configuration;
mod mqtt;

use crate::configuration::Settings;

fn settings() -> &'static Settings {
    static SETTINGS: OnceLock<Settings> = OnceLock::new();

    SETTINGS.get_or_init(|| Settings::new().unwrap())
}

#[tokio::main]
async fn main() {
    let settings: &Settings = settings();

    mqtt::run(settings.mqtt)
}
