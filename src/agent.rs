use crate::config;
use crate::cil;

use openrouter_rs::{
    OpenRouterClient,
    api::chat::*,
    types::{Role},
};
use fancy_print::{FancyPrinter, Animation};
use std::time::Duration;



#[tokio::main]
pub async fn agent() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_api_key_from_config().trim().to_string();
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
    let response = client.chat().create(&request).await?;

    let printer = FancyPrinter::builder()
    .animation(Animation::CharacterCycling)
    .time_delay(Duration::from_millis(1))
    .multi_line(false)
    .ignore_newlines(false)
    .build();
    
  printer.print(response.choices[0].content().unwrap_or(""));
    // cil::commit(&commit_message);
    // cil::push();
    println!();
    Ok(())
}
