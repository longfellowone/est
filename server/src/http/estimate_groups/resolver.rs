use crate::http::estimate_groups_item::loader::GroupAssembliesLoader;
use crate::http::estimate_groups_item::resolver::EstimateGroupLineItem;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EstimateGroup {
    pub group_id: Uuid,
    pub estimate_id: Uuid,
    pub name: String,
}

#[Object]
impl EstimateGroup {
    async fn id(&self) -> ID {
        self.group_id.into()
    }

    async fn group(&self) -> String {
        self.name.to_owned()
    }

    async fn line_items(&self, ctx: &Context<'_>) -> Result<Vec<EstimateGroupLineItem>> {
        let items = ctx
            .data_unchecked::<DataLoader<GroupAssembliesLoader>>()
            .load_one(self.group_id)
            .await?;

        Ok(items.unwrap_or_default())
    }
}
