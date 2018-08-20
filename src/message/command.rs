#[derive(Debug)]
pub enum IrcMessageCommand {
    None,

    // Authentication
    Nick(String),
    User(String, Option<String>),

    // Channels
    Join(String),
    Who(String),

    // Misc
    Ping(String),
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
            }
            IrcMessageCommand::Join(channel) => {
                command = "JOIN";
                params = Some(channel);
            }
            _ => panic!("Unimplemented command: {:?}", self),
        };
        let mut buf = String::with_capacity(command.len());
        buf.push_str(command);
        if params.is_some() {
            buf.push_str(&" :");
            buf.push_str(params.unwrap());
        }
        buf
    }
}