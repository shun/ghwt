use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_set_first_time() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    let mut cmd = Command::cargo_bin("ghwt").unwrap();
    cmd.env("GHWT_CONFIG_PATH", config_path.to_str().unwrap());
    cmd.arg("config")
        .arg("set")
        .arg("test.key")
        .arg("test_value");

    cmd.assert().success().stdout(
        predicate::str::contains("Key 'test.key' set to value 'test_value'"),
    );

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("key = \"test_value\""));
}

#[test]
fn test_set_add_new_key_to_existing_config() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    fs::write(&config_path, "[core]\nroot = \"/tmp\"").unwrap();

    let mut cmd = Command::cargo_bin("ghwt").unwrap();
    cmd.env("GHWT_CONFIG_PATH", config_path.to_str().unwrap());
    cmd.arg("config")
        .arg("set")
        .arg("user.name")
        .arg("testuser");

    cmd.assert().success();

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("root = \"/tmp\""));
    assert!(content.contains("name = \"testuser\""));
}

#[test]
fn test_set_update_value_in_existing_config() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    fs::write(&config_path, "[core]\nroot = \"/old/path\"").unwrap();

    let mut cmd = Command::cargo_bin("ghwt").unwrap();
    cmd.env("GHWT_CONFIG_PATH", config_path.to_str().unwrap());
    cmd.arg("config")
        .arg("set")
        .arg("core.root")
        .arg("/new/path");

    cmd.assert().success();

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("root = \"/new/path\""));
    assert!(!content.contains("root = \"/old/path\""));
}

#[test]
fn test_set_quiet_suppresses_message() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    let mut cmd = Command::cargo_bin("ghwt").unwrap();
    cmd.env("GHWT_CONFIG_PATH", config_path.to_str().unwrap());
    cmd.arg("config")
        .arg("set")
        .arg("test.key")
        .arg("test_value")
        .arg("-q");

    cmd.assert().success().stdout(predicate::str::is_empty());
}
