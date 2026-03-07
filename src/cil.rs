use std::process::Command;
 
 
  Command::new("pwd").status().expect("Failed to list terminal");