use crate::http::assembly::items::AssemblyItem;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use async_trait::async_trait;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct AssemblyItemLoader(PgPool);

impl AssemblyItemLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<Uuid> for AssemblyItemLoader {
    type Value = Vec<AssemblyItem>;
    type Error = FieldError;

    async fn load(&self, assembly_ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let assembly_items = AssemblyItem::fetch_in_assembly(assembly_ids, &self.0).await?;

        Ok(assembly_items
            .into_iter()
            .into_group_map_by(|item| item.assembly_id))
    }
}
