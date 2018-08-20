use std::io::{Write, BufRead};
use std::net::{TcpListener};
use std::thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::ops::DerefMut;
use crate::message::{IrcMessageRequest, IrcMessageCommand, Respond};
use super::{User, Channel, ServerState, ClientState};

pub struct Server {
    host: String,
    port: u16,
    state: ServerState,
}

impl Server {
    pub fn new(host: Option<String>, port: Option<u16>) -> Self {
        Self {
            host: host.unwrap_or_else(|| "127.0.0.1".to_owned()),
            port: port.unwrap_or(6667),
            state: ServerState::new(),
        }
    }

    pub fn listen(&self) {
        let mut next_user_id = 0_usize;
        let mut threads = Vec::new();

        // Macro for simple server-to-client communication
        macro_rules! send {
            ($writer:expr; $variant:expr) => (
                $writer.stream().write_all(format!("{}\r\n", $variant.to_string()).as_ref()).unwrap()
            );
        }

        // Bind the tcp listener socket
        let listener = TcpListener::bind((self.host.as_ref(), self.port)).unwrap();

        // Accept new clients
        for client in listener.incoming() {

            // Get shared references important stuff
            let user_list = self.state.users();
            let channel_list = self.state.channels();
            let host = Arc::new(self.host.clone());
            let client = client.unwrap();

            // Create a new user for this client
            let user = User::new(next_user_id);
            let user = Arc::new(RwLock::new(user));
            next_user_id += 1;

            // Add the new user to the user list
            {
                let mut w = user_list.write().unwrap();
                w.deref_mut().push(user.clone());
            }

            // Create client state
            let client_state = Arc::new(ClientState::new(client, user));
            self.state.add_client(client_state.clone());

            // Spawn a thread to handle the client
            let handle = thread::spawn(move || {

                // Get the remote address of the client
                let addr = client_state.stream().peer_addr().unwrap();

                // Get a buffered reader for the incoming data
                let mut reader = std::io::BufReader::new(client_state.stream().try_clone().unwrap());

                // Handle new messages in a loop
                loop {

                    // Read the next line
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();

                    // Debug: Print the line
                    print!("[{:?}] {}", addr, line);

                    // Parse the message
                    let cmd = IrcMessageRequest::parse(&line);

                    // Read the nickname
                    let nick = {
                        let user = client_state.user();
                        let user = user.read().unwrap();
                        user.nickname()
                    };

                    match cmd.command {

                        IrcMessageCommand::Nick(nickname) => {

                            // Set nickname
                            let user = client_state.user();
                            let mut user = user.write().unwrap();
                            user.set_nickname(nickname);
                        }

                        IrcMessageCommand::User(username, realname) => {

                            // Set username and realname
                            {
                                let user = client_state.user();
                                let mut user = user.write().unwrap();
                                user.set_names(username, realname);
                            }

                            // Send welcome sequence
                            send!(client_state; Respond::to(&host.clone(), &nick).welcome(format!("Welcome to the zircond test network, {}", nick)));
                            send!(client_state; Respond::to(&host.clone(), &nick).your_host("Your host is zircond, running version 0.01".to_owned()));
                            send!(client_state; Respond::to(&host.clone(), &nick).motd_start());
                            send!(client_state; Respond::to(&host.clone(), &nick).motd(r"Zircon IRCd"));
                            send!(client_state; Respond::to(&host.clone(), &nick).motd_end());
                        }

                        IrcMessageCommand::Join(channel_name) => {

                            // Get the writeable channel list
                            let mut w = channel_list.write().unwrap();

                            // Test whether the channel already exists
                            if !w.deref_mut().iter().any(|channel| channel.name == channel_name) {

                                // Create the new channel
                                let channel = Channel::new(channel_name.clone());

                                // Add the user to the channel
                                channel.join_user(client_state.user());

                                // Add the channel the list
                                w.deref_mut().push(channel);
                            }

                            // Send the join acknowledgement to the user
                            send!(client_state; Respond::to(&host.clone(), &nick).join(channel_name));
                        }

                        IrcMessageCommand::Ping(challenge) => {
                            send!(client_state; Respond::to(&host.clone(), &nick).pong(challenge));
                        }

                        com => println!("Unhandled command: {:?}", com),
                    }

                    // Flush writes
                    client_state.stream().flush().unwrap();
                }
            });
            threads.push(handle);
        }
    }
}