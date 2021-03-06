use crate::http::estimate::resolver::Estimate;
use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
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
    ) -> Result<CreateEstimatePayload> {
        todo!()
        // let pool = ctx.data_unchecked::<PgPool>();
        //
        // let estimate = Estimate {
        //     estimate_id: Uuid::new_v4(),
        //     project_id: Uuid::parse_str(&input.project_id)?,
        //     estimate: input.estimate,
        // };
        //
        // let estimate = Estimate::create(estimate, pool).await?;
        //
        // let payload = CreateEstimatePayload {
        //     estimate: Some(estimate),
        // };
        //
        // Ok(payload)
    }

    async fn delete_estimate(
        &self,
        ctx: &Context<'_>,
        input: DeleteEstimateInput,
    ) -> Result<DeleteEstimatePayload> {
        todo!()
        // let pool = ctx.data_unchecked::<PgPool>();
        //
        // let id = Uuid::parse_str(&input.id)?;
        //
        // let id = Estimate::delete(id, pool).await?;
        //
        // let payload = DeleteEstimatePayload { id: id.into() };
        //
        // Ok(payload)
    }

    async fn add_assembly_to_estimate(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: AddAssemblyToEstimateInput,
    ) -> Result<AddAssemblyToEstimatePayload> {
        todo!()
        // let pool = ctx.data_unchecked::<PgPool>();
        //
        // let id = Uuid::parse_str(&id)?;
        // let assembly_id = Uuid::parse_str(&input.assembly_id)?;
        // // TODO: Remove Option
        // let quantity = input.quantity.unwrap_or(1);
        //
        // let estimate = Estimate::add_assembly(id, assembly_id, quantity, pool).await?;
        //
        // let payload = AddAssemblyToEstimatePayload {
        //     estimate: Some(estimate),
        // };
        //
        // Ok(payload)
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
    pub assembly_id: ID,
    pub quantity: Option<i32>,
}

#[derive(SimpleObject)]
pub struct AddAssemblyToEstimatePayload {
    pub estimate: Option<Estimate>,
}
