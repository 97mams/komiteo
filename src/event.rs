use crate::{config::config, views::tui::mety};
use crate::utils::Command;
use crate::command::cmd;
use colored::Colorize;
use rustyline::{DefaultEditor, error::ReadlineError};

use crate::git::cil;

pub async fn input() -> rustyline::Result<()> {

    let mut rl = DefaultEditor::new()?;
    loop {
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str())?;
            let value = line.split_whitespace().collect::<Vec<&str>>();
            match value[0] {
                "log" => {
                    println!("{}", cil::state().green());
                },
                "komiteo" => {
                    mety();
                },
                "reconfig" => {
                    reconfig();
                },
                "help" => {
                    println!("Voici les commandes disponibles :");
                    for cmd in &build_list_of_commands() {
                        println!("- {} ", cmd.green());
                    }
                },
                "exit" => break,
                _ => {
                    if value[0].len() <= 10 {
                        // if true in the config file, then we can use the command
                        let command = Command::parse_cmd(value);
                       println!("{}",cmd::komiteo_cmd(command.cmd, command.arg));
                    } else {
                        config::save_api_key(line.as_str());
                    }
                }
            }


        },
        Err(ReadlineError::Interrupted) => break, // Ctrl-C
            Err(ReadlineError::Eof) => break,         // Ctrl-D
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
    }
}
    Ok(())
}

fn reconfig() {
    println!("Veuillez entrer votre nouvelle clé API :");
}

fn build_list_of_commands() -> Vec<String> {
    let mut commands = Vec::new();
     commands.push(add_point_separated("komiteo", "commande pour lancer le commit manuel.".to_string()));
    commands.push(add_point_separated("reconfig", "Reconfigure le clé API".to_string()));
    commands.push(add_point_separated("help", "Afficher les commandes disponibles".to_string()));
    commands.push(add_point_separated("exit", "Quitter l'application".to_string()));
    
    commands
}

fn add_point_separated(namecmd: &str, doc: String)-> String {
    let mut result = namecmd.to_owned();
    let number_of_pointers = 20;
    let calculated_pointers = number_of_pointers - namecmd.len();
    result.push_str(".".repeat(calculated_pointers).as_str());
    result.push_str(doc.as_str());

    return result;
}