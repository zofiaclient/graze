use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    message: String,
    amount: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            message: "Hello, world!".to_string(),
            amount: 3,
        }
    }
}

fn load_config() -> graze::Result<Config, toml::de::Error> {
    graze::load_or_write_default(
        "Config.toml",
        |s| toml::from_str(s),
        |c| toml::to_string(c).unwrap(),
        Config::default,
    )
}

fn main() {
    match load_config() {
        Ok(config) => {
            for _ in 0..config.amount {
                println!("{}", config.message);
            }
        }
        Err(err) => {
            eprintln!("An error occurred while loading the configuration");
            eprintln!("{err}");
        }
    }
}
