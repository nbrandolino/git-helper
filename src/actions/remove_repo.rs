use crate::config::{read_config, write_config};
use colored::Colorize;
use std::path::Path;

/// Removes a repository by full path or repository name
pub fn main(repo_identifier: &str, config_path: &Path) {
    let mut config = read_config(config_path);

    // Directly remove if it exists as a full path
    if config.repositories.remove(repo_identifier) {
        write_config(config_path, &config);
        println!("{}", format!("✔ Removed repository: {}", repo_identifier).green());
        return;
    }

    // Otherwise, search for a matching repository name
    let repo_to_remove = config.repositories
        .iter()
        .find(|repo_path| Path::new(repo_path)
        .file_name()
        .map(|name| name.to_string_lossy()) == Some(repo_identifier.into()))
        .cloned(); // Clone the path to avoid borrowing issues

    if let Some(repo_path) = repo_to_remove {
        config.repositories.remove(&repo_path); // Now we mutate config.repositories safely
        write_config(config_path, &config);
        println!("{}", format!("✔ Removed repository: {}", repo_path).green());
    } else {
        eprintln!("{}", format!("❌ Repository not found: {}", repo_identifier).red());
    }
}
