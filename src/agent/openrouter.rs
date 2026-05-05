use std::time::Duration;

use crate::config::config;
use crate::git::cil;

use indicatif::{ProgressBar, ProgressStyle};
use openrouter_rs::{
    OpenRouterClient,
    api::chat::*,
    types::{Role},
};
// use fancy_print::{FancyPrinter, Animation};
// use std::time::Duration;

pub async fn agent() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_api_key_from_config().trim().to_string();
    let key:&str = &config;
    let diff = cil::diff();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // Choose the tick strings (the characters that cycle)
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", " "]),
    );
    pb.set_message("En attente...");

    let client = OpenRouterClient::builder()
        .api_key(key)
        .build()?;

    if diff.is_empty() {
        println!("");
        return Ok(());
    }

    let request = ChatCompletionRequest::builder()
        .model("openrouter/owl-alpha")
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
    let message = response.choices[0].content().unwrap_or("").to_string();
    let commit_message = cil::commit(&message);
    // cil::push();
    pb.finish_with_message("Terminé!");
    println!("{}", commit_message);

    Ok(())
}
