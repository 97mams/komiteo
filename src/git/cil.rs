use std::{
  env,
  process::Command};

use crate::utils::clean_file_name;

pub fn git_init() -> String {
  let cmd = Command::new("git")
    .arg("init")
    .output()
    .expect("Failed to execute git init");

  let response =String::from_utf8_lossy(&cmd.stdout).to_string();

  if response.is_empty() {
    return "".to_string();
  }
  response
 }
 
pub fn check_status() -> Vec<String> {
  let cmd = Command::new("git")
    .args(&["status", "--porcelain"])
    .output()
    .expect("Failed to execute git status");

  let response =String::from_utf8_lossy(&cmd.stdout).to_string();

  response.lines().map(|s| s.to_string()).rev().collect()
 }

pub fn diff(file_name: String) -> String {
  let file = clean_file_name(file_name);

  stagefile(file.clone());
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
    .arg("-m")
    .arg(message)
    .output()
    .expect("Failed to execute git commit");

  let response =String::from_utf8_lossy(&cmd.stdout).to_string();

  if response.is_empty() {
    return "".to_string();
  }
  response
}

pub fn stagefile(file_name: String)  {
    Command::new("git")
    .arg("add")
    .arg(file_name)
    .status()
    .expect("Failed to execute git add");
}

pub fn check_folder_git() -> bool {
  let path_git = env::current_dir().unwrap().join(".git");

  path_git.exists()
}