use crate::http::product::resolver::Product;
use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use uuid::Uuid;

#[derive(Default)]
pub struct ProductMutations;

#[Object]
impl ProductMutations {
    async fn update_product(
        &self,
        ctx: &Context<'_>,
        input: UpdateProductInput,
    ) -> Result<UpdateProductPayload> {
        let product = Product {
            product_id: Uuid::parse_str(&input.product_id).unwrap(),
            product: input.product.unwrap_or("product 3 - updated".to_string()),
            cost: input.cost.unwrap(),
            labour: input.labour.unwrap(),
        };

        let payload = UpdateProductPayload {
            product: Some(product),
        };

        Ok(payload)
    }
}

#[derive(InputObject)]
struct UpdateProductInput {
    product_id: ID,
    product: Option<String>,
    cost: Option<i32>,
    labour: Option<i32>,
}

#[derive(SimpleObject)]
struct UpdateProductPayload {
    product: Option<Product>,
}
