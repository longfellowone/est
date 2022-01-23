use crate::error::AppError;
use crate::graphql::estimate::{CreateEstimateInput, EstimatePayload};
use crate::graphql::project::{CreateProjectInput, DeleteProjectInput, ProjectPayload};
use crate::postgres::{Estimate, Project};
use async_graphql::{Context, Object, Result};
use sqlx::PgPool;
use uuid::Uuid;

pub struct MutationRoot;

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
    ) -> Result<EstimatePayload, AppError> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let estimate = Estimate {
            id: Uuid::parse_str(&input.id).unwrap(),
            description: input.description,
            cost: 0,
        };

        let project_id = Uuid::parse_str(&input.project_id).unwrap();

        let payload = EstimatePayload {
            estimate: Some(Estimate::create(estimate, project_id, pg_pool).await?),
        };

        Ok(payload)
    }

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<ProjectPayload> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let project = Project {
            id: Uuid::parse_str(&input.id).unwrap(),
            project: input.project,
        };

        let payload = ProjectPayload {
            project: Some(Project::create(project, pg_pool).await?),
        };

        Ok(payload)
    }

    // async fn delete_project(
    //     &self
    //     ctx: &Context<'_>,
    //     input: DeleteProjectInput,
    // ) -> Result<>
}
