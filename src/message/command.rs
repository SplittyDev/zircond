#[derive(Debug)]
pub enum IrcMessageCommand {
    None,
    Nick(String),
    User(String, Option<String>)
}

impl ToString for IrcMessageCommand {
    fn to_string(&self) -> String {
        let command: &str;
        let mut params: Option<&str> = None;
        match self {
            IrcMessageCommand::None => panic!("Unable to build NONE command."),
            IrcMessageCommand::Nick(user) => {
                command = "NICK";
                params = Some(user);
            },
            _ => panic!("Unimplemented command: {:?}", self),
        };
        let mut buf = String::with_capacity(command.len());
        buf.push_str(command.as_ref());
        if params.is_some() {
            buf.push_str(&" :");
            buf.push_str(params.unwrap());
        }
        buf
    }
}