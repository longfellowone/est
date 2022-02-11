use uuid::Uuid;

pub mod loader;
pub mod queries;
mod resolver;

pub struct Product {
    pub product_id: Uuid,
    pub product: String,
    pub cost: i32,
}
