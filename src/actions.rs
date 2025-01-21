use crate::config::{read_config, write_config};
use super::utils::{expand_path, validate_git_repo};
use std::path::Path;

// add repo to config file
pub fn add_repo(repo_path: &str, config_path: &Path) {
    let expanded_path = expand_path(repo_path);
    if let Err(err) = validate_git_repo(&expanded_path) {
        eprintln!("Failed to add repository: {}", err);
        return;
    }

    let mut config = read_config(config_path);
    if !config.repositories.insert(expanded_path.to_string_lossy().to_string()) {
        println!("Repository already exists: {}", repo_path);
        return;
    }

    write_config(config_path, &config);
    println!("Added repository: {}", expanded_path.display());
}

// remove repo from config file
pub fn remove_repo(repo_identifier: &str, config_path: &Path) {
    let mut config = read_config(config_path);
    if config.repositories.remove(repo_identifier) {
        write_config(config_path, &config);
        println!("Removed repository: {}", repo_identifier);
    }
    else {
        eprintln!("Repository not found: {}", repo_identifier);
    }
}

// list repos in config file
pub fn list_repos(config_path: &Path) {
    let config = read_config(config_path);
    if config.repositories.is_empty() {
        println!("No repositories configured.");
    }
    else {
        println!("Configured repositories:");
        for repo in &config.repositories {
            println!("- {}", repo);
        }
    }
}

// pull all repos in config file
pub fn pull_all(repo_path: &str) {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("Cannot pull repository: {}", err);
        return;
    }

    println!("Pulling repository at: {}", repo_path);

    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("pull")
        .arg("--all")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output();

    match output {
        Ok(status) if status.status.success() => {
            println!("Successfully pulled: {}", repo_path);
        }
        Ok(_) => {
            eprintln!("Failed to pull repository at: {}", repo_path);
        }
        Err(err) => {
            eprintln!("Error pulling repository at {}: {:?}", repo_path, err);
        }
    }
}

// fetch all repos in config file
pub fn fetch_all(repo_path: &str) {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("Cannot fetch repository: {}", err);
        return;
    }

    println!("Fetching repository at: {}", repo_path);

    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("fetch")
        .arg("--all")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output();

    match output {
        Ok(status) if status.status.success() => {
            println!("Successfully fetched: {}", repo_path);
        }
        Ok(_) => {
            eprintln!("Failed to fetch repository at: {}", repo_path);
        }
        Err(err) => {
            eprintln!("Error fetching repository at {}: {:?}", repo_path, err);
        }
    }
}
