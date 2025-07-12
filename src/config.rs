use std::fs;
use std::path::PathBuf;
use toml_edit::{value, DocumentMut, Value};
use anyhow::Result;

pub struct Config {
    doc: DocumentMut,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_path = Self::get_config_path()?;
        let default_root = dirs_next::home_dir()
            .ok_or_else(|| "Could not find home directory".to_string())?
            .join("ghwt");

        let doc = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            content
                .parse::<DocumentMut>()
                .map_err(|e| format!("Failed to parse config file: {}", e))?
        } else {
            DocumentMut::new()
        };

        let mut config = Self { doc };

        if config.get_value("core.root").is_none() {
            config.doc["core"]["root"] = value(default_root.to_str().unwrap());
        }

        Ok(config)
    }

    pub fn get_config_path() -> Result<PathBuf> {
        if let Ok(path) = std::env::var("GHWT_CONFIG_PATH") {
            return Ok(PathBuf::from(path));
        }
        let config_dir =
            dirs_next::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("ghwt").join("config.toml"))
    }

    #[cfg(test)]
    fn new(doc: DocumentMut) -> Self {
        Self { doc }
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        if key.is_empty() || key.starts_with('.') || key.ends_with('.') || key.contains("..") {
            return None;
        }

        let mut current_item = self.doc.as_item();
        for part in key.split('.') {
            current_item = current_item.get(part)?;
        }

        current_item.as_value()
    }

    /// Sets a value in the configuration.
    /// 
    /// # Arguments
    /// * `key_str` - A dot-separated key path (must not be empty)
    /// * `value_str` - The string value to set
    pub fn set_value(&mut self, key_str: &str, value_str: &str) -> Result<()> {
        let mut current_item = self.doc.as_item_mut();

        let keys: Vec<&str> = key_str.split('.').collect();
        let (last_key, parent_keys) = keys.split_last().unwrap();

        for key in parent_keys {
            if current_item.get(key).is_none() {
                current_item[key] = toml_edit::table();
            }
            current_item = &mut current_item[key];
        }

        current_item[last_key] = value(value_str);

        Ok(())
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, self.doc.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml_edit::DocumentMut;

    // Helper function to create a standard Config instance for tests.
    fn setup_config() -> Config {
        let toml_str = r#"
            [core]
            root = "/path/to/root"
            [section]
            sub_section = { key = "value" }
            value_key = 123
        "#;
        let doc = toml_str.parse::<DocumentMut>().expect("Failed to parse toml");
        Config::new(doc)
    }

    #[test]
    fn test_get_value_for_top_level_key() {
        let config = setup_config();
        // For now, this will fail, which is expected in TDD.
        let value = config.get_value("core.root").expect("Value should exist for core.root");
        assert_eq!(value.as_str(), Some("/path/to/root"));
    }

    #[test]
    fn test_get_value_for_nested_key() {
        let config = setup_config();
        let value = config
            .get_value("section.sub_section.key")
            .expect("Value should exist for nested key");
        assert_eq!(value.as_str(), Some("value"));
    }

    #[test]
    fn test_get_value_for_nonexistent_top_level_key() {
        let config = setup_config();
        assert!(config.get_value("nonexistent.key").is_none());
    }

    #[test]
    fn test_get_value_for_nonexistent_nested_key() {
        let config = setup_config();
        assert!(config.get_value("section.nonexistent_key").is_none());
    }

    #[test]
    fn test_get_value_for_nonexistent_intermediate_table() {
        let config = setup_config();
        assert!(config.get_value("nonexistent_section.key").is_none());
    }

    #[test]
    fn test_get_value_for_key_pointing_to_table() {
        // "section" is a table, not a value, so it should return None.
        let config = setup_config();
        assert!(config.get_value("section").is_none());
    }

    #[test]
    fn test_get_value_for_empty_key() {
        let config = setup_config();
        assert!(config.get_value("").is_none());
    }

    #[test]
    fn test_get_value_for_key_with_leading_dot() {
        let config = setup_config();
        assert!(config.get_value(".core.root").is_none());
    }

    #[test]
    fn test_get_value_for_key_with_trailing_dot() {
        let config = setup_config();
        assert!(config.get_value("core.root.").is_none());
    }

    #[test]
    fn test_get_value_for_key_with_double_dot() {
        let config = setup_config();
        assert!(config.get_value("core..root").is_none());
    }

    #[test]
    fn test_set_value_new_key() {
        let mut config = setup_config();
        config.set_value("new.key", "new_value").unwrap();
        let value = config.get_value("new.key").unwrap();
        assert_eq!(value.as_str(), Some("new_value"));
    }

    #[test]
    fn test_set_value_update_existing() {
        let mut config = setup_config();
        config.set_value("core.root", "/new/path").unwrap();
        let value = config.get_value("core.root").unwrap();
        assert_eq!(value.as_str(), Some("/new/path"));
    }

    #[test]
    fn test_set_value_nested_key() {
        let mut config = setup_config();
        config.set_value("core.editor", "vim").unwrap();
        let value = config.get_value("core.editor").unwrap();
        assert_eq!(value.as_str(), Some("vim"));
    }

    #[test]
    fn test_save_writes_to_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test_config.toml");

        let mut config = setup_config();
        config.set_value("test.key", "test_value").unwrap();
        config.save(&path).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("[test]"));
        assert!(content.contains("key = \"test_value\""));
    }

    #[test]
    fn test_save_creates_directory_if_not_exists() {
        // Setup: Create a temp directory, but don't create the subdirectory.
        let dir = tempfile::tempdir().unwrap();
        let subdir_path = dir.path().join("subdir");
        let path = subdir_path.join("test_config.toml");

        let mut config = setup_config();
        config.set_value("test.key", "test_value").unwrap();
        
        // The save method should create the directory if it doesn't exist
        config.save(&path).unwrap();
        
        // Verify the file was created
        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("[test]"));
        assert!(content.contains("key = \"test_value\""));
    }

    #[test]
    fn test_save_handles_write_error() {
        // Setup: Create a read-only directory.
        let dir = tempfile::tempdir().unwrap();
        let readonly_dir = dir.path().join("readonly");
        fs::create_dir(&readonly_dir).unwrap();
        
        // Make the directory read-only
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&readonly_dir, perms).unwrap();
        
        let path = readonly_dir.join("test_config.toml");
        let config = setup_config();
        
        // This should fail because the directory is read-only
        let result = config.save(&path);
        assert!(result.is_err());
    }
}
