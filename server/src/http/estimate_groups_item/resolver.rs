use crate::http::assembly::loader::GroupItemLoader;
use crate::http::assembly::resolver::Assembly;
use crate::http::assembly_components::loader::AssemblyComponentLoader;
use crate::http::assembly_components::resolver::AssemblyComponent;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use chrono::format::Item;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EstimateGroupLineItem {
    pub id: Uuid,
    pub group_id: Uuid,
    pub assembly_id: Uuid,
    pub quantity: i32,
}

#[Object]
impl EstimateGroupLineItem {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }

    async fn assembly(&self, ctx: &Context<'_>) -> Result<Assembly> {
        // TODO: Select all in, if Some(assembly) return Assembly, if Some(product) return Product
        // Return assembly of 1 item if product

        let item = ctx
            .data_unchecked::<DataLoader<GroupItemLoader>>()
            .load_one(self.assembly_id)
            .await?;

        Ok(item.unwrap())
    }
}
