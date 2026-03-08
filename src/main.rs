mod hello;
mod config;
mod cil;
mod ai;
fn main() {
     let api_key = config::get_api_key();
    let cmd = cil::cil();
    let ai_agent_response = ai::ai_agent(&api_key.api_key, &cmd);
    println!("OpenRouter response: {:?}", ai_agent_response);
}