use crate::config::read_config;
use crate::utils::validate_git_repo;
use std::path::Path;
use std::process::Command;

/// clones all remote branches as local branches
pub fn main(repo_identifier: &str, config_path: &Path) {
    let config = read_config(config_path);

    // find repository matching the identifier
    let repo_path = config
        .repositories
        .iter()
        .find(|repo| repo.contains(repo_identifier));

    match repo_path {
        Some(repo_path) => {
            let path = Path::new(repo_path);

            if let Err(err) = validate_git_repo(path) {
                eprintln!("Error: {}", err);
                return;
            }

            println!("Cloning remote branches for repository: {}", repo_path);

            // fetch remote branches
            let fetch_output = Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .arg("fetch")
                .arg("--all")
                .output();

            if let Err(err) = fetch_output {
                eprintln!("Error fetching remote branches: {:?}", err);
                return;
            }

            // get the list of remote branches
            let branch_output = Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .arg("branch")
                .arg("-r")
                .output();

            match branch_output {
                Ok(output) => {
                    if output.status.success() {
                        let branches = String::from_utf8_lossy(&output.stdout);
                        for branch in branches.lines() {
                            // skip symbolic refs
                            if branch.contains("->") {
                                continue;
                            }
                            let branch_name = branch.trim().replace("origin/", "");

                            // check if the branch already exists locally
                            let local_branch_check = Command::new("git")
                                .arg("-C")
                                .arg(repo_path)
                                .arg("branch")
                                .output();

                            match local_branch_check {
                                Ok(local_output) => {
                                    if local_output.status.success() {
                                        let local_branches = String::from_utf8_lossy(&local_output.stdout);
                                        if local_branches.contains(&branch_name) {
                                            println!("Branch '{}' already exists locally, skipping.", branch_name);
                                            continue;
                                        }
                                    } else {
                                        eprintln!(
                                            "Failed to check local branches: {:?}",
                                            String::from_utf8_lossy(&local_output.stderr)
                                        );
                                        continue;
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Error checking local branches: {:?}", err);
                                    continue;
                                }
                            }

                            // checkout a local branch from the remote branch
                            let checkout_output = Command::new("git")
                                .arg("-C")
                                .arg(repo_path)
                                .arg("checkout")
                                .arg("-b")
                                .arg(&branch_name)
                                .arg(format!("origin/{}", branch_name))
                                .output();

                            if let Ok(output) = checkout_output {
                                if output.status.success() {
                                    println!("Created local branch: {}", branch_name);
                                } else {
                                    eprintln!(
                                        "Failed to create local branch: {}\nError: {}",
                                        branch_name,
                                        String::from_utf8_lossy(&output.stderr)
                                    );
                                }
                            } else {
                                eprintln!(
                                    "Error running git checkout for branch '{}': {:?}",
                                    branch_name, checkout_output
                                );
                            }
                        }
                    } else {
                        eprintln!(
                            "Failed to list remote branches for repository: {}",
                            repo_path
                        );
                    }
                }
                Err(err) => {
                    eprintln!("Error listing remote branches: {:?}", err);
                }
            }
        }
        None => {
            eprintln!(
                "Repository identifier '{}' not found in configuration.",
                repo_identifier
            );
        }
    }
}
