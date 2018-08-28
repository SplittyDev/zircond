use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    server: ServerConfigServer,
    client: Option<ServerConfigClient>,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfigServer {
    listen: String,
    host: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfigClient {
    autojoin: Option<Vec<String>>,
}

impl ServerConfig {

    pub fn get_host(&self) -> &str {
        self.server.host.as_ref()
    }

    pub fn get_addr(&self) -> (&str, u16) {
        (self.server.listen.as_ref(), self.server.port)
    }

    pub fn get_autojoin_channels(&self) -> Option<Vec<String>> {
        if let Some(client) = &self.client {
            if let Some(channels) = &client.autojoin {
                Some(channels.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server: ServerConfigServer::default(),
            client: Some(ServerConfigClient::default()),
        }
    }
}

impl Default for ServerConfigServer {
    fn default() -> Self {
        Self {
            listen: "127.0.0.1".to_string(),
            host: "127.0.0.1".to_string(),
            port: 6667,
        }
    }
}

impl Default for ServerConfigClient {
    fn default() -> Self {
        Self {
            autojoin: Some(vec!["#chat".to_owned()]),
        }
    }
}