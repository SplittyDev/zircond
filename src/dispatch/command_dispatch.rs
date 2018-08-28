use std::net::TcpStream;
use crate::server::Server;

pub trait CommandDispatch {
    fn dispatch(&self, server: &mut Server, client: &mut TcpStream, client_id: usize);
}