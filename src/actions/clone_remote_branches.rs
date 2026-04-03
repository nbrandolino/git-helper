use crate::config::read_config;
use crate::utils::validate_git_repo;
use colored::Colorize;
use std::path::Path;
use std::process::Command;

/// clones all remote branches as local branches
pub fn main(repo_identifier: &str, config_path: &Path) {
    let config = read_config(config_path);

    if repo_identifier == "all" {
        for repo_path in &config.repositories {
            clone_branches(repo_path);
        }
    }
    else {
        let repo_path = config
            .repositories
            .iter()
            .find(|repo| {
                Path::new(repo)
                    .file_name()
                    .map(|name| name.to_string_lossy() == repo_identifier)
                    .unwrap_or(false)
            });

        if let Some(repo_path) = repo_path {
            clone_branches(repo_path);
        }
        else {
            eprintln!("{}", format!("❌ Repository identifier '{}' not found in configuration.", repo_identifier).red());
        }
    }
}

fn clone_branches(repo_path: &str) {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("{}", format!("❌ Error: {}", err).red());
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
        eprintln!("{}", format!("❌ Error fetching remote branches: {:?}", err).red());
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
            if !output.status.success() {
                eprintln!("{}", format!("❌ Failed to list remote branches for repository: {}", repo_path).red());
                return;
            }

            // fetch local branches ONCE before the loop
            let local_branches_output = Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .arg("branch")
                .output();

            let local_branches = match local_branches_output {
                Ok(ref local_output) if local_output.status.success() => {
                    String::from_utf8_lossy(&local_output.stdout).to_string()
                }
                Ok(ref local_output) => {
                    eprintln!("{}", format!("❌ Failed to check local branches: {:?}",
                        String::from_utf8_lossy(&local_output.stderr)).red());
                    return;
                }
                Err(err) => {
                    eprintln!("{}", format!("❌ Error checking local branches: {:?}", err).red());
                    return;
                }
            };

            let branches = String::from_utf8_lossy(&output.stdout);
            for branch in branches.lines() {
                // skip symbolic refs
                if branch.contains("->") {
                    continue;
                }
                let branch_name = branch.trim().replace("origin/", "");

                // check against the locally cached branch list
                if local_branches.contains(&branch_name) {
                    println!("{}", format!("⚠ Branch '{}' already exists locally, skipping.", branch_name).yellow());
                    continue;
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
                        println!("{}", format!("✔ Created local branch: {}", branch_name).green());
                    }
                    else {
                        eprintln!("{}", format!("❌ Failed to create local branch: {}\nError: {}",
                            branch_name,
                            String::from_utf8_lossy(&output.stderr)).red());
                    }
                }
                else {
                    eprintln!("{}", format!("❌ Error running git checkout for branch '{}': {:?}",
                        branch_name, checkout_output).red());
                }
            }

            // checkout the default branch after cloning
            let default_branch_output = Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .arg("symbolic-ref")
                .arg("refs/remotes/origin/HEAD")
                .output();

            if let Ok(output) = default_branch_output {
                if output.status.success() {
                    let default_branch = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .replace("refs/remotes/origin/", "");

                    let checkout_default_output = Command::new("git")
                        .arg("-C")
                        .arg(repo_path)
                        .arg("checkout")
                        .arg(&default_branch)
                        .output();

                    if let Ok(checkout_output) = checkout_default_output {
                        if checkout_output.status.success() {
                            println!("{}", format!("✔ Checked out default branch: {}", default_branch).green());
                        }
                        else {
                            eprintln!("{}", format!("❌ Failed to checkout default branch: {}\nError: {}",
                                default_branch,
                                String::from_utf8_lossy(&checkout_output.stderr)).red());
                        }
                    }
                }
                else {
                    eprintln!("{}", "❌ Failed to determine default branch.".red());
                }
            }
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error listing remote branches: {:?}", err).red());
        }
    }
}
