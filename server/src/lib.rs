use crate::config::Configuration;
use crate::projects::Project;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{
    Context, EmptyMutation, EmptySubscription, Object, Result as GraphQlResult, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::http::{Method, StatusCode};
use axum::response::{self, IntoResponse};
use axum::{routing::get, AddExtensionLayer, Router};
use sqlx::PgPool;
use std::net::TcpListener;
use tokio::time::{sleep, Duration};
use tower::ServiceBuilder;
use tower_http::cors::{any, CorsLayer};
use tower_http::trace::TraceLayer;

pub mod config;
pub mod error;
pub mod projects;

pub struct App {
    router: Router,
    listener: TcpListener,
}

// Lookahead example
// https://cs.github.com/cthit/hubbit2/blob/40cd6541c9b9daa6c65198fe6a763b5d794e8dc0/backend/src/schema/stats.rs#L420

// TODO: Dump TCListener

impl App {
    pub async fn new(config: Configuration) -> Self {
        let schema = initialize_schema(&config).await;

        let cors = CorsLayer::new()
            .allow_methods(vec![Method::GET, Method::POST])
            .allow_origin(any());

        let middleware = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(cors)
            // .layer(AddExtensionLayer::new(pg_pool))
            .layer(AddExtensionLayer::new(schema));

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
        .route("/", get(graphql_playground).post(graphql_handler)) //
        .route("/health_check", get(health_check))
        .route("/projects", get(projects::list).post(projects::create))
        .route(
            "/projects/:id",
            get(projects::get)
                .post(projects::update)
                .delete(projects::delete),
        )
}

async fn initialize_schema(config: &Configuration) -> GraphqlSchema {
    let pg_pool = config.postgres.pool().await;

    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("failed to migrate database");

    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pg_pool)
        .finish()
}

type GraphqlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<GraphqlSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn projects(&self, ctx: &Context<'_>) -> GraphQlResult<Vec<Project>> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let projects = Project::fetch_all(pg_pool).await.unwrap();

        sleep(Duration::from_millis(500)).await;

        Ok(projects)
    }
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use crate::config::{Http, Postgres};
    use crate::{initialize_schema, Configuration, Project};
    use serde::{Deserialize, Deserializer};

    #[tokio::test]
    async fn test_projects_query() {
        let config = Configuration {
            http: Http {
                host: "127.0.0.1".to_string(),
                port: 0,
            },
            postgres: Postgres {
                host: "127.0.0.1".to_string(),
                port: 5432,
                user: "postgres".to_string(),
                password: "postgres".to_string(),
                database: "postgres".to_string(),
                sslmode: false,
            },
        };

        let schema = initialize_schema(&config).await;

        // #[derive(Debug, Deserialize)]
        // struct Object {
        //     projects: Vec<Project>,
        // }

        let response = schema.execute("query { projects { id project } }").await;
        let json_value = response.data.into_json().unwrap();
        // let object = serde_json::from_value::<Object>(json_value).unwrap();

        assert_eq!(
            json_value,
            serde_json::json!({
                "projects": [
                    {
                        "id" : 1,
                        "project" : "Project 1"
                    },
                    {
                        "id" : 2,
                        "project" : "Project 2"
                    },
                    {
                        "id" : 3,
                        "project" : "Project 3"
                    }
                ]
            })
        );
    }
}
