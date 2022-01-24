use crate::postgres::project::Project;
use crate::postgres::Estimate;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

pub struct QueryRoot;

// https://www.apollographql.com/blog/graphql/basics/designing-graphql-mutations/

#[Object]
impl QueryRoot {
    async fn project(&self, ctx: &Context<'_>, id: ID) -> Result<Project> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let id = Uuid::parse_str(&id).unwrap();

        let payload = Project::fetch_one(id, pg_pool).await?;

        Ok(payload)
    }

    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let projects = Project::fetch_all(pg_pool).await?;

        Ok(projects)
    }

    async fn estimate(&self, ctx: &Context<'_>, id: ID) -> Result<Estimate> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let id = Uuid::parse_str(&id).unwrap();

        let estimate = Estimate::fetch_one(id, pg_pool).await?;

        Ok(estimate)
    }
}
