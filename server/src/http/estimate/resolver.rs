use crate::http::estimate::EstimateResolver;
use crate::http::estimate_group::EstimateGroup;
use crate::http::estimate_line_item::loader::EstimateAssembliesLoader;
use crate::http::estimate_line_item::EstimateLineItem;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object, Result, ID};
use sqlx::PgPool;

#[Object(name = "Estimate")]
impl EstimateResolver {
    async fn id(&self) -> ID {
        ID::from(self.estimate_id)
    }

    async fn estimate(&self) -> String {
        self.estimate.to_string()
    }

    async fn group(&self) -> Result<Vec<EstimateGroup>> {
        let group = EstimateGroup {
            group_id: Default::default(),
            group: "group 1".to_string(),
        };

        Ok(vec![group])
    }

    // // TODO: This needs to be loader
    // async fn cost(&self, ctx: &Context<'_>) -> Result<i64> {
    //     let pool = ctx.data_unchecked::<PgPool>();
    //
    //     let assemblies = sqlx::query!(
    //         // language=PostgreSQL
    //         r#"
    //         select ea.quantity, a.cost
    //         from estimate_assemblies ea
    //         inner join assembly a using (assembly_id)
    //         where ea.estimate_id = $1
    //         "#,
    //         self.estimate_id
    //     )
    //     .fetch_all(pool)
    //     .await?;
    //
    //     let total = assemblies.into_iter().fold(0, |total, assembly| {
    //         total + (assembly.quantity * assembly.cost) as i64
    //     });
    //
    //     // TODO: Delete this
    //     // let cost = EstimateItem::cost(self.estimate_id, pool).await?;
    //
    //     Ok(total)
    // }
}
