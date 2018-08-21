use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use super::User;

pub struct ClientState {
    stream: Arc<TcpStream>,
    user: User,
}

impl ClientState {
    pub fn new(stream: Arc<TcpStream>, user: User) -> Self {
        Self {
            stream,
            user,
        }
    }

    pub fn stream(&self) -> &TcpStream {
        &self.stream
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}