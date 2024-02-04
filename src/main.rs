use std::sync::OnceLock;

mod settings;

use settings::Settings;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

fn main() {
    println!("{:?}", SETTINGS);
}
