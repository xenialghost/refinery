use std::sync::OnceLock;

mod settings;

use settings::Settings;

fn config() -> &'static Settings {
    static SETTINGS: OnceLock<Settings> = OnceLock::new();

    SETTINGS.get_or_init(|| Settings::new().unwrap())
}

fn main() {
    println!("{:?}", config());
}
