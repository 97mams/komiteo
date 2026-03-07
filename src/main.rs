mod hello;
mod config;
fn main() {
    let config = config::get_api_key();
    println!("API Key: {}", config.api_key);
}