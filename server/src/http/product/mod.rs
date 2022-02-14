use uuid::Uuid;

pub mod loader;
pub mod queries;
mod resolver;

/// A Product
#[derive(Debug, Clone)]
pub struct Product {
    pub product_id: Uuid,
    pub product: String,
    /// Cost in cents
    pub cost: i32,
    /// Labour in minutes
    pub labour: i32,
}
