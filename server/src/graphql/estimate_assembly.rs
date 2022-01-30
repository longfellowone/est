use crate::estimating::assembly_item::AssemblyItem;
use crate::estimating::EstimateAssembly;
use crate::graphql::loaders::AssemblyItemLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
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

    async fn items(&self, ctx: &Context<'_>) -> Result<Vec<AssemblyItem>> {
        let result = ctx
            .data_unchecked::<DataLoader<AssemblyItemLoader>>()
            .load_one(self.id)
            .await?;

        Ok(result.unwrap_or_default())
    }
}