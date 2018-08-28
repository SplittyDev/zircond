use std::net::TcpStream;

use crate::server::Server;
use crate::dispatch::command_dispatch::CommandDispatch;

pub struct Dummy;
impl CommandDispatch for Dummy {
    fn dispatch(server: &mut Server, client: &mut TcpStream, client_id: usize, params: &[String]) {

    }
}