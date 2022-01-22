use crate::postgres::{Estimate, Project};
use async_graphql::Object;

#[Object]
impl Project {
    async fn id(&self) -> async_graphql::ID {
        self.id.into()
    }

    async fn project(&self) -> String {
        self.project.to_string()
    }

    async fn estimates(&self) -> Vec<Estimate> {
        println!("{:?}", self.id);

        let e1 = Estimate {
            id: Default::default(),
            description: "".to_string(),
            price: 100,
        };
        let e2 = Estimate {
            id: Default::default(),
            description: "".to_string(),
            price: 200,
        };

        vec![e1, e2]
    }
}
