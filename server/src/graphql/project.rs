use crate::graphql::loaders::EstimateLoader;
use crate::postgres::{Estimate, Project};
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Default)]
pub struct ProjectQueries;

#[Object]
impl ProjectQueries {
    async fn project(&self, ctx: &Context<'_>, id: ID) -> Result<Project> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let id = Uuid::parse_str(&id)?;

        let project = Project::fetch_one(id, pg_pool).await?;

        Ok(project)
    }

    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let projects = Project::fetch_all(pg_pool).await?;

        Ok(projects)
    }
}

#[Object]
impl Project {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn project(&self) -> String {
        self.project.to_string()
    }

    async fn estimates(&self, ctx: &Context<'_>) -> Result<Vec<Estimate>> {
        let result = ctx
            .data_unchecked::<DataLoader<EstimateLoader>>()
            .load_one(self.id)
            .await?;

        Ok(result.unwrap_or_default())
    }
}

#[derive(Default)]
pub struct ProjectMutations;

#[Object]
impl ProjectMutations {
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

        let id = Uuid::parse_str(&input.id)?;

        let id = Project::delete(id, pg_pool).await?;

        let payload = DeleteProjectPayload { id: id.into() };

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
