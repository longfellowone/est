use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Assembly {
    pub id: Uuid,
    pub assembly: String,
}

impl Assembly {
    pub async fn fetch_one(id: Uuid, pg_pool: &PgPool) -> Result<Assembly, AppError> {
        sqlx::query_as!(
            Assembly,
            r#"
            SELECT id, assembly
            FROM assembly
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }
}
