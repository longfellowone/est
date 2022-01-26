use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Assembly {
    pub id: Uuid,
    pub assembly: String,
    pub quantity: i32,
}

impl Assembly {
    pub async fn fetch_all_for_estimate(
        estimate_id: Uuid,
        pg_pool: &PgPool,
    ) -> Result<Vec<Assembly>, AppError> {
        sqlx::query_as!(
            Assembly,
            r#"
            SELECT a.id, a.assembly, ea.quantity
            FROM assembly a
            INNER JOIN estimate_assemblies ea on ea.assembly_id = a.id
            INNER JOIN estimate e on e.id = ea.estimate_id
            WHERE ea.estimate_id = $1
            "#,
            estimate_id
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }
}
