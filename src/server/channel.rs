#[derive(Serialize, Deserialize)]
pub struct ChannelUserInfo {
    client_id: usize,
}

impl ChannelUserInfo {
    pub fn new(client_id: usize) -> Self {
        Self {
            client_id
        }
    }

    pub fn client_id(&self) -> usize {
        self.client_id
    }
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub topic: Option<String>,
    users: Vec<ChannelUserInfo>,
}

impl Channel {
    pub fn new(name: String) -> Self {
        Self {
            name,
            topic: None,
            users: Vec::new(),
        }
    }

    pub fn users(&self) -> &Vec<ChannelUserInfo> {
        &self.users
    }

    pub fn join_user(&mut self, client_id: usize) {
        let user_info = ChannelUserInfo::new(client_id);
        self.users.push(user_info);
    }

    pub fn part_user(&mut self, client_id: usize) {
        self.users.iter().position(|user| {
            user.client_id() == client_id
        }).map(|i| self.users.remove(i));
    }
}