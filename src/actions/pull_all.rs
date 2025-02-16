use crate::utils::{validate_git_repo};
use colored::Colorize;
use std::path::Path;

// pull all repos in config file
pub fn main(repo_path: &str) {
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
            println!("{}", format!("✔ Successfully pulled: {}", repo_path).green());
        }
        Ok(_) => {
            eprintln!("{}", format!("❌ Failed to pull repository at: {}", repo_path).red());
        }
        Err(err) => {
            eprintln!("Error pulling repository at {}: {:?}", repo_path, err);
        }
    }
}
