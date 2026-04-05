use colored::Colorize;
use std::fs;
use std::io::ErrorKind;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub repositories: HashSet<String>,
}

pub fn read_config(config_path: &Path) -> Config {
    match fs::read_to_string(config_path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{}", format!("❌ Error parsing config file '{}': {}", config_path.display(), e).red());
                std::process::exit(1);
            }
        },
        Err(e) if e.kind() == ErrorKind::NotFound => {
            Config::default()
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Could not read config file '{}': {}", config_path.display(), e).red());
            std::process::exit(1);
        }
    }
}

pub fn write_config(config_path: &Path, config: &Config) {
    let content = match toml::to_string(config) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{}", format!("❌ Failed to serialize configuration: {}", err).red());
            return;
        }
    };
    if let Err(err) = fs::write(config_path, content) {
        eprintln!("{}", format!("❌ Failed to write configuration: {}", err).red());
    }
}
