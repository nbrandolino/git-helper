use crate::config::read_config;
use crate::utils::validate_git_repo;
use std::path::Path;
use std::process::Command;

/// displays detailed status report for all managed repositories
pub fn main(config_path: &Path) {
    let config = read_config(config_path);

    if config.repositories.is_empty() {
        println!("No repositories configured.");
        return;
    }

    println!("Detailed Status Report:\n");

    for repo_path in &config.repositories {
        let path = Path::new(repo_path);

        if let Err(err) = validate_git_repo(path) {
            eprintln!("Skipping invalid repository: {}. Error: {}", repo_path, err);
            continue;
        }

        println!("Repository: {}", repo_path);

        // get current branch
        let branch_output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output();

        match branch_output {
            Ok(output) if output.status.success() => {
                let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("  Current Branch: {}", branch);
            }
            Ok(_) => eprintln!("  Failed to determine current branch."),
            Err(err) => eprintln!("  Error retrieving branch info: {:?}", err),
        }

        // get latest commit
        let commit_output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("log")
            .arg("-1")
            .arg("--oneline")
            .output();

        match commit_output {
            Ok(output) if output.status.success() => {
                let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("  Latest Commit: {}", commit);
            }
            Ok(_) => eprintln!("  Failed to retrieve latest commit."),
            Err(err) => eprintln!("  Error retrieving latest commit: {:?}", err),
        }

        // check for pending changes
        let status_output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("status")
            .arg("--short")
            .output();

        match status_output {
            Ok(output) if output.status.success() => {
                let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if status.is_empty() {
                    println!("  Pending Changes: None");
                } else {
                    println!("  Pending Changes:\n{}", status);
                }
            }
            Ok(_) => eprintln!("  Failed to retrieve status."),
            Err(err) => eprintln!("  Error retrieving status: {:?}", err),
        }

        println!();
    }
}
