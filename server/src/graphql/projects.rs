use crate::postgres::projects::Project;
use async_graphql::Object;

#[Object]
impl Project {
    async fn id(&self) -> async_graphql::ID {
        self.id.into()
    }

    async fn project(&self) -> String {
        self.project.to_string()
    }
}
