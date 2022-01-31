#![allow(dead_code)]
use crate::error::{sqlx_error, AppError};
use rust_decimal::Decimal;
// use serde::Deserialize;
// use std::collections::HashMap;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EstimateAssembly {
    pub id: Uuid,
    pub estimate_id: Uuid,
    pub assembly: String,
    pub cost: Decimal,
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
//
// #[derive(Debug, Deserialize)]
// struct EstimateItem {
//     assembly_quantity: u32,
//     item_id: Uuid,
//     item_quantity: u32,
//     price: f64,
// }
//
// fn calculate_estimate_total(estimate_items: &[EstimateItem]) -> i32 {
//     let mut item_totals = HashMap::new();
//
//     // use reduce here instead? nuke hashmap
//     estimate_items.iter().for_each(|item| {
//         let total = item.assembly_quantity * item.item_quantity;
//
//         *item_totals.entry(item.item_id).or_insert(0) += total;
//     });
//
//     let id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
//     println!("total for item: {:?}", item_totals.get(&id).unwrap());
//
//     // item_totals.reduce -> total
//     0
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::estimating::estimate_assembly::{calculate_estimate_total, EstimateItem};
//     use uuid::Uuid;
//
//     fn estimate_items() -> Vec<EstimateItem> {
//         let item1 = EstimateItem {
//             assembly_quantity: 2,
//             item_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
//             item_quantity: 5,
//             price: 10.0,
//         };
//
//         let item2 = EstimateItem {
//             assembly_quantity: 2,
//             item_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
//             item_quantity: 5,
//             price: 10.0,
//         };
//
//         vec![item1, item2]
//     }
//
//     #[test]
//     fn it_works() {
//         let estimate_items = estimate_items();
//
//         assert_eq!(
//             estimate_items[0].item_id,
//             Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
//         );
//
//         let total = calculate_estimate_total(&estimate_items);
//
//         assert_eq!(total, 200)
//     }
// }
