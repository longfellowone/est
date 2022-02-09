use crate::http::assembly::Assembly;
use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Default)]
pub struct AssemblyMutations;

#[Object]
impl AssemblyMutations {
    async fn add_item_to_assembly(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: AddItemToAssemblyInput,
    ) -> Result<AddItemToAssemblyPayload> {
        let pool = ctx.data_unchecked::<PgPool>();

        let id = Uuid::parse_str(&id).unwrap();
        let item_id = Uuid::parse_str(&input.item_id).unwrap();

        let mut tx = pool.begin().await?;

        sqlx::query!(
            // language=PostgreSQL
            r#"
            insert into assembly_items (assembly_id, item_id, quantity)
            values ($1, $2, $3)
            "#,
            id,
            item_id,
            input.quantity
        )
        .execute(&mut tx)
        .await?;

        let items = sqlx::query!(
            // language=PostgreSQL
            r#"
            select ai.quantity, i.cost
            from item i
            inner join assembly_items ai using (item_id)
            where ai.assembly_id = $1
            "#,
            id
        )
        .fetch_all(&mut tx)
        .await?;

        let total = items
            .into_iter()
            .fold(0, |total, item| total + (item.quantity * item.cost));

        let assembly = sqlx::query_as!(
            Assembly,
            // language=PostgreSQL
            r#"
            update assembly
            set cost = coalesce($2, assembly.cost)
            where assembly_id = $1
            returning assembly_id, assembly, cost
            "#,
            id,
            total,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        let payload = AddItemToAssemblyPayload {
            assembly: Some(assembly),
        };

        Ok(payload)
    }

    async fn update_assembly_item_quantity(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: UpdateAssemblyItemQuantityInput,
    ) -> Result<UpdateAssemblyItemQuantityPayload> {
        let pool = ctx.data_unchecked::<PgPool>();

        let assembly_id = Uuid::parse_str(&id).unwrap();
        let item_id = Uuid::parse_str(&input.item_id).unwrap();

        let mut tx = pool.begin().await?;

        tx.commit().await?;

        todo!()
    }
}

#[derive(InputObject)]
pub struct UpdateAssemblyItemQuantityInput {
    item_id: ID,
    quantity: i32,
}

#[derive(SimpleObject)]
pub struct UpdateAssemblyItemQuantityPayload {
    assembly: Option<Assembly>,
}

#[derive(InputObject)]
pub struct AddItemToAssemblyInput {
    item_id: ID,
    quantity: i32,
}

#[derive(SimpleObject)]
pub struct AddItemToAssemblyPayload {
    assembly: Option<Assembly>,
}
