use crate::config::read_config;
use crate::utils::validate_git_repo;
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn clone_remote_branches(repo_identifier: &str, config_path: &Path, quiet: bool) -> bool {
    let config = read_config(config_path);

    if repo_identifier == "all" {
        let mut success = true;
        for repo_path in &config.repositories {
            if !clone_branches(repo_path, quiet) {
                success = false;
            }
        }
        return success;
    }

    let matches: Vec<&String> = config
        .repositories
        .iter()
        .filter(|repo| {
            Path::new(repo)
                .file_name()
                .map(|name| name.to_string_lossy() == repo_identifier)
                .unwrap_or(false)
        })
        .collect();

    match matches.len() {
        0 => {
            eprintln!("{}", format!("❌ Repository identifier '{}' not found in configuration.", repo_identifier).red());
            false
        }
        1 => clone_branches(matches[0], quiet),
        _ => {
            eprintln!("{}", format!(
                "❌ Ambiguous identifier '{}' matches multiple repositories. Please use the full path:",
                repo_identifier
            ).red());
            for path in &matches {
                eprintln!("   - {}", path);
            }
            false
        }
    }
}

fn clone_branches(repo_path: &str, quiet: bool) -> bool {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("{}", format!("❌ Error: {}", err).red());
        return false;
    }

    if !quiet { println!("Cloning remote branches for repository: {}", repo_path); }

    // fetch remote branches
    let fetch_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("fetch")
        .arg("--all")
        .output();

    match fetch_output {
        Ok(output) if output.status.success() => {}
        Ok(output) => {
            eprintln!("{}", format!("❌ Failed to fetch remote branches for '{}': {}",
                repo_path,
                String::from_utf8_lossy(&output.stderr)).red());
            return false;
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error fetching remote branches for '{}': {}", repo_path, err).red());
            return false;
        }
    }

    // list remote branches
    let branch_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("branch")
        .arg("-r")
        .output();

    match branch_output {
        Ok(output) if output.status.success() => {
            let branches = match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("{}", format!("❌ Remote branch list for '{}' contains invalid UTF-8.", repo_path).red());
                    return false;
                }
            };

            let mut had_error = false;

            for branch in branches.lines() {
                let branch = branch.trim();

                // skip HEAD pointer
                if branch.contains("->") {
                    continue;
                }

                // strip "origin/" prefix
                let branch_name = match branch.strip_prefix("origin/") {
                    Some(name) => name.to_string(),
                    None => continue,
                };

                // skip if local branch already exists
                let local_check = Command::new("git")
                    .arg("-C")
                    .arg(repo_path)
                    .arg("branch")
                    .arg("--list")
                    .arg(&branch_name)
                    .output();

                if let Ok(out) = local_check {
                    if !out.stdout.is_empty() {
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

                match checkout_output {
                    Ok(output) if output.status.success() => {
                        if !quiet {
                            println!("{}", format!("✔ Created local branch: {}", branch_name).green());
                        }
                    }
                    Ok(output) => {
                        eprintln!("{}", format!("❌ Failed to create local branch '{}': {}",
                            branch_name,
                            String::from_utf8_lossy(&output.stderr)).red());
                        had_error = true;
                    }
                    Err(err) => {
                        eprintln!("{}", format!("❌ Error running git checkout for branch '{}': {}",
                            branch_name, err).red());
                        had_error = true;
                    }
                }
            }

            // checkout the default branch after cloning
            let default_branch_output = Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .arg("symbolic-ref")
                .arg("refs/remotes/origin/HEAD")
                .output();

            match default_branch_output {
                Ok(output) if output.status.success() => {
                    let raw = match String::from_utf8(output.stdout) {
                        Ok(s) => s,
                        Err(_) => {
                            eprintln!("{}", "❌ Default branch name contains invalid UTF-8.".red());
                            return false;
                        }
                    };
                    let default_branch = raw.trim().replace("refs/remotes/origin/", "");

                    let checkout_default_output = Command::new("git")
                        .arg("-C")
                        .arg(repo_path)
                        .arg("checkout")
                        .arg(&default_branch)
                        .output();

                    match checkout_default_output {
                        Ok(output) if output.status.success() => {
                            if !quiet {
                                println!("{}", format!("✔ Checked out default branch: {}", default_branch).green());
                            }
                        }
                        Ok(output) => {
                            eprintln!("{}", format!("❌ Failed to checkout default branch '{}': {}",
                                default_branch,
                                String::from_utf8_lossy(&output.stderr)).red());
                            had_error = true;
                        }
                        Err(err) => {
                            eprintln!("{}", format!("❌ Error checking out default branch '{}': {}",
                                default_branch, err).red());
                            had_error = true;
                        }
                    }
                }
                Ok(output) => {
                    eprintln!("{}", format!("❌ Failed to determine default branch for '{}': {}",
                        repo_path,
                        String::from_utf8_lossy(&output.stderr)).red());
                    had_error = true;
                }
                Err(err) => {
                    eprintln!("{}", format!("❌ Error determining default branch for '{}': {}",
                        repo_path, err).red());
                    had_error = true;
                }
            }

            !had_error
        }
        Ok(output) => {
            eprintln!("{}", format!("❌ Failed to list remote branches for '{}': {}",
                repo_path,
                String::from_utf8_lossy(&output.stderr)).red());
            false
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error listing remote branches for '{}': {}", repo_path, err).red());
            false
        }
    }
}
