use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub repositories: HashSet<String>,
}

// read config file
pub fn read_config(config_path: &Path) -> Config {
    match fs::read_to_string(config_path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error parsing config: {}", e);
                Config::default()
            }
        },
        Err(e) => {
            eprintln!("Error reading config file: {}", e);
            Config::default()
        }
    }
}

// write to config file
pub fn write_config(config_path: &Path, config: &Config) {
    let content = toml::to_string(config).expect("Failed to serialize configuration");
    if let Err(err) = fs::write(config_path, content) {
        eprintln!("Failed to write configuration: {}", err);
    }
}
