mod configuration;
mod startup;
mod messages;

use crate::configuration::Settings;

#[tokio::main]
async fn main() {
    let settings: Settings = Settings::new().unwrap();

    startup::run(settings).await;
}
