use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use futures_util::StreamExt;
use openrouter_rs::{OpenRouterClient, api::chat::*, types::Role};

mod hello;
mod config;
mod cil;


#[tokio::main]
async  fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_api_key().api_key;
    let key:&str = &config;
    let diff = cil::diff();
let client = OpenRouterClient::builder()
    .api_key(key)
    .build()?;

let request = ChatCompletionRequest::builder()
    .model("arcee-ai/trinity-large-preview:free")
    .messages(vec![Message::new(Role::User, format!("Generate a short Git commit message in English based on this git diff.

Format:
<type>: <body>

Rules:
- Use conventional types (feat, fix, refactor, chore, docs, test)
- Present tense
- One line only

Git diff:
{}", diff))])
    .build()?;

let mut stream = client.chat().stream(&request).await?;

let mut commit_message = String::new();

while let Some(result) = stream.next().await {
    if let Ok(response) = result {
        if let Some(content) = response.choices[0].content() {
            commit_message.push_str(content);
            print!("{}", content);
        }
    }
}
cil::commit(&commit_message);
cil::push();
println!();
    Ok(())
}


fn handle_events(input: &mut String) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            // Sous Windows, évite le double déclenchement
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => return Ok(true), // On veut quitter
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => { input.pop(); }
                    _ => {}
                }
            }
        }
    }
    Ok(false) // On continue
}
