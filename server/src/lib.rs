use crate::config::Configuration;
use axum::http::StatusCode;
use axum::{routing::get, AddExtensionLayer, Router};

use axum::response::IntoResponse;
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub mod config;
mod error;
pub mod projects;

pub struct App {
    router: Router,
    listener: TcpListener,
}

impl App {
    pub async fn new(config: Configuration) -> Self {
        let pg_pool = config.postgres.pool().await;

        sqlx::migrate!("./migrations")
            .run(&pg_pool)
            .await
            .expect("failed to migrate database");

        let middleware = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(pg_pool));

        let routes = initialize_routes();

        let router = Router::new().merge(routes).layer(middleware);

        let listener =
            TcpListener::bind(&config.http.address()).expect("failed to bind TCP listener");

        App { router, listener }
    }

    pub async fn run(self) -> hyper::Result<()> {
        tracing::debug!("listening on {:?}", self.addr());

        axum::Server::from_tcp(self.listener)
            .expect("failed to start server")
            .serve(self.router.into_make_service())
            .await
    }

    pub fn addr(&self) -> String {
        format!(
            "{}",
            self.listener
                .local_addr()
                .expect("failed to get local_addr from listener")
        )
    }
}

fn initialize_routes() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/projects", get(projects::list).post(projects::create))
        .route(
            "/projects/:id",
            get(projects::get)
                .post(projects::update)
                .delete(projects::delete),
        )
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
