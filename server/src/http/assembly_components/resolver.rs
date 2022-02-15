use crate::http::product::loader::ProductLoader;
use crate::http::product::resolver::Product;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AssemblyComponent {
    pub id: Uuid,
    pub assembly_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

#[Object]
impl AssemblyComponent {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }

    async fn product(&self, ctx: &Context<'_>) -> Result<Product> {
        let product = ctx
            .data_unchecked::<DataLoader<ProductLoader>>()
            .load_one(self.product_id)
            .await?;

        Ok(product.unwrap())
    }
}
