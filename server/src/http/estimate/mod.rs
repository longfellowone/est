use crate::error::{AppError, sqlx_error};
use sqlx::PgPool;
use uuid::Uuid;

pub mod loader;
pub mod mutations;
pub mod queries;
mod resolver;

#[derive(Debug, Clone)]
pub struct EstimateResolver {
    pub estimate_id: Uuid,
    pub project_id: Uuid,
    pub estimate: String,
}

impl EstimateResolver {
    #[allow(dead_code)]
    pub async fn fetch_all_for_project(
        project_id: Uuid,
        pool: &PgPool,
    ) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            EstimateResolver,
            // language=PostgreSQL
            r#"
            select estimate_id, project_id, estimate
            from estimate
            where project_id = $1
            "#,
            project_id
        )
        .fetch_all(pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_one(id: Uuid, pool: &PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
            EstimateResolver,
            // language=PostgreSQL
            r#"
            select estimate_id, project_id, estimate
            from estimate
            where estimate_id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn fetch_in_project(ids: &[Uuid], pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            EstimateResolver,
            // language=PostgreSQL
            r#"
            select estimate_id, project_id, estimate
            from estimate
            where project_id = any($1)
            "#,
            ids,
        )
        .fetch_all(pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn create(estimate: EstimateResolver, pool: &PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
            EstimateResolver,
            // language=PostgreSQL
            r#"
            insert into estimate (estimate_id, project_id, estimate) 
            values ($1, $2, $3)
            returning estimate_id, project_id, estimate
            "#,
            estimate.estimate_id,
            estimate.project_id,
            estimate.estimate,
        )
        .fetch_one(pool)
        .await
        .map_err(sqlx_error)
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<Uuid, AppError> {
        // TODO: Change to soft delete
        let result = sqlx::query!(
            // language=PostgreSQL
            r#"
            delete from estimate 
            where estimate_id = $1
            "#,
            id
        )
        .execute(pool)
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
        quantity: i32,
        pool: &PgPool,
    ) -> Result<Self, AppError> {
        sqlx::query_as!(
            EstimateResolver,
            // language=PostgreSQL
            r#"
            with insert as (
                insert into estimate_assemblies (estimate_id, assembly_id, quantity)
                values ($1, $2, $3)
--                 returning estimate_id
            )
            select e.estimate_id as "estimate_id!",
                   e.project_id as "project_id!",
                   e.estimate as "estimate!"
            from estimate e
            where e.estimate_id = $1
--             inner join insert i using (estimate_id)
            "#,
            estimate_id,
            assembly_id,
            quantity
        )
        .fetch_one(pool)
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
    // TODO: Delete this?

    pub async fn cost(estimate_id: Uuid, pool: &PgPool) -> Result<i64, AppError> {
        let estimate_items = sqlx::query_as!(
            EstimateItem,
            // language=PostgreSQL
            r#"
            select ea.quantity as "assembly_quantity",
                   ai.quantity as "item_quantity",
                   i.cost as "item_cost"
            from estimate e
            inner join estimate_assemblies ea using (estimate_id)
            inner join assembly_items ai using (assembly_id)
            inner join item i using (item_id)
            where e.estimate_id = $1
            "#,
            estimate_id
        )
        .fetch_all(pool)
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
