use crate::config::{read_config, write_config};
use crate::utils::expand_path;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn main(directory: &str, config_path: &Path) {
    let dir_path = expand_path(directory);

    if !dir_path.is_dir() {
        eprintln!("{}", format!("❌ Error: '{}' is not a valid directory.", directory).red());
        return;
    }

    let mut config = read_config(config_path);
    let mut found_repos = 0;

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join(".git").exists() {
                let repo_path = path.to_string_lossy().to_string();
                if config.repositories.insert(repo_path.clone()) {
                    println!("{}", format!("✔ Added Git repository: {}", repo_path).green());
                    found_repos += 1;
                }
            }
        }
    }

    if found_repos > 0 {
        write_config(config_path, &config);
    } else {
        println!("{}", format!("⚠ No new Git repositories found.").yellow());
    }
}
