// Macro for simple server-to-client communication
macro_rules! send {
    ($writer:expr; $variant:expr) => {
        std::io::Write::write_all($writer, format!("{}\r\n", $variant.to_string()).as_ref()).unwrap()
    };
}

pub fn dispatch(dispatcher: &impl CommandDispatch, mut server: &mut Server, mut client: &mut TcpStream, client_id: usize) {
    dispatcher.dispatch(&mut server, &mut client, client_id);
}

mod command_dispatch;
pub(crate) use self::command_dispatch::CommandDispatch;

mod set_nick;
pub(crate) use self::set_nick::SetNick;

mod join_channel;
pub(crate) use self::join_channel::JoinChannel;

mod part_channel;
pub(crate) use self::part_channel::PartChannel;

mod private_message;
pub(crate) use self::private_message::PrivateMessage;

use std::net::TcpStream;
use crate::server::Server;