use crate::error::AppError;
use crate::postgres::{Estimate, Project};
use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Default)]
pub struct EstimateQueries;

#[Object]
impl EstimateQueries {
    async fn estimate(&self, ctx: &Context<'_>, id: ID) -> Result<Estimate> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let id = Uuid::parse_str(&id).unwrap();

        println!("{:?}", id);

        let estimate = Estimate::fetch_one(id, pg_pool).await?;

        Ok(estimate)
    }
}

#[Object]
impl Estimate {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn description(&self) -> String {
        self.description.to_string()
    }

    // TODO: Calculate this every time, remove field from struct?
    async fn cost(&self) -> i32 {
        self.cost
    }

    async fn project(&self, ctx: &Context<'_>) -> Option<Project> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        match Project::fetch_one(self.id, pg_pool).await {
            Ok(estimate) => Some(estimate),
            Err(_) => None,
        }
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
    ) -> Result<CreateEstimatePayload, AppError> {
        let pg_pool = ctx.data_unchecked::<PgPool>();

        let estimate = Estimate {
            id: Uuid::new_v4(),
            description: input.description,
            cost: 0,
        };

        let project_id = Uuid::parse_str(&input.project_id).unwrap();

        let estimate = Estimate::create(estimate, project_id, pg_pool).await?;

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

        let id = Uuid::parse_str(&input.id).unwrap();

        let id = Estimate::delete(id, pg_pool).await?;

        let payload = DeleteEstimatePayload { id: id.into() };

        Ok(payload)
    }
}

#[derive(Debug, InputObject)]
pub struct CreateEstimateInput {
    pub project_id: ID,
    pub description: String,
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
