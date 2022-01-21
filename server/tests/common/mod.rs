use server::config::{Configuration, Http, Postgres};
use server::App;
use sqlx::Executor;
use uuid::Uuid;

pub struct TestApp {
    pub addr: String,
}

// schema is private
// TODO: add .execute() method to TestApp

impl TestApp {
    pub async fn new() -> Self {
        let test_database = format!("test_{}", Uuid::new_v4().to_string().replace('-', ""));

        let config = Configuration {
            http: Http {
                host: "localhost".to_string(),
                port: 0,
            },
            postgres: Postgres {
                host: "localhost".to_string(),
                port: 5432,
                user: "postgres".to_string(),
                password: "postgres".to_string(),
                database: test_database.clone(),
                sslmode: false,
            },
        };

        let mut pg_connection = config.postgres.connection().await;

        pg_connection
            .execute(format!("CREATE DATABASE {};", &config.postgres.database).as_str())
            .await
            .ok();

        // Run migrations to insert test data
        // sqlx::migrate!("./tests/migrations")
        //     .run(&mut pg_connection)
        //     .await
        //     .unwrap();

        let app = App::new(config).await;

        let addr = format!("http://{}", app.addr());

        tokio::spawn(async move { app.run().await });

        TestApp { addr }
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        // TODO: Need sqlx blocking API
        // https://github.com/launchbadge/sqlx/issues/1163

        // conn.execute(
        //     format!(
        //         r#"
        //         SELECT pg_terminate_backend(pid)
        //         FROM pg_stat_activity
        //         WHERE datname = '{}';
        //         "#,
        //         &self.test_database
        //     )
        //     .as_str(),
        // )
        // .await
        // .ok();

        // "DROP DATABASE {}"
    }
}
