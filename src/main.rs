use openrouter_rs::{
    OpenRouterClient,
    api::chat::*,
    types::{Role},
};

// mod hello;
// mod cmd;
mod config;
mod cil;
// mod agent;
// mod tui;


#[tokio::main]
async  fn main()  -> Result<(), Box<dyn std::error::Error>> {
   // let _= tui::render();
   // let _ = agent::agent();
   let config = config::get_api_key_from_config().trim().to_string();
    let key:&str = &config;
    let diff = cil::diff();

    let client = OpenRouterClient::builder()
        .api_key(key)
        .build()?;

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
    // cil::commit(&commit_message);
    // cil::push();
    println!("{:?}", response.choices);
    Ok(())
}
