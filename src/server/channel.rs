use std::sync::{Arc, RwLock, Mutex};
use std::ops::DerefMut;
use super::User;

pub struct Channel {
    pub name: String,
    users: Arc<RwLock<Vec<Arc<RwLock<User>>>>>,
}

impl Channel {
    pub fn new(name: String) -> Self {
        Self {
            name,
            users: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn join_user(&self, user: Arc<RwLock<User>>) {
        let mut w = self.users.write().unwrap();
        w.deref_mut().push(user);
    }

    pub fn part_user(&self, id: usize) {
        let mut w = self.users.write().unwrap();
        let user_list = w.deref_mut();
        user_list.iter().position(|user| {
            user.read().unwrap().id == id
        }).map(|i| user_list.remove(i));
    }
}