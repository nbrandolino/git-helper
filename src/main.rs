use git_helper::cli;
use git_helper::config;
use git_helper::actions::*;
use git_helper::utils;
use std::path::PathBuf;
use colored::Colorize;

// main function
fn main() {
    let matches = cli::build_cli().get_matches();

    // Determine config file path
    let config_path = if let Some(config_file) = matches.get_one::<String>("config") {
        PathBuf::from(config_file)
    }
    else {
        dirs_next::home_dir()
            .map(|home| home.join(".config/git-helper/git-helper.toml"))
            .unwrap_or_else(|| {
                eprintln!("{}", format!("❌ Unable to find home directory. Please set HOME environment variable correctly.").red());
                std::process::exit(1);
            })
    };

    utils::ensure_config_dir_exists(&config_path);

    // add-repo
    if let Some(repo_path) = matches.get_one::<String>("add-repo") {
        add_repo::main(repo_path, &config_path);
    }
    // remove-repo
    else if let Some(repo_identifier) = matches.get_one::<String>("remove-repo") {
        remove_repo::main(repo_identifier, &config_path);
    }
    // list-repos
    else if matches.get_flag("list-repos") {
        list_repos::main(&config_path);
    }
    // detect
    else if let Some(directory) = matches.get_one::<String>("detect-repos") {
        detect_repos::main(directory, &config_path);
    }
    // pull-all
    else if matches.get_flag("pull-all") {
        let config = config::read_config(&config_path);
        for repo in &config.repositories {
            pull_all::main(repo);
        }
    }
    // push-all
    else if matches.get_flag("push-all") {
        let config = config::read_config(&config_path);
        for repo in &config.repositories {
            push_all::main(repo);
        }
    }
    // fetch-all
    else if matches.get_flag("fetch-all") {
        let config = config::read_config(&config_path);
        for repo in &config.repositories {
            fetch_all::main(repo);
        }
    }
    // clone remote branches
    else if let Some(repo_identifier) = matches.get_one::<String>("clone-remote-branches") {
        clone_remote_branches::main(repo_identifier, &config_path);
    }
    else {
        println!("No action specified. Use --help for usage.");
    }
}
