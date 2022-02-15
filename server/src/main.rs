use server::config::Configuration;
use server::config::{Http, Postgres};
use server::http::App;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "server=debug,tower_http=error,sqlx=error");
    }

    tracing_subscriber::fmt::init();

    let config = Configuration {
        http: Http {
            host: "0.0.0.0".to_string(),
            port: 8080,
        },
        postgres: Postgres {
            host: "db-postgresql-sfo3-66943-do-user-3317000-0.b.db.ondigitalocean.com".to_string(),
            port: 25060,
            user: "doadmin".to_string(),
            password: "odOAHxm1yf4jEFLr".to_string(),
            database: "postgres".to_string(),
            sslmode: true,
        },
    };

    let pool = config.postgres.pool().await;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to migrate database");

    App::new(config, pool).run().await
}
