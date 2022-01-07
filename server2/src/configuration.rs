use sqlx::postgres::{PgConnectOptions, PgSslMode};

pub struct Configuration {
    pub http: Http,
    pub postgres: Postgres,
}

impl Configuration {
    pub fn test() -> Self {
        Configuration {
            http: Http {
                host: "127.0.0.1".to_string(),
                port: 0,
            },
            postgres: Postgres {
                host: "127.0.0.1".to_string(),
                port: 5432,
                user: "postgres".to_string(),
                password: "postgres".to_string(),
                database: "app".to_string(),
                sslmode: false,
            },
        }
    }
}

pub struct Http {
    pub host: String,
    pub port: u16,
}

impl Http {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub sslmode: bool,
}

impl Postgres {
    pub fn connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username("postgres")
            .password("postgres")
            .host("127.0.0.1")
            .port(5432)
            .database("postgres")
            .ssl_mode(PgSslMode::Prefer)
    }
}
