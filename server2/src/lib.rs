use crate::configuration::Configuration;
use axum::http::StatusCode;
use axum::{routing::get, AddExtensionLayer, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub mod configuration;
mod postgres;

pub struct App {
    router: Router,
    listener: TcpListener,
}

impl App {
    pub async fn new(config: Configuration) -> Self {
        let listener = TcpListener::bind(&config.http.address()).unwrap();

        let pg_pool = PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(config.postgres.connect_options());

        // TODO: migration needs database to exist before it can connect for migrations
        // TODO: use TestApp in /tests
        sqlx::migrate!("./migrations").run(&pg_pool).await.unwrap();

        let middleware = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(pg_pool));

        let routes = Router::new()
            .route("/", get(index))
            .route("/health_check", get(health_check));

        let router = Router::new().merge(routes).layer(middleware);

        App { router, listener }
    }

    pub async fn run(self) -> hyper::Result<()> {
        tracing::debug!("listening on {:?}", self.listener.local_addr().unwrap());

        axum::Server::from_tcp(self.listener)
            .unwrap()
            .serve(self.router.into_make_service())
            .await
    }
}

async fn index() -> String {
    "Hello, World!".to_string()
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

// TODO: Add tracing using LAZY
// https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_check() {
        let app = App::new(Configuration::test()).await;

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/health_check")
            .body(Body::empty())
            .unwrap();

        let response = app.router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty())
    }
}

// Add description to assembly
// Use rust_decimal with postgres feature
// https://docs.rs/rust_decimal/latest/rust_decimal/#db-postgres

// zero2prod axum
// https://github.com/mattiapenati/zero2prod/tree/main/src

// RequestId
// https://github.com/mattiapenati/zero2prod/blob/db22778804b55d5fa864d2266f6379b3fcd6b870/src/request_id.rs
