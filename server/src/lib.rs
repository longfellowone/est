use crate::config::Configuration;
use axum::http::Method;
use axum::response::IntoResponse;
use axum::{routing::get, AddExtensionLayer, Router};
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{any, CorsLayer};
use tower_http::trace::TraceLayer;

pub mod config;
pub mod error;
pub mod estimating;
pub mod graphql;

pub struct App {
    router: Router,
    listener: TcpListener,
}

// Lookahead example
// https://cs.github.com/cthit/hubbit2/blob/40cd6541c9b9daa6c65198fe6a763b5d794e8dc0/backend/src/schema/stats.rs#L420

impl App {
    pub async fn new(config: Configuration) -> Self {
        let pg_pool = config.postgres.pool().await;

        // TODO: keep getting "VersionMissing" error when have more than 1 migration
        sqlx::migrate!("./migrations")
            .run(&pg_pool)
            .await
            .expect("failed to migrate database");

        let schema = graphql::schema(pg_pool).await;

        let cors = CorsLayer::new()
            .allow_methods(vec![Method::GET, Method::POST])
            .allow_origin(any());

        let middleware = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(cors)
            .layer(AddExtensionLayer::new(schema));

        let routes = Router::new().route("/", get(graphql::playground).post(graphql::handler));

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
