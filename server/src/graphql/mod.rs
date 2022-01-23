use crate::graphql::mutation_root::MutationRoot;
use crate::IntoResponse;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response;
use query_root::QueryRoot;
use sqlx::PgPool;

pub(crate) mod estimate;
mod mutation_root;
mod project;
mod query_root;

pub type GraphqlSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn schema(pg_pool: PgPool) -> GraphqlSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pg_pool)
        .finish()
}

pub async fn handler(schema: Extension<GraphqlSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
