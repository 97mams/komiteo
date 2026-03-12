use reqwest::Client;
use serde::{Deserialize, Serialize};

mod hello;
mod config;
mod cil;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Deserialize, Debug)]
pub struct ResponseBody {
    response: String,
}

#[tokio::main]
async  fn main() -> Result<(), Box<dyn std::error::Error>> {
    hello::hello_welcome();
    let cmd = cil::cil();

    let client = Client::new();

    let body = RequestBody {
        model: "qwen3:8b".to_string(),
        prompt: cmd,
        stream: false,
    };

    let res = client
        .post("http://localhost:8080/generate")
        .json(&body)
        .send()
        .await?;

    let response_body = res.json::<ResponseBody>().await?;
    println!("{:?}", response_body.response);

    Ok(())
}