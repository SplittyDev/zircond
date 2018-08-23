#[derive(Debug)]
pub enum IrcMessageCommand {
    None,

    //
    // Authentication
    //

    /// NICK - Set nickname
    /// 
    /// * `0` - Nickname
    Nick(String),
    
    /// USER - Set username and realname
    /// 
    /// * `0` - Username
    /// * `1` - Realname
    User(String, Option<String>),

    //
    // Channels
    //

    /// JOIN - Join a channel
    /// 
    /// * `0` - Channel name
    Join(String),

    /// WHO - List users in a specific channel
    /// 
    /// * `0` - Channel name
    Who(String),

    //
    // Messaging
    //

    /// PRIVMSG - Private message
    /// 
    /// * `0` - Target
    /// * `1` - Message
    Privmsg(String, String),

    //
    // Misc
    //

    /// PING - Server availability check
    /// 
    /// * `0` - Ping id
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