pub struct Configuration {
    pub host: String,
    pub port: u16,
}

impl Configuration {
    pub fn test() -> Self {
        Configuration {
            host: "127.0.0.1".to_string(),
            port: 0,
        }
    }
    pub fn address(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
