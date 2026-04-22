
use std::fs;
use crate::hello;
use cfonts::{Align, Colors, Options, say};

pub struct Config {
    pub api_key: String,
}

pub fn get_api_key() -> Config {

    let config_path = dirs::home_dir()
        .unwrap()
        .join(".komiteo/config.toml");

    if config_path.exists() {
        say(Options {
            text: String::from("KOMITEO"),
             align: Align::Center,
             colors: vec![Colors::YellowBright, Colors::YellowBright],
            ..Options::default()
        });
        let content = fs::read_to_string(config_path).unwrap();
        return Config {
            api_key: content.trim().to_string(),
        };
    }

   let hello = hello::hello_welcome();

    let config=  Config {
        api_key: hello,
    };
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(&config_path, config.api_key.as_bytes()).unwrap();

    config
}