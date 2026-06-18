use std::time::Duration;

use crate::config::config;
use crate::views::hello;
use crate::git::cil;
use crate::utils;

use crossterm::style::Stylize;
use indicatif::{ ProgressBar, ProgressStyle };
use openrouter_rs::{ OpenRouterClient, api::chat::*, types::{ Role } };

pub async fn agent(diff: String, file: String) -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_api_key_from_config().trim().to_string();
    let name = utils::extract_name(file.clone());
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

    let request = ChatCompletionRequest::builder()
        .model("openrouter/owl-alpha")
        .messages(
            vec![
                Message::new(
                    Role::User,
                    format!("You are an expert Git commit message generator.

You are given:

* `diff`: the Git diff of the changes.
* `file`: the modified file name.

Your task is to generate a single Conventional Commit message.

## Output Format

<type>(<file>): <description>

Examples:

* feat(login-form): add password validation
* fix(api): handle null response
* refactor(user-service): simplify authentication flow

## Rules

1. Analyze the provided diff and determine the most appropriate commit type:

   * feat
   * fix
   * refactor
   * perf
   * docs
   * style
   * test
   * build
   * ci
   * chore

2. Generate a concise description that summarizes the most significant change.

3. Use imperative verbs:

   * add
   * update
   * fix
   * remove
   * improve
   * refactor
   * optimize
   * simplify
   * rename

4. Always use the provided `file` value as the scope.

5. Keep the description short and meaningful.

## Empty Diff Handling

If the diff is empty, unavailable, or provides insufficient information:

1. Use the file name as contextual information.
2. Infer the most reasonable change.
3. Generate a valid Conventional Commit message.
4. Never mention that the diff is empty.
5. Never ask for more information.

## Output Requirements

* Output exactly one line.
* No markdown.
* No code fences.
* No quotes.
* No explanations.
* No extra text.

Input:

File:
{file}

Diff:
{diff}


", diff = diff, file = name)
                )
            ]
        )
        .build()?;

    let response = client.chat().create(&request).await?;
    let message = response.choices[0].content().unwrap_or("").to_string();
    let _ = cil::commit(&message);
    pb.finish_with_message(file.trim().green().to_string());

    hello::display_text_with_typing_effect(format!(" - {} \n", &message).as_mut_str());

    Ok(())
}
