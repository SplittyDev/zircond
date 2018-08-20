use std::sync::Arc;
use std::sync::RwLock;
use super::{User, Channel, ClientState};

pub type Shared<T> = Arc<RwLock<T>>;
pub type SharedVec<T> = Shared<Vec<T>>;

pub struct ServerState {
    channels: SharedVec<Channel>,
    clients: SharedVec<Arc<ClientState>>,
    users: SharedVec<Shared<User>>,
}

macro_rules! impl_read {
    ($name:ident: $fn:ident -> $rettype:ty) => {
        pub fn $name<F, R>(&self, f: F) -> R where F: (Fn(&$rettype) -> R) {
            f(&*self.$fn.clone().read().unwrap())
        }
    };
}

macro_rules! impl_write {
    ($name:ident: $fn:ident -> $rettype:ty) => {
        pub fn $name<F, R>(&self, f: F) -> R where F: (Fn(&mut $rettype) -> R) {
            let tmp = self.$fn.clone();
            let mut tmp = tmp.write().unwrap();
            f(&mut *tmp)
        }
    };
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

    impl_read!(read_channels: channels -> Vec<Channel>);
    impl_write!(write_channels: channels -> Vec<Channel>);

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
}