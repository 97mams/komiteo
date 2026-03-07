use std::process::Command;
 
pub fn cil() -> String {
   Command::new("pwd").status().expect("Failed to display diff");
  let cmd = Command::new("git")
    .arg("diff")
    .output()
    .expect("Failed to execute git diff");

  String::from_utf8_lossy(&cmd.stdout).to_string()
 }