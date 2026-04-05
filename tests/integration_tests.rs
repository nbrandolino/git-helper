use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

// create a bare temporary directory that is NOT a git repo
fn tmp_dir() -> TempDir {
    tempfile::tempdir().expect("failed to create temp dir")
}

// create a temporary directory that IS a valid git repo (`git init`)
fn tmp_git_repo() -> TempDir {
    let dir = tmp_dir();
    Command::new("git")
        .args(["init", dir.path().to_str().unwrap()])
        .output()
        .expect("git init failed");
    dir
}

// write a minimal config TOML to `path` and return the path
fn write_toml(path: &Path, repos: &[&str]) -> PathBuf {
    let entries: Vec<String> = repos.iter().map(|r| format!("\"{}\"", r)).collect();
    let content = format!("repositories = [{}]\n", entries.join(", "));
    fs::write(path, content).expect("write toml failed");
    path.to_path_buf()
}

#[cfg(test)]
mod validate_git_repo_tests {
    use super::*;
    use git_helper::utils::validate_git_repo;

    #[test]
    fn ok_for_valid_git_repo() {
        let repo = tmp_git_repo();
        assert!(validate_git_repo(repo.path()).is_ok());
    }

    #[test]
    fn err_when_path_does_not_exist() {
        let result = validate_git_repo(Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn err_when_path_is_not_a_directory() {
        let dir = tmp_dir();
        let file = dir.path().join("somefile.txt");
        fs::write(&file, "hello").unwrap();
        let result = validate_git_repo(&file);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a directory"));
    }

    #[test]
    fn err_when_directory_has_no_dot_git() {
        let dir = tmp_dir(); // plain dir, no git init
        let result = validate_git_repo(dir.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a valid Git repository"));
    }
}

#[cfg(test)]
mod expand_path_tests {
    use super::*;
    use git_helper::utils::expand_path;

    #[test]
    fn dot_resolves_to_current_dir() {
        let cwd = std::env::current_dir().unwrap();
        let expanded = expand_path(".").unwrap();
        assert_eq!(expanded, cwd);
    }

    #[test]
    fn absolute_path_to_existing_dir_works() {
        let dir = tmp_dir();
        let expanded = expand_path(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(expanded, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn err_for_nonexistent_path() {
        let result = expand_path("/this/path/does/not/exist");
        assert!(result.is_err());
    }

    #[test]
    fn tilde_prefix_resolves_without_panic() {
        // We can't guarantee the home dir exists in CI, so just check it
        // doesn't return the raw "~" string on success.
        if let Ok(expanded) = expand_path("~") {
            assert!(!expanded.to_string_lossy().starts_with('~'));
        }
    }
}

#[cfg(test)]
mod ensure_config_dir_exists_tests {
    use super::*;
    use git_helper::utils::ensure_config_dir_exists;

    #[test]
    fn creates_missing_parent_directories() {
        let base = tmp_dir();
        let config_path = base.path().join("a").join("b").join("config.toml");
        assert!(!config_path.parent().unwrap().exists());
        ensure_config_dir_exists(&config_path);
        assert!(config_path.parent().unwrap().exists());
    }

    #[test]
    fn no_panic_when_parent_already_exists() {
        let base = tmp_dir();
        let config_path = base.path().join("config.toml");
        // Parent already exists; should be a no-op
        ensure_config_dir_exists(&config_path);
        assert!(base.path().exists());
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use git_helper::config::{read_config, write_config, Config};

    #[test]
    fn read_config_returns_default_when_file_missing() {
        let config = read_config(Path::new("/no/such/file.toml"));
        assert!(config.repositories.is_empty());
    }

    #[test]
    fn read_config_parses_existing_file() {
        let dir = tmp_dir();
        let path = dir.path().join("config.toml");
        write_toml(&path, &["/repo/a", "/repo/b"]);

        let config = read_config(&path);
        assert_eq!(config.repositories.len(), 2);
        assert!(config.repositories.contains("/repo/a"));
        assert!(config.repositories.contains("/repo/b"));
    }

    #[test]
    fn write_config_then_read_roundtrip() {
        let dir = tmp_dir();
        let path = dir.path().join("config.toml");

        let mut repos = HashSet::new();
        repos.insert("/some/repo".to_string());
        repos.insert("/another/repo".to_string());

        let original = Config { repositories: repos };
        write_config(&path, &original);

        let loaded = read_config(&path);
        assert_eq!(loaded.repositories, original.repositories);
    }

    #[test]
    fn write_config_overwrites_existing_data() {
        let dir = tmp_dir();
        let path = dir.path().join("config.toml");
        write_toml(&path, &["/old/repo"]);

        let mut repos = HashSet::new();
        repos.insert("/new/repo".to_string());
        let config = Config { repositories: repos };
        write_config(&path, &config);

        let loaded = read_config(&path);
        assert!(!loaded.repositories.contains("/old/repo"));
        assert!(loaded.repositories.contains("/new/repo"));
    }
}

#[cfg(test)]
mod add_repo_tests {
    use super::*;
    use git_helper::actions::add_repo::add_repo;
    use git_helper::config::read_config;

    #[test]
    fn adds_valid_git_repo_to_config() {
        let repo = tmp_git_repo();
        let cfg_dir = tmp_dir();
        let cfg_path = cfg_dir.path().join("config.toml");

        add_repo(repo.path().to_str().unwrap(), &cfg_path, true);

        let config = read_config(&cfg_path);
        let canonical = repo.path().canonicalize().unwrap();
        assert!(config.repositories.contains(canonical.to_str().unwrap()));
    }

    #[test]
    fn does_not_add_duplicate_repo() {
        let repo = tmp_git_repo();
        let cfg_dir = tmp_dir();
        let cfg_path = cfg_dir.path().join("config.toml");

        let repo_str = repo.path().to_str().unwrap();
        add_repo(repo_str, &cfg_path, true);
        add_repo(repo_str, &cfg_path, true);

        let config = read_config(&cfg_path);
        assert_eq!(config.repositories.len(), 1);
    }

    #[test]
    fn does_not_add_non_git_directory() {
        let plain_dir = tmp_dir();
        let cfg_dir = tmp_dir();
        let cfg_path = cfg_dir.path().join("config.toml");

        add_repo(plain_dir.path().to_str().unwrap(), &cfg_path, true);

        let config = read_config(&cfg_path);
        assert!(config.repositories.is_empty());
    }

    #[test]
    fn does_not_add_nonexistent_path() {
        let cfg_dir = tmp_dir();
        let cfg_path = cfg_dir.path().join("config.toml");

        add_repo("/this/path/does/not/exist", &cfg_path, true);

        let config = read_config(&cfg_path);
        assert!(config.repositories.is_empty());
    }
}

#[cfg(test)]
mod remove_repo_tests {
    use super::*;
    use git_helper::actions::remove_repo::remove_repo;
    use git_helper::config::read_config;

    fn setup_config_with_repos(repos: &[&str]) -> (TempDir, PathBuf) {
        let dir = tmp_dir();
        let path = dir.path().join("config.toml");
        write_toml(&path, repos);
        (dir, path)
    }

    #[test]
    fn removes_by_exact_full_path() {
        let (_dir, path) = setup_config_with_repos(&["/repo/foo", "/repo/bar"]);
        remove_repo("/repo/foo", &path, true);
        let config = read_config(&path);
        assert!(!config.repositories.contains("/repo/foo"));
        assert!(config.repositories.contains("/repo/bar"));
    }

    #[test]
    fn removes_by_repo_name() {
        let (_dir, path) = setup_config_with_repos(&["/some/path/myrepo"]);
        remove_repo("myrepo", &path, true);
        let config = read_config(&path);
        assert!(config.repositories.is_empty());
    }

    #[test]
    fn no_change_when_repo_not_found() {
        let (_dir, path) = setup_config_with_repos(&["/repo/existing"]);
        remove_repo("nonexistent", &path, true);
        let config = read_config(&path);
        assert_eq!(config.repositories.len(), 1);
    }

    #[test]
    fn does_not_remove_on_ambiguous_name() {
        // Two repos with the same directory name should not be removed silently.
        let (_dir, path) =
            setup_config_with_repos(&["/path/one/myrepo", "/path/two/myrepo"]);
        remove_repo("myrepo", &path, true);
        // Both must still be present because the identifier is ambiguous.
        let config = read_config(&path);
        assert_eq!(config.repositories.len(), 2);
    }
}

#[cfg(test)]
mod detect_repos_tests {
    use super::*;
    use git_helper::actions::detect_repos::detect_repos;
    use git_helper::config::read_config;

    fn build_mixed_dir() -> TempDir {
        let parent = tmp_dir();
        for name in &["repo_a", "repo_b"] {
            let repo_path = parent.path().join(name);
            fs::create_dir_all(&repo_path).unwrap();
            Command::new("git")
                .args(["init", repo_path.to_str().unwrap()])
                .output()
                .unwrap();
        }
        fs::create_dir_all(parent.path().join("not_repo")).unwrap();
        parent
    }

    #[test]
    fn detects_all_git_repos_in_directory() {
        let parent = build_mixed_dir();
        let cfg_dir = tmp_dir();
        let cfg_path = cfg_dir.path().join("config.toml");

        detect_repos(parent.path().to_str().unwrap(), &cfg_path, true);

        let config = read_config(&cfg_path);
        assert_eq!(config.repositories.len(), 2);
    }

    #[test]
    fn does_not_add_plain_directories() {
        let parent = build_mixed_dir();
        let cfg_dir = tmp_dir();
        let cfg_path = cfg_dir.path().join("config.toml");

        detect_repos(parent.path().to_str().unwrap(), &cfg_path, true);

        let config = read_config(&cfg_path);
        let has_not_repo = config
            .repositories
            .iter()
            .any(|r| r.ends_with("not_repo"));
        assert!(!has_not_repo);
    }

    #[test]
    fn does_not_duplicate_already_known_repos() {
        let parent = build_mixed_dir();
        let cfg_dir = tmp_dir();
        let cfg_path = cfg_dir.path().join("config.toml");

        detect_repos(parent.path().to_str().unwrap(), &cfg_path, true);
        detect_repos(parent.path().to_str().unwrap(), &cfg_path, true);

        let config = read_config(&cfg_path);
        assert_eq!(config.repositories.len(), 2);
    }

    #[test]
    fn handles_invalid_directory_gracefully() {
        let cfg_dir = tmp_dir();
        let cfg_path = cfg_dir.path().join("config.toml");

        // Should not panic; config stays empty.
        detect_repos("/this/does/not/exist", &cfg_path, true);

        let config = read_config(&cfg_path);
        assert!(config.repositories.is_empty());
    }
}

#[cfg(test)]
mod cli_tests {
    use git_helper::cli::build_cli;

    fn parse(args: &[&str]) -> clap::ArgMatches {
        build_cli()
            .try_get_matches_from(args)
            .expect("arg parsing failed")
    }

    #[test]
    fn quiet_flag_is_false_by_default() {
        let m = parse(&["git-helper"]);
        assert!(!m.get_flag("quiet"));
    }

    #[test]
    fn quiet_short_flag_sets_true() {
        let m = parse(&["git-helper", "-q"]);
        assert!(m.get_flag("quiet"));
    }

    #[test]
    fn quiet_long_flag_sets_true() {
        let m = parse(&["git-helper", "--quiet"]);
        assert!(m.get_flag("quiet"));
    }

    #[test]
    fn add_repo_short_captures_path() {
        let m = parse(&["git-helper", "-a", "/some/repo"]);
        assert_eq!(m.get_one::<String>("add-repo").map(|s| s.as_str()), Some("/some/repo"));
    }

    #[test]
    fn add_repo_long_captures_path() {
        let m = parse(&["git-helper", "--add-repo", "/some/repo"]);
        assert_eq!(m.get_one::<String>("add-repo").map(|s| s.as_str()), Some("/some/repo"));
    }

    #[test]
    fn remove_repo_short_captures_identifier() {
        let m = parse(&["git-helper", "-r", "myrepo"]);
        assert_eq!(m.get_one::<String>("remove-repo").map(|s| s.as_str()), Some("myrepo"));
    }

    #[test]
    fn list_repos_flag_defaults_false() {
        let m = parse(&["git-helper"]);
        assert!(!m.get_flag("list-repos"));
    }

    #[test]
    fn list_repos_short_flag() {
        let m = parse(&["git-helper", "-l"]);
        assert!(m.get_flag("list-repos"));
    }

    #[test]
    fn detect_repos_captures_directory() {
        let m = parse(&["git-helper", "-d", "."]);
        assert_eq!(m.get_one::<String>("detect-repos").map(|s| s.as_str()), Some("."));
    }

    #[test]
    fn pull_flag_short() {
        let m = parse(&["git-helper", "-p"]);
        assert!(m.get_flag("pull"));
    }

    #[test]
    fn push_flag_short() {
        let m = parse(&["git-helper", "-P"]);
        assert!(m.get_flag("push"));
    }

    #[test]
    fn clone_remote_branches_captures_identifier() {
        let m = parse(&["git-helper", "-c", "myrepo"]);
        assert_eq!(
            m.get_one::<String>("clone-remote-branches").map(|s| s.as_str()),
            Some("myrepo")
        );
    }

    #[test]
    fn config_path_short_flag() {
        let m = parse(&["git-helper", "-C", "/custom/path.toml"]);
        assert_eq!(
            m.get_one::<String>("config").map(|s| s.as_str()),
            Some("/custom/path.toml")
        );
    }

    #[test]
    fn invalid_flag_returns_error() {
        let result = build_cli().try_get_matches_from(["git-helper", "--not-a-real-flag"]);
        assert!(result.is_err());
    }
}
