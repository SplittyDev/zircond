use std::net::TcpStream;
use std::sync::Arc;
use std::sync::RwLock;
use std::ops::DerefMut;
use super::{User, Channel};

pub type Shared<T> = Arc<RwLock<T>>;
pub type SharedVec<T> = Shared<Vec<T>>;

pub struct ServerState {
    channels: SharedVec<Channel>,
    clients: SharedVec<TcpStream>,
    users: SharedVec<Shared<User>>,
}

impl ServerState {

    //! Creates a new `ServerState`.
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(Vec::new())),
            clients: Arc::new(RwLock::new(Vec::new())),
            users: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn channels(&self) -> SharedVec<Channel> {
        self.channels.clone()
    }

    pub fn clients(&self) -> SharedVec<TcpStream> {
        self.clients.clone()
    }

    pub fn users(&self) -> SharedVec<Shared<User>> {
        self.users.clone()
    }
}

pub struct ClientState {
}