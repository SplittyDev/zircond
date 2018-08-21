#[macro_use]
mod macros {
    macro_rules! impl_read {
        ($name:ident: $fn:ident -> $rettype:ty) => {
            pub fn $name<F, R>(&self, f: F) -> R where F: (Fn(&$rettype) -> R) {
                f(&*self.$fn.clone().read().unwrap())
            }
        };
    }

    macro_rules! impl_write {
        ($name:ident: $fn:ident -> $rettype:ty) => {
            pub fn $name<F, R>(&self, f: F) -> R where F: (Fn(&mut $rettype) -> R) {
                let tmp = self.$fn.clone();
                let mut tmp = tmp.write().unwrap();
                f(&mut *tmp)
            }
        };
    }
}

mod user;
mod channel;
mod user_list;
mod channel_list;
mod server_state;
mod client_state;
mod action;
mod server;

pub use self::user::User;
pub use self::channel::Channel;
pub use self::user_list::UserList;
pub use self::channel_list::ChannelList;
pub use self::server_state::ServerState;
pub use self::client_state::ClientState;
pub use self::action::IrcAction;
pub use self::server::Server;

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