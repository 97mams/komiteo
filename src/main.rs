mod hello;
mod config;
mod cil;
fn main() {
    config::get_api_key();
    let cmd = cil::cil();
    println!("Git diff executed successfully {:?}", cmd);
}