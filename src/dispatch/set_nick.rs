use std::net::TcpStream;

use crate::dispatch::command_dispatch::CommandDispatch;

use crate::server::Server;
use crate::message::Respond;

pub struct SetNick {
    pub nickname: String,
}

impl CommandDispatch for SetNick {
    fn dispatch(&self, mut server: &mut Server, mut client: &mut TcpStream, client_id: usize) {

        // Look for nickname collisions
        if server.users.find_by_name(&self.nickname).is_some() {

            // Report name collision
            let current_nick = server.users.find(client_id).unwrap().nickname();
            send!(client; Respond::to(server.config.get_host(), &current_nick).err_nickname_in_use(self.nickname.clone()));

        } else {

            // Set the nickname
            server.users.find_mut(client_id).unwrap().set_nickname(self.nickname.clone());

            // Send the welcome sequence
            let nick = &self.nickname;
            send!(client; Respond::to(server.config.get_host(), nick).welcome(format!("Welcome, {}!", nick)));
            send!(client; Respond::to(server.config.get_host(), nick).your_host(format!("Your host is {}, running Zircond.", server.config.get_host())));
            send!(client; Respond::to(server.config.get_host(), nick).motd_start());
            send!(client; Respond::to(server.config.get_host(), nick).motd(&format!("Zircon IRCd v{}", &crate_version!())));
            send!(client; Respond::to(server.config.get_host(), nick).motd("Zircond is open source! Contribute here: https://github.com/splittydev/zircond"));
            send!(client; Respond::to(server.config.get_host(), &nick).motd_end());

            // Join autojoin channels
            if let Some(channels) = server.config.get_autojoin_channels() {
                for channel in channels {

                    // Make sure to not join any channels the user is already a part of
                    if let Some(existing_channel) = server.channels.find(&channel) {
                        if existing_channel.contains(client_id) {
                            continue;
                        }
                    }

                    // Join the channel
                    super::dispatch(&crate::dispatch::JoinChannel {
                        channel_name: channel,
                        channel_key: None,
                    }, &mut server, &mut client, client_id);
                }
            }
        }
    }
}