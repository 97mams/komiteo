use std::fs;
mod hello;

struct Config {
    api_key: String,
}

fn get_api_key() -> Config {
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".komiteo/config.toml");

    if config_path.exists() {
        let content = fs::read_to_string(config_path).unwrap();
        return Config {
            api_key: content.trim().to_string(),
        };
    }

   let hello = hello::hello_welcome();

    let mut config=  Config {
        api_key: hello.trim().to_string(),
    };
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(&config_path, hello.trim()).unwrap();

    config
}