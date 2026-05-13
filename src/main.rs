
mod config;
mod agent;
mod git;
mod command;
mod views;
mod event;
mod utils;

use crate::views::tui;

#[tokio::main]
async fn main() {
    tui::render().await;
   let _ = event::input().await;
}