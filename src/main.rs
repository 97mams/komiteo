use futures_util::StreamExt;
use openrouter_rs::{OpenRouterClient, api::chat::*, types::Role};

mod hello;
mod config;
// mod cil;


#[tokio::main]
async  fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_api_key().api_key;
    let key:&str = &config;
    // let cmd = cil::cil();
let client = OpenRouterClient::builder()
    .api_key(key)
    .build()?;

let request = ChatCompletionRequest::builder()
    .model("arcee-ai/trinity-large-preview:free")
    .messages(vec![Message::new(Role::User, "hello world")])
    .build()?;

let mut stream = client.chat().stream(&request).await?;

while let Some(result) = stream.next().await {
    if let Ok(response) = result {
        if let Some(content) = response.choices[0].content() {
            print!("{}", content);
        }
    }
}
    Ok(())
}