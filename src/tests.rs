use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct Config {
    range: usize,
}

#[test]
fn load_or_write_default() {
    let config = crate::load_or_write_default(
        "Config1.toml",
        |s| toml::from_str(s),
        |c| toml::to_string(c).unwrap(),
        Config::default,
    );
    assert!(config.is_ok());
}

#[test]
fn load_or_default() {
    let config = crate::load_or_default("Config2.toml", |s| toml::from_str(s), Config::default);
    assert!(config.is_ok());
}

#[test]
fn load() {
    let content = toml::to_string(&Config::default()).expect("Could not convert config to string");
    fs::write("Config3.toml", content).expect("Could not write default config to path");

    let config: Result<Config, _> = crate::load_from_path("Config3.toml", |s| toml::from_str(s));
    assert!(config.is_ok());
}
