use crate::http::assembly::items::loader::AssemblyItemLoader;
use crate::http::assembly::queries::AssemblyQueries;
use crate::http::estimate::assemblies::loader::EstimateAssembliesLoader;
use crate::http::estimate::loader::EstimateLoader;
use crate::http::estimate::mutations::EstimateMutations;
use crate::http::estimate::queries::EstimateQueries;
use crate::http::project::loader::ProjectLoader;
use crate::http::project::mutations::ProjectMutations;
use crate::http::project::queries::ProjectQueries;
use async_graphql::dataloader::DataLoader;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response;
use axum::response::IntoResponse;
use sqlx::PgPool;

// Lookahead example
// https://cs.github.com/cthit/hubbit2/blob/40cd6541c9b9daa6c65198fe6a763b5d794e8dc0/backend/src/schema/stats.rs#L420

#[derive(MergedObject, Default)]
pub struct QueryRoot(ProjectQueries, EstimateQueries, AssemblyQueries);

#[derive(MergedObject, Default)]
pub struct MutationRoot(ProjectMutations, EstimateMutations);

pub type GraphqlSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn schema(pool: PgPool) -> GraphqlSchema {
    let project_loader = DataLoader::new(ProjectLoader::new(pool.clone()), tokio::spawn);
    let estimates_loader = DataLoader::new(EstimateLoader::new(pool.clone()), tokio::spawn);
    let assembly_items_loader =
        DataLoader::new(AssemblyItemLoader::new(pool.clone()), tokio::spawn);
    let estimate_assemblies_loader =
        DataLoader::new(EstimateAssembliesLoader::new(pool.clone()), tokio::spawn);

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool)
    .data(project_loader)
    .data(estimates_loader)
    .data(assembly_items_loader)
    .data(estimate_assemblies_loader)
    .finish()
}

pub async fn handler(schema: Extension<GraphqlSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
