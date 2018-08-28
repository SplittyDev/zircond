
// Macro for simple server-to-client communication
macro_rules! send {
    ($writer:expr; $variant:expr) => {
        std::io::Write::write_all($writer, format!("{}\r\n", $variant.to_string()).as_ref()).unwrap()
    };
}

mod command_dispatch;
mod join_channel;

pub(crate) use self::command_dispatch::CommandDispatch;
pub use self::join_channel::JoinChannel;

use std::net::TcpStream;
use crate::server::Server;

pub fn dispatch(dispatcher: &impl CommandDispatch, mut server: &mut Server, mut client: &mut TcpStream, client_id: usize) {
    dispatcher.dispatch(&mut server, &mut client, client_id);
}