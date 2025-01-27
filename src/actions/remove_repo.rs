use crate::config::{read_config, write_config};
use std::path::Path;

// remove repo from config file
pub fn main(repo_identifier: &str, config_path: &Path) {
    let mut config = read_config(config_path);
    if config.repositories.remove(repo_identifier) {
        write_config(config_path, &config);
        println!("Removed repository: {}", repo_identifier);
    }
    else {
        eprintln!("Repository not found: {}", repo_identifier);
    }
}
