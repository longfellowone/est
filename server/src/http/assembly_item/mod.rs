use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

pub mod loader;
mod resolver;

#[derive(Debug, Clone)]
pub struct AssemblyItem {
    pub item_id: Uuid,
    pub assembly_id: Uuid,
    pub item: String,
    pub cost: i32,
    pub quantity: i32,
}

impl AssemblyItem {
    pub async fn fetch_all(assembly_id: Uuid, pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            AssemblyItem,
            // language=PostgreSQL
            r#"
            select i.item_id, ai.assembly_id, i.item, i.cost, ai.quantity
            from item i
            inner join assembly_items ai using (item_id)
            where ai.assembly_id = $1
            "#,
            assembly_id
        )
        .fetch_all(pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_in_assembly(ids: &[Uuid], pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            AssemblyItem,
            // language=PostgreSQL
            r#"
            select i.item_id as "item_id!", 
                   ai.assembly_id as "assembly_id!",
                   i.item as "item!", 
                   i.cost as "cost!", 
                   ai.quantity as "quantity!"
            from item i
            inner join assembly_items ai using (item_id)
            where assembly_id = any ($1)
            "#,
            ids
        )
        .fetch_all(pool)
        .await
        .map_err(sqlx_error)
    }
}
