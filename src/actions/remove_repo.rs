use crate::config::{read_config, write_config};
use colored::Colorize;
use std::path::Path;

pub fn remove_repo(repo_identifier: &str, config_path: &Path, quiet: bool) -> bool {
    let mut config = read_config(config_path);

    // full path
    if config.repositories.remove(repo_identifier) {
        if let Err(err) = write_config(config_path, &config) {
            eprintln!("{}", format!("❌ {}", err).red());
            return false;
        }
        if !quiet {
            println!("{}", format!("✔ Removed repository: {}", repo_identifier).green());
        }
        return true;
    }

    let matches: Vec<String> = config.repositories
        .iter()
        .filter(|repo_path| Path::new(repo_path)
            .file_name()
            .map(|name| name.to_string_lossy() == repo_identifier)
            .unwrap_or(false))
        .cloned()
        .collect();

    match matches.len() {
        0 => {
            eprintln!("{}", format!("❌ Repository not found: {}", repo_identifier).red());
            false
        }
        1 => {
            config.repositories.remove(&matches[0]);
            if let Err(err) = write_config(config_path, &config) {
                eprintln!("{}", format!("❌ {}", err).red());
                return false;
            }
            if !quiet {
                println!("{}", format!("✔ Removed repository: {}", matches[0]).green());
            }
            true
        }
        _ => {
            eprintln!("{}", format!(
                "❌ Ambiguous identifier '{}' matches multiple repositories. Please use the full path:",
                repo_identifier
            ).red());
            for path in &matches {
                eprintln!("   - {}", path);
            }
            false
        }
    }
}
