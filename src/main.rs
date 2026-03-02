use std::process::Command;
use std::str;
use colored::*;

fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo 'hello windows'"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git diff")
            .output()
            .expect("failed to execute process")
    };

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        println!("Output: {}", stdout);
    } else {
        let stderr = str::from_utf8(&output.stderr).unwrap();
        eprintln!("Error: {}", stderr.on_red());
    }
}