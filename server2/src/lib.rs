use crate::configuration::Configuration;
use axum::{routing::get, Router};
use std::net::TcpListener;
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

        let app = Router::new()
            .route("/", get(index))
            .layer(TraceLayer::new_for_http());

        App { app, listener }
    }

    pub async fn run(self) {
        tracing::debug!("listening on {:?}", self.listener.local_addr());

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
// Add description to assembly

// zero2prod axum
// https://github.com/mattiapenati/zero2prod/tree/main/src

// RequestId
// https://github.com/mattiapenati/zero2prod/blob/db22778804b55d5fa864d2266f6379b3fcd6b870/src/request_id.rs
