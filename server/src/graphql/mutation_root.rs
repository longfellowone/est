use crate::error::AppError;
use crate::graphql::estimate::{
    CreateEstimateInput, CreateEstimatePayload, DeleteEstimateInput, DeleteEstimatePayload,
};
use crate::graphql::project::{
    CreateProjectInput, CreateProjectPayload, DeleteProjectInput, DeleteProjectPayload,
};
use crate::postgres::{Estimate, Project};
use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use uuid::Uuid;

pub struct MutationRoot;

// TODO: Add [UserError] to payload types

#[Object]
impl MutationRoot {
    async fn temp(&self) -> Project {
        println!("called");
        Project {
            id: Default::default(),
            project: "".to_string(),
        }
    }

    async fn create_estimate(
        &self,
        ctx: &Context<'_>,
        input: CreateEstimateInput,
    ) -> Result<CreateEstimatePayload, AppError> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let estimate = Estimate {
            id: Uuid::new_v4(),
            description: input.description,
            cost: 0,
        };

        let project_id = Uuid::parse_str(&input.project_id).unwrap();

        let estimate = Estimate::create(estimate, project_id, pg_pool).await?;

        let payload = CreateEstimatePayload {
            estimate: Some(estimate),
        };

        Ok(payload)
    }

    async fn delete_estimate(
        &self,
        ctx: &Context<'_>,
        input: DeleteEstimateInput,
    ) -> Result<DeleteEstimatePayload> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let id = Uuid::parse_str(&input.id).unwrap();

        let id = Estimate::delete(id, pg_pool).await?;

        let payload = DeleteEstimatePayload { id: id.into() };

        Ok(payload)
    }

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<CreateProjectPayload> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let project = Project {
            id: Uuid::new_v4(),
            project: input.project,
        };

        let project = Project::create(project, pg_pool).await?;

        let payload = CreateProjectPayload {
            project: Some(project),
        };

        Ok(payload)
    }

    async fn delete_project(
        &self,
        ctx: &Context<'_>,
        input: DeleteProjectInput,
    ) -> Result<DeleteProjectPayload> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let id = Uuid::parse_str(&input.id).unwrap();

        let id = Project::delete(id, pg_pool).await?;

        let payload = DeleteProjectPayload { id: id.into() };

        Ok(payload)
    }
}
