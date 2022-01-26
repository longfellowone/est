use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Estimate {
    pub id: Uuid,
    pub project_id: Uuid,
    pub estimate: String,
    pub cost: i32,
}

impl Estimate {
    pub async fn fetch_all_for_project(
        project_id: Uuid,
        pg_pool: &PgPool,
    ) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            SELECT id, project_id, estimate, cost
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
            SELECT id, project_id, estimate, cost
            FROM estimate
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_in_project(ids: &[Uuid], pg_pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            SELECT id, project_id, estimate, cost
            FROM estimate
            WHERE project_id = ANY($1)
            "#,
            ids,
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn create(estimate: Estimate, pg_pool: &PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            INSERT INTO estimate (id, project_id, estimate, cost) 
            VALUES ($1, $2, $3, $4)
            RETURNING id, project_id, estimate, cost
            "#,
            estimate.id,
            estimate.project_id,
            estimate.estimate,
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

    pub async fn add_assembly(
        estimate_id: Uuid,
        assembly_id: Uuid,
        pg_pool: &PgPool,
    ) -> Result<Self, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            WITH estimate_assemblies AS (
                INSERT INTO estimate_assemblies (estimate_id, assembly_id, quantity)
                VALUES ($1, $2, $3)
                RETURNING estimate_id
            )
            SELECT e.id as "id!",
                   e.project_id as "project_id!",
                   e.estimate as "estimate!",
                   e.cost as "cost!"
            FROM estimate_assemblies ea
            INNER JOIN estimate e on e.id = ea.estimate_id
            "#,
            estimate_id,
            assembly_id,
            0
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }
}
