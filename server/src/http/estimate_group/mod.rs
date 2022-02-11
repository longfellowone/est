use uuid::Uuid;

mod resolver;

#[derive(Debug)]
pub struct EstimateGroup {
    pub group_id: Uuid,
    pub group: String,
}
