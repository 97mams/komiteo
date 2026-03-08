use std::process::Command;
use futures_util::StreamExt;
use openrouter_rs::{OpenRouterClient, api::chat::*};

pub async fn ai_agent(api_key: &str, prompt: &str){
    let client = OpenRouterClient::builder()
        .api_key(api_key)
        .build();
    let request = ChatCompilationRequest::builder()
        .model("arcee-ai/trinity-large-preview:free")
        .messages(vec![
            Message::new(Role::User,prompt)])
        .build();
    let mut stream = client.chat().stream(&request).await.unwrap();

    while let Some(result) = stream.next().await {
        if let Ok(response) = result {
            if let Some(content) = response.choices[0].content() {
                print!("{}", content);
            }
        }
    }
}