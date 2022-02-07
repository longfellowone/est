use crate::http::estimate::EstimateResolver;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use async_trait::async_trait;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct EstimateLoader(PgPool);

impl EstimateLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<Uuid> for EstimateLoader {
    type Value = Vec<EstimateResolver>;
    type Error = FieldError;

    async fn load(&self, project_ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let estimates = EstimateResolver::fetch_in_project(project_ids, &self.0).await?;
        // TODO: Use sqlx::query! and map here so project_id is not needed in estimate resolver

        Ok(estimates
            .into_iter()
            .into_group_map_by(|estimate| estimate.project_id))
    }
}
