use crate::postgres::Estimate;
use async_graphql::Object;

#[Object]
impl Estimate {
    async fn id(&self) -> async_graphql::ID {
        self.id.into()
    }

    async fn description(&self) -> String {
        self.description.to_string()
    }

    async fn cost(&self) -> i32 {
        self.price
    }
}
