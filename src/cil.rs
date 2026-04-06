use std::process::Command;
 
pub fn diff() -> String {
   Command::new("pwd").status().expect("Failed to display diff");
  let cmd = Command::new("git")
    .arg("diff")
    .output()
    .expect("Failed to execute git diff");

  String::from_utf8_lossy(&cmd.stdout).to_string()
 }

 pub fn commit(message: &str) -> String {
  Command::new("pwd").status().expect("Failed to display diff");
  let cmd = Command::new("git")
    .arg("commit")
    .args(&["-am", message])
    .output()
    .expect("Failed to execute git commit");

  String::from_utf8_lossy(&cmd.stdout).to_string()
 }