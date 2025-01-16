use clap::{Command, Arg, value_parser};
use dirs_next;

// add repo function
fn add_repo(repo_path: &str, config_path: &std::path::Path) {
    let mut config_content = std::fs::read_to_string(config_path).unwrap_or_default();
    config_content.push_str(&format!("{}\n", repo_path));
    std::fs::write(config_path, config_content).expect("Failed to write to config file");
    println!("Added repository: {}", repo_path);
}

// remove repo function
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

// list repos function
fn list_repos(config_path: &std::path::Path) {
    let config_content = std::fs::read_to_string(config_path).unwrap_or_default();
    println!("Configured repositories:");
    for line in config_content.lines() {
        println!("- {}", line);
    }
}

// pull all repos function
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

// main function
fn main() {
    let matches = Command::new("git-helper")
        .version("1.1")
        .author("nbrandolino")
        .about("A helper tool for managing multiple git repositories")
        // add repo to config file
        .arg(
            Arg::new("add-repo")
                .long("add-repo")
                .short('a')
                .help("Adds a new repository to be managed")
                .value_parser(value_parser!(String)),
        )
        // remove repo from config file
        .arg(
            Arg::new("remove-repo")
                .long("remove-repo")
                .short('r')
                .help("Removes a repository from being managed")
                .value_parser(value_parser!(String)),
        )
        // list repos
        .arg(
            Arg::new("list-repos")
                .long("list-repos")
                .short('l')
                .help("Lists all repositories being managed")
                .action(clap::ArgAction::SetTrue),
        )
        // pull all repos
        .arg(
            Arg::new("pull-all")
                .long("pull-all")
                .short('p')
                .help("Pulls all managed repositories")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // set config file
    let config_path = dirs_next::home_dir()
        .expect("Unable to find home directory")
        .join(".config/git-helper/git-helper.conf");

    // add repo
    if let Some(repo_path) = matches.get_one::<String>("add-repo") {
        add_repo(repo_path, &config_path);
    }
    // remove repo
    else if let Some(repo_path) = matches.get_one::<String>("remove-repo") {
        remove_repo(repo_path, &config_path);
    }
    // list repos
    else if matches.get_flag("list-repos") {
        list_repos(&config_path);
    }
    // pull all repos
    else if matches.get_flag("pull-all") {
        let config_content = std::fs::read_to_string(&config_path)
            .unwrap_or_else(|_| panic!("Failed to read config file at {:?}", config_path));

        for line in config_content.lines() {
            let repo_path = line.trim();
            if !repo_path.is_empty() {
                pull_all(repo_path);
            }
        }
    }
    // if no arguments are passed, display the help menu
    else {
        println!("No action specified. Use --help for usage.");
    }
}
