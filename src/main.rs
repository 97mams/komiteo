
mod config;
mod agent;
mod git;
mod views;

use crate::views::tui;

#[tokio::main]
async fn main() {
    tui::render().await
}