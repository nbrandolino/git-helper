use crate::config::{read_config, write_config};
use crate::utils::expand_path;
use std::fs;
use std::path::Path;

// scan dir for repos
pub fn main(scan_path: &str, config_path: &Path) {
    let expanded_path = expand_path(scan_path);

    if !expanded_path.exists() || !expanded_path.is_dir() {
        eprintln!("Invalid path: {}. Please provide a valid directory.", expanded_path.display());
        return;
    }

    println!("Scanning for Git repositories in: {}", expanded_path.display());

    let mut config = read_config(config_path);
    let mut new_repos = Vec::new();

    if let Ok(entries) = fs::read_dir(expanded_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if is_git_repo(&path) {
                let repo_path = path.to_string_lossy().to_string();
                if config.repositories.insert(repo_path.clone()) {
                    new_repos.push(repo_path);
                }
            }
        }
    }

    if new_repos.is_empty() {
        println!("No new repositories detected.");
    }
    else {
        write_config(config_path, &config);
        println!("Added {} new repositories:", new_repos.len());
        for repo in new_repos {
            println!("- {}", repo);
        }
    }
}

// checks if git repo
fn is_git_repo(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }
    let git_dir = path.join(".git");
    git_dir.exists() && git_dir.is_dir()
}
