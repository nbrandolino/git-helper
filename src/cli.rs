use crate::constants::NAME;
use crate::constants;

// cli arguments
pub fn build_cli() -> clap::Command {
    clap::Command::new(NAME)
        .version(constants::VERSION)
        .author(constants::AUTHOR)
        .about("A helper tool for managing multiple git repositories")
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
            clap::Arg::new("pull-all")
                .long("pull-all")
                .short('p')
                .help("Pulls all managed repositories")
                .action(clap::ArgAction::SetTrue),
        )
}
