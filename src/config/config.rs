use crate::views::hello;
use std::fs;

pub fn save_api_key(api_key: &str) {
    if api_validator(api_key) {
        let config_path = dirs::home_dir().unwrap().join(".komiteo/config.toml");

        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        fs::write(config_path, api_key.as_bytes()).unwrap();

        hello::display_text_with_typing_effect(
            "\nClé API enregistrée avec succès !\n Votre agent de auto-commit est actif.\n",
        );
    } else {
        println!("\nClé API invalide. Veuillez réessayer.\n");
        return;
    }
}

pub fn get_api_key_from_config() -> String {
    let config_path = dirs::home_dir().unwrap().join(".komiteo/config.toml");

    let content = fs::read_to_string(config_path).unwrap();
    content.trim().to_string()
}

pub fn check_api_key() -> bool {
    let config_path = dirs::home_dir().unwrap().join(".komiteo/config.toml");

    config_path.exists()
}

pub fn api_validator(api_key: &str) -> bool {
    api_key.len() == 73
}
