use crate::config::{read_config};

// show git graph
pub fn main(repo_identifier: &str, config_path: &std::path::Path) {
    let config = read_config(config_path);

    // find a repository that contains the identifier
    let repo_path = config
        .repositories
        .iter()
        .find(|repo| repo.contains(repo_identifier));

    match repo_path {
        Some(repo_path) => {
            let path = std::path::Path::new(repo_path);

            if let Err(err) = crate::utils::validate_git_repo(path) {
                eprintln!("Cannot display graph for repository: {}", err);
                return;
            }

            println!("Displaying git graph for repository: {}", repo_identifier);

            let output = std::process::Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .arg("log")
                .arg("--graph")
                .arg("--oneline")
                .arg("--all")
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .output();

            match output {
                Ok(status) if status.status.success() => {
                    println!("Successfully displayed graph for: {}", repo_identifier);
                }
                Ok(_) => {
                    eprintln!("Failed to display graph for repository at: {}", repo_path);
                }
                Err(err) => {
                    eprintln!("Error displaying graph for repository at {}: {:?}", repo_path, err);
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
