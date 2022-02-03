use crate::http::project::Project;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Default)]
pub struct ProjectQueries;

#[Object]
impl ProjectQueries {
    async fn project(&self, ctx: &Context<'_>, id: ID) -> Result<Project> {
        let pool = ctx.data_unchecked::<PgPool>();
        let id = Uuid::parse_str(&id)?;

        let project = Project::fetch_one(id, pool).await?;

        Ok(project)
    }

    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let pool = ctx.data_unchecked::<PgPool>();
        let projects = Project::fetch_all(pool).await?;

        Ok(projects)
    }
}
