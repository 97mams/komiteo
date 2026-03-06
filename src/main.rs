use std::process::Command;
mod hello;


#[warn(unused_must_use)]
fn main() {
   let _ = hello::hello_welcome().expect("Failed to run hello welcome");
    Command::new("pwd").status().expect("Failed to list terminal");
}