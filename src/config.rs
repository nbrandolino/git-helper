use std::fs;
use std::io::ErrorKind;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub repositories: HashSet<String>,
}

pub fn read_config(config_path: &Path) -> Result<Config, String> {
    match fs::read_to_string(config_path) {
        Ok(content) => toml::from_str(&content)
            .map_err(|e| format!("❌ Error parsing config file '{}': {}", config_path.display(), e)),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(Config::default()),
        Err(e) => Err(format!("❌ Could not read config file '{}': {}", config_path.display(), e)),
    }
}

pub fn write_config(config_path: &Path, config: &Config) -> Result<(), String> {
    let content = toml::to_string(config)
        .map_err(|e| format!("Failed to serialize configuration: {}", e))?;
    fs::write(config_path, content)
        .map_err(|e| format!("Failed to write configuration: {}", e))
}
