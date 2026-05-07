struct command {
    cmd: String,
    arg: Vec<&str>,
}

impl command {
    pub fn parse_cmd(cmd: &str) -> command {
    let mut parts = cmd.split_whitespace();
    let mut args = Vec::new();
    while let Some(part) = parts.next() {
        args.push(part);
    }
    command {cmd: args[0].to_string(), arg: args[1..].to_vec()}
}}