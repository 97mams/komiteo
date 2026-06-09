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

pub fn clean_file_name(file_name: String) -> String {
     file_name.trim().split(" ").last().unwrap_or("").to_string()
}

pub fn extract_name(file_name:String) -> String {
    let name_with_extension = file_name.trim().split("/").last().unwrap_or("").to_string();
    let name = name_with_extension.split(".").next().unwrap_or("").to_string();
    name
}