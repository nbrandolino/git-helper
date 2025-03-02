use crate::utils::validate_git_repo;
use colored::Colorize;
use std::path::Path;
use std::process::Command;

/// push all repos in config file
pub fn main(repo_path: &str) {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("{}", format!("❌ Cannot push repository: {}", err).red());
        return;
    }

    println!("Pushing repository at: {}", repo_path);

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("push")
        .arg("--all")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output();

    match output {
        Ok(status) if status.status.success() => {
            println!("{}", format!("✔ Successfully pushed: {}", repo_path).green());
        }
        Ok(_) => {
            eprintln!("{}", format!("❌ Failed to push repository at: {}", repo_path).red());
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error pushing repository at {}: {:?}", repo_path, err).red());
        }
    }
}
