use crate::http::estimate_groups_item::resolver::EstimateGroupItem;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use async_trait::async_trait;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct GroupAssembliesLoader(PgPool);

impl GroupAssembliesLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<Uuid> for GroupAssembliesLoader {
    type Value = Vec<EstimateGroupItem>;
    type Error = FieldError;

    async fn load(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let items = sqlx::query_as!(
            EstimateGroupItem,
            // language=PostgreSQL
            r#"
            select id, group_id, assembly_id, quantity
            from estimate_group_items
            where group_id = any ($1)
            "#,
            ids // group ids
        )
        .fetch_all(&self.0)
        .await?;

        Ok(items
            .into_iter()
            .into_group_map_by(|line_item| line_item.group_id))
    }
}
