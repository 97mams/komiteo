mod agent;
mod command;
mod config;
mod event;
mod git;
mod utils;
mod views;
mod args;

use clap::Parser;

use crate::views::tui;

#[tokio::main]
async fn main() {
    println!("delete this line, bat now i am don't want to code"); 
    let _ = args::Komiteo::parse();
    tui::render().await;
    let _ = event::input().await;
}
