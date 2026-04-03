use crate::utils::validate_git_repo;
use colored::Colorize;
use std::path::Path;

// pull all repos in config file
pub fn main(repo_path: &str) {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("{}", format!("❌ Cannot pull repository: {}", err).red());
        return;
    }

    println!("Pulling repository at: {}", repo_path);

    let result = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("pull")
        .arg("--all")
        .status();

    match result {
        Ok(status) if status.success() => {
            println!("{}", format!("✔ Successfully pulled: {}", repo_path).green());
        }
        Ok(_) => {
            eprintln!("{}", format!("❌ Failed to pull repository at: {}", repo_path).red());
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error pulling repository at {}: {:?}", repo_path, err).red());
        }
    }
}
