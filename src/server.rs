use std::io::{Read, Write, BufRead};
use std::net::{TcpListener, SocketAddr};
use std::thread;

struct IrcMessageTag(String, Option<String>);

impl ToString for IrcMessageTag {
    fn to_string(&self) -> String {

        // Pre-calculate the exact tag size
        let len = self.0.len() + self.1.as_ref().map_or(0, |tag| tag.len() + 1);

        // Allocate a perfectly fitted string
        let mut buf = String::with_capacity(len);

        // Push the tag parts
        buf.push_str(self.0.as_ref());
        if self.1.is_some() {
            buf.push('=');
            buf.push_str(self.1.as_ref().unwrap().as_ref());
        }
        
        buf
    }
}

enum IrcMessageTags {
    One(IrcMessageTag),
    Many(Vec<IrcMessageTag>),
}

impl ToString for IrcMessageTags {
    fn to_string(&self) -> String {

        // Map multiple tags to their string representations
        let tags = match self {
            IrcMessageTags::Many(tags) => Some(tags.iter().map(|tag| tag.to_string()).collect::<Vec<_>>()),
            _ => None,
        };

        // Get the exact combined length of all tags
        let len = match self {
            IrcMessageTags::One(tag) => tag.to_string().len(),
            IrcMessageTags::Many(_) => tags.iter().map(|tag| tag.len()).fold(0, |a, b| a + b),
        };

        // Get the tag separator count
        let len_separators = match self {
            IrcMessageTags::Many(tags) => if tags.len() > 1 { tags.len() - 1 } else { 0 },
            _ => 0,
        };

        // Create a perfectly fitted string
        let mut buf = String::with_capacity(len + len_separators + 1);

        // Push the tag prefix '@'
        buf.push('@');

        match self {

            // Push a single tag to the string buffer
            IrcMessageTags::One(tag) => buf.push_str(tag.to_string().as_ref()),

            // Push multiple tags to the string buffer
            IrcMessageTags::Many(_) => {

                // Unwrap the tag strings.
                // This is guaranteed to work since the tags variable is
                // always Some(_) if the IrcMessageTags variant is Many(_)
                let tags = tags.unwrap();

                for (i, tag) in tags.iter().enumerate() {

                    // Push separator if necessary
                    if i > 0 {
                        buf.push(';');
                    }

                    buf.push_str(tag);
                }
            }
        }

        buf
    }
}

struct IrcMessagePrefix(String);

struct IrcMessage {
    tags: Option<IrcMessageTags>,
    prefix: Option<IrcMessagePrefix>,
    command: IrcMessageCommand,
}

impl IrcMessage {
    pub fn new(command: IrcMessageCommand, prefix: Option<IrcMessagePrefix>, tags: Option<IrcMessageTags>) -> Self {
        Self {
            tags,
            prefix,
            command,
        }
    }

    pub fn parse(line: String) -> Self {
        
        IrcMessage::default()
    }
}

impl Default for IrcMessage {
    fn default() -> Self {
        Self::new(IrcMessageCommand::None, None, None)
    }
}

enum IrcMessageCommand {
    None,
    Nick(String),
    User(String, Option<String>)
}

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn new(host: Option<String>, port: Option<u16>) -> Self {
        Self {
            host: host.unwrap_or("127.0.0.1".to_owned()),
            port: port.unwrap_or(6667),
        }
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind((self.host.as_ref(), self.port)).unwrap();
        let mut threads = Vec::new();
        for client in listener.incoming() {
            let handle = thread::spawn(|| {
                let client = client.unwrap();
                let addr = client.peer_addr().unwrap();
                let mut reader = std::io::BufReader::new(client);
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    print!("[{:?}] {}", addr, line);
                }
            });
            threads.push(handle);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}