#![feature(uniform_paths)]

mod message;
mod parser;
mod server;

use server::Server;

fn main() {
    let mut server = Server::new(None, None);
    server.listen();
}
