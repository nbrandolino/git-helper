use std::path::{Path, PathBuf};
use std::fs;

// ensure config dir exists
pub fn ensure_config_dir_exists(config_path: &Path) {
    if let Some(parent_dir) = config_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).expect("Failed to create configuration directory");
        }
    }
}

// expand path if needed
pub fn expand_path(input: &str) -> PathBuf {
    if input == "." {
        std::env::current_dir().expect("Unable to resolve current directory")
    } else if input.starts_with('~') {
        dirs_next::home_dir()
            .map(|home_dir| home_dir.join(&input[1..]))
            .unwrap_or_else(|| {
                panic!("Unable to determine home directory. Please set HOME environment variable correctly.");
            })
    } else {
        PathBuf::from(input)
    }
}

// validate if repo is a git repo
pub fn validate_git_repo(repo_path: &Path) -> Result<(), String> {
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
