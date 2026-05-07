// use std::io;

// use colored::Colorize;

// use crate::config::config;

use rustyline::{DefaultEditor, error::ReadlineError};

pub fn input()->rustyline::Result<()>{

    let mut rl = DefaultEditor::new()?;
    loop {
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str())?;
            println!("Vous avez entré: {}", line);
            // cmd::komiteo_cmd(cmd, arg)
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