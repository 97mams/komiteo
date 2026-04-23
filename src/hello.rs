use std::{io::{self, Write}, thread};

use boxy_cli::prelude::*;
use cfonts::{Align, Colors, Options, say};

struct App {
    api_key: String,
}

pub fn hello_welcome() -> String {

    say(Options {
            text: String::from("KOMITEO"),
             align: Align::Center,
             colors: vec![Colors::YellowBright, Colors::YellowBright],
            ..Options::default()
        });

    my_block();

    let paragraphes = "👋 Bonjour ! Bienvenue dans l'aventure KOMITEO.

Avant de pouvoir générer des messages de commit parfaits et de pusher votre code
en un clin d'œil, nous devons configurer un petit quelque chose.

KOMITEO utilise la puissance de l'IA d'OpenRouter pour comprendre vos changements
et écrire des messages de commit clairs, concis et standardisés.\n";

    display_text_with_typing_effect(paragraphes);

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).expect("Failed to read line");
    let api_key = api_key.trim().to_string();

    let app = App { api_key };
    return app.api_key;



}

fn my_block() {
    return  Boxy::builder()
        .box_type(BoxType::Rounded)
        .color("#FFED29")
        .add_segment("Le CLI qui automatise votre flux Git avec l'intelligence d'OpenRouter.", "#FFED29", BoxAlign::Center)
        .width(100)
        .build()
        .display();
}

fn display_text_with_typing_effect(text: &str) {
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(std::time::Duration::from_millis(50));
    }
}