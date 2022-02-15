use crate::http::estimate_groups::resolver::EstimateGroup;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use async_trait::async_trait;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct EstimateGroupsLoader(PgPool);

impl EstimateGroupsLoader {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<Uuid> for EstimateGroupsLoader {
    type Value = Vec<EstimateGroup>;
    type Error = FieldError;

    async fn load(&self, estimate_ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let groups = sqlx::query_as!(
            EstimateGroup,
            r#"
            select group_id, estimate_id, name
            from estimate_groups
            where estimate_id = any ($1)
            "#,
            estimate_ids
        )
        .fetch_all(&self.0)
        .await?;

        Ok(groups
            .into_iter()
            .into_group_map_by(|group| group.estimate_id))
    }
}
