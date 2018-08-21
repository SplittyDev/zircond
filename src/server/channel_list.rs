use super::Channel;

pub struct ChannelList {
    channels: Vec<Channel>,
}

impl ChannelList {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
        }
    }

    pub fn add(&mut self, channel: Channel) {
        self.channels.push(channel);
    }

    pub fn find(&mut self, channel_name: &str) -> Option<&mut Channel> {
        self.channels.iter_mut().find(|channel| channel.name == channel_name)
    }
}