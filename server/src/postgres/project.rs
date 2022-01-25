use crate::error::{sqlx_error, AppError};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub project: String,
}

impl Project {
    pub async fn fetch_all(pg_pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Project,
            r#"
            SELECT id, project
            From project
            "#
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_one(id: Uuid, pg_pool: &PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
            Project,
            r#"
            SELECT id, project
            FROM project
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn create(new_project: Project, pg_pool: &PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
            Project,
            r#"
            INSERT INTO project (id, project)
            VALUES ($1, $2)
            RETURNING *
            "#,
            new_project.id,
            new_project.project
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn delete(id: Uuid, pg_pool: &PgPool) -> Result<Uuid, AppError> {
        // TODO: Change to soft delete
        let result = sqlx::query!(
            r#"
            DELETE FROM project 
            WHERE id = $1
            "#,
            id
        )
        .execute(pg_pool)
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
