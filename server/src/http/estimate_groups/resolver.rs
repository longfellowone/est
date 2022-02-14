use crate::http::estimate_groups::EstimateGroup;
use crate::http::estimate_groups_item::loader::GroupAssembliesLoader;
use crate::http::estimate_groups_item::EstimateGroupItem;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use uuid::Uuid;

#[Object]
impl EstimateGroup {
    async fn id(&self) -> ID {
        self.group_id.into()
    }

    async fn group(&self) -> String {
        self.name.to_owned()
    }

    async fn items(&self, ctx: &Context<'_>) -> Result<Vec<EstimateGroupItem>> {
        let items = ctx
            .data_unchecked::<DataLoader<GroupAssembliesLoader>>()
            .load_one(self.group_id)
            .await?;

        Ok(items.unwrap_or_default())
    }
}
