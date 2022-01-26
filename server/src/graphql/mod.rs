use crate::graphql::estimate::{EstimateMutations, EstimateQueries};
use crate::graphql::loaders::{EstimateLoader, ProjectLoader};
use crate::graphql::project::{ProjectMutations, ProjectQueries};
use crate::IntoResponse;
use async_graphql::dataloader::DataLoader;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response;
use sqlx::PgPool;

mod estimate;
mod loaders;
mod project;
mod assembly;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ProjectQueries, EstimateQueries);

#[derive(MergedObject, Default)]
pub struct MutationRoot(ProjectMutations, EstimateMutations);

pub type GraphqlSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn schema(pg_pool: PgPool) -> GraphqlSchema {
    let project_loader = DataLoader::new(ProjectLoader::new(pg_pool.clone()), tokio::spawn);
    let estimate_loader = DataLoader::new(EstimateLoader::new(pg_pool.clone()), tokio::spawn);

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pg_pool)
    .data(project_loader)
    .data(estimate_loader)
    .finish()
}

pub async fn handler(schema: Extension<GraphqlSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
