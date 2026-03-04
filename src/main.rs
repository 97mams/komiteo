use std::process::Command;


fn hello () {
    let tesxt: &str = "
    
    ----------------
    | Mamisoa ito  |
    ----------------

    ";
    println!("{}", tesxt)
}

fn seting(tokken: &str) {
    let output = Command::new("touch .")
        .args(["config", "--global", "user.name", "Mamisoa ito"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Git user.name set successfully.");
    } else {
        eprintln!("Failed to set Git user.name: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn main() {
    hello();
}