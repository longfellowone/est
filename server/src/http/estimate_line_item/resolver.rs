use crate::http::assembly::Assembly;
use crate::http::assembly_component::loader::AssemblyItemLoader;
use crate::http::assembly_component::AssemblyComponent;
use crate::http::estimate_line_item::EstimateLineItem;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};

#[Object]
impl EstimateLineItem {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }

    async fn assembly(&self) -> Assembly {
        Assembly {
            assembly_id: Default::default(),
            assembly: "Assembly 1".to_string(),
            cost: 0,
        }
    }

    // async fn components(&self, ctx: &Context<'_>) -> Result<Vec<AssemblyComponent>> {
    //     // let result = ctx
    //     //     .data_unchecked::<DataLoader<AssemblyItemLoader>>()
    //     //     .load_one(self.assembly_id)
    //     //     .await?;
    //     //
    //     // Ok(result.unwrap_or_default())
    // }

    // async fn cost(&self) -> i32 {
    //     self.cost
    //     // let pool = ctx.data_unchecked::<PgPool>();
    //     //
    //     // // TODO: This needs to be loader
    //     // let items = sqlx::query!(
    //     //     // language=PostgreSQL
    //     //     r#"
    //     //     select ai.quantity, i.cost
    //     //     from item i
    //     //     inner join assembly_items ai using (item_id)
    //     //     where ai.assembly_id = $1
    //     //     "#,
    //     //     self.assembly_id
    //     // )
    //     // .fetch_all(pool)
    //     // .await?;
    //     //
    //     // let total = items
    //     //     .into_iter()
    //     //     .fold(0, |total, item| total + (item.quantity * item.cost));
    //     //
    //     // Ok(total)
    // }
}
