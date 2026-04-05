use crate::utils::validate_git_repo;
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn pull(repo_path: &str) {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("{}", format!("❌ Cannot pull repository: {}", err).red());
        return;
    }

    println!("Pulling repository at: {}", repo_path);

    // check for uncommitted changes
    let status_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("status")
        .arg("--porcelain")
        .output();

    match status_output {
        Ok(output) if !output.stdout.is_empty() => {
            eprintln!("{}", format!("⚠ Skipping '{}': repository has uncommitted changes.", repo_path).yellow());
            return;
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error checking status of '{}': {:?}", repo_path, err).red());
            return;
        }
        _ => {}
    }

    // save the current branch
    let current_branch_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .output();

    let current_branch = match current_branch_output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        Ok(_) => {
            eprintln!("{}", format!("⚠ Skipping '{}': repository is in detached HEAD state.", repo_path).yellow());
            return;
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error determining current branch for '{}': {:?}", repo_path, err).red());
            return;
        }
    };

    // fetch all remotes
    let fetch_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("fetch")
        .arg("--all")
        .output();

    if let Err(err) = fetch_output {
        eprintln!("{}", format!("❌ Error fetching remotes for '{}': {:?}", repo_path, err).red());
        return;
    }

    // list all local branches
    let branch_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("branch")
        .output();

    let branches = match branch_output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        Ok(output) => {
            eprintln!("{}", format!("❌ Failed to list branches for '{}': {}",
                repo_path,
                String::from_utf8_lossy(&output.stderr)).red());
            return;
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error listing branches for '{}': {:?}", repo_path, err).red());
            return;
        }
    };

    for branch in branches.lines() {
        let branch_name = branch.trim().trim_start_matches("* ");

        // check if the branch has a remote tracking branch
        let upstream_output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg(format!("{}@{{upstream}}", branch_name))
            .output();

        let has_upstream = matches!(upstream_output, Ok(ref o) if o.status.success());
        if !has_upstream {
            continue;
        }

        // checkout the branch
        let checkout_output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("checkout")
            .arg(branch_name)
            .output();

        if let Ok(output) = checkout_output {
            if !output.status.success() {
                eprintln!("{}", format!("❌ Failed to checkout branch '{}' in '{}': {}",
                    branch_name, repo_path,
                    String::from_utf8_lossy(&output.stderr)).red());
                continue;
            }
        } else {
            eprintln!("{}", format!("❌ Error checking out branch '{}' in '{}'.", branch_name, repo_path).red());
            continue;
        }

        // pull the branch
        let pull_output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("pull")
            .output();

        match pull_output {
            Ok(output) if output.status.success() => {
                println!("{}", format!("✔ Pulled branch '{}' in '{}'.", branch_name, repo_path).green());
            }
            Ok(output) => {
                eprintln!("{}", format!("❌ Failed to pull branch '{}' in '{}': {}",
                    branch_name, repo_path,
                    String::from_utf8_lossy(&output.stderr)).red());
            }
            Err(err) => {
                eprintln!("{}", format!("❌ Error pulling branch '{}' in '{}': {:?}", branch_name, repo_path, err).red());
            }
        }
    }

    // restore the original branch
    let restore_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("checkout")
        .arg(&current_branch)
        .output();

    match restore_output {
        Ok(output) if output.status.success() => {
            println!("{}", format!("✔ Restored original branch '{}' in '{}'.", current_branch, repo_path).green());
        }
        Ok(output) => {
            eprintln!("{}", format!("❌ Failed to restore branch '{}' in '{}': {}",
                current_branch, repo_path,
                String::from_utf8_lossy(&output.stderr)).red());
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error restoring branch '{}' in '{}': {:?}", current_branch, repo_path, err).red());
        }
    }
}
