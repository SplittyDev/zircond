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
    parameters: Vec<String>,
}

impl<'a> ResponseBuilder<'a> {
    pub fn new(host: &'a str, target: &'a str) -> Self {
        ResponseBuilder {
            host,
            target,
            code: None,
            parameters: Vec::new(),
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
}

impl<'a> ToString for ResponseBuilder<'a> {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&format!(":{} {} {}", self.host, self.code.unwrap(), self.target));
        for (i, parameter) in self.parameters.iter().enumerate() {
            let parameter = parameter.trim();
            buf.push(' ');
            if i == self.parameters.len() - 1 {
                buf.push(':');
            }
            buf.push_str(parameter);
        }
        println!("[Server] {}", buf);
        buf
    }
}