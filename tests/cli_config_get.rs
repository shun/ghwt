use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

#[test]
fn test_config_get_core_root_with_custom_config() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let config_path = dir.path().join("config.toml");
    let mut file = File::create(&config_path)?;
    let toml_content = r#"[core]
root = "/custom/path"
"#;
    file.write_all(toml_content.as_bytes())?;

    let mut cmd = Command::cargo_bin("ghwt")?;
    cmd.env("GHWT_CONFIG_PATH", config_path.to_str().unwrap());
    cmd.arg("config").arg("get").arg("core.root");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("/custom/path"));

    Ok(())
}

#[test]
fn test_config_get_core_root_with_default_config() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    // Don't create a config file, so it uses the default.

    let mut cmd = Command::cargo_bin("ghwt")?;
    cmd.env("HOME", dir.path().to_str().unwrap()); // Set HOME to a temp dir to avoid using the real user config
    cmd.arg("config").arg("get").arg("core.root");

    let expected_path = dir.path().join("ghwt");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected_path.to_str().unwrap()));

    Ok(())
}

#[test]
fn test_config_get_other_key_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ghwt")?;
    cmd.arg("config").arg("get").arg("other.key");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Only 'core.root' is supported for the get command."));

    Ok(())
}
