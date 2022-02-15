use crate::http::product::resolver::Product;
use async_graphql::dataloader::Loader;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::FieldError;
use async_trait::async_trait;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ProductLoader(PgPool);

impl ProductLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<Uuid> for ProductLoader {
    type Value = Product;
    type Error = FieldError;

    async fn load(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let product = sqlx::query_as!(
            Product,
            // language=PostgreSQL
            r#"
            select product_id, product, cost, labour
            from product
            where product_id = any ($1)
            "#,
            ids,
        )
        .fetch(&self.0)
        .map_ok(|product: Product| (product.product_id, product))
        .try_collect()
        .await?;

        Ok(product)
    }
}
