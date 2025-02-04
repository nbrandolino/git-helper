use clap::{Arg, Command};
use crate::constants::NAME;
use crate::constants;

// cli arguments
pub fn build_cli() -> clap::Command {
    clap::Command::new(NAME)
        .version(constants::VERSION)
        .author(constants::AUTHOR)
        .about("A helper tool for managing multiple git repositories")
        // use config from different location
        .arg(
            Arg::new("config")
                .long("config")
                .short('C')
                .help("Specify a custom configuration file")
                .value_parser(clap::value_parser!(String))
        )
        // add-repo
        .arg(
            clap::Arg::new("add-repo")
                .long("add-repo")
                .short('a')
                .help("Adds a new repository to be managed")
                .value_parser(clap::value_parser!(String)),
        )
        // remove-repo
        .arg(
            clap::Arg::new("remove-repo")
                .long("remove-repo")
                .short('r')
                .help("Removes a repository by path or name")
                .value_parser(clap::value_parser!(String)),
        )
        // list-repos
        .arg(
            clap::Arg::new("list-repos")
                .long("list-repos")
                .short('l')
                .help("Lists all repositories being managed")
                .action(clap::ArgAction::SetTrue),
        )
        // detailed status report
        .arg(
            clap::Arg::new("status")
                .long("status")
                .short('s')
                .help("Displays a detailed status report for all managed repositories")
                .action(clap::ArgAction::SetTrue),
        )
        // show-graph
        .arg(
            clap::Arg::new("show-graph")
                .long("show-graph")
                .short('g')
                .help("Displays the Git commit graph for a specified repository")
                .value_parser(clap::value_parser!(String)),
        )
        // pull-all
        .arg(
            clap::Arg::new("pull-all")
                .long("pull-all")
                .short('p')
                .help("Pulls all managed repositories")
                .action(clap::ArgAction::SetTrue),
        )
        // fetch all
        .arg(
            clap::Arg::new("fetch-all")
                .long("fetch-all")
                .short('f')
                .help("Fetches all managed repositories")
                .action(clap::ArgAction::SetTrue),
        )
        // clone remote branches
        .arg(
            clap::Arg::new("clone-remote-branches")
                .long("clone-remote-branches")
                .short('c')
                .help("Creates local branches for all remote branches of a specified repository")
                .value_parser(clap::value_parser!(String)),
        )
}
