use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    /// Retrieve config at ./backend-config.toml
    ///
    /// # Panics
    ///
    /// Will panic if file not exist or format not matched
    pub fn retrieve_config() -> Self {
        let config_file_path = "./backend-config.toml";
        let config_str = fs::read_to_string(&config_file_path).expect(&format!(
            "Unable to open config file at {}.",
            &config_file_path
        ));
        let config = match toml::from_str(&config_str) {
            Ok(config) => config,
            Err(error) => panic!("Config file parse failed: {}", error),
        };
        config
    }
}
