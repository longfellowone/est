use crate::http::assembly_item::AssemblyItem;
use async_graphql::{Object, ID};

#[Object]
impl AssemblyItem {
    async fn id(&self) -> ID {
        ID::from(self.item_id)
    }

    async fn item(&self) -> String {
        self.item.to_string()
    }

    async fn cost(&self) -> i32 {
        self.cost
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }
}
