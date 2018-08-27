#![feature(uniform_paths)]

mod protocol;
mod message;
mod parser;
mod server;
mod config;

use server::Server;
use config::ServerConfig;

use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {

    // Read or create configuration
    let config = {
        if std::path::Path::new("config.toml").exists() {
            read_config()
        } else {
            create_config()
        }
    }?;

    // Create server
    let mut server = Server::new(config);

    // Listen
    server.listen();

    // Exit with no errors
    Ok(())
}

fn create_config() -> std::io::Result<ServerConfig> {

    // Create default configuration
    let config = ServerConfig::default();

    // Serialize config to string
    if let Ok(config_text) = toml::to_string(&config) {
        
        // Write to file
        let mut file = File::create("config.toml")?;
        file.write_all(config_text.as_ref())?;

        // Return the configuration
        Ok(config)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to serialize default configuration."))
    }
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
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to deserialize configuration."))
    }
}