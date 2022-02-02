use crate::error::{sqlx_error, AppError};
use sqlx::PgPool;
use uuid::Uuid;

pub mod loader;
pub mod mutations;
pub mod queries;
pub mod resolver;

#[derive(Debug, Clone)]
pub struct Estimate {
    pub id: Uuid,
    pub project_id: Uuid,
    pub estimate: String,
}

impl Estimate {
    pub async fn fetch_all_for_project(
        project_id: Uuid,
        pg_pool: &PgPool,
    ) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            SELECT id, project_id, estimate
            FROM estimate
            WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_one(id: Uuid, pg_pool: &PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            SELECT id, project_id, estimate
            FROM estimate
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_in_project(ids: &[Uuid], pg_pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            SELECT id, project_id, estimate
            FROM estimate
            WHERE project_id = ANY($1)
            "#,
            ids,
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn create(estimate: Estimate, pg_pool: &PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            INSERT INTO estimate (id, project_id, estimate) 
            VALUES ($1, $2, $3)
            RETURNING id, project_id, estimate
            "#,
            estimate.id,
            estimate.project_id,
            estimate.estimate,
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn delete(id: Uuid, pg_pool: &PgPool) -> Result<Uuid, AppError> {
        // TODO: Change to soft delete
        let result = sqlx::query!(
            r#"
            DELETE FROM estimate 
            WHERE id = $1
            "#,
            id
        )
        .execute(pg_pool)
        .await
        .map_err(sqlx_error);

        // TODO: Improve this? - Return deleted status from soft delete
        if let Ok(query) = result {
            if query.rows_affected() == 0 {
                return Err(AppError::BadRequest);
            }
        }

        Ok(id)
    }

    pub async fn add_assembly(
        estimate_id: Uuid,
        assembly_id: Uuid,
        pg_pool: &PgPool,
    ) -> Result<Self, AppError> {
        sqlx::query_as!(
            Estimate,
            r#"
            WITH insert AS (
                INSERT INTO estimate_assemblies (estimate_id, assembly_id, quantity)
                VALUES ($1, $2, $3)
                RETURNING estimate_id
            )
            SELECT e.id as "id!",
                   e.project_id as "project_id!",
                   e.estimate as "estimate!"
            FROM estimate e
            INNER JOIN insert i on i.estimate_id = e.id
            "#,
            estimate_id,
            assembly_id,
            0
        )
        .fetch_one(pg_pool)
        .await
        .map_err(sqlx_error)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct EstimateItem {
    assembly_quantity: i32,
    item_quantity: i32,
    item_cost: i32,
}

impl EstimateItem {
    pub async fn cost(estimate_id: Uuid, pg_pool: &PgPool) -> Result<i64, AppError> {
        let estimate_items = sqlx::query_as!(
            EstimateItem,
            r#"
            SELECT ea.quantity as "assembly_quantity",
                   ai.quantity as "item_quantity",
                   i.cost as "item_cost"
            FROM estimate e
            INNER JOIN estimate_assemblies ea on ea.estimate_id = e.id
            INNER JOIN assembly_items ai on ai.assembly_id = ea.assembly_id
            INNER JOIN item i on i.id = ai.item_id
            WHERE e.id = $1
            "#,
            estimate_id
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)?;

        let total = calculate_estimate_total(&estimate_items);

        Ok(total)
    }
}

fn calculate_estimate_total(estimate_items: &[EstimateItem]) -> i64 {
    estimate_items.into_iter().fold(0, |total, item| {
        total + (item.assembly_quantity * item.item_quantity * item.item_cost) as i64
    })
}

#[cfg(test)]
mod tests {
    use crate::http::estimate::{calculate_estimate_total, EstimateItem};

    fn estimate_items() -> Vec<EstimateItem> {
        let item1 = EstimateItem {
            assembly_quantity: 2,
            item_quantity: 5,
            item_cost: 10,
        };

        let item2 = EstimateItem {
            assembly_quantity: 2,
            item_quantity: 5,
            item_cost: 10,
        };

        vec![item1, item2]
    }

    #[test]
    fn test_estimate_total_is_correct() {
        let estimate_items = estimate_items();

        let total = calculate_estimate_total(&estimate_items);

        assert_eq!(total, 200)
    }
}
