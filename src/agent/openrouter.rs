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
    println!("name: {}", name); 
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
                    format!("Generate a short Git commit message in English based on this git diff.

    Format:
    <type>: <body>

    Rules:
    - Use conventional types (feat, fix, refactor, chore, docs, test) with file name {file} in parentheses if possible.
    - Present tense
    - One line only
    Git diff:
    {diff}
    
    Act as a Git expert. I need you to write a clear, concise, and professional commit message based only on the file name and its role.
    Here is the information:
    - Role / Action: [Briefly explain what this file does or what changed, e.g., Added password hashing logic]

    Writing constraints:
    1. Use the Conventional Commits format (e.g., feat(scope): message, fix(scope): message, chore, docs, refactor...).
    2. The subject line must be short (maximum 50-72 characters).
    3. Use the imperative mood (e.g.,add, fix, improve, implement).
    4. Provide 3 different options: One short/minimalist, one standard/conventional, and one with a short body/description if applicable.
    Please provide the commit messages now.

    You are an expert Git commit message generator.

Analyze the provided git diff and generate a single concise commit message.

Rules:

* Output ONLY the commit message.
* Do not use markdown.
* Do not add explanations.
* Keep the message short and meaningful.
* Use Conventional Commits format.

Format:
(): 

Examples:
feat(main.rs): add command parser
fix/config.rs: handle missing environment variables
refactor(watcher.rs): simplify file monitoring logic
docs(readme.md): update installation instructions
test/parser.rs): add command parsing tests

Type selection:

* feat: new functionality
* fix: bug fix
* refactor: code restructuring without behavior changes
* perf: performance improvement
* docs: documentation changes
* test: tests added or modified
* style: formatting or code style changes
* build: build system changes
* ci: CI/CD changes
* chore: maintenance tasks

Filename selection:

* Use the primary modified file name.
* Remove the path and keep only the file name.
* If multiple files are modified, choose the file that best represents the main change.
* If the change affects the whole project, use "app".

Description rules:

* Describe the intent of the change, not individual code lines.
* Use present tense verbs such as add, fix, improve, remove, simplify, implement, update, optimize, rename, migrate.
* Ignore whitespace-only or formatting-only changes unless they are the main purpose.

Special cases:

* If the diff is empty or contains no meaningful changes, generate:
  chore(app): continue development progress

Git Diff:
{{DIFF}}

Commit Message:

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
