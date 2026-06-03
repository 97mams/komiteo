use std::time::Duration;

use crate::config::config;
use crate::views::hello;
use crate::git::cil;

use crossterm::style::Stylize;
use indicatif::{ ProgressBar, ProgressStyle };
use openrouter_rs::{ OpenRouterClient, api::chat::*, types::{ Role } };

pub async fn agent(diff: String, file: String) -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_api_key_from_config().trim().to_string();
    let key: &str = &config;
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", " -"])
    );
    pb.set_message("En attente...");

    let client = OpenRouterClient::builder().api_key(key).build()?;

//     if diff.is_empty() {
//         prompt = "Act as a Git expert. I need you to write a clear, concise, and professional commit message based only on the file name and its role.
// Here is the information:
// - File: [Insert file name here, e.g., src/auth/auth.service.ts]
// - Role / Action: [Briefly explain what this file does or what changed, e.g., Added password hashing logic]

// Writing constraints:
// 1. Use the Conventional Commits format (e.g., feat(scope): message, fix(scope): message, chore, docs, refactor...).
// 2. The subject line must be short (maximum 50-72 characters).
// 3. Use the imperative mood (e.g.,add, fix, improve, implement).
// 4. Provide 3 different options: One short/minimalist, one standard/conventional, and one with a short body/description if applicable.

// Please provide the commit messages now.";
//         return Ok(());
//     }

    let request = ChatCompletionRequest::builder()
        .model("openrouter/owl-alpha")
        .messages(
            vec![
                Message::new(
                    Role::User,
                    format!("Generate a short Git commit message in English based on this git diff.

    Format:
    <type>: <body>

    Rules:
    - Use conventional types (feat, fix, refactor, chore, docs, test)
    - Present tense
    - One line only

    Git diff:
    {}", diff)
                )
            ]
        )
        .build()?;

    let response = client.chat().create(&request).await?;
    let message = response.choices[0].content().unwrap_or("").to_string();
    // let _ = cil::commit(&message);
    pb.finish_with_message(file.trim().green().to_string());

    hello::display_text_with_typing_effect(format!(" - {} \n", &message).as_mut_str());

    Ok(())
}
