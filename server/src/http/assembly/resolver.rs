use crate::http::assembly::Assembly;
use crate::http::assembly_components::loader::AssemblyComponentLoader;
use crate::http::assembly_components::AssemblyComponent;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[Object]
impl Assembly {
    async fn id(&self) -> ID {
        ID::from(self.assembly_id)
    }

    async fn assembly(&self) -> String {
        self.assembly.to_string()
    }

    async fn components(&self, ctx: &Context<'_>) -> Result<Vec<AssemblyComponent>> {
        let components = ctx
            .data_unchecked::<DataLoader<AssemblyComponentLoader>>()
            .load_one(self.assembly_id)
            .await?;

        Ok(components.unwrap_or_default())
    }
}
