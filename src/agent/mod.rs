pub mod openrouter;

pub async fn run_agent_with_spinner() {
    let mut response = String::from("On attente...").as_str();
    
    match openrouter::agent().await {
        Ok(agent) => {
            response = agent.as_str();
            println!("{}", response);
        }
        Err(e) => {
            print!("Error: {}", e);
        }
    }
}