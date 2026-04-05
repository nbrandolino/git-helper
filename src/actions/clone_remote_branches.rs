use crate::config::read_config;
use crate::utils::validate_git_repo;
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn clone_remote_branches(repo_identifier: &str, config_path: &Path, quiet: bool) {
    let config = read_config(config_path);

    if repo_identifier == "all" {
        for repo_path in &config.repositories {
            clone_branches(repo_path, quiet);
        }
    }
    else {
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
            }
            1 => {
                clone_branches(matches[0], quiet);
            }
            _ => {
                eprintln!("{}", format!(
                    "❌ Ambiguous identifier '{}' matches multiple repositories. Please use the full path:",
                    repo_identifier
                ).red());
                for path in &matches {
                    eprintln!("   - {}", path);
                }
            }
        }
    }
}

fn clone_branches(repo_path: &str, quiet: bool) {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("{}", format!("❌ Error: {}", err).red());
        return;
    }

    if !quiet { println!("Cloning remote branches for repository: {}", repo_path); }

    // fetch remote branches
    let fetch_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("fetch")
        .arg("--all")
        .output();

    if let Err(err) = fetch_output {
        eprintln!("{}", format!("❌ Error fetching remote branches for '{}': {:?}", repo_path, err).red());
        return;
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
            let branches = String::from_utf8_lossy(&output.stdout);

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

                if let Ok(output) = checkout_output {
                    if output.status.success() {
                        if !quiet { println!("{}", format!("✔ Created local branch: {}", branch_name).green()); }
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
                            if !quiet { println!("{}", format!("✔ Checked out default branch: {}", default_branch).green()); }
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
        Ok(output) => {
            eprintln!("{}", format!("❌ Failed to list remote branches for '{}': {}",
                repo_path,
                String::from_utf8_lossy(&output.stderr)).red());
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error listing remote branches: {:?}", err).red());
        }
    }
}
