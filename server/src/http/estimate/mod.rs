pub mod loader;
pub mod mutations;
pub mod queries;
pub mod resolver;

// #[derive(Debug, serde::Deserialize)]
// pub struct EstimateItem {
//     assembly_quantity: i32,
//     item_quantity: i32,
//     item_cost: i32,
// }
//
// impl EstimateItem {
//     // TODO: Delete this?
//
//     pub async fn cost(estimate_id: Uuid, pool: &PgPool) -> Result<i64, AppError> {
//         let estimate_items = sqlx::query_as!(
//             EstimateItem,
//             // language=PostgreSQL
//             r#"
//             select ea.quantity as "assembly_quantity",
//                    ai.quantity as "item_quantity",
//                    i.cost as "item_cost"
//             from estimate e
//             inner join estimate_assemblies ea using (estimate_id)
//             inner join assembly_component ai using (assembly_id)
//             inner join item i using (product_id)
//             where e.estimate_id = $1
//             "#,
//             estimate_id
//         )
//         .fetch_all(pool)
//         .await
//         .map_err(sqlx_error)?;
//
//         let total = calculate_estimate_total(&estimate_items);
//
//         Ok(total)
//     }
// }
//
// fn calculate_estimate_total(estimate_items: &[EstimateItem]) -> i64 {
//     estimate_items.into_iter().fold(0, |total, item| {
//         total + (item.assembly_quantity * item.item_quantity * item.item_cost) as i64
//     })
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::http::estimate::{calculate_estimate_total, EstimateItem};
//
//     fn estimate_items() -> Vec<EstimateItem> {
//         let item1 = EstimateItem {
//             assembly_quantity: 2,
//             item_quantity: 5,
//             item_cost: 10,
//         };
//
//         let item2 = EstimateItem {
//             assembly_quantity: 2,
//             item_quantity: 5,
//             item_cost: 10,
//         };
//
//         vec![item1, item2]
//     }
//
//     #[test]
//     fn test_estimate_total_is_correct() {
//         let estimate_items = estimate_items();
//
//         let total = calculate_estimate_total(&estimate_items);
//
//         assert_eq!(total, 200)
//     }
// }
