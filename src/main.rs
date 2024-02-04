mod settings;

use settings::Settings;

fn main() {
    let settings: Result<Settings, config::ConfigError> = Settings::new();

    println!("{:?}", settings);
}
