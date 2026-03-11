mod hello;
mod config;
mod cil;
mod ai;
fn main() {
    let api_key = config::get_api_key();
    let cmd = cil::cil();
    let content = ai::ai_agent(&api_key.api_key, &cmd);
    println!("{:?}", content.unwrap());
}