// use std::{io::{self, Write}, thread};
// use boxy_cli::prelude::*;
// use cfonts::{say, Options};

pub fn hero() -> &'static str {
    return "Le CLI qui automatise votre flux Git avec l'intelligence d'OpenRouter. \n\n";
}

pub fn description() -> &'static str {
    return "- Bonjour ! Bienvenue dans l'aventure KOMITEO.

Avant de pouvoir générer des messages de commit parfaits et de pusher votre code
en un clin d'œil, nous devons configurer un petit quelque chose.

KOMITEO utilise la puissance de l'IA d'OpenRouter pour comprendre vos changements
et écrire des messages de commit clairs, concis et standardisés.\n\nFaite (ctl+clic) sur le lien, allez sur le site, créez un compte (c'est souvent gratuit ou très peu cher pour
    commencer) et générez une clé API.\nhttps://openrouter.ai/\n"
}

//box 
// fn my_block(size: usize, text: &str) {
//     return  Boxy::builder()
//         .box_type(BoxType::Rounded)
//         .color("#FFED29")
//         .add_segment(text, "#FFED29", BoxAlign::Center)
//         .width(size)
//         .build()
//         .display();
// }


//animated text
// fn display_text_with_typing_effect(text: &str) {
//     for c in text.chars() {
//         print!("{}", c);
//         io::stdout().flush().unwrap();
//         thread::sleep(std::time::Duration::from_millis(50));
//     }
// }