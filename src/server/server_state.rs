use std::sync::Arc;
use std::sync::RwLock;
use super::{User, Channel, ClientState};

pub type Shared<T> = Arc<RwLock<T>>;
pub type SharedVec<T> = Shared<Vec<T>>;

pub struct ServerState {
    channels: SharedVec<Shared<Channel>>,
    clients: SharedVec<Arc<ClientState>>,
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

    pub fn channels(&self) -> SharedVec<Shared<Channel>> {
        self.channels.clone()
    }

    impl_read!(read_channels: channels -> Vec<Shared<Channel>>);
    impl_write!(write_channels: channels -> Vec<Shared<Channel>>);

    pub fn clients(&self) -> SharedVec<Arc<ClientState>> {
        self.clients.clone()
    }

    impl_read!(read_clients: clients -> Vec<Arc<ClientState>>);
    impl_write!(write_clients: clients -> Vec<Arc<ClientState>>);

    pub fn users(&self) -> SharedVec<Shared<User>> {
        self.users.clone()
    }

    impl_read!(read_users: users -> Vec<Shared<User>>);
    impl_write!(write_users: users -> Vec<Shared<User>>);

    pub fn add_client(&self, client: Arc<ClientState>) {
        let mut w = self.clients.write().unwrap();
        w.push(client);
    }

    pub fn get_channel(&self, channel_name: &str) -> Option<Shared<Channel>> {
        self.read_channels(|channels| {
            for channel in channels {
                let channel_x = channel.read().unwrap();
                if channel_x.name == channel_name {
                    return Some(channel.clone());
                }
            }
            None
        })
    }
}