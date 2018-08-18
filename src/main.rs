#![feature(uniform_paths)]

mod server;
use server::Server;

fn main() {
    let server = Server::new(None, None);
    server.listen();
}
