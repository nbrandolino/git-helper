use clap::{Command, Arg, value_parser};
use dirs_next;
use std::path::{Path, PathBuf};

// ensure the config dir exists
fn ensure_config_dir_exists(config_path: &Path) {
    if let Some(parent_dir) = config_path.parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).expect("Failed to create configuration directory");
        }
    }
}

// expand path (resolves `~` and `.`)
fn expand_path(input: &str) -> PathBuf {
    if input == "." {
        std::env::current_dir().expect("Unable to resolve current directory")
    } else if input.starts_with("~") {
        if let Some(home_dir) = dirs_next::home_dir() {
            home_dir.join(&input[1..])
        } else {
            panic!("Unable to determine home directory. Ensure the home directory is set correctly.");
        }
    } else {
        PathBuf::from(input)
    }
}

// validate if a path is a valid git repository
fn validate_git_repo(repo_path: &Path) -> Result<(), String> {
    if !repo_path.exists() {
        return Err(format!("Path does not exist: {}", repo_path.display()));
    }
    if !repo_path.is_dir() {
        return Err(format!("Path is not a directory: {}", repo_path.display()));
    }
    let git_dir = repo_path.join(".git");
    if !git_dir.exists() {
        return Err(format!("Path is not a valid Git repository: {}", repo_path.display()));
    }
    Ok(())
}

// add repo function
fn add_repo(repo_path: &str, config_path: &Path) {
    let expanded_path = expand_path(repo_path);
    if let Err(err) = validate_git_repo(&expanded_path) {
        eprintln!("Failed to add repository: {}", err);
        return;
    }

    let mut config_content = std::fs::read_to_string(config_path).unwrap_or_default();
    if config_content.lines().any(|line| line.trim() == expanded_path.to_str().unwrap_or("")) {
        println!("Repository already exists: {}", repo_path);
        return;
    }

    config_content.push_str(&format!("{}\n", expanded_path.display()));
    if let Err(err) = std::fs::write(config_path, config_content) {
        eprintln!("Failed to write to config file: {}", err);
    } else {
        println!("Added repository: {}", expanded_path.display());
    }
}

// remove repo function (by path or name)
fn remove_repo(repo_identifier: &str, config_path: &Path) {
    let config_content = std::fs::read_to_string(config_path).unwrap_or_default();
    let new_content: String = config_content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed != repo_identifier && !trimmed.contains(repo_identifier)
        })
        .map(|line| format!("{}\n", line))
        .collect();
    if let Err(err) = std::fs::write(config_path, new_content) {
        eprintln!("Failed to write to config file: {}", err);
    } else {
        println!("Removed repository: {}", repo_identifier);
    }
}

// list repos function
fn list_repos(config_path: &Path) {
    match std::fs::read_to_string(config_path) {
        Ok(config_content) => {
            println!("Configured repositories:");
            for line in config_content.lines() {
                println!("- {}", line);
            }
        }
        Err(err) => {
            eprintln!("Failed to read config file: {}", err);
        }
    }
}

// pull all repos function
fn pull_all(repo_path: &str) {
    let path = Path::new(repo_path);

    if let Err(err) = validate_git_repo(path) {
        eprintln!("Cannot pull repository: {}", err);
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
        .version("1.3.2")
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
        // remove repo from config file (by path or name)
        .arg(
            Arg::new("remove-repo")
                .long("remove-repo")
                .short('r')
                .help("Removes a repository by path or name")
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

    // set config file path
    let config_path = dirs_next::home_dir()
        .map(|home| home.join(".config/git-helper/git-helper.conf"))
        .unwrap_or_else(|| {
            eprintln!("Unable to find home directory. Please set HOME environment variable correctly.");
            std::process::exit(1);
        });

    // ensure config dir exists
    ensure_config_dir_exists(&config_path);

    // add repo
    if let Some(repo_path) = matches.get_one::<String>("add-repo") {
        add_repo(repo_path, &config_path);
    }
    // remove repo by path or name
    else if let Some(repo_identifier) = matches.get_one::<String>("remove-repo") {
        remove_repo(repo_identifier, &config_path);
    }
    // list repos
    else if matches.get_flag("list-repos") {
        list_repos(&config_path);
    }
    // pull all repos
    else if matches.get_flag("pull-all") {
        match std::fs::read_to_string(&config_path) {
            Ok(config_content) => {
                for line in config_content.lines() {
                    let repo_path = line.trim();
                    if !repo_path.is_empty() {
                        pull_all(repo_path);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to read config file: {}", err);
            }
        }
    }
    // if no arguments are passed, display the help menu
    else {
        println!("No action specified. Use --help for usage.");
    }
}
