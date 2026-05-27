use std::process::Command;

pub fn komiteo_cmd(cmd: String, arg: Vec<String>) -> String {
    let output = Command::new(cmd)
                    .args(arg)
                    .output()
                    .expect("Failed to execute this command !");
    return String::from_utf8_lossy(&output.stdout).to_string();
}
