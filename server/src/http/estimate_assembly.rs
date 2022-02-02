use crate::estimating::assembly_item::AssemblyItem;
use crate::estimating::EstimateAssembly;
use crate::http::loaders::AssemblyItemLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;

#[Object]
impl EstimateAssembly {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }

    async fn assembly(&self) -> String {
        self.assembly.to_string()
    }

    async fn cost(&self, ctx: &Context<'_>) -> Result<i32> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        // TODO: This needs to be loader
        let items = sqlx::query!(
            // language=PostgreSQL
            r#"
            select ai.quantity, i.cost
            from item i
            inner join assembly_items ai on ai.item_id = i.id
            where ai.assembly_id = $1
            "#,
            self.id
        )
        .fetch_all(pg_pool)
        .await?;

        let total = items
            .into_iter()
            .fold(0, |total, item| total + (item.quantity * item.cost));

        Ok(total)
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
