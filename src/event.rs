use std::io;

use colored::Colorize;

use crate::config::config;

pub fn input(){
    let mut input = String::new();
    print!("\nSi vous avez déjà une clé API, veuillez la saisir ici: \n");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let value = input.trim();

    if value.is_empty() {
        println!("Entrer" );
        println!("{}","commit".green());
        return;
    }

    if config::check_api_key() {
        println!("liste api key déjà configuré: {}", config::get_api_key_from_config());
        config::save_api_key(input.trim());
} else {
        let inpu_client = input.trim().to_string();
        let args = inpu_client.split_whitespace().collect::<Vec<&str>>();
        println!("Vous avez entré: {}", args[0]);
        // cmd::komiteo_cmd(cmd, arg)
    }

}