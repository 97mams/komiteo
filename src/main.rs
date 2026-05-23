mod agent;
mod command;
mod config;
mod event;
mod git;
mod utils;
mod views;

use crate::views::tui;

#[tokio::main]
async fn main() {
    tui::render().await;
    let _ = event::input().await;
    println!("I am lazy, I will do it later");
}
