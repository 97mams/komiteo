use std::process::Command;

pub fn komiteo_cmd(cmd: &str, arg: &str) -> String {
    let output = Command::new(cmd)
                    .arg(arg)
                    .output()
                    .expect("Faild to execut this command !");
    return String::from_utf8_lossy(&output.stdout).to_string();
}