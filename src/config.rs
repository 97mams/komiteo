
use std::fs;
// use crate::hello;
// use cfonts::{Align, Colors, Options, say};

pub struct Config {
    pub api_key: String,
}

fn save_api_key(api_key: &str) {
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".komiteo/config.toml");

    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(config_path, api_key.as_bytes()).unwrap();
}

pub fn get_api_key_from_config() -> String {
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".komiteo/config.toml");

    let content = fs::read_to_string(config_path).unwrap();
    content.trim().to_string()
}

pub fn check_api_key() -> bool {
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".komiteo/config.toml");

    config_path.exists()
}