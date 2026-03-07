use dialoguer::Input;

pub fn hello_welcome(){
    let name: String = Input::new()
        .with_prompt("What is your name?")
        .interact_text().expect("Failed to read input");

    println!("Hello, {}! Welcome to the Rust programming world!", name);
}