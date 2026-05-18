mod agent;
mod command;
mod config;
mod event;
mod git;
mod utils;
mod views;
mod watcher;

use crate::views::tui;

#[tokio::main]
async fn main() {
    tui::render().await;
    let _ = event::input().await;
    // if let Err(e) = watcher::start::watch(".") {
    //     eprintln!("Error watching files: {:?}", e);
    // }
}
