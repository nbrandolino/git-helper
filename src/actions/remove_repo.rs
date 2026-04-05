use crate::config::{read_config, write_config};
use colored::Colorize;
use std::path::Path;

pub fn remove_repo(repo_identifier: &str, config_path: &Path) {
    let mut config = read_config(config_path);

    // full path
    if config.repositories.remove(repo_identifier) {
        write_config(config_path, &config);
        println!("{}", format!("✔ Removed repository: {}", repo_identifier).green());
        return;
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
        }
        1 => {
            config.repositories.remove(&matches[0]);
            write_config(config_path, &config);
            println!("{}", format!("✔ Removed repository: {}", matches[0]).green());
        }
        _ => {
            eprintln!("{}", format!(
                "❌ Ambiguous identifier '{}' matches multiple repositories. Please use the full path:",
                repo_identifier
            ).red());
            for path in &matches {
                eprintln!("   - {}", path);
            }
        }
    }
}
