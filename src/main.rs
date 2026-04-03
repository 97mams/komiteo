
<<<<<<< HEAD
// use futures_util::StreamExt;
// use openrouter_rs::{OpenRouterClient, api::chat::*, types::Role};
// use crossterm::{
//     event::{self, Event, KeyCode, KeyEventKind},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::io;
=======
mod hello;
mod config;
// mod cil;
>>>>>>> parent of afbfe60... feat: update cil module usage and modify chat request message format

// mod hello;
// mod config;
// mod cil;

use color_eyre::eyre::{Ok,Result};
use ratatui::{
    crossterm::{
        event::{self, Event},
        terminal,
    },
    DefaultTerminal
};
#[tokio::main]
async  fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_api_key().api_key;
    let key:&str = &config;
    // let cmd = cil::cil();
let client = OpenRouterClient::builder()
    .api_key(key)
    .build()?;

<<<<<<< HEAD

// #[tokio::main]
fn main() -> Result<()> {
    // let config = config::get_api_key().api_key;
    // let key:&str = &config;
    // let diff = cil::diff();
    // let client = OpenRouterClient::builder()
    //     .api_key(key)
    //     .build()?;

    // let request = ChatCompletionRequest::builder()
    //     .model("arcee-ai/trinity-large-preview:free")
    //     .messages(vec![Message::new(Role::User, format!("Generate a short Git commit message in English based on this git diff.

    // Format:
    // <type>: <body>

    // Rules:
    // - Use conventional types (feat, fix, refactor, chore, docs, test)
    // - Present tense
    // - One line only

    // Git diff:
    // {}", diff))])
    //     .build()?;

    // let mut stream = client.chat().stream(&request).await?;

    // let mut commit_message = String::new();
=======
let request = ChatCompletionRequest::builder()
    .model("arcee-ai/trinity-large-preview:free")
    .messages(vec![Message::new(Role::User, "hello world")])
    .build()?;

    // while let Some(result) = stream.next().await {
    //     if let Ok(response) = result {
    //         if let Some(content) = response.choices[0].content() {
    //             commit_message.push_str(content);
    //             print!("{}", content);
    //         }
    //     }
    // }
    // cil::commit(&commit_message);
    // cil::push();
    // println!();
    // Ok(())
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();

    return result;
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char('q') => break,
                _ => {
                }
            }
        }
    }
while let Some(result) = stream.next().await {
    if let Ok(response) = result {
        if let Some(content) = response.choices[0].content() {
            print!("{}", content);
        }
    }
}
    Ok(())
} 
