use crate::message::{IrcMessageTags, IrcMessagePrefix, IrcMessageCommand};
use crate::parser::IrcMessageParser;

pub struct IrcMessageRequest {
    tags: Option<IrcMessageTags>,
    prefix: Option<IrcMessagePrefix>,
    pub command: IrcMessageCommand,
}

impl IrcMessageRequest {
    pub fn new(command: IrcMessageCommand, prefix: Option<IrcMessagePrefix>, tags: Option<IrcMessageTags>) -> Self {
        Self {
            tags,
            prefix,
            command,
        }
    }

    pub fn parse(line: String) -> Self {
        IrcMessageParser::parse(line)
    }
}

impl Default for IrcMessageRequest {
    fn default() -> Self {
        Self::new(IrcMessageCommand::None, None, None)
    }
}