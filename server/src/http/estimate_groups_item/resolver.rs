use crate::http::assembly::loader::GroupItemLoader;
use crate::http::assembly::Assembly;
use crate::http::assembly_components::loader::AssemblyComponentLoader;
use crate::http::assembly_components::AssemblyComponent;
use crate::http::estimate_groups_item::EstimateGroupItem;
use crate::http::item::Item;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use uuid::Uuid;

#[Object]
impl EstimateGroupItem {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }

    async fn item(&self, ctx: &Context<'_>) -> Result<Item> {
        // TODO: Select all in, if Some(assembly) return Assembly, if Some(product) return Product

        let item = ctx
            .data_unchecked::<DataLoader<GroupItemLoader>>()
            .load_one(self.assembly_id)
            .await?;

        Ok(Item::Assembly(item.unwrap()))
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
