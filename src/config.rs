#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    server: ServerConfigServer,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfigServer {
    listen: String,
    host: String,
    port: u16,
}

impl ServerConfig {

    pub fn get_host(&self) -> &str {
        self.server.host.as_ref()
    }

    pub fn get_addr(&self) -> (&str, u16) {
        (self.server.listen.as_ref(), self.server.port)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server: ServerConfigServer::default(),
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