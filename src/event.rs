// use std::io;

// use colored::Colorize;

use crate::config::config;
use crate::utils::Command;
use crate::command::cmd;
use rustyline::{DefaultEditor, error::ReadlineError};

pub fn input()->rustyline::Result<()>{

    let defalut_commands = vec!["reconfig", "help", "exit"];

    let mut rl = DefaultEditor::new()?;
    loop {
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str())?;
            let value = line.split_whitespace().collect::<Vec<&str>>();
            match value[0] {
                "reconfig" => {
                    reconfig();
                },
                "help" => {
                    println!("Voici les commandes disponibles :");
                    for cmd in &defalut_commands {
                        println!("- {} ", cmd);
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