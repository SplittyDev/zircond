use crate::protocol::*;

pub enum CommandType {
    None,
    Code(&'static str),
    Name(&'static str),
}

pub struct Respond;

impl<'a> Respond {
    pub fn to(host: &'a str, target: &'a str) -> ResponseBuilder<'a> {
        ResponseBuilder::new(host, target)
    }
}

pub struct ResponseBuilder<'a> {
    source: &'a str,
    target: &'a str,
    command: CommandType,
    parameters: Vec<String>,
    auto_insert_trailing_separator: bool,
}

impl<'a> ResponseBuilder<'a> {
    pub fn new(source: &'a str, target: &'a str) -> Self {
        ResponseBuilder {
            source,
            target,
            command: CommandType::None,
            parameters: Vec::new(),
            auto_insert_trailing_separator: true,
        }
    }

    pub fn welcome(mut self, message: String) -> Self {
        self.command = CommandType::Code(RPL_WELCOME);
        self.parameters.push(message);
        self
    }

    pub fn your_host(mut self, message: String) -> Self {
        self.command = CommandType::Code(RPL_YOURHOST);
        self.parameters.push(message);
        self
    }

    pub fn names_reply(mut self, channel_mode: &str, channel: &str, nickname: &str) -> Self {
        self.command = CommandType::Code(RPL_NAMREPLY);
        self.parameters.push(channel.to_owned());
        self.parameters.push(channel_mode.to_owned());
        self.parameters.push(format!("@{}", nickname));
        self
    }

    pub fn names_end(mut self, channel: &str) -> Self {
        self.command = CommandType::Code(RPL_ENDOFNAMES);
        self.parameters.push(channel.to_owned());
        self.parameters.push("End of /NAMES list.".to_owned());
        self
    }

    pub fn motd_start(mut self) -> Self {
        self.auto_insert_trailing_separator = false;
        self.command = CommandType::Code(RPL_MOTDSTART);
        self.parameters.push(format!(":{} Message of the day", self.source));
        self
    }

    pub fn motd(mut self, message: &str) -> Self {
        self.auto_insert_trailing_separator = false;
        self.command = CommandType::Code(RPL_MOTD);
        self.parameters.push(format!(":- {}", message));
        self
    }

    pub fn motd_end(mut self) -> Self {
        self.command = CommandType::Code(RPL_ENDOFMOTD);
        self.parameters.push("End of MOTD.".to_owned());
        self
    }

    pub fn privmsg(mut self, target: String, message: String) -> Self {
        self.command = CommandType::Name("PRIVMSG");
        self.parameters.push(target);
        self.parameters.push(message);
        self
    }

    pub fn pong(mut self, challenge: String) -> Self {
        self.command = CommandType::Name("PONG");
        self.parameters.push(challenge);
        self
    }

    pub fn join(mut self, channel: String) -> Self {
        self.command = CommandType::Name("JOIN");
        self.parameters.push(channel);
        self
    }
    
    pub fn topic(mut self, topic: String) -> Self {
        self.command = CommandType::Name("TOPIC");
        self.parameters.push(self.target.to_owned());
        self.parameters.push(topic);
        self
    }
}

impl<'a> ToString for ResponseBuilder<'a> {
    fn to_string(&self) -> String {
        let mut buf = {
            format!(
                ":{} {}",
                self.source,
                match self.command {
                    CommandType::Code(code) => code,
                    CommandType::Name(name) => name,
                    CommandType::None => panic!("Unable to build response from empty command!"),
                },
            )
        };
        if let CommandType::Code(_) = self.command {
            buf.push(' ');
            buf.push_str(self.target);
        };
        for (i, parameter) in self.parameters.iter().enumerate() {
            buf.push(' ');
            if self.auto_insert_trailing_separator && i == self.parameters.len() - 1 {
                buf.push(':');
            }
            buf.push_str(parameter);
        }
        println!("[Server] {}", buf);
        buf
    }
}