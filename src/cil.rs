use std::process::Command;
 
pub fn diff() -> String {
  let cmd = Command::new("git")
    .arg("diff")
    .output()
    .expect("Failed to execute git diff");

  String::from_utf8_lossy(&cmd.stdout).to_string()
 }

//  pub fn commit(message: &str) {
//   Command::new("git")
//     .arg("commit")
//     .arg("-am")
//     .arg(message)
//     .status()
//     .expect("Failed to execute git commit");
//  }

//  pub fn push() {
//   Command::new("git")
//     .arg("push")
//     .status()
//     .expect("Failed to execute git push");
//  }