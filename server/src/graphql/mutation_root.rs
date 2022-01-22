use crate::postgres::Project;
use async_graphql::Object;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn temp(&self) -> Project {
        println!("called");
        Project {
            id: Default::default(),
            project: "".to_string(),
        }
    }
}
