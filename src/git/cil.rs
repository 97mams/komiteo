use std::process::Command;

use crate::utils::clean_file_name;
 
pub fn check_status() -> Vec<String> {
  let cmd = Command::new("git")
    .args(&["status", "--porcelain"])
    .output()
    .expect("Failed to execute git status");

  let response =String::from_utf8_lossy(&cmd.stdout).to_string();

  response.lines().map(|s| s.to_string()).collect()
 }

pub fn diff(file_name: String) -> String {
  let file = clean_file_name(file_name);
  let cmd = Command::new("git")
    .arg("diff")
    .arg(file)
    .output()
    .expect("Failed to execute git diff");

  let response =String::from_utf8_lossy(&cmd.stdout).to_string();

  if response.is_empty() {
    return "".to_string();
  }
  response
 }

 pub fn commit(message: &str) -> String {
  let cmd = Command::new("git")
    .arg("commit")
    .arg("-am")
    .arg(message)
    .output()
    .expect("Failed to execute git commit");

  let response =String::from_utf8_lossy(&cmd.stdout).to_string();

  if response.is_empty() {
    return "".to_string();
  }
  response
 }

//  pub fn push() {
//   Command::new("git")
//     .arg("push")
//     .status()
//     .expect("Failed to execute git push");
//  }