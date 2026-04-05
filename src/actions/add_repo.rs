use crate::config::{read_config, write_config};
use crate::utils::{expand_path, validate_git_repo};
use colored::Colorize;
use std::path::Path;

pub fn add_repo(repo_path: &str, config_path: &Path) {
    let expanded_path = match expand_path(repo_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", format!("❌ {}", e).red());
            return;
        }
    };
    if let Err(err) = validate_git_repo(&expanded_path) {
        eprintln!("{}", format!("❌ Failed to add repository: {}", err).red());
        return;
    }

    let mut config = read_config(config_path);
    if !config.repositories.insert(expanded_path.to_string_lossy().to_string()) {
        println!("{}", format!("⚠ Repository already exists: {}", repo_path).yellow());
        return;
    }

    write_config(config_path, &config);
    println!("{}", format!("✔ Added repository: {}", expanded_path.display()).green());
}
