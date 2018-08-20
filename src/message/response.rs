pub struct Respond;

impl<'a> Respond {
    pub fn to(host: &'a str, target: &'a str) -> ResponseBuilder<'a> {
        ResponseBuilder::new(host, target)
    }
}

pub struct ResponseBuilder<'a> {
    host: &'a str,
    target: &'a str,
    code: Option<&'static str>,
    command: Option<&'static str>,
    parameters: Vec<String>,
    auto_insert_trailing_separator: bool,
    replace_host_with_target: bool,
}

impl<'a> ResponseBuilder<'a> {
    pub fn new(host: &'a str, target: &'a str) -> Self {
        ResponseBuilder {
            host,
            target,
            code: None,
            command: None,
            parameters: Vec::new(),
            auto_insert_trailing_separator: true,
            replace_host_with_target: false,
        }
    }

    pub fn welcome(mut self, message: String) -> Self {
        self.code = Some("001");
        self.parameters.push(message);
        self
    }

    pub fn your_host(mut self, message: String) -> Self {
        self.code = Some("002");
        self.parameters.push(message);
        self
    }

    pub fn names_reply(mut self, channel: &str, nickname: &str) -> Self {
        self.code = Some("353");
        self.parameters.push(format!("= {}", channel));
        self.parameters.push(format!("@{}", nickname));
        self
    }

    pub fn names_end(mut self, channel: &str) -> Self {
        self.code = Some("366");
        self.parameters.push(channel.to_owned());
        self.parameters.push("End of /NAMES list.".to_owned());
        self
    }

    pub fn motd_start(mut self) -> Self {
        self.auto_insert_trailing_separator = false;
        self.code = Some("375");
        self.parameters.push(format!(":{} Message of the day", self.host));
        self
    }

    pub fn motd(mut self, message: &str) -> Self {
        self.auto_insert_trailing_separator = false;
        self.code = Some("372");
        self.parameters.push(format!(":- {}", message));
        self
    }

    pub fn motd_end(mut self) -> Self {
        self.code = Some("376");
        self.parameters.push("End of MOTD.".to_owned());
        self
    }

    pub fn pong(mut self, challenge: String) -> Self {
        self.command = Some("PONG");
        self.parameters.push(challenge);
        self
    }

    pub fn join(mut self, channel: String) -> Self {
        self.replace_host_with_target = true;
        self.command = Some("JOIN");
        self.parameters.push(channel);
        self
    }
}

impl<'a> ToString for ResponseBuilder<'a> {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        if let Some(command) = self.command {
            if self.replace_host_with_target {
                buf.push_str(&format!(":{} {}", self.target, command));
            } else {
                buf.push_str(&format!(":{} {}", self.host, command));
            }
        } else if let Some(code) = self.code {
            buf.push_str(&format!(":{} {} {}", self.host, code, self.target));
        } else {
            panic!("Unable to send response without code or command set!");
        }
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