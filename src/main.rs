use std::process::Command;
mod hello;


#[warn(unused_must_use)]
fn main() {
hello::hello_welcome();
    Command::new("pwd").status().expect("Failed to list terminal");
}