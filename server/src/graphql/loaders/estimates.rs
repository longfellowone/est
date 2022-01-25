use crate::postgres::Estimate;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use async_trait::async_trait;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
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
    type Error = FieldError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let estimates = Estimate::fetch_in_project(keys, &self.0).await?;

        Ok(estimates
            .into_iter()
            .into_group_map_by(|estimate| estimate.project_id))
    }
}
