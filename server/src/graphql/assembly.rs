use crate::estimating::assembly_item::AssemblyItem;
use crate::estimating::Assembly;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[Object]
impl Assembly {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }

    async fn assembly(&self) -> String {
        self.assembly.to_string()
    }

    async fn items(&self, ctx: &Context<'_>) -> Result<Vec<AssemblyItem>> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let items = AssemblyItem::fetch_all(self.id, pg_pool).await?;

        Ok(items)
    }
}

#[derive(Default)]
pub struct AssemblyQueries;

#[Object]
impl AssemblyQueries {
    async fn assembly(&self, ctx: &Context<'_>, id: ID) -> Result<Assembly> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let id = Uuid::parse_str(&id)?;

        let assembly = Assembly::fetch_one(id, pg_pool).await?;

        Ok(assembly)
    }
}
