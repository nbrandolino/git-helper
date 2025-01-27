use crate::utils::{validate_git_repo};
use std::path::Path;

// fetch all repos in config file
pub fn main(repo_path: &str) {
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
