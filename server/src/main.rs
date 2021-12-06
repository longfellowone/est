// https://github.com/LukeMathWalker/zero-to-production/blob/main/tests/api/helpers.rs
// https://github.com/diesel-rs/diesel/blob/75c688c3b246295f6f7182e0fec1b58cc685b4ed/diesel_tests/tests/select.rs
// http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch11-03-test-organization.html
// https://joshleeb.com/posts/rust-integration-tests.html

use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::FromRow;
use uuid::Uuid;

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

    sqlx::migrate!().run(&pg_pool).await.unwrap();

    #[derive(Debug, FromRow)]
    struct Projects {
        id: Uuid,
        name: String,
    }

    // let res = sqlx::query_as::<_, Projects>("SELECT nothing FROM projects")
    //     .fetch_all(&pg_pool)
    //     .await
    //     .unwrap();

    let res = sqlx::query_as!(Projects, "SELECT id, name FROM projects")
        .fetch_all(&pg_pool)
        .await
        .unwrap();

    // sqlx::query!("INSERT INTO projects (id, name) VALUES ('21d957d2-80a2-4399-97cb-938a181ad27a','project 4')")
    //     .execute(&pg_pool)
    //     .await
    //     .unwrap();

    println!("{:?}", res[0]);
}
