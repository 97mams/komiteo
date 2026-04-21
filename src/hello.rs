use std::{io};


struct App {
    api_key: String,
}

pub fn hello_welcome() -> String {

    println!("Welcome to Komity, the AI-powered Git commit message generator!");
    println!("Please enter your OpenRouter API key:");
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).expect("Failed to read line");
    let api_key = api_key.trim().to_string();

    let app = App { api_key };
    return app.api_key;


}