use crate::error::AppError;
use crate::http::assembly_components::loader::AssemblyComponentLoader;
use crate::http::assembly_components::resolver::AssemblyComponent;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;
use std::result;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Assembly {
    pub assembly_id: Uuid,
    pub assembly: String,
}

#[Object]
impl Assembly {
    async fn id(&self) -> ID {
        ID::from(self.assembly_id)
    }

    async fn assembly(&self) -> String {
        self.assembly.to_string()
    }

    async fn components(&self, ctx: &Context<'_>) -> Result<Vec<AssemblyComponent>> {
        let components = ctx
            .data_unchecked::<DataLoader<AssemblyComponentLoader>>()
            .load_one(self.assembly_id)
            .await?;

        Ok(components.unwrap_or_default())
    }
}

impl Assembly {
    pub async fn fetch_one(id: Uuid, pool: &PgPool) -> result::Result<Assembly, AppError> {
        unimplemented!()
        // sqlx::query_as!(
        //     Assembly,
        //     // language=PostgreSQL
        //     r#"
        //     select assembly_id, assembly, cost
        //     from assembly
        //     where assembly_id = $1
        //     "#,
        //     id
        // )
        // .fetch_one(pool)
        // .await
        // .map_err(sqlx_error)
    }
}
