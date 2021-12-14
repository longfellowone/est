#![allow(dead_code)]
// https://github.com/LukeMathWalker/zero-to-production/blob/main/tests/api/helpers.rs
// https://github.com/diesel-rs/diesel/blob/75c688c3b246295f6f7182e0fec1b58cc685b4ed/diesel_tests/tests/select.rs
// http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch11-03-test-organization.html
// https://joshleeb.com/posts/rust-integration-tests.html

// use chrono::{DateTime, Utc};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
// use sqlx::FromRow;
// use uuid::Uuid;

#[actix_web::main]
async fn main() {
    let pg_options = PgConnectOptions::new()
        .username("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5432)
        .database("postgres")
        .ssl_mode(PgSslMode::Prefer);

    let pg_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(pg_options)
        .await
        .unwrap();

    // sqlx::migrate!().run(&pg_pool).await.unwrap();

    // Setup products. Then setup assemblies. Then estimates. Then projects
    // https://www.one-tab.com/page/GH3FJHoARRe1_t48x6FyxA

    #[derive(Debug)]
    struct Estimate {
        id: i32,
        estimate: String,
        // assemblies: Vec<Assembly>,
    }

    #[derive(Debug)]
    struct Assembly {
        id: i32,
        assembly: String,
    }

    let estimate_id = 1;

    let estimate = sqlx::query_as!(
        Estimate,
        r#"
        SELECT id, estimate
        FROM estimates 
        WHERE id = $1
        "#,
        estimate_id
    )
    .fetch_one(&pg_pool)
    .await
    .unwrap();

    println!("{:?}", estimate);

    let assemblies = sqlx::query_as!(
        Assembly,
        r#"
        SELECT a.id, a.assembly
        FROM assemblies a 
        INNER JOIN estimate_assemblies ea on a.id = ea.assembly_id 
        WHERE ea.estimate_id = $1
        "#,
        estimate_id
    )
    .fetch_all(&pg_pool)
    .await
    .unwrap();

    println!("{:?}", assemblies)
}
