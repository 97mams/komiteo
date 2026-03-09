use futures_util::StreamExt;
use openrouter_rs::{
    OpenRouterClient,
    api::chat::*,
    types::Role,
};

pub async fn ai_agent(
    api_key: &str,
    prompt: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    let client = OpenRouterClient::builder()
        .api_key(api_key)
        .build()?; // pas unwrap

    let request = ChatCompletionRequest::builder()
        .model("arcee-ai/trinity-large-preview:free")
        .messages(vec![Message::new(Role::User, prompt)])
        .build()?;

    let mut stream = client.chat().stream(&request).await?;

    while let Some(result) = stream.next().await {

        match result {
            Ok(response) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(content) = choice.content() {
                        print!("{}", content);
                    }
                }
            }

            Err(err) => {
                eprintln!("Stream error: {}", err);
            }
        }
    }

    Ok(())
}