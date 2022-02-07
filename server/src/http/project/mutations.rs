use crate::http::project::Project;
use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Default)]
pub struct ProjectMutations;

#[Object]
impl ProjectMutations {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<CreateProjectPayload> {
        let pool = ctx.data_unchecked::<PgPool>();

        let project = Project {
            project_id: Uuid::new_v4(),
            project: input.project,
        };

        let project = Project::create(project, pool).await?;

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
        let pool = ctx.data_unchecked::<PgPool>();

        let id = Uuid::parse_str(&input.id)?;

        let id = Project::delete(id, pool).await?;

        let payload = DeleteProjectPayload { id: id.into() };

        Ok(payload)
    }

    async fn update_project(
        &self,
        ctx: &Context<'_>,
        input: UpdateProjectInput,
    ) -> Result<UpdateProjectPayload> {
        let pool = ctx.data_unchecked::<PgPool>();

        let project = Project {
            project_id: Uuid::parse_str(&input.id)?,
            project: input.project,
        };

        let project = Project::update(project, pool).await?;

        let payload = UpdateProjectPayload {
            project: Some(project),
        };

        Ok(payload)
    }
}

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

#[derive(InputObject)]
pub struct UpdateProjectInput {
    pub id: ID,
    pub project: String,
}

#[derive(SimpleObject)]
pub struct UpdateProjectPayload {
    project: Option<Project>,
}
