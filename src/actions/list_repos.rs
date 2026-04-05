use crate::config::read_config;
use colored::Colorize;
use std::path::Path;

pub fn list_repos(config_path: &Path, quiet: bool) {
    if quiet {
        return;
    }

    let config = read_config(config_path);
    if config.repositories.is_empty() {
        println!("{}", "⚠ No repositories configured.".yellow());
    } else {
        println!("Configured repositories:");
        for repo in &config.repositories {
            println!("- {}", repo);
        }
    }
}
