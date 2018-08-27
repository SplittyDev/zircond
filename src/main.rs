#![feature(uniform_paths)]

#[macro_use]
extern crate serde_derive;

mod protocol;
mod message;
mod parser;
mod server;
mod config;

use server::Server;
use config::ServerConfig;

use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {

    // Read config
    let config = read_config()?;

    // Create server with config
    let mut server = Server::new(config);

    // Listen
    server.listen();
    Ok(())
}

fn read_config() -> std::io::Result<ServerConfig> {

    // Open config file
    let mut config_file = File::open("config.toml")?;

    // Read file into buffer
    let mut config_text = String::new();
    config_file.read_to_string(&mut config_text)?;

    // Deserialize configuration
    if let Ok(config) = toml::from_str(&config_text) {
        Ok(config)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse config."))
    }
}