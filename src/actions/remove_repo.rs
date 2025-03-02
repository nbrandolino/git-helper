use crate::config::{read_config, write_config};
use colored::Colorize;
use std::path::Path;

// remove repo
pub fn main(repo_identifier: &str, config_path: &Path) {
    let mut config = read_config(config_path);

    // full path
    if config.repositories.remove(repo_identifier) {
        write_config(config_path, &config);
        println!("{}", format!("✔ Removed repository: {}", repo_identifier).green());
        return;
    }

    // repo name
    let repo_to_remove = config.repositories
        .iter()
        .find(|repo_path| Path::new(repo_path)
        .file_name()
        .map(|name| name.to_string_lossy()) == Some(repo_identifier.into()))
        .cloned();

    if let Some(repo_path) = repo_to_remove {
        config.repositories.remove(&repo_path);
        write_config(config_path, &config);
        println!("{}", format!("✔ Removed repository: {}", repo_path).green());
    } else {
        eprintln!("{}", format!("❌ Repository not found: {}", repo_identifier).red());
    }
}
