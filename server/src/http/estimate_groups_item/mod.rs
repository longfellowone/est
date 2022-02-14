use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

pub mod loader;
mod resolver;

#[derive(Debug, Clone)]
pub struct EstimateGroupItem {
    pub id: Uuid,
    pub group_id: Uuid,
    pub assembly_id: Uuid,
    pub quantity: i32,
}

// impl EstimateLineItem {
//     pub async fn fetch_in_estimate(
//         estimate_ids: &[Uuid],
//         pool: &PgPool,
//     ) -> Result<Vec<Self>, AppError> {
//         sqlx::query_as!(
//             EstimateAssembly,
//             // language=PostgreSQL
//             r#"
//             select ea.estimate_id as "estimate_id!",
//                    a.assembly_id as "assembly_id!",
//                    a.assembly as "assembly!",
//                    a.cost as "cost!",
//                    ea.quantity as "quantity!"
//             from assembly a
//             inner join estimate_assemblies ea using (assembly_id)
//             where estimate_id = ANY ($1)
//             -- TODO: How should this be ordered? by created at desc?
//             order by a.assembly_id
//             "#,
//             estimate_ids
//         )
//         .fetch_all(pool)
//         .await
//         .map_err(sqlx_error)
//     }
// }
