use crate::http::assembly_item::loader::AssemblyItemLoader;
use crate::http::assembly_item::AssemblyItem;
use crate::http::estimate_assembly::EstimateAssembly;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};

#[Object]
impl EstimateAssembly {
    async fn id(&self) -> ID {
        ID::from(self.assembly_id)
    }

    async fn assembly(&self) -> String {
        self.assembly.to_string()
    }

    async fn cost(&self) -> i32 {
        self.cost
        // let pool = ctx.data_unchecked::<PgPool>();
        //
        // // TODO: This needs to be loader
        // let items = sqlx::query!(
        //     // language=PostgreSQL
        //     r#"
        //     select ai.quantity, i.cost
        //     from item i
        //     inner join assembly_items ai using (item_id)
        //     where ai.assembly_id = $1
        //     "#,
        //     self.assembly_id
        // )
        // .fetch_all(pool)
        // .await?;
        //
        // let total = items
        //     .into_iter()
        //     .fold(0, |total, item| total + (item.quantity * item.cost));
        //
        // Ok(total)
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }

    async fn items(&self, ctx: &Context<'_>) -> Result<Vec<AssemblyItem>> {
        let result = ctx
            .data_unchecked::<DataLoader<AssemblyItemLoader>>()
            .load_one(self.assembly_id)
            .await?;

        Ok(result.unwrap_or_default())
    }
}
