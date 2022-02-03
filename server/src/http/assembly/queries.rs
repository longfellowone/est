use crate::http::assembly::Assembly;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Default)]
pub struct AssemblyQueries;

#[Object]
impl AssemblyQueries {
    async fn assembly(&self, ctx: &Context<'_>, id: ID) -> Result<Assembly> {
        let pool = ctx.data_unchecked::<PgPool>();

        let id = Uuid::parse_str(&id)?;

        let assembly = Assembly::fetch_one(id, pool).await?;

        Ok(assembly)
    }
}
