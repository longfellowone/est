use crate::postgres::Estimate;
use async_graphql::{InputObject, Object, SimpleObject};

#[derive(Debug, InputObject)]
pub struct CreateEstimateInput {
    pub id: String,
    pub project_id: String,
    pub description: String,
}

#[derive(SimpleObject)]
pub struct EstimatePayload {
    pub estimate: Option<Estimate>,
}

#[Object]
impl Estimate {
    async fn id(&self) -> async_graphql::ID {
        self.id.into()
    }

    async fn description(&self) -> String {
        self.description.to_string()
    }

    // TODO: Calculate this every time, remove field from struct?
    async fn cost(&self) -> i32 {
        self.cost
    }

    // TODO: Add project to estimate?
}
