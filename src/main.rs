use clap::{Command, Arg};
use dirs_next;

// pull all function
fn pull_all(repo_path: &str) {
    let path = std::path::Path::new(repo_path);

    if !path.exists() || !path.is_dir() {
        eprintln!("Path does not exist or is not a directory: {}", repo_path);
        return;
    }

    println!("Pulling repository at: {}", repo_path);

    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("pull")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output();

    match output {
        Ok(status) if status.status.success() => {
            println!("Successfully pulled: {}", repo_path);
        }
        Ok(_) => {
            eprintln!("Failed to pull repository at: {}", repo_path);
        }
        Err(err) => {
            eprintln!("Error pulling repository at {}: {:?}", repo_path, err);
        }
    }
}

fn add_repo(repo_path: &str, config_path: &std::path::Path) {
    let mut config_content = std::fs::read_to_string(config_path).unwrap_or_default();
    config_content.push_str(&format!("{}\n", repo_path));
    std::fs::write(config_path, config_content).expect("Failed to write to config file");
    println!("Added repository: {}", repo_path);
}

fn remove_repo(repo_path: &str, config_path: &std::path::Path) {
    let config_content = std::fs::read_to_string(config_path).unwrap_or_default();
    let new_content: String = config_content
        .lines()
        .filter(|line| line.trim() != repo_path)
        .map(|line| format!("{}\n", line))
        .collect();
    std::fs::write(config_path, new_content).expect("Failed to write to config file");
    println!("Removed repository: {}", repo_path);
}

fn list_repos(config_path: &std::path::Path) {
    let config_content = std::fs::read_to_string(config_path).unwrap_or_default();
    println!("Configured repositories:");
    for line in config_content.lines() {
        println!("- {}", line);
    }
}

fn main() {
    let matches = Command::new("git-helper")
        .version("1.1")
        .author("nbrandolino")
        .about("A helper tool for managing multiple git repositories.")
        .subcommand(
            Command::new("add-repo")
                .about("Adds a new repository to the config file.")
                .arg(
                    Arg::new("repo")
                        .help("Full path to the repository.")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("remove-repo")
                .about("Removes a repository from the config file.")
                .arg(
                    Arg::new("repo")
                        .help("Full path to the repository.")
                        .required(true),
                ),
        )
        .subcommand(Command::new("list-repos").about("Lists all repositories in the config file."))
        .subcommand(Command::new("pull-all").about("Pulls all repositories."))
        .get_matches();

    let config_path = dirs_next::home_dir()
        .expect("Unable to find home directory")
        .join(".config/git-helper/git-helper.conf");

    if let Some(matches) = matches.subcommand_matches("add-repo") {
        let repo_path = matches.get_one::<String>("repo").unwrap();
        add_repo(repo_path, &config_path);
    } else if let Some(matches) = matches.subcommand_matches("remove-repo") {
        let repo_path = matches.get_one::<String>("repo").unwrap();
        remove_repo(repo_path, &config_path);
    } else if let Some(_) = matches.subcommand_matches("list-repos") {
        list_repos(&config_path);
    } else if let Some(_) = matches.subcommand_matches("pull-all") {
        let config_content = std::fs::read_to_string(&config_path)
            .unwrap_or_else(|_| panic!("Failed to read config file at {:?}", config_path));

        for line in config_content.lines() {
            let repo_path = line.trim();
            if !repo_path.is_empty() {
                pull_all(repo_path);
            }
        }
    } else {
        println!("Test");
    }
}
