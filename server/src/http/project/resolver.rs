use crate::error::{sqlx_error, AppError};
use crate::http::estimate::loader::EstimateLoader;
use crate::http::estimate::resolver::Estimate;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::result;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Project {
    pub project_id: Uuid,
    pub project: String,
}

#[Object]
impl Project {
    async fn id(&self) -> ID {
        ID::from(self.project_id)
    }

    async fn project(&self) -> String {
        self.project.to_string()
    }

    async fn estimates(&self, ctx: &Context<'_>) -> Result<Vec<Estimate>> {
        let result = ctx
            .data_unchecked::<DataLoader<EstimateLoader>>()
            .load_one(self.project_id)
            .await?;

        Ok(result.unwrap_or_default())
    }
}

impl Project {
    pub async fn fetch_all(pool: &PgPool) -> result::Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Project,
            // language=PostgreSQL
            r#"
            select project_id, project
            from project
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_one(id: Uuid, pool: &PgPool) -> result::Result<Self, AppError> {
        sqlx::query_as!(
            Project,
            // language=PostgreSQL
            r#"
            select project_id, project
            from project
            where project_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn create(new_project: Project, pool: &PgPool) -> result::Result<Self, AppError> {
        sqlx::query_as!(
            Project,
            // language=PostgreSQL
            r#"
            insert into project (project_id, project)
            values ($1, $2)
            returning project_id, project
            "#,
            new_project.project_id,
            new_project.project
        )
        .fetch_one(pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn update(project: Project, pool: &PgPool) -> result::Result<Self, AppError> {
        sqlx::query_as!(
            Project,
            // language=PostgreSQL
            r#"
            update project
            set project = coalesce($2, project.project)
            where project_id = $1
            returning project_id, project
            "#,
            project.project_id,
            project.project,
        )
        .fetch_one(pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> result::Result<Uuid, AppError> {
        // TODO: Change to soft delete
        let result = sqlx::query!(
            // language=PostgreSQL
            r#"
            delete from project 
            where project_id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(sqlx_error);

        // TODO: Improve this? - Return deleted status from soft delete
        if let Ok(query) = result {
            if query.rows_affected() == 0 {
                return Err(AppError::BadRequest);
            }
        }

        Ok(id)
    }
}
