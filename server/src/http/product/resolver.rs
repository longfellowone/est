use crate::http::product::Product;
use async_graphql::{Object, ID};

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
