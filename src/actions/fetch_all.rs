use crate::utils::validate_git_repo;
use colored::Colorize;
use std::path::Path;

// fetch all repos in config file
pub fn main(repo_path: &str) {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("{}", format!("❌ Cannot fetch repository: {}", err).red());
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
            println!("{}", format!("✔ Successfully fetched: {}", repo_path).green());
        }
        Ok(_) => {
            eprintln!("{}", format!("❌ Failed to fetch repository at: {}", repo_path).red());
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error fetching repository at {}: {:?}", repo_path, err).red());
        }
    }
}
