use crossterm::style::Stylize;

use crate::views::hello;
use crate::event;
// use crate::cmd;
use crate::agent::openrouter;
use crate::config::config;
// use crate::hello;

pub async  fn render() {
    hello::logo();
    hello::hero();
    if !config::check_api_key() {
        hello::description();
    }
   if let Err(e) = openrouter::agent().await {
        println!("Erreur: {:?}", e);
    }
    let _ = event::input();
    println!("{}", "Ctrl+c pour quitter".grey());
}
