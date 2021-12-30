pub struct Configuration {
    pub host: String,
    pub port: u16,
}

impl Configuration {
    pub fn address(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
