use std::process::Command;
mod hello;


#[warn(unused_must_use)]
fn main() {
    let _ = hello::hello_welcome();
    Command::new("pwd").status().expect("Failed to list terminal");
}