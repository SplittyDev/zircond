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
    /// * `0` - Channel name(s)
    /// * `1` - Channel key(s)
    Join(Vec<String>, Option<Vec<String>>),

    /// PART - Leave a channel
    /// 
    /// * `0` - Channel name(s)
    /// * `1` - Reason
    Part(Vec<String>, Option<String>),

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