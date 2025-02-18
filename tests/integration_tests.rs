use std::fs;
use std::path::Path;
use git_helper::config::{read_config, write_config};

// create a test config file
fn setup_test_config() -> String {
    let test_config_path = "./tests/test_git_helper_config.toml";
    let _ = fs::remove_file(test_config_path); // Ensure a fresh start
    test_config_path.to_string()
}

#[test]
fn test_add_repo() {
    let config_path = setup_test_config();
    let test_repo = "/tmp/test_repo";

    let mut config = read_config(Path::new(&config_path));
    config.repositories.insert(test_repo.to_string());
    write_config(Path::new(&config_path), &config);

    let updated_config = read_config(Path::new(&config_path));
    assert!(updated_config.repositories.contains(test_repo), "Repository should be added");
}

#[test]
fn test_remove_repo() {
    let config_path = setup_test_config();
    let test_repo = "/tmp/test_repo";

    let mut config = read_config(Path::new(&config_path));
    config.repositories.insert(test_repo.to_string());
    write_config(Path::new(&config_path), &config);

    config.repositories.remove(test_repo);
    write_config(Path::new(&config_path), &config);

    let updated_config = read_config(Path::new(&config_path));
    assert!(!updated_config.repositories.contains(test_repo), "Repository should be removed");
}

#[test]
fn test_list_repos() {
    let config_path = setup_test_config();
    let test_repo_1 = "/tmp/test_repo_1";
    let test_repo_2 = "/tmp/test_repo_2";

    let mut config = read_config(Path::new(&config_path));
    config.repositories.insert(test_repo_1.to_string());
    config.repositories.insert(test_repo_2.to_string());
    write_config(Path::new(&config_path), &config);

    let updated_config = read_config(Path::new(&config_path));
    assert_eq!(updated_config.repositories.len(), 2, "Should list two repositories");
    assert!(updated_config.repositories.contains(test_repo_1));
    assert!(updated_config.repositories.contains(test_repo_2));
}

#[test]
fn test_invalid_config_file() {
    let invalid_path = "./tests/nonexistent_config.toml";
    let config = read_config(Path::new(invalid_path));
    assert!(config.repositories.is_empty(), "Invalid config should return an empty repository set");
}
