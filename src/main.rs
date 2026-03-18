// use futures_util::StreamExt;
// use openrouter_rs::{OpenRouterClient, api::chat::*, types::Role};
// use crossterm::{
//     event::{self, Event, KeyCode, KeyEventKind},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::io;

// mod hello;
// mod config;
// mod cil;

use color_eyre::eyre::{Ok,Result};
use ratatui::{DefaultTerminal,Frame};


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

    ratatui::run(app)?;

    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

fn afotsy_maty (){
    println!("Arabaina tratriny asaramany eh!");
}