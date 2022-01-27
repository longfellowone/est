use crate::estimating::EstimateAssembly;
use async_graphql::{Object, ID};
use rust_decimal::prelude::ToPrimitive;

#[Object]
impl EstimateAssembly {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }

    async fn assembly(&self) -> String {
        self.assembly.to_string()
    }

    async fn cost(&self) -> f64 {
        self.cost.round_dp(2).to_f64().unwrap()
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }
}
