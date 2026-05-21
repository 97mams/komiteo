mod agent;
mod command;
mod config;
mod event;
mod git;
mod utils;
mod views;

use tokio::time::{
    timeout,
    Duration,
};

use crate::views::tui;

#[tokio::main]
async fn main() {
    tui::render().await;
    let _ = event::input().await;

    let duration = Duration::from_secs(2);

    match timeout(duration, r()).await {
        Ok(r) => println!("Result: {:?}", r),
        Err(_) => println!("Operation timed out after {} seconds.", duration.as_secs()),
    }
    
}

async fn r () -> Result<String, ()> {
    Ok("Hello, world!".to_string())
}