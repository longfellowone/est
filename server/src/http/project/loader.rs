use crate::http::project::Project;
use async_graphql::dataloader::Loader;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::FieldError;
use async_trait::async_trait;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ProjectLoader(PgPool);

impl ProjectLoader {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

// TODO: Not used, left as example for future
#[async_trait]
impl Loader<Uuid> for ProjectLoader {
    type Value = Project;
    type Error = FieldError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        Ok(sqlx::query_as!(
            Project,
            // language=PostgreSQL
            r#"
            SELECT project_id, project
            FROM project
            WHERE project_id = ANY($1)
            "#,
            keys,
        )
        .fetch(&self.0)
        .map_ok(|project: Project| (project.project_id, project))
        .try_collect()
        .await?)
    }
}
