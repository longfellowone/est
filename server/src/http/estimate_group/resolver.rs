use crate::http::estimate_group::EstimateGroup;
use crate::http::estimate_line_item::EstimateLineItem;
use async_graphql::{Context, Object, Result, ID};

#[Object]
impl EstimateGroup {
    async fn id(&self) -> ID {
        self.group_id.into()
    }

    async fn group(&self) -> String {
        self.group.to_owned()
    }

    async fn line_items(&self, ctx: &Context<'_>) -> Result<Vec<EstimateLineItem>> {
        let line_item = EstimateLineItem {
            id: Default::default(),
            quantity: 0,
        };

        Ok(vec![line_item])

        // let result = ctx
        //     .data_unchecked::<DataLoader<EstimateAssembliesLoader>>()
        //     .load_one(self.estimate_id)
        //     .await?;
        //
        // Ok(result.unwrap_or_default())
    }
}
