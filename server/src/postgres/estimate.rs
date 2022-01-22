use uuid::Uuid;

pub struct Estimate {
    pub id: Uuid,
    pub description: String,
    pub price: i32,
}
