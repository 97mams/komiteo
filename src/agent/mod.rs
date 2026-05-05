use spinners::{Spinner, Spinners};

pub mod openrouter;

pub async fn run_agent_with_spinner() {
    let mut sp = Spinner::new(Spinners::Dots9, "Attend...".into());
    
    match openrouter::agent().await {
        Ok(agent) => {
            
            println!("{}", agent);
            sp.stop();
        }
        Err(e) => {
            print!("Error: {}", e);
            sp.stop();
        }
    }
}