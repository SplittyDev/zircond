mod user;
mod channel;
pub use self::user::User;
pub use self::channel::Channel;

use std::io::{Write, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::ops::DerefMut;
use crate::message::{IrcMessageRequest, IrcMessageCommand, Respond};

pub struct Server {
    host: String,
    port: u16,
    clients: Arc<RwLock<Vec<TcpStream>>>,
    users: Arc<RwLock<Vec<Arc<RwLock<User>>>>>,
    channels: Arc<RwLock<Vec<Channel>>>,
}

impl Server {
    pub fn new(host: Option<String>, port: Option<u16>) -> Self {
        Self {
            host: host.unwrap_or_else(|| "127.0.0.1".to_owned()),
            port: port.unwrap_or(6667),
            clients: Arc::new(RwLock::new(Vec::new())),
            users: Arc::new(RwLock::new(Vec::new())),
            channels: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn listen(&self) {
        let mut next_user_id = 0_usize;
        let mut threads = Vec::new();

        // Macro for simple server-to-client communication
        macro_rules! send {
            ($writer:expr; $variant:expr) => (
                $writer.write_all(format!("{}\r\n", $variant.to_string()).as_ref()).unwrap()
            );
        }

        // Bind the tcp listener socket
        let listener = TcpListener::bind((self.host.as_ref(), self.port)).unwrap();

        // Accept new clients
        for client in listener.incoming() {

            // Get shared references important stuff
            let user_list = self.users.clone();
            let client_list = self.clients.clone();
            let channel_list = self.channels.clone();
            let host = Arc::new(self.host.clone());
            let mut client = client.unwrap();

            // Add the new client to the client list
            {
                let mut w = client_list.write().unwrap();
                w.deref_mut().push(client.try_clone().unwrap());
            }

            // Create a new user for this client
            let user = User::new(next_user_id);
            let user = Arc::new(RwLock::new(user));
            next_user_id += 1;

            // Add the new user to the user list
            {
                let mut w = user_list.write().unwrap();
                w.deref_mut().push(user.clone());
            }

            // Spawn a thread to handle the client
            let handle = thread::spawn(move || {

                // Get the remote address of the client
                let addr = client.peer_addr().unwrap();

                // Get a buffered reader for the incoming data
                let mut reader = std::io::BufReader::new(client.try_clone().unwrap());

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
                        let user = user.clone();
                        let user = user.read().unwrap();
                        user.nickname()
                    };

                    match cmd.command {

                        IrcMessageCommand::Nick(nickname) => {

                            // Set nickname
                            let mut user = user.write().unwrap();
                            user.set_nickname(nickname);
                        }

                        IrcMessageCommand::User(username, realname) => {

                            // Set username and realname
                            {
                                let mut user = user.write().unwrap();
                                user.set_names(username, realname);
                            }

                            // Send welcome sequence
                            send!(client; Respond::to(&host.clone(), &nick).welcome(format!("Welcome to the zircond test network, {}", nick)));
                            send!(client; Respond::to(&host.clone(), &nick).your_host("Your host is zircond, running version 0.01".to_owned()));
                            send!(client; Respond::to(&host.clone(), &nick).motd_start());
                            send!(client; Respond::to(&host.clone(), &nick).motd(r"Zircon IRCd"));
                            send!(client; Respond::to(&host.clone(), &nick).motd_end());
                        }

                        IrcMessageCommand::Join(channel_name) => {

                            // Get the writeable channel list
                            let mut w = channel_list.write().unwrap();

                            // Test whether the channel already exists
                            if !w.deref_mut().iter().any(|channel| channel.name == channel_name) {

                                // Create the new channel
                                let channel = Channel::new(channel_name.clone());

                                // Add the user to the channel
                                channel.join_user(user.clone());

                                // Add the channel the list
                                w.deref_mut().push(channel);
                            }

                            // Send the join acknowledgement to the user
                            send!(client; Respond::to(&host.clone(), &nick).join(channel_name));
                        }

                        IrcMessageCommand::Ping(challenge) => {
                            send!(client; Respond::to(&host.clone(), &nick).pong(challenge));
                        }

                        com => println!("Unhandled command: {:?}", com),
                    }

                    // Flush writes
                    client.flush().unwrap();
                }
            });
            threads.push(handle);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::message::*;

    #[test]
    fn irc_message_tag_struct_to_string() {
        let tag = IrcMessageTag("foo".to_owned(), None).to_string();
        assert_eq!("foo", tag);
    }

    #[test]
    fn irc_message_tag_struct_to_string_with_argument() {
        let tag = IrcMessageTag("foo".to_owned(), Some("bar".to_owned())).to_string();
        assert_eq!("foo=bar", tag);
    }

    #[test]
    fn irc_message_tags_enum_to_string_one() {
        let tags_a = IrcMessageTags::One(IrcMessageTag("foo".to_owned(), None)).to_string();
        let tags_b = IrcMessageTags::One(IrcMessageTag("foo".to_owned(), Some("bar".to_owned()))).to_string();
        assert_eq!("@foo", tags_a);
        assert_eq!("@foo=bar", tags_b);
    }

    #[test]
    fn irc_message_tags_enum_to_string_many_actually_one() {
        let tags_a = IrcMessageTags::Many(vec![IrcMessageTag("foo".to_owned(), None)]).to_string();
        let tags_b = IrcMessageTags::Many(vec![IrcMessageTag("foo".to_owned(), Some("bar".to_owned()))]).to_string();
        assert_eq!("@foo", tags_a);
        assert_eq!("@foo=bar", tags_b);
    }

    #[test]
    fn irc_message_tags_enum_to_string_many_actually_many() {
        let tags_a = IrcMessageTags::Many(vec![
            IrcMessageTag("foo".to_owned(), None),
            IrcMessageTag("bar".to_owned(), None),
        ]).to_string();
        let tags_b = IrcMessageTags::Many(vec![
            IrcMessageTag("foo".to_owned(), Some("bar".to_owned())),
            IrcMessageTag("baz".to_owned(), Some("bax".to_owned())),
        ]).to_string();
        let tags_c = IrcMessageTags::Many(vec![
            IrcMessageTag("foo".to_owned(), None),
            IrcMessageTag("bar".to_owned(), Some("baz".to_owned())),
        ]).to_string();
        let tags_d = IrcMessageTags::Many(vec![
            IrcMessageTag("foo".to_owned(), Some("bar".to_owned())),
            IrcMessageTag("baz".to_owned(), None),
        ]).to_string();
        assert_eq!("@foo;bar", tags_a);
        assert_eq!("@foo=bar;baz=bax", tags_b);
        assert_eq!("@foo;bar=baz", tags_c);
        assert_eq!("@foo=bar;baz", tags_d);
    }

    #[test]
    fn irc_message_prefix_to_string() {
        let prefix = IrcMessagePrefix("irc.foo.bar".to_owned()).to_string();
        assert_eq!(":irc.foo.bar", prefix);
    }

    #[test]
    fn irc_message_command_to_string_nick() {
        let cmd = IrcMessageCommand::Nick("foo".to_owned()).to_string();
        assert_eq!("NICK :foo", cmd);
    }
}