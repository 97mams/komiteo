
mod config;
mod agent;
mod git;
mod command;
mod views;
mod event;
mod utils;
mod watcher;

use crate::views::tui;

#[tokio::main]
async fn main() {
    tui::render().await;
   let _ = event::input().await;
   if let Err(e) = watcher::start::watch(".") {
        eprintln!("Error watching files: {:?}", e);
    }
}