// #![allow(dead_code)]
use crate::error::{sqlx_error, AppError};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EstimateAssembly {
    pub id: Uuid,
    pub assembly: String,
    pub cost: Decimal,
    pub quantity: i32,
}

impl EstimateAssembly {
    pub async fn fetch_all(
        estimate_id: Uuid,
        pg_pool: &PgPool,
    ) -> Result<Vec<EstimateAssembly>, AppError> {
        sqlx::query_as!(
            EstimateAssembly,
            r#"
            SELECT a.id, a.assembly, a.cost, ea.quantity
            FROM assembly a
            INNER JOIN estimate_assemblies ea on ea.assembly_id = a.id
            INNER JOIN estimate e on e.id = ea.estimate_id
            WHERE ea.estimate_id = $1
            ORDER BY a.cost
            "#,
            estimate_id
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }
}

// fn calculate_total_for_each_item(estimate_items: &[EstimateItem]) -> i32 {
//     let mut item_totals = HashMap::new();
//     // let mut item_costs = HashMap::new(); - or create a struct and store both
//
//     estimate_items.into_iter().for_each(|item| {
//         let total = item.assembly_quantity * item.item_quantity;
//
//         *item_totals.entry(item.item).or_insert(0) += total;
//
//         // Also insert into item_costs
//     });
//
//     let id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
//     println!("total for item: {:?}", item_totals.get(&id).unwrap());
//
//     0
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::estimating::item::{calculate_total_for_each_item, EstimateItem};
//     use uuid::Uuid;
//
//     fn estimate_items() -> Vec<EstimateItem> {
//         let item1 = EstimateItem {
//             assembly_quantity: 2,
//             item: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
//             item_quantity: 5,
//             price: 10.0,
//         };
//
//         let item2 = EstimateItem {
//             assembly_quantity: 2,
//             item: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
//             item_quantity: 5,
//             price: 10.0,
//         };
//
//         // let item3 = EstimateItem {
//         //     assembly_quantity: 0,
//         //     item: Default::default(),
//         //     item_quantity: 0,
//         //     price: 0.0
//         // };
//
//         vec![item1, item2]
//     }
//
//     #[test]
//     fn it_works() {
//         let estimate_items = estimate_items();
//
//         assert_eq!(
//             estimate_items[0].item,
//             Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
//         );
//
//         let total = calculate_total_for_each_item(&estimate_items);
//
//         assert_eq!(total, 200)
//     }
// }
