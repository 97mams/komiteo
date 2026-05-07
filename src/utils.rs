pub struct Command {
    pub cmd: String,
    pub arg: Vec<String>,
}

impl Command {
    pub fn parse_cmd(value: Vec<&str>) -> Command {
        let mut iter = value.into_iter();

        let cmd = iter.next().unwrap_or("").to_string();

        let arg = iter.map(|s| s.to_string()).collect();

        Command { cmd, arg }
    }
}