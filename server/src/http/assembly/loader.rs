use crate::http::assembly::Assembly;
use async_graphql::dataloader::Loader;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::FieldError;
use async_trait::async_trait;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct GroupItemLoader(PgPool);

impl GroupItemLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<Uuid> for GroupItemLoader {
    type Value = Assembly;
    type Error = FieldError;

    async fn load(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let assembly = sqlx::query_as!(
            Assembly,
            // language=PostgreSQL
            r#"
            select assembly_id, assembly
            from assembly
            where assembly_id = any ($1)
            "#,
            ids
        )
        .fetch(&self.0)
        .map_ok(|assembly: Assembly| (assembly.assembly_id, assembly))
        .try_collect()
        .await?;

        Ok(assembly)
    }
}
