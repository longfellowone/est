use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

pub mod loader;
mod resolver;

#[derive(Debug, Clone)]
pub struct AssemblyComponent {
    pub id: Uuid,
    pub quantity: i32,
}

// impl AssemblyItem {
//     pub async fn fetch_all(assembly_id: Uuid, pool: &PgPool) -> Result<Vec<Self>, AppError> {
//         sqlx::query_as!(
//             AssemblyItem,
//             // language=PostgreSQL
//             r#"
//             select i.product_id, ai.assembly_id, i.item, i.cost, ai.quantity
//             from item i
//             inner join assembly_component ai using (product_id)
//             where ai.assembly_id = $1
//             "#,
//             assembly_id
//         )
//         .fetch_all(pool)
//         .await
//         .map_err(sqlx_error)
//     }
//
//     pub async fn fetch_in_assembly(ids: &[Uuid], pool: &PgPool) -> Result<Vec<Self>, AppError> {
//         sqlx::query_as!(
//             AssemblyItem,
//             // language=PostgreSQL
//             r#"
//             select p.product_id as "item_id!",
//                    ac.assembly_id as "assembly_id!",
//                    p.item as "item!",
//                    p.cost as "cost!",
//                    ac.quantity as "quantity!"
//             from item p
//             inner join assembly_component ac using (product_id)
//             where assembly_id = any ($1)
//             "#,
//             ids
//         )
//         .fetch_all(pool)
//         .await
//         .map_err(sqlx_error)
//     }
// }
