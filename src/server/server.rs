use std::io::{Write, BufRead};
use std::net::{TcpListener};
use std::thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::channel;
use crate::message::{IrcMessageRequest, IrcMessageCommand, Respond};
use super::{User, Channel, UserList, ChannelList, IrcAction};

pub struct Server {
    host: String,
    port: u16,
    users: UserList,
    channels: ChannelList,
}

impl Server {
    pub fn new(host: Option<String>, port: Option<u16>) -> Self {
        Self {
            host: host.unwrap_or_else(|| "127.0.0.1".to_owned()),
            port: port.unwrap_or(6667),
            users: UserList::new(),
            channels: ChannelList::new(),
        }
    }

    pub fn listen(&mut self) {

        // Get crate version
        let crate_version = format!(
            "{}.{}.{}{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
            option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
        );

        // Create thread collection
        let mut threads = Vec::new();

        // Create asynchronous channel
        let (sender, recv) = channel();

        // Bind the tcp listener socket
        let listener = TcpListener::bind((self.host.as_ref(), self.port)).unwrap();

        // Spawn listening thread
        thread::spawn(move || {

            let next_client_id = Arc::new(RwLock::new(0usize));

            // Accept new clients
            for client in listener.incoming() {

                // Create a new client
                let client = Arc::new(client.unwrap());

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
                    sender.send((client.try_clone().unwrap(), client_id, IrcAction::UserConnect())).unwrap();

                    // Get the remote address of the client
                    let addr = client.peer_addr().unwrap();

                    // Get a buffered reader for the incoming data
                    let mut reader = std::io::BufReader::new(client.try_clone().unwrap());

                    // Handle new messages in a loop
                    loop {

                        // Get a mutable handle for the client tcp stream
                        let client = client.try_clone().unwrap();

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

                            IrcMessageCommand::Join(channel) => {
                                sender.send((client, client_id, IrcAction::UserJoinChannel(channel))).unwrap()
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
                    client.shutdown(std::net::Shutdown::Both).unwrap();
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

                    // Set the nickname
                    my_user!(rw).set_nickname(nickname);
                }

                IrcAction::UserSetNames(username, realname) => {

                    // Set username and realname
                    my_user!(rw).set_names(username, realname);

                    // Get the nickname
                    let nick = my_user!(r).nickname();

                    // Send the welcome sequence
                    send!(client; Respond::to(&self.host, &nick).welcome(format!("Welcome to the zircond test network, {}", nick)));
                    send!(client; Respond::to(&self.host, &nick).your_host(format!("Your host is zircond, running version {}", &crate_version)));
                    send!(client; Respond::to(&self.host, &nick).motd_start());
                    send!(client; Respond::to(&self.host, &nick).motd(&format!("Zircon IRCd v{}", &crate_version)));
                    send!(client; Respond::to(&self.host, &nick).motd_end());
                }

                IrcAction::UserJoinChannel(channel_name) => {

                    // Get the current user
                    let my_user = my_user!(r);

                    // Test whether the channel already exists
                    if self.channels.find(&channel_name).is_none() {

                        // Create a new channel
                        let channel = Channel::new(channel_name.clone());

                        // Add the new channel to the channel list
                        self.channels.add(channel);
                    }

                    // Find the channel
                    let channel = self.channels.find(&channel_name).unwrap();

                    // Add the user to the channel
                    channel.join_user(client_id);

                    // Send join acknowledgement to the user
                    let nick = my_user.nickname();
                    send!(client; Respond::to(&nick, &nick).join(channel_name.clone()));

                    // Test whether the channel has a topic
                    if let Some(topic) = &channel.topic {

                        // Tell the client about the topic
                        send!(client; Respond::to(&nick, &channel_name).topic(topic.clone()));
                    }

                    // Iterate over all users in the channel
                    for user_info in channel.users() {
                        
                        // Find the user
                        if let Some(channel_user) = self.users.find(user_info.client_id()) {

                            // Get channel mode
                            // "=": public
                            // "@": secret (+s)
                            // "*": private (+p)
                            let channel_mode = "=";
                            
                            // Tell the client about the user
                            send!(client; Respond::to(&self.host, &nick).names_reply(&channel_name, channel_mode, "", &channel_user.nickname()))
                        }
                    }

                    // Mark the end of the user list    
                    send!(client; Respond::to(&self.host, &nick).names_end(&channel_name));

                    // Iterate over all users in the channel
                    for other_client in channel.users() {

                        // Skip this user if it is the current user
                        if other_client.client_id() == client_id {
                            continue;
                        }

                        // Find user by user id
                        if let Some(other_user) = self.users.find_mut(other_client.client_id()) {

                            // Tell the user's client about the join
                            send!(other_user.stream(); Respond::to(&nick, &nick).join(channel_name.clone()));
                        }
                    }
                }

                IrcAction::ChannelListUsers(channel_name) => {

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
                    //                 send!(client; Respond::to(&self.host, &user.nickname()).names_reply(&channel_name, channel_mode, &channel_user.nickname()))
                    //             }
                    //         }

                    //         // Mark the end of the user list
                    //         send!(client; Respond::to(&self.host, &user.nickname()).names_end(&channel_name));
                    //     }
                    // }
                }

                IrcAction::Privmsg(target, message) => {

                    // Get the nickname of the current user
                    let user_nick = my_user!(r).nickname();

                    // Determine whether the target is a user or a channel
                    if target.starts_with('#') {

                        // Find the channel
                        if let Some(channel) = self.channels.find(&target) {

                            // Find all users in the channel
                            for other_user_info in channel.users() {

                                // Skip this user if it is the current user
                                if other_user_info.client_id() == client_id {
                                    continue;
                                }

                                // Find the user
                                if let Some(other_user) = self.users.find_mut(other_user_info.client_id()) {

                                    // Relay the private message to the other user
                                    let nick = other_user.nickname();
                                    send!(other_user.stream(); Respond::to(&user_nick, &nick).privmsg(target.clone(), message.clone()));
                                }
                            }
                        }
                    } else if let Some(other_user) = self.users.find_by_name_mut(&target) {
                        let other_nick = other_user.nickname();
                        send!(other_user.stream(); Respond::to(&user_nick, &other_nick).privmsg(target.clone(), message.clone()));
                    }
                }

                IrcAction::Pong(id) => {

                    // Respond to ping
                    send!(client; Respond::to(&self.host, &my_user!(r).nickname()).pong(id));
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