use crate::http::estimate_assembly::loader::EstimateAssembliesLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::futures_util::TryStreamExt;
use async_graphql::{Context, Object, ID};
use uuid::Uuid;

#[derive(Default)]
pub struct TestingQueries;

#[Object]
impl TestingQueries {
    async fn test_estimate(&self, ctx: &Context<'_>, estimate_id: ID) -> TestEstimate {
        let id = Uuid::parse_str(&estimate_id).unwrap();
        let name = "Estimate Name".to_string();

        let result = ctx
            .data_unchecked::<DataLoader<EstimateAssembliesLoader>>()
            .load_one(id)
            .await
            .unwrap();

        // Use lookahead here, else return none for assemblies
        let assemblies = result
            .unwrap_or_default()
            .into_iter()
            .map(|assembly| TestAssembly {
                id: assembly.assembly_id,
                name: assembly.assembly,
                cost: assembly.cost,
                quantity: assembly.quantity,
            })
            .collect::<Vec<TestAssembly>>();

        let cost = assemblies.iter().fold(0, |total, assembly| {
            total + (assembly.quantity * assembly.cost) as u64
        });

        TestEstimate {
            id,
            name,
            cost,
            assemblies: Some(assemblies),
        }
    }
}

pub struct TestEstimate {
    id: Uuid,
    name: String,
    cost: u64,
    assemblies: Option<Vec<TestAssembly>>,
}

#[Object]
impl TestEstimate {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }

    async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn cost(&self) -> u64 {
        self.cost
    }

    async fn assemblies(&self) -> &[TestAssembly] {
        self.assemblies.as_ref().unwrap()
    }
}

#[derive(Clone)]
pub struct TestAssembly {
    id: Uuid,
    name: String,
    cost: i32,
    quantity: i32,
}

#[Object]
impl TestAssembly {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }

    async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn cost(&self) -> i32 {
        self.cost
    }

    async fn quantity(&self) -> i32 {
        self.quantity
    }

    async fn item(&self) -> String {
        "list of items".to_string()
    }
}

// pub struct TestItem {
//     id: Uuid,
//     item: String,
//     cost: i32,
//     quantity: i32,
// }
