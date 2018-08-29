
// Networking
use std::io::{Write, BufRead};
use std::net::{TcpListener};

// Threading / Synchronization
use std::thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::channel;

use crate::config::ServerConfig;
use crate::message::{IrcMessageRequest, IrcMessageCommand, Respond};
use super::{User, UserList, ChannelList, IrcAction};

pub struct Server {
    pub config: ServerConfig,
    pub users: UserList,
    pub channels: ChannelList,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            users: UserList::new(),
            channels: ChannelList::new(),
        }
    }

    pub fn listen(&mut self) {

        // Create thread collection
        let mut threads = Vec::new();

        // Create asynchronous channel
        let (sender, recv) = channel();

        // Bind the tcp listener socket
        let listener = TcpListener::bind(self.config.get_addr()).unwrap();

        // Spawn listening thread
        thread::spawn(move || {

            let next_client_id = Arc::new(RwLock::new(0usize));

            // Accept new clients
            for client in listener.incoming() {

                // Create a new client
                let shared_client = Arc::new(client.unwrap());

                // Handle the client in a separate thread
                let sender = sender.clone();
                let next_client_id = next_client_id.clone();
                let handle = thread::spawn(move || {

                    // Get client id
                    let client_id = {
                        let mut w = next_client_id.write().unwrap();
                        *w += 1;
                        *w - 1
                    };

                    // Register the new client with the server
                    sender.send((shared_client.try_clone().unwrap(), client_id, IrcAction::UserConnect())).unwrap();

                    // Get the remote address of the client
                    let addr = shared_client.peer_addr().unwrap();

                    // Get a buffered reader for the incoming data
                    let mut reader = std::io::BufReader::new(shared_client.try_clone().unwrap());

                    // Handle new messages in a loop
                    loop {

                        // Get a mutable handle for the client tcp stream
                        let client = shared_client.try_clone().unwrap();

                        // Read the next line
                        let mut line = String::new();
                        reader.read_line(&mut line).unwrap();

                        // Test for EOF
                        if line.is_empty() {
                            sender.send((client, client_id, IrcAction::Disconnect())).unwrap();
                            break;
                        }

                        // Debug
                        print!("[{:?}] {}", addr, line);

                        // Parse the irc message
                        let cmd = IrcMessageRequest::parse(&line);

                        // Handle the command
                        match cmd.command {

                            IrcMessageCommand::Nick(nickname) => {
                                sender.send((client, client_id, IrcAction::UserSetNick(nickname))).unwrap()
                            }

                            IrcMessageCommand::User(username, realname) => {
                                sender.send((client, client_id, IrcAction::UserSetNames(username, realname))).unwrap()
                            }

                            IrcMessageCommand::Join(channels, keys) => {

                                // Minor optimization if there is only one channel.
                                // In this case, we don't need to obtain another shared handle to the tcp stream.
                                if channels.len() == 1 {
                                    let key = match &keys { Some(keys) => Some(keys[0].to_owned()), _ => None };
                                    sender.send((client, client_id, IrcAction::UserJoinChannel(channels[0].clone(), key))).unwrap();
                                    continue;
                                }

                                // Multiple channels
                                for (i, channel) in channels.iter().enumerate() {
                                    let client = shared_client.try_clone().unwrap();
                                    let key = match &keys { Some(keys) => keys.get(i).map(|k| k.to_owned()), _ => None };
                                    sender.send((client, client_id, IrcAction::UserJoinChannel(channel.to_owned(), key))).unwrap()
                                }
                            }

                            IrcMessageCommand::Part(channels) => {

                                // Minor optimization if there is only one channel.
                                // In this case, we don't need to obtain another shared handle to the tcp stream.
                                if channels.len() == 1 {
                                    sender.send((client, client_id, IrcAction::UserPartChannel(channels[0].clone()))).unwrap();
                                    continue;
                                }

                                // Multiple channels
                                for channel in channels {
                                    let client = shared_client.try_clone().unwrap();
                                    sender.send((client, client_id, IrcAction::UserPartChannel(channel.to_owned()))).unwrap()
                                }
                            }

                            IrcMessageCommand::Privmsg(target, message) => {
                                sender.send((client, client_id, IrcAction::Privmsg(target, message))).unwrap()
                            }

                            IrcMessageCommand::Who(channel) => {
                                sender.send((client, client_id, IrcAction::ChannelListUsers(channel))).unwrap();
                            }

                            IrcMessageCommand::Ping(id) => {
                                sender.send((client, client_id, IrcAction::Pong(id))).unwrap();
                            }

                            com => println!("Unhandled command: {:?}", com),
                        }
                    }

                    // Kill the client
                    shared_client.shutdown(std::net::Shutdown::Both).unwrap();
                });

                // Keep track of the thread handle
                threads.push(handle);
            }
        });

        // Macro for simple server-to-client communication
        macro_rules! send {
            ($writer:expr; $variant:expr) => (
                $writer.write_all(format!("{}\r\n", $variant.to_string()).as_ref()).unwrap()
            );
        }

        // Receive actions
        for (mut client, client_id, action) in recv {

            macro_rules! dispatch {
                ($dispatcher:expr) => {
                    crate::dispatch::dispatch(&$dispatcher, self, &mut client, client_id);
                };
            }

            // Provide easy access to the sender of the action
            macro_rules! my_user {
                (r) => {
                    self.users.find(client_id).unwrap()
                };
                (rw) => {
                    self.users.find_mut(client_id).unwrap()
                }
            }

            // Handle the action
            #[allow(unreachable_patterns)]
            match action {

                IrcAction::UserConnect() => {
                    let user = User::new(client_id, client);
                    self.users.add(user);
                }
                
                IrcAction::UserSetNick(nickname) => {
                    dispatch!(crate::dispatch::SetNick {
                        nickname,
                    })
                }

                IrcAction::UserSetNames(username, realname) => {

                    // Set username and realname
                    my_user!(rw).set_names(username, realname);
                }

                IrcAction::UserJoinChannel(channel_name, channel_key) => {
                    dispatch!(crate::dispatch::JoinChannel {
                        channel_name,
                        channel_key,
                    })
                }

                IrcAction::UserPartChannel(channel_name) => {
                    dispatch!(crate::dispatch::PartChannel {
                        channel_name,
                    })
                }

                IrcAction::ChannelListUsers(_channel_name) => {

                    // // Find the user
                    // if let Some(user) = self.users.find(client_id) {

                    //     // Find the channel
                    //     if let Some(channel) = self.channels.find(&channel_name) {

                    //         // Iterate over all users in the channel
                    //         for user_info in channel.users() {
                                
                    //             // Find the user
                    //             if let Some(channel_user) = self.users.find(user_info.client_id()) {

                    //                 // Get channel mode
                    //                 // "=": public
                    //                 // "@": secret (+s)
                    //                 // "*": private (+p)
                    //                 let channel_mode = "=";

                    //                 // Tell the client about the user
                    //                 send!(client; Respond::to(self.config.get_host(), &user.nickname()).names_reply(&channel_name, channel_mode, &channel_user.nickname()))
                    //             }
                    //         }

                    //         // Mark the end of the user list
                    //         send!(client; Respond::to(self.config.get_host(), &user.nickname()).names_end(&channel_name));
                    //     }
                    // }
                }

                IrcAction::Privmsg(target, message) => {
                    dispatch!(crate::dispatch::PrivateMessage {
                        target,
                        message,
                    })
                }

                IrcAction::Pong(id) => {

                    // Respond to ping
                    send!(client; Respond::to(self.config.get_host(), &my_user!(r).nickname()).pong(id));
                }

                IrcAction::Disconnect() => {

                    // Disconnect the user
                    println!("Connection lost: {}", my_user!(r).nickname());
                    self.users.disconnect(client_id);
                }

                _ => println!("Unimplemented action: {:?}", action)
            }
        }
    }
}