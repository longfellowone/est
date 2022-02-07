use crate::http::estimate::loader::EstimateLoader;
use crate::http::estimate::EstimateResolver;
use crate::http::project::Project;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};

#[Object]
impl Project {
    async fn id(&self) -> ID {
        ID::from(self.project_id)
    }

    async fn project(&self) -> String {
        self.project.to_string()
    }

    async fn estimates(&self, ctx: &Context<'_>) -> Result<Vec<EstimateResolver>> {
        let result = ctx
            .data_unchecked::<DataLoader<EstimateLoader>>()
            .load_one(self.project_id)
            .await?;

        Ok(result.unwrap_or_default())
    }
}
