pub mod constants;
mod actions;
mod cli;
mod config;
mod utils;
use std::path::PathBuf;
use actions::add_repo;
use actions::clone_remote_branches;
use actions::detect_repos;
use actions::fetch_all;
use actions::list_repos;
use actions::pull_all;
use actions::remove_repo;
use actions::show_git_graph;
use actions::status_report;

// main function
fn main() {
    let matches = cli::build_cli().get_matches();

    // get custom config path if provided
    let config_path = if let Some(path) = matches.get_one::<String>("config") {
        PathBuf::from(path)
    }
    else {
        dirs_next::home_dir()
            .map(|home| home.join(".config/git-helper/git-helper.toml"))
            .unwrap_or_else(|| {
                eprintln!("Unable to find home directory. Please set HOME environment variable correctly.");
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
    // detect-repos
    else if let Some(scan_path) = matches.get_one::<String>("detect-repos") {
        detect_repos::main(scan_path, &config_path);
    }
    // detailed status report
    else if matches.get_flag("status") {
        status_report::main(&config_path);
    }
    // show-graph
    else if let Some(repo_identifier) = matches.get_one::<String>("show-graph") {
        show_git_graph::main(repo_identifier, &config_path);
    }
    // pull-all
    else if matches.get_flag("pull-all") {
        let config = config::read_config(&config_path);
        for repo in &config.repositories {
            pull_all::main(repo);
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
