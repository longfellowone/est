use crate::estimating::Project;
use async_graphql::dataloader::Loader;
use async_graphql::futures_util::TryStreamExt;
use async_trait::async_trait;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct ProjectLoader(PgPool);

impl ProjectLoader {
    pub(crate) fn new(pg_pool: PgPool) -> Self {
        Self(pg_pool)
    }
}

// TODO: Not used, left as example for future
#[async_trait]
impl Loader<Uuid> for ProjectLoader {
    type Value = Project;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        Ok(sqlx::query_as!(
            Project,
            r#"
            SELECT id, project
            FROM project
            WHERE id = ANY($1)
            "#,
            keys,
        )
        .fetch(&self.0)
        .map_ok(|project: Project| (project.id, project))
        .try_collect()
        .await?)
    }
}
