use crate::constants;

pub fn build_cli() -> clap::Command {
    clap::Command::new(constants::NAME)
        .version(constants::VERSION)
        .author(constants::AUTHOR)
        .about("A helper tool for managing multiple git repositories")
        .arg(
            clap::Arg::new("config")
                .long("config")
                .short('C')
                .help("Specify an alternative configuration file")
                .value_name("PATH")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("add-repo")
                .long("add-repo")
                .short('a')
                .help("Adds a new repository to be managed")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("remove-repo")
                .long("remove-repo")
                .short('r')
                .help("Removes a repository by path or name")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("list-repos")
                .long("list-repos")
                .short('l')
                .help("Lists all repositories being managed")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("detect-repos")
                .long("detect-repos")
                .short('d')
                .help("Detects Git repositories in the immediate children of the specified directory and adds them to the configuration")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("pull")
                .long("pull")
                .short('p')
                .help("Pulls all managed repositories")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("push")
                .long("push")
                .short('P')
                .help("Pushes all managed repositories to their remotes")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("clone-remote-branches")
                .long("clone-remote-branches")
                .short('c')
                .help("Creates local branches for all remote branches of a specified repository")
                .value_parser(clap::value_parser!(String)),
        )
}
