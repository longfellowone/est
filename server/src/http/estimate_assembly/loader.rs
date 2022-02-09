use crate::http::estimate_assembly::EstimateAssembly;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use async_trait::async_trait;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct EstimateAssembliesLoader(PgPool);

impl EstimateAssembliesLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<Uuid> for EstimateAssembliesLoader {
    type Value = Vec<EstimateAssembly>;
    type Error = FieldError;

    async fn load(&self, estimate_ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let estimate_assemblies =
            EstimateAssembly::fetch_in_estimate(estimate_ids, &self.0).await?;

        Ok(estimate_assemblies
            .into_iter()
            .into_group_map_by(|assemlby| assemlby.estimate_id))
    }
}
