use sqlx::postgres::{PgConnectOptions, PgConnection, PgPoolOptions, PgSslMode};
use sqlx::{Connection, PgPool};
use uuid::Uuid;

pub struct Configuration {
    pub http: Http,
    pub postgres: Postgres,
}

impl Configuration {
    pub fn test() -> Self {
        Self {
            http: Http {
                host: "127.0.0.1".to_string(),
                port: 0,
            },
            postgres: Postgres {
                host: "127.0.0.1".to_string(),
                port: 5432,
                user: "postgres".to_string(),
                password: "postgres".to_string(),
                database: format!("test_{}", Uuid::new_v4().to_string().replace('-', "")),
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
    pub async fn pool(&self) -> PgPool {
        PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(self.connect_options_with_db())
    }

    pub async fn connection(&self) -> PgConnection {
        PgConnection::connect_with(&self.connect_options())
            .await
            .expect("failed to connect to database")
    }

    fn connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.user)
            .password(&self.password)
            .host(&self.host)
            .port(self.port)
            .ssl_mode(PgSslMode::Prefer)
    }

    fn connect_options_with_db(&self) -> PgConnectOptions {
        self.connect_options().database(&self.database)
    }
}
