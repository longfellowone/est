use crate::error::AppError;
use crate::postgres::{Estimate, Project};
use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use sqlx::PgPool;

#[derive(InputObject)]
pub struct CreateProjectInput {
    pub project: String,
}

#[derive(SimpleObject)]
pub struct CreateProjectPayload {
    pub project: Option<Project>,
}

#[derive(InputObject)]
pub struct DeleteProjectInput {
    pub id: ID,
}

#[derive(SimpleObject)]
pub struct DeleteProjectPayload {
    pub id: ID,
}

#[Object]
impl Project {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn project(&self) -> String {
        self.project.to_string()
    }

    async fn estimates(&self, ctx: &Context<'_>) -> Result<Vec<Estimate>, AppError> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        // TODO: Create a loader for estimates
        let estimate = Estimate::fetch_all(self.id, pg_pool).await;

        estimate
    }
}
