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
            host: "localhost".to_string(),
            port: 8080,
        },
        postgres: Postgres {
            host: "localhost".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "postgres".to_string(),
            sslmode: false,
        },
    };

    let pool = config.postgres.pool().await;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to migrate database");

    App::new(config, pool).run().await
}
