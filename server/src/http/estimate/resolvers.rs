use crate::estimating::estimate::EstimateItem;
use crate::estimating::estimate_assembly::EstimateAssembly;
use crate::estimating::Estimate;
use crate::http::loaders::EstimateAssembliesLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[Object]
impl Estimate {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }

    async fn estimate(&self) -> String {
        self.estimate.to_string()
    }

    async fn cost(&self, ctx: &Context<'_>) -> Result<i64> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let cost = EstimateItem::cost(self.id, pg_pool).await?;

        Ok(cost)
    }

    async fn assemblies(&self, ctx: &Context<'_>) -> Result<Vec<EstimateAssembly>> {
        let result = ctx
            .data_unchecked::<DataLoader<EstimateAssembliesLoader>>()
            .load_one(self.id)
            .await?;

        Ok(result.unwrap_or_default())
    }
}

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

#[derive(Default)]
pub struct EstimateMutations;

#[Object]
impl EstimateMutations {
    async fn create_estimate(
        &self,
        ctx: &Context<'_>,
        input: CreateEstimateInput,
    ) -> Result<CreateEstimatePayload> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let estimate = Estimate {
            id: Uuid::new_v4(),
            project_id: Uuid::parse_str(&input.project_id)?,
            estimate: input.estimate,
        };

        let estimate = Estimate::create(estimate, pg_pool).await?;

        let payload = CreateEstimatePayload {
            estimate: Some(estimate),
        };

        Ok(payload)
    }

    async fn delete_estimate(
        &self,
        ctx: &Context<'_>,
        input: DeleteEstimateInput,
    ) -> Result<DeleteEstimatePayload> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let id = Uuid::parse_str(&input.id)?;

        let id = Estimate::delete(id, pg_pool).await?;

        let payload = DeleteEstimatePayload { id: id.into() };

        Ok(payload)
    }

    async fn add_assembly_to_estimate(
        &self,
        ctx: &Context<'_>,
        input: AddAssemblyToEstimateInput,
    ) -> Result<AddAssemblyToEstimatePayload> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let estimate_id = Uuid::parse_str(&input.estimate_id)?;
        let assembly_id = Uuid::parse_str(&input.assembly_id)?;

        let estimate = Estimate::add_assembly(estimate_id, assembly_id, pg_pool).await?;

        let payload = AddAssemblyToEstimatePayload {
            estimate: Some(estimate),
        };

        Ok(payload)
    }
}

#[derive(InputObject)]
pub struct CreateEstimateInput {
    pub project_id: ID,
    pub estimate: String,
}

#[derive(SimpleObject)]
pub struct CreateEstimatePayload {
    pub estimate: Option<Estimate>,
}

#[derive(InputObject)]
pub struct DeleteEstimateInput {
    pub id: ID,
}

#[derive(SimpleObject)]
pub struct DeleteEstimatePayload {
    pub id: ID,
}

#[derive(InputObject)]
pub struct AddAssemblyToEstimateInput {
    pub estimate_id: ID,
    pub assembly_id: ID,
}

#[derive(SimpleObject)]
pub struct AddAssemblyToEstimatePayload {
    pub estimate: Option<Estimate>,
}
