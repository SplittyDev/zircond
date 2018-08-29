use std::net::TcpStream;

use crate::dispatch::command_dispatch::CommandDispatch;

use crate::server::Server;
use crate::message::Respond;

pub struct PrivateMessage {
    pub target: String,
    pub message: String,
}

impl CommandDispatch for PrivateMessage {
    fn dispatch(&self, server: &mut Server, _client: &mut TcpStream, client_id: usize) {

        // Get the nickname of the current user
        let user_nick = server.users.find(client_id).unwrap().nickname();

        // Determine whether the target is a user or a channel
        if self.target.starts_with('#') {

            // Find the channel
            if let Some(channel) = server.channels.find(&self.target) {

                // Find all users in the channel
                for other_user_info in channel.users() {

                    // Skip the current user
                    if other_user_info.client_id() == client_id {
                        continue;
                    }

                    // Find the user
                    if let Some(other_user) = server.users.find_mut(other_user_info.client_id()) {

                        // Relay the private message to the other user
                        send!(other_user.stream(); Respond::to(&user_nick, &self.target).privmsg(self.message.clone()));
                    }
                }
            }
        } else if let Some(other_user) = server.users.find_by_name_mut(&self.target) {

            // Send the private message to the other user
            send!(other_user.stream(); Respond::to(&user_nick, &self.target).privmsg(self.message.clone()));
        }
    }
}