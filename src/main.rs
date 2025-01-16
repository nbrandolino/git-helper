use dirs_next;
use std::env;

// show help info
fn show_help() {
    println!("Usage: git-helper [options]");
    println!("");
    println!("Options:");
    println!("    -h, --help                  Display this help message.");
    println!("    -v, --version               Display version information.");
    println!("    -pa, --pull-all             Pull all repos in config file.");
    println!("");
    println!("Example:");
    println!("    $ git-helper -pa");
}

// show version info
fn show_version() {
    println!("git-helper Version 1.0");
    println!("Licensed under the terms of the GNU General Public License.");
}

// pull all function
fn pull_all(repo_path: &str) {
    let path = std::path::Path::new(repo_path);

    // checks to see if the paths in the config file exist or not
    if !path.exists() || !path.is_dir() {
        eprintln!("Path does not exist or is not a directory: {}", repo_path);
        return;
    }

    println!("Pulling repository at: {}", repo_path);

    // run the git pull command
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
    let args: Vec<String> = env::args().collect();

    // help check
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        show_help()
    }
    // version check
    else if args.len() == 2 && (args[1] == "-v" || args[1] == "--version") {
        show_version()
    }
    // pull all check
    else if args.len() == 2 && (args[1] == "-pa" || args[1] == "--pull-all") {
        let config_path = dirs_next::home_dir()
            .expect("Unable to find home directory")
            .join(".config/git-helper/git-helper.conf");

        let config_content = std::fs::read_to_string(&config_path)
            .unwrap_or_else(|_| panic!("Failed to read config file at {:?}", config_path));

        for line in config_content.lines() {
            let repo_path = line.trim();
            if !repo_path.is_empty() {
                pull_all(repo_path);
            }
        }
    }
    else {
        show_help()
    }
}
