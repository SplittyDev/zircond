#[derive(Deserialize)]
pub struct ServerConfig {
    server: ServerConfigServer,
}

#[derive(Deserialize)]
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