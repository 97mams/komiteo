use std::{io::{self, Write}, thread};

use boxy_cli::prelude::*;
use cfonts::{Align, Colors, Options, say};

struct App {
    api_key: String,
}

pub fn hello_welcome() -> Result<String, Box<dyn std::error::Error>> {

    say(Options {
            text: String::from("KOMITEO"),
             align: Align::Center,
             colors: vec![Colors::YellowBright, Colors::YellowBright],
            ..Options::default()
        });

    my_block(70, "Le CLI qui automatise votre flux Git avec l'intelligence d'OpenRouter.");

    let paragraphes = "👋 Bonjour ! Bienvenue dans l'aventure KOMITEO.

Avant de pouvoir générer des messages de commit parfaits et de pusher votre code
en un clin d'œil, nous devons configurer un petit quelque chose.

KOMITEO utilise la puissance de l'IA d'OpenRouter pour comprendre vos changements
et écrire des messages de commit clairs, concis et standardisés.\n";

    display_text_with_typing_effect(paragraphes);

    let title = "🔑 Clé API OpenRouter";

    my_block(40, title);

    let instructions = "💡 KOMITEO a besoin de votre clé API OpenRouter pour fonctionner.

Si vous n'en avez pas encore, ne vous inquiétez pas ! C'est rapide et facile.";
    
    display_text_with_typing_effect(instructions);

    
    
    let instructions_ex = "\nFaite (ctl+clic) sur le lien, allez sur le site, créez un compte (c'est souvent gratuit ou très peu cher pour
    commencer) et générez une clé API.
    ";
    let link_openrouter ="\nhttps://openrouter.ai/\n";
    display_text_with_typing_effect(instructions_ex);
    println!("\x1B[1;34m{} \x1B[0m", link_openrouter);

    let case_if_have_api_key = "✅ Si vous avez déjà une clé API, entrez-la ci-dessous :\n";
    display_text_with_typing_effect(case_if_have_api_key);

    let mut api_key = String::new();

    let mut rl = rustyline::DefaultEditor::new()?;
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => println!("Line: {:?}", line),
        Err(_) => println!("No input"),
    }

    io::stdin().read_line(&mut api_key).expect("Failed to read line");
    let api_key = api_key.trim().to_string();

    let app = App { api_key };
    return Ok(app.api_key);



}

fn my_block(size: usize, text: &str) {
    return  Boxy::builder()
        .box_type(BoxType::Rounded)
        .color("#FFED29")
        .add_segment(text, "#FFED29", BoxAlign::Center)
        .width(size)
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