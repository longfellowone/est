use crate::configuration::Configuration;
use axum::http::StatusCode;
use axum::{routing::get, Router};
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
    pub fn new(config: Configuration) -> Self {
        let listener = TcpListener::bind(config.address()).unwrap();

        let routes = Router::new()
            .route("/", get(index))
            .route("/health_check", get(health_check));

        // Run migrations

        let middleware = ServiceBuilder::new().layer(TraceLayer::new_for_http());

        let router = Router::new().merge(routes).layer(middleware);

        App { router, listener }
    }

    pub async fn run(self) {
        tracing::debug!("listening on {:?}", self.listener.local_addr().unwrap());

        axum::Server::from_tcp(self.listener)
            .unwrap()
            .serve(self.router.into_make_service())
            .await
            .unwrap();
    }
}

async fn index() -> String {
    "Hello, World!".to_string()
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

// Add tracing using LAZY
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_check() {
        let app = App::new(Configuration::test());

        let request = Request::builder()
            .method("GET")
            .uri("/health_check")
            .body(Body::empty())
            .unwrap();

        let response = app.router.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK)
    }
}

// Add description to assembly

// zero2prod axum
// https://github.com/mattiapenati/zero2prod/tree/main/src

// RequestId
// https://github.com/mattiapenati/zero2prod/blob/db22778804b55d5fa864d2266f6379b3fcd6b870/src/request_id.rs
