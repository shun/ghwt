use std::fs;
use std::path::PathBuf;
use toml::Value;

pub struct Config {
    data: Value,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_path = Self::get_config_path()?;
        let default_root = dirs_next::home_dir()
            .ok_or_else(|| "Could not find home directory".to_string())?
            .join("ghwt");

        let mut config_data = if config_path.exists() {
            let content = fs::read_to_string(&config_path).map_err(|e| format!("Failed to read config file: {}", e))?;
            toml::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))?
        } else {
            Value::Table(toml::map::Map::new())
        };

        if config_data.get("core").and_then(|c| c.get("root")).is_none() {
            let core_table = config_data
                .as_table_mut()
                .unwrap()
                .entry("core")
                .or_insert_with(|| Value::Table(toml::map::Map::new()));
            core_table
                .as_table_mut()
                .unwrap()
                .insert("root".to_string(), Value::String(default_root.to_str().unwrap().to_string()));
        }

        Ok(Self { data: config_data })
    }

    fn get_config_path() -> Result<PathBuf, String> {
        if let Ok(path) = std::env::var("GHWT_CONFIG_PATH") {
            return Ok(PathBuf::from(path));
        }
        let config_dir = dirs_next::config_dir().ok_or_else(|| "Could not find config directory".to_string())?;
        Ok(config_dir.join("ghwt").join("config.toml"))
    }

    #[cfg(test)]
    fn new(data: Value) -> Self {
        Self { data }
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        if key.is_empty() || key.starts_with('.') || key.ends_with('.') || key.contains("..") {
            return None;
        }

        let mut current_value = &self.data;
        for part in key.split('.') {
            current_value = current_value.get(part)?;
        }

        if current_value.is_table() {
            None
        } else {
            Some(current_value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::toml;

    // Helper function to create a standard Config instance for tests.
    fn setup_config() -> Config {
        let toml_value = toml! {
            [core]
            root = "/path/to/root"
            [section]
            sub_section = { key = "value" }
            value_key = 123
        };
        Config::new(Value::Table(toml_value))
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
        assert_eq!(config.get_value("nonexistent.key"), None);
    }

    #[test]
    fn test_get_value_for_nonexistent_nested_key() {
        let config = setup_config();
        assert_eq!(config.get_value("section.nonexistent_key"), None);
    }

    #[test]
    fn test_get_value_for_nonexistent_intermediate_table() {
        let config = setup_config();
        assert_eq!(config.get_value("nonexistent_section.key"), None);
    }

    #[test]
    fn test_get_value_for_key_pointing_to_table() {
        // "section" is a table, not a value, so it should return None.
        let config = setup_config();
        assert_eq!(config.get_value("section"), None);
    }

    #[test]
    fn test_get_value_for_empty_key() {
        let config = setup_config();
        assert_eq!(config.get_value(""), None);
    }

    #[test]
    fn test_get_value_for_key_with_leading_dot() {
        let config = setup_config();
        assert_eq!(config.get_value(".core.root"), None);
    }

    #[test]
    fn test_get_value_for_key_with_trailing_dot() {
        let config = setup_config();
        assert_eq!(config.get_value("core.root."), None);
    }

    #[test]
    fn test_get_value_for_key_with_double_dot() {
        let config = setup_config();
        assert_eq!(config.get_value("core..root"), None);
    }
}
