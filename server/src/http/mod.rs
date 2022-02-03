use crate::config::Configuration;
use axum::http::Method;
use axum::routing::get;
use axum::{AddExtensionLayer, Router};
use sqlx::PgPool;
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{any, CorsLayer};
use tower_http::trace::TraceLayer;

mod assembly;
pub mod estimate;
mod graphql;
pub mod project;

pub struct App {
    router: Router,
    listener: TcpListener,
}

impl App {
    pub fn new(config: Configuration, pool: PgPool) -> Self {
        let schema = graphql::schema(pool);

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
