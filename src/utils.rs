struct command {
    cmd: String,
    arg: Vec<&str>,
}

impl command {
    pub fn parse_cmd(value: Vec<&str>) -> command {
    let mut args = Vec::new();
    while let Some(value) = value.next() {
        args.push(value);
    }
    command {cmd: args[0].to_string(), arg: args[1..].to_vec()}
}}