use crate::postgres::assembly::Assembly;
use async_graphql::{Object, ID};

#[Object]
impl Assembly {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn assembly(&self) -> String {
        self.assembly.to_string()
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }
}
