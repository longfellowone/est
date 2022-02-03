use crate::http::estimate::Estimate;
use async_graphql::{Context, InputObject, Object, SimpleObject, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Default)]
pub struct EstimateMutations;

#[Object]
impl EstimateMutations {
    async fn create_estimate(
        &self,
        ctx: &Context<'_>,
        input: CreateEstimateInput,
    ) -> async_graphql::Result<CreateEstimatePayload> {
        let pool = ctx.data_unchecked::<PgPool>();

        let estimate = Estimate {
            estimate_id: Uuid::new_v4(),
            project_id: Uuid::parse_str(&input.project_id)?,
            estimate: input.estimate,
        };

        let estimate = Estimate::create(estimate, pool).await?;

        let payload = CreateEstimatePayload {
            estimate: Some(estimate),
        };

        Ok(payload)
    }

    async fn delete_estimate(
        &self,
        ctx: &Context<'_>,
        input: DeleteEstimateInput,
    ) -> async_graphql::Result<DeleteEstimatePayload> {
        let pool = ctx.data_unchecked::<PgPool>();

        let id = Uuid::parse_str(&input.id)?;

        let id = Estimate::delete(id, pool).await?;

        let payload = DeleteEstimatePayload { id: id.into() };

        Ok(payload)
    }

    async fn add_assembly_to_estimate(
        &self,
        ctx: &Context<'_>,
        input: AddAssemblyToEstimateInput,
    ) -> async_graphql::Result<AddAssemblyToEstimatePayload> {
        let pool = ctx.data_unchecked::<PgPool>();

        let estimate_id = Uuid::parse_str(&input.estimate_id)?;
        let assembly_id = Uuid::parse_str(&input.assembly_id)?;

        let estimate = Estimate::add_assembly(estimate_id, assembly_id, pool).await?;

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
