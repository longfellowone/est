#![allow(dead_code)]
use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EstimateAssembly {
    pub id: Uuid,
    pub estimate_id: Uuid,
    pub assembly: String,
    pub cost: i32,
    pub quantity: i32,
}

impl EstimateAssembly {
    pub async fn fetch_in_estimate(
        estimate_ids: &[Uuid],
        pg_pool: &PgPool,
    ) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            EstimateAssembly,
            r#"
            SELECT a.id as "id!", 
                   ea.estimate_id as "estimate_id!", 
                   a.assembly as "assembly!", 
                   a.cost as "cost!", 
                   ea.quantity as "quantity!"
            FROM assembly a
            INNER JOIN estimate_assemblies ea on ea.assembly_id = a.id
            WHERE estimate_id = ANY ($1)
            "#,
            estimate_ids
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }
}
