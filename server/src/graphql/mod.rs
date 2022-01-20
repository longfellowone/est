use crate::{Configuration, IntoResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response;
use query_root::QueryRoot;

mod query_root;

type GraphqlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn schema(config: &Configuration) -> GraphqlSchema {
    let pg_pool = config.postgres.pool().await;

    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("failed to migrate database");

    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pg_pool)
        .finish()
}

pub async fn handler(schema: Extension<GraphqlSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
