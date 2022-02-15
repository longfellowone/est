use crate::http::estimate_groups::loader::EstimateGroupsLoader;
use crate::http::estimate_groups::resolver::EstimateGroup;
use crate::http::estimate_groups_item::loader::GroupAssembliesLoader;
use crate::http::estimate_groups_item::resolver::EstimateGroupLineItem;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Estimate {
    pub estimate_id: Uuid,
    // pub project_id: Uuid,
    pub estimate: String,
}

// #[Object(name = "Estimate")]
#[Object]
impl Estimate {
    async fn id(&self) -> ID {
        ID::from(self.estimate_id)
    }

    async fn estimate(&self) -> String {
        self.estimate.to_string()
    }

    async fn groups(&self, ctx: &Context<'_>) -> Result<Vec<EstimateGroup>> {
        let groups = ctx
            .data_unchecked::<DataLoader<EstimateGroupsLoader>>()
            .load_one(self.estimate_id)
            .await?;

        Ok(groups.unwrap_or_default())
    }
}

//     let total = assemblies.into_iter().fold(0, |total, assembly| {
//         total + (assembly.quantity * assembly.cost) as i64
//     });
