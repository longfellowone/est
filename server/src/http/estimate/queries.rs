use crate::http::estimate::resolver::Estimate;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Default)]
pub struct EstimateQueries;

#[Object]
impl EstimateQueries {
    async fn estimate(&self, ctx: &Context<'_>, id: ID) -> Result<Estimate> {
        let pool = ctx.data_unchecked::<PgPool>();
        let id = Uuid::parse_str(&id)?;

        let estimate = sqlx::query_as!(
            Estimate,
            r#"
            select estimate_id, estimate
            from estimate
            where estimate_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        // TODO: Maybe load assemblies here, pass to EstimateResolver
        // (so cost can be calculated without loading from DB twice)
        // How would this work with Vec<Estimate> ?
        // https://docs.rs/async-graphql/2.9.2/async_graphql/context/struct.ContextBase.html#method.look_ahead

        Ok(estimate)
    }

    // TODO: Create estimatesByProject
}
