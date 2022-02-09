use crate::http::assembly_items::AssemblyItem;
use crate::http::assembly::Assembly;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;

#[Object]
impl Assembly {
    async fn id(&self) -> ID {
        ID::from(self.assembly_id)
    }

    async fn cost(&self) -> i32 {
        self.cost
    }

    async fn assembly(&self) -> String {
        self.assembly.to_string()
    }

    async fn items(&self, ctx: &Context<'_>) -> Result<Vec<AssemblyItem>> {
        let pool = ctx.data_unchecked::<PgPool>();

        let items = AssemblyItem::fetch_all(self.assembly_id, pool).await?;

        Ok(items)
    }
}
