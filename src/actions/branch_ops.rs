use crate::utils::validate_git_repo;
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub enum BranchOperation {
    Pull,
    Push,
}

impl BranchOperation {
    fn verb(&self) -> &str {
        match self {
            BranchOperation::Pull => "pull",
            BranchOperation::Push => "push",
        }
    }

    fn action_label(&self) -> &str {
        match self {
            BranchOperation::Pull => "Pulling",
            BranchOperation::Push => "Pushing",
        }
    }

    fn past_tense(&self) -> &str {
        match self {
            BranchOperation::Pull => "Pulled",
            BranchOperation::Push => "Pushed",
        }
    }

    fn git_command(&self) -> &str {
        match self {
            BranchOperation::Pull => "pull",
            BranchOperation::Push => "push",
        }
    }
}

pub fn run_on_branches(repo_path: &str, operation: BranchOperation, quiet: bool) -> bool {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("{}", format!("❌ Cannot {} repository: {}", operation.verb(), err).red());
        return false;
    }

    if !quiet {
        println!("{} repository at: {}", operation.action_label(), repo_path);
    }

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
            return true;
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error checking status of '{}': {:?}", repo_path, err).red());
            return false;
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
            match String::from_utf8(output.stdout) {
                Ok(s) => s.trim().to_string(),
                Err(_) => {
                    eprintln!("{}", format!("❌ Branch name for '{}' contains invalid UTF-8.", repo_path).red());
                    return false;
                }
            }
        }
        Ok(_) => {
            eprintln!("{}", format!("⚠ Skipping '{}': repository is in detached HEAD state.", repo_path).yellow());
            return true;
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error determining current branch for '{}': {:?}", repo_path, err).red());
            return false;
        }
    };

    // fetch all remotes (pull only)
    if let BranchOperation::Pull = operation {
        let fetch_output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("fetch")
            .arg("--all")
            .output();

        match fetch_output {
            Ok(output) if output.status.success() => {}
            Ok(output) => {
                eprintln!("{}", format!("❌ Failed to fetch remotes for '{}': {}",
                    repo_path,
                    String::from_utf8_lossy(&output.stderr)).red());
                return false;
            }
            Err(err) => {
                eprintln!("{}", format!("❌ Error fetching remotes for '{}': {:?}", repo_path, err).red());
                return false;
            }
        }
    }

    // list all local branches
    let branch_output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("branch")
        .output();

    let branches = match branch_output {
        Ok(output) if output.status.success() => {
            match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("{}", format!("❌ Branch list for '{}' contains invalid UTF-8.", repo_path).red());
                    return false;
                }
            }
        }
        Ok(output) => {
            eprintln!("{}", format!("❌ Failed to list branches for '{}': {}",
                repo_path,
                String::from_utf8_lossy(&output.stderr)).red());
            return false;
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error listing branches for '{}': {:?}", repo_path, err).red());
            return false;
        }
    };

    let mut had_error = false;

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
                had_error = true;
                continue;
            }
        } else {
            eprintln!("{}", format!("❌ Error checking out branch '{}' in '{}'.", branch_name, repo_path).red());
            had_error = true;
            continue;
        }

        // run the git operation
        let op_output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg(operation.git_command())
            .output();

        match op_output {
            Ok(output) if output.status.success() => {
                if !quiet {
                    println!("{}", format!("✔ {} branch '{}' in '{}'.",
                        operation.past_tense(), branch_name, repo_path).green());
                }
            }
            Ok(output) => {
                eprintln!("{}", format!("❌ Failed to {} branch '{}' in '{}': {}",
                    operation.verb(), branch_name, repo_path,
                    String::from_utf8_lossy(&output.stderr)).red());
                had_error = true;
            }
            Err(err) => {
                eprintln!("{}", format!("❌ Error {}ing branch '{}' in '{}': {:?}",
                    operation.verb(), branch_name, repo_path, err).red());
                had_error = true;
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
            if !quiet {
                println!("{}", format!("✔ Restored original branch '{}' in '{}'.",
                    current_branch, repo_path).green());
            }
        }
        Ok(output) => {
            eprintln!("{}", format!("❌ Failed to restore branch '{}' in '{}': {}",
                current_branch, repo_path,
                String::from_utf8_lossy(&output.stderr)).red());
            had_error = true;
        }
        Err(err) => {
            eprintln!("{}", format!("❌ Error restoring branch '{}' in '{}': {:?}",
                current_branch, repo_path, err).red());
            had_error = true;
        }
    }

    !had_error
}
