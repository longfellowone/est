use uuid::Uuid;

pub mod loader;
mod resolver;

#[derive(Debug)]
pub struct EstimateGroup {
    pub group_id: Uuid,
    pub name: String,
}
