use crate::postgres::Estimate;
use async_graphql::{InputObject, Object, SimpleObject, ID};

#[derive(Debug, InputObject)]
pub struct CreateEstimateInput {
    pub project_id: ID,
    pub description: String,
}

#[derive(SimpleObject)]
pub struct CreateEstimatePayload {
    pub estimate: Option<Estimate>,
}

#[derive(InputObject)]
pub struct DeleteEstimateInput {
    pub id: ID,
}

#[derive(SimpleObject)]
pub struct DeleteEstimatePayload {
    pub id: ID,
}

#[Object]
impl Estimate {
    async fn id(&self) -> ID {
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
