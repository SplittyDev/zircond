use std::io::Write;
use std::net::TcpStream;

use crate::server::{Server, User, Channel};
use crate::dispatch::command_dispatch::CommandDispatch;
use crate::message::Respond;

pub struct JoinChannel {
    pub channel_name: String,
    pub channel_key: Option<String>,
}

impl CommandDispatch for JoinChannel {
    fn dispatch(&self, server: &mut Server, client: &mut TcpStream, client_id: usize) {

        // Get the current user
        let my_user = server.users.find(client_id).unwrap();

        // Test whether the channel already exists
        if server.channels.find(&self.channel_name).is_none() {
        
            // Create a new channel
            let channel = Channel::new(self.channel_name.clone());

            // Add the new channel to the channel list
            server.channels.add(channel);
        }

        // Find the channel
        let channel = server.channels.find(&self.channel_name).unwrap();

        // Add the user to the channel
        channel.join_user(client_id);

        // Send join acknowledgement to the user
        let nick = my_user.nickname();
        send!(client; Respond::to(&nick, &nick).join(self.channel_name.clone()));

        // Test whether the channel has a topic
        if let Some(topic) = &channel.topic {
        
            // Tell the client about the topic
            send!(client; Respond::to(&nick, &self.channel_name).topic(topic.clone()));
        }

        // Iterate over all users in the channel
        for user_info in channel.users() {
            
            // Find the user
            if let Some(channel_user) = server.users.find(user_info.client_id()) {
            
                // Get channel mode
                // "=": public
                // "@": secret (+s)
                // "*": private (+p)
                let channel_mode = "=";
                
                // Tell the client about the user
                send!(client; Respond::to(server.config.get_host(), &nick).names_reply(&self.channel_name, channel_mode, "", &channel_user.nickname()))
            }
        }

        // Mark the end of the user list    
        send!(client; Respond::to(server.config.get_host(), &nick).names_end(&self.channel_name));

        // Iterate over all users in the channel
        for other_client in channel.users() {
        
            // Skip this user if it is the current user
            if other_client.client_id() == client_id {
                continue;
            }

            // Find user by user id
            if let Some(other_user) = server.users.find_mut(other_client.client_id()) {
            
                // Tell the user's client about the join
                send!(other_user.stream(); Respond::to(&nick, &nick).join(self.channel_name.clone()));
            }
        }
    }
}