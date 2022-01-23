use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Estimate {
    pub id: Uuid,
    pub description: String,
    pub cost: i32,
}

impl Estimate {
    pub async fn fetch_all(project_id: Uuid, pg_pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            SELECT id, description, cost
            FROM estimate
            WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_one(id: Uuid, pg_pool: &PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            SELECT id, description, cost
            FROM estimate
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn create(
        estimate: Estimate,
        project_id: Uuid,
        pg_pool: &PgPool,
    ) -> Result<Self, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            INSERT INTO estimate (id, project_id, description, cost) 
            VALUES ($1, $2, $3, $4)
            RETURNING id, description, cost
            "#,
            estimate.id,
            project_id,
            estimate.description,
            estimate.cost
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }
}
