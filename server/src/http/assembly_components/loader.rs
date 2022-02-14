use crate::http::assembly_components::AssemblyComponent;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use async_trait::async_trait;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct AssemblyComponentLoader(PgPool);

impl AssemblyComponentLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<Uuid> for AssemblyComponentLoader {
    type Value = Vec<AssemblyComponent>;
    type Error = FieldError;

    async fn load(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let assembly_components = sqlx::query_as!(
            AssemblyComponent,
            // language=PostgreSQL
            r#"
            select id,
                   assembly_id,
                   product_id,
                   quantity
            from assembly_component
            where assembly_id = any ($1)
            "#,
            ids
        )
        .fetch_all(&self.0)
        .await?;

        Ok(assembly_components
            .into_iter()
            .into_group_map_by(|assembly_component| assembly_component.assembly_id))
    }
}
