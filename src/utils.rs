use std::path::{Path, PathBuf};
use std::fs;

pub fn ensure_config_dir_exists(config_path: &Path) -> Result<(), String> {
    if let Some(parent_dir) = config_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)
                .map_err(|err| format!("❌ Failed to create configuration directory '{}': {}", parent_dir.display(), err))?;
        }
    }
    Ok(())
}

pub fn expand_path(input: &str) -> Result<PathBuf, String> {
    let path = if input == "." {
        std::env::current_dir()
            .map_err(|e| format!("Unable to resolve current directory: {}", e))?
    } else if input.starts_with('~') {
        let rest = if input.starts_with("~/") { &input[2..] } else { &input[1..] };
        dirs_next::home_dir()
            .map(|home_dir| home_dir.join(rest))
            .ok_or_else(|| "Unable to determine home directory. Please set HOME correctly.".to_string())?
    } else {
        PathBuf::from(input)
    };

    std::fs::canonicalize(&path)
        .map_err(|e| format!("Unable to resolve path '{}': {}", path.display(), e))
}

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
