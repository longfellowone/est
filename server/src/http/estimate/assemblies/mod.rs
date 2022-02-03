use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

pub mod loader;
mod resolver;

#[derive(Debug, Clone)]
pub struct EstimateAssembly {
    pub estimate_id: Uuid,
    pub assembly_id: Uuid,
    pub assembly: String,
    pub cost: i32,
    pub quantity: i32,
}

impl EstimateAssembly {
    pub async fn fetch_in_estimate(
        estimate_ids: &[Uuid],
        pool: &PgPool,
    ) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            EstimateAssembly,
            // language=PostgreSQL
            r#"
            SELECT ea.estimate_id as "estimate_id!", 
                   a.assembly_id as "assembly_id!",                    
                   a.assembly as "assembly!", 
                   a.cost as "cost!", 
                   ea.quantity as "quantity!"
            FROM assembly a
            INNER JOIN estimate_assemblies ea using (assembly_id)
            WHERE estimate_id = ANY ($1)
            "#,
            estimate_ids
        )
        .fetch_all(pool)
        .await
        .map_err(sqlx_error)
    }
}
