use crate::configuration::Configuration;
use axum::http::StatusCode;
use axum::{routing::get, Router};
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub mod configuration;
mod postgres;

pub struct App {
    app: Router,
    listener: TcpListener,
}

impl App {
    pub fn new(config: Configuration) -> Self {
        let listener = TcpListener::bind(config.address()).unwrap();

        let routes = Router::new()
            .route("/", get(index))
            .route("/health_check", get(health_check));

        let middleware = ServiceBuilder::new().layer(TraceLayer::new_for_http());

        let app = Router::new().merge(routes).layer(middleware);

        App { app, listener }
    }

    pub async fn run(self) {
        tracing::debug!("listening on {:?}", self.listener.local_addr().unwrap());

        axum::Server::from_tcp(self.listener)
            .unwrap()
            .serve(self.app.into_make_service())
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

// Add description to assembly

// zero2prod axum
// https://github.com/mattiapenati/zero2prod/tree/main/src

// RequestId
// https://github.com/mattiapenati/zero2prod/blob/db22778804b55d5fa864d2266f6379b3fcd6b870/src/request_id.rs
