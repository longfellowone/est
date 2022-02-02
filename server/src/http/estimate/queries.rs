use crate::http::estimate::Estimate;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Default)]
pub struct EstimateQueries;

#[Object]
impl EstimateQueries {
    async fn estimate(&self, ctx: &Context<'_>, id: ID) -> Result<Estimate> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let id = Uuid::parse_str(&id)?;

        let estimate = Estimate::fetch_one(id, pg_pool).await?;

        // TODO: Maybe load assemblies here, pass to EstimateResolver
        // (so cost can be calculated without loading from DB twice)
        // How would this work with Vec<Estimate> ?

        Ok(estimate)
    }
}
