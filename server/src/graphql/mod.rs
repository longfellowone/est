use crate::graphql::estimate::{EstimateMutations, EstimateQueries};
use crate::graphql::project::{ProjectMutations, ProjectQueries};
use crate::IntoResponse;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response;
use sqlx::PgPool;

mod estimate;
mod project;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ProjectQueries, EstimateQueries);

#[derive(MergedObject, Default)]
pub struct MutationRoot(ProjectMutations, EstimateMutations);

pub type GraphqlSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn schema(pg_pool: PgPool) -> GraphqlSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pg_pool)
    .finish()
}

pub async fn handler(schema: Extension<GraphqlSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
