use crate::config::read_config;
use colored::Colorize;
use std::path::Path;

pub fn list_repos(config_path: &Path, quiet: bool) {
    if quiet {
        return;
    }

    let config = match read_config(config_path) {
        Ok(c) => c,
        Err(e) => { eprintln!("{}", e.red()); return; }
    };
    if config.repositories.is_empty() {
        println!("{}", "⚠ No repositories configured.".yellow());
    } else {
        println!("Configured repositories:");
        for repo in &config.repositories {
            println!("- {}", repo);
        }
    }
}
