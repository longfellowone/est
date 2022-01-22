use crate::postgres::project::Project;
use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use uuid::Uuid;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn project(&self, ctx: &Context<'_>, id: String) -> Result<Project> {
        let id = Uuid::parse_str(&id).unwrap();

        println!("{:?}", id);

        let pg_pool = ctx.data_unchecked::<PgPool>();
        let project = Project::fetch_one(id, pg_pool).await?;

        Ok(project)
    }

    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let projects = Project::fetch_all(pg_pool).await.unwrap();

        Ok(projects)
    }
}
