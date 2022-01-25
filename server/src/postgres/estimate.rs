use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Estimate {
    pub id: Uuid,
    pub project_id: Uuid,
    pub description: String,
    pub cost: i32,
}

impl Estimate {
    pub async fn fetch_all(project_id: Uuid, pg_pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            SELECT id, project_id, description, cost
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
            SELECT id, project_id, description, cost
            FROM estimate
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn create(estimate: Estimate, pg_pool: &PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            INSERT INTO estimate (id, project_id, description, cost) 
            VALUES ($1, $2, $3, $4)
            RETURNING id, project_id, description, cost
            "#,
            estimate.id,
            estimate.project_id,
            estimate.description,
            estimate.cost
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn delete(id: Uuid, pg_pool: &PgPool) -> Result<Uuid, AppError> {
        // TODO: Change to soft delete
        let result = sqlx::query!(
            r#"
            DELETE FROM estimate 
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
