#[derive(Debug)]
pub enum IrcAction {

    // Users
    UserConnect(),
    UserSetNick(String),
    UserSetNames(String, Option<String>),
    UserJoinChannel(String),

    // Channels
    ChannelListUsers(String),

    // Misc
    Pong(String),
}