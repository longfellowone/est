use crate::error::{sqlx_error, AppError};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AssemblyItem {
    pub id: Uuid,
    pub assembly_id: Uuid,
    pub item: String,
    pub cost: Decimal,
    pub quantity: i32,
}

impl AssemblyItem {
    pub async fn fetch_all(assembly_id: Uuid, pg_pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            AssemblyItem,
            r#"
            SELECT i.id, ai.assembly_id, i.item, i.cost, ai.quantity
            FROM item i
            INNER JOIN assembly_items ai on ai.item_id = i.id
            INNER JOIN assembly a on a.id = ai.assembly_id
            WHERE a.id = $1
            "#,
            assembly_id
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_in_assembly(ids: &[Uuid], pg_pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            AssemblyItem,
            r#"
            SELECT i.id as "id!", 
                   ai.assembly_id as "assembly_id!",
                   i.item as "item!", 
                   i.cost as "cost!", 
                   ai.quantity as "quantity!"
            FROM item i
            INNER JOIN assembly_items ai on i.id = ai.item_id
            WHERE assembly_id = ANY ($1)
            "#,
            ids
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }
}
