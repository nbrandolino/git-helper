use git_helper::cli;
use git_helper::config;
use git_helper::actions::*;
use git_helper::utils;
use std::path::PathBuf;
use colored::Colorize;
use dirs_next;

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

    if let Some(repo_path) = matches.get_one::<String>("add-repo") {
        add_repo::add_repo(repo_path, &config_path);
    }
    else if let Some(repo_identifier) = matches.get_one::<String>("remove-repo") {
        remove_repo::remove_repo(repo_identifier, &config_path);
    }
    else if matches.get_flag("list-repos") {
        list_repos::list_repos(&config_path);
    }
    else if let Some(directory) = matches.get_one::<String>("detect-repos") {
        detect_repos::detect_repos(directory, &config_path);
    }
    else if matches.get_flag("pull") {
        let config = config::read_config(&config_path);
        for repo in &config.repositories {
            pull::pull(repo);
        }
    }
    else if matches.get_flag("push") {
        let config = config::read_config(&config_path);
        for repo in &config.repositories {
            push::push(repo);
        }
    }
    else if let Some(repo_identifier) = matches.get_one::<String>("clone-remote-branches") {
        clone_remote_branches::clone_remote_branches(repo_identifier, &config_path);
    }
    else {
        println!("No action specified. Use --help for usage.");
    }
}
