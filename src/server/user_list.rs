use super::User;

pub struct UserList {
    users: Vec<User>,
}

impl UserList {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
        }
    }

    pub fn add(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn find(&self, client_id: usize) -> Option<&User> {
        self.users.iter().find(|user| user.id == client_id)
    }

    pub fn find_mut(&mut self, client_id: usize) -> Option<&mut User> {
        self.users.iter_mut().find(|user| user.id == client_id)
    }

    pub fn disconnect(&mut self, client_id: usize) -> bool {
        self.users.iter().position(|user| user.id == client_id).map(|user| self.users.remove(user)).is_some()
    }
}