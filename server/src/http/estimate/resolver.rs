use crate::estimating::estimate_assembly::EstimateAssembly;
use crate::http::estimate::Estimate;
use crate::http::estimate::EstimateItem;
use crate::http::loaders::EstimateAssembliesLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;

#[Object]
impl Estimate {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }

    async fn estimate(&self) -> String {
        self.estimate.to_string()
    }

    async fn cost(&self, ctx: &Context<'_>) -> Result<i64> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let cost = EstimateItem::cost(self.id, pg_pool).await?;

        Ok(cost)
    }

    async fn assemblies(&self, ctx: &Context<'_>) -> Result<Vec<EstimateAssembly>> {
        let result = ctx
            .data_unchecked::<DataLoader<EstimateAssembliesLoader>>()
            .load_one(self.id)
            .await?;

        Ok(result.unwrap_or_default())
    }
}
