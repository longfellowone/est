use crate::http::estimate::assemblies::loader::EstimateAssembliesLoader;
use crate::http::estimate::assemblies::EstimateAssembly;
use crate::http::estimate::Estimate;
use crate::http::estimate::EstimateItem;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;

#[Object]
impl Estimate {
    async fn id(&self) -> ID {
        ID::from(self.estimate_id)
    }

    async fn estimate(&self) -> String {
        self.estimate.to_string()
    }

    async fn cost(&self, ctx: &Context<'_>) -> Result<i64> {
        let pool = ctx.data_unchecked::<PgPool>();

        let cost = EstimateItem::cost(self.estimate_id, pool).await?;

        Ok(cost)
    }

    async fn assemblies(&self, ctx: &Context<'_>) -> Result<Vec<EstimateAssembly>> {
        let result = ctx
            .data_unchecked::<DataLoader<EstimateAssembliesLoader>>()
            .load_one(self.estimate_id)
            .await?;

        Ok(result.unwrap_or_default())
    }
}
