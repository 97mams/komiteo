use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
}

fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join("komiteo/config.toml")
}

pub fn load_config() -> Option<Config> {
    let path = config_path();
    if path.exists() {
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    } else {
        None
    }
}