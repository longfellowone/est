use crate::http::assembly::loader::GroupItemLoader;
use crate::http::assembly::resolver::Assembly;
use crate::http::assembly_components::loader::AssemblyComponentLoader;
use crate::http::assembly_components::resolver::AssemblyComponent;
use crate::http::item::Item;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EstimateGroupItem {
    pub id: Uuid,
    pub group_id: Uuid,
    pub assembly_id: Uuid,
    pub quantity: i32,
}

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
}
