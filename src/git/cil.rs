use std::process::Command;
 
pub fn diff() -> String {
  let cmd = Command::new("git")
    .arg("diff")
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