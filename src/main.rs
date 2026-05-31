mod agent;
mod command;
mod config;
mod event;
mod git;
mod utils;
mod views;
mod args;

use clap::Parser;

use crate::{git::cil, views::tui};

#[tokio::main]
async fn main() { 
    println!("log {}", cil::check_log());
    let _ = args::Komiteo::parse();
    tui::render().await;
    let _ = event::input().await;
}
