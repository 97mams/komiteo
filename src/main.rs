
mod config;
mod agent;
mod git;
mod views;
mod event;

use crate::views::tui;

#[tokio::main]
async fn main() {
    tui::render().await;
    event::input();
}