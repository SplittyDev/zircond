use std::net::TcpStream;

#[derive(Debug)]
pub struct User {
    pub id: usize,
    nickname: Option<String>,
    username: Option<String>,
    realname: Option<String>,
    stream: TcpStream,
}

impl User {
    pub fn new(id: usize, stream: TcpStream) -> User {
        Self {
            id,
            nickname: None,
            username: None,
            realname: None,
            stream,
        }
    }

    pub fn stream(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    pub fn nickname(&self) -> String {
        match self.nickname {
            Some(ref nickname) => nickname.clone(),
            None => "<unidentified>".to_owned(),
        }
    }

    pub fn username(&self) -> String {
        match self.username {
            Some(ref username) => username.clone(),
            None => "<unidentified>".to_owned(),
        }
    }

    pub fn realname(&self) -> String {
        match self.realname {
            Some(ref realname) => realname.clone(),
            None => "<unidentified>".to_owned(),
        }
    }

    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = Some(nickname);
    }

    pub fn set_names(&mut self, username: String, realname: Option<String>) {
        self.realname = Some(realname.unwrap_or_else(|| username.clone()));
        self.username = Some(username);
    }
}