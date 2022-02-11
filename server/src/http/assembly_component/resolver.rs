use crate::http::assembly_component::AssemblyComponent;
use crate::http::product::Product;
use async_graphql::{Object, ID};

#[Object]
impl AssemblyComponent {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }

    async fn product(&self) -> Product {
        Product {
            product_id: Default::default(),
            product: "Product 1".to_string(),
            cost: 0,
        }
    }
}
