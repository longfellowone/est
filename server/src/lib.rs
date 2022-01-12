use crate::config::{Configuration, Postgres};
use axum::http::StatusCode;
use axum::{routing::get, AddExtensionLayer, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub mod config;
mod projects;

pub struct App {
    router: Router,
    listener: TcpListener,
}

impl App {
    pub async fn new(config: Configuration) -> Self {
        let pg_pool = initialize_postgres(config.postgres).await;

        let middleware = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(pg_pool));

        let routes = initialize_routes();

        let router = Router::new().merge(routes).layer(middleware);

        let listener = TcpListener::bind(&config.http.address()).unwrap();

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

fn initialize_routes() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/projects", get(projects::list).post(projects::create))
        .route("/projects/:id", get(projects::get).delete(projects::delete))
}

async fn initialize_postgres(postgres: Postgres) -> PgPool {
    let mut conn = PgConnection::connect_with(&postgres.connect_options_without_db())
        .await
        .unwrap();

    conn.execute(format!("CREATE DATABASE {};", &postgres.database).as_str())
        .await
        .ok();

    let pg_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(postgres.connect_options());

    sqlx::migrate!("./migrations").run(&pg_pool).await.unwrap();

    pg_pool
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

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

// zero2prod axum
// https://github.com/mattiapenati/zero2prod/tree/main/src
