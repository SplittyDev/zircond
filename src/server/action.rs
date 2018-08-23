#[derive(Debug)]
pub enum IrcAction {

    //
    // Users
    //

    /// User / Connect
    UserConnect(),

    /// User / Set Nick
    /// 
    /// * `0` - Nickname
    UserSetNick(String),

    /// User / Set Names
    /// 
    /// * `0` - Username
    /// * `1` - Realname
    UserSetNames(String, Option<String>),

    /// User / Join Channel
    /// 
    /// * `0` - Channel name
    UserJoinChannel(String),

    //
    // Channels
    //
    
    /// Channel / List Users
    /// 
    /// * `0` - Channel name
    ChannelListUsers(String),

    //
    // Messaging
    //

    /// Private message
    /// 
    /// * `0` - Target
    /// * `1` - Message
    Privmsg(String, String),

    //
    // Misc
    //

    /// Reply to ping
    /// 
    /// * `0` - Ping id
    Pong(String),
}