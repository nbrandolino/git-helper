pub mod constants;
mod actions;
mod cli;
mod config;
mod utils;

// main function
fn main() {
    let matches = cli::build_cli().get_matches();

    let config_path = dirs_next::home_dir()
        .map(|home| home.join(".config/git-helper/git-helper.toml"))
        .unwrap_or_else(|| {
            eprintln!("Unable to find home directory. Please set HOME environment variable correctly.");
            std::process::exit(1);
        });

    utils::ensure_config_dir_exists(&config_path);

    // add-repo
    if let Some(repo_path) = matches.get_one::<String>("add-repo") {
        actions::add_repo(repo_path, &config_path);
    }
    // remove-repo
    else if let Some(repo_identifier) = matches.get_one::<String>("remove-repo") {
        actions::remove_repo(repo_identifier, &config_path);
    }
    // list-repos
    else if matches.get_flag("list-repos") {
        actions::list_repos(&config_path);
    }
    // show-graph
    else if let Some(repo_identifier) = matches.get_one::<String>("show-graph") {
        actions::show_git_graph(repo_identifier, &config_path);
    }
    // pull-all
    else if matches.get_flag("pull-all") {
        let config = config::read_config(&config_path);
        for repo in &config.repositories {
            actions::pull_all(repo);
        }
    }
    // fetch-all
    else if matches.get_flag("fetch-all") {
        let config = config::read_config(&config_path);
        for repo in &config.repositories {
            actions::fetch_all(repo);
        }
    }
    else {
        println!("No action specified. Use --help for usage.");
    }
}
