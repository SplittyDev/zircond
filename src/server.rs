mod user;
mod channel;
pub use self::user::User;
pub use self::channel::Channel;

use std::io::{Read, Write, BufRead};
use std::net::{TcpListener, SocketAddr};
use std::thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Mutex;
use std::ops::DerefMut;
use crate::message::{IrcMessageRequest, IrcMessageCommand, Respond};

pub struct Server {
    host: String,
    port: u16,
    users: Arc<RwLock<Vec<Arc<Mutex<User>>>>>,
    channels: RwLock<Vec<Channel>>,
}

impl Server {
    pub fn new(host: Option<String>, port: Option<u16>) -> Self {
        Self {
            host: host.unwrap_or("127.0.0.1".to_owned()),
            port: port.unwrap_or(6667),
            users: Arc::new(RwLock::new(Vec::new())),
            channels: RwLock::new(Vec::new()),
        }
    }

    pub fn listen(&self) {
        macro_rules! send {
            ($writer:expr; $variant:expr) => (
                $writer.write(format!("{}\r\n", $variant.to_string()).as_ref()).unwrap()
            );
        }
        let listener = TcpListener::bind((self.host.as_ref(), self.port)).unwrap();
        let mut threads = Vec::new();
        for client in listener.incoming() {
            let user = Arc::new(Mutex::new(User::new()));
            let user_list = self.users.clone();
            let host = Arc::new(self.host.clone());
            let handle = thread::spawn(move || {
                let mut client = client.unwrap();
                let addr = client.peer_addr().unwrap();
                let mut reader = std::io::BufReader::new(client.try_clone().unwrap());
                {
                    let mut w = user_list.write().unwrap();
                    w.deref_mut().push(user.clone());
                }
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    print!("[{:?}] {}", addr, line);
                    let cmd = IrcMessageRequest::parse(line);
                    match cmd.command {
                        IrcMessageCommand::Nick(nickname) => {
                            let mut user = user.lock().unwrap();
                            user.set_nickname(nickname);
                        },
                        IrcMessageCommand::User(username, realname) => {
                            let nick: String;
                            {
                                let mut user = user.lock().unwrap();
                                nick = user.nickname();
                                user.set_names(username, realname);
                            }
                            send!(client; Respond::to(&host.clone(), &nick).welcome(format!("Welcome to the zircond test network, {}", nick)));
                            send!(client; Respond::to(&host.clone(), &nick).your_host("Your host is zircond, running version 0.01".to_owned()));
                        },
                        com => println!("Unhandled command: {:?}", com),
                    }
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