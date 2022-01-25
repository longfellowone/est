use crate::postgres::Estimate;
use async_graphql::dataloader::Loader;
use async_trait::async_trait;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct EstimateLoader(PgPool);

impl EstimateLoader {
    pub(crate) fn new(pg_pool: PgPool) -> Self {
        Self(pg_pool)
    }
}

#[async_trait]
impl Loader<Uuid> for EstimateLoader {
    type Value = Vec<Estimate>;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        // TODO: Check return type and move to postgres::Project

        let result = sqlx::query_as!(
            Estimate,
            r#"
            SELECT id, project_id, description, cost
            FROM estimate
            WHERE project_id = ANY($1)
            "#,
            keys,
        )
        .fetch_all(&self.0)
        .await?;

        let mut project_estimates = HashMap::new();

        for (project, estimates) in &result.into_iter().group_by(|estimate| estimate.project_id) {
            project_estimates
                .entry(project)
                .or_insert(estimates.collect::<Vec<Estimate>>());
        }

        Ok(project_estimates)
    }
}
