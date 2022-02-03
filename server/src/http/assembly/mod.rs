pub mod items;
pub mod mutations;
pub mod queries;
mod resolver;

use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Assembly {
    pub assembly_id: Uuid,
    pub assembly: String,
}

impl Assembly {
    pub async fn fetch_one(id: Uuid, pool: &PgPool) -> Result<Assembly, AppError> {
        sqlx::query_as!(
            Assembly,
            // language=PostgreSQL
            r#"
            select assembly_id, assembly
            from assembly
            where assembly_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(sqlx_error)
    }
}
