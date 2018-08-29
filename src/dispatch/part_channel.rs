use std::net::TcpStream;

use crate::dispatch::command_dispatch::CommandDispatch;

use crate::server::Server;
use crate::message::Respond;

pub struct PartChannel {
    pub channel_name: String,
}

impl CommandDispatch for PartChannel {
    fn dispatch(&self, server: &mut Server, client: &mut TcpStream, client_id: usize) {

        // Get the user's nickname
        let nick = server.users.find(client_id).unwrap().nickname();

        // Find the channel
        if let Some(channel) = server.channels.find(&self.channel_name) {

            // TODO: Handle user not in channel (ERR_NOTONCHANNEL)

            // Remove the user from the channel
            channel.part_user(client_id);

            // Notify the user about the PART
            send!(client; Respond::to(&nick, &nick).part(self.channel_name.clone(), nick.clone()));

            // Relay the PART message to all other users in the channel
            for other_client in channel.users() {
            
                // Skip the current user
                if other_client.client_id() == client_id {
                    continue;
                }

                // Find user by user id
                if let Some(other_user) = server.users.find_mut(other_client.client_id()) {
                
                    // Tell the user's client about the join
                    send!(other_user.stream(); Respond::to(&nick, &nick).part(self.channel_name.clone(), nick.clone()));
                }
            }
        }

        // TODO: Handle channel not found (ERR_NOSUCHCHANNEL)
    }
}