use async_graphql::{Object, ID};
use uuid::Uuid;

#[Object]
impl Product {
    async fn id(&self) -> ID {
        self.product_id.into()
    }

    async fn product(&self) -> String {
        self.product.to_owned()
    }

    async fn cost(&self) -> i32 {
        self.cost
    }

    async fn labour(&self) -> i32 {
        self.labour
    }
}

/// A Product
#[derive(Debug, Clone)]
pub struct Product {
    pub product_id: Uuid,
    pub product: String,
    /// Cost in cents
    pub cost: i32,
    /// Labour in minutes
    pub labour: i32,
}
