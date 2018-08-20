use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use super::User;

pub struct ClientState {
    stream: TcpStream,
    user: Arc<RwLock<User>>,
}

impl ClientState {
    pub fn new(stream: TcpStream, user: Arc<RwLock<User>>) -> Self {
        Self {
            stream,
            user,
        }
    }

    pub fn stream(&self) -> &TcpStream {
        &self.stream
    }

    pub fn user(&self) -> Arc<RwLock<User>> {
        self.user.clone()
    }
}