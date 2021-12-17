#![allow(dead_code)]

use anyhow::Result;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<()> {
    let pg_options = PgConnectOptions::new()
        .username("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5432)
        .database("postgres")
        .ssl_mode(PgSslMode::Prefer);

    let _pg_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(pg_options)
        .await
        .unwrap();

    // TODO: move to App
    // let config = Configuration::new()?;
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Starting server...");

    // TODO: let app = App::new(config) -> app.run()
    server::run(listener)?.await?;

    Ok(())
}
