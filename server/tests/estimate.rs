mod common;

#[cfg(test)]
mod tests {
    use crate::common::{TestApp, Vars};
    use reqwest_graphql::Client;
    use serde_json::Value;
    use server::error::AppError;
    use server::postgres::Estimate;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_estimates_query() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            query {
                project(id: "00000000-0000-0000-0000-000000000001") {
                    estimates {
                        id
                        estimate
                        cost
                    }
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
            "project": {
                "estimates": [
                    {
                        "id": "00000000-0000-0000-0000-000000000001",
                        "estimate": "Estimate 1",
                        "cost": 100,
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000002",
                        "estimate": "Estimate 2",
                        "cost": 200,
                    },
                ]
            }
        });

        assert_eq!(left, right)
    }

    #[tokio::test]
    async fn test_estimate_by_id_query() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            query {
                estimate(id: "00000000-0000-0000-0000-000000000001") {
                    id
                    estimate
                    cost                   
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
            "estimate": {
                "id": "00000000-0000-0000-0000-000000000001",
                "estimate": "Estimate 1",
                "cost": 100,
            }
        });

        assert_eq!(left, right)
    }

    #[tokio::test]
    async fn test_create_estimate() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            mutation {
                createEstimate(
                    input: {
                        projectId: "00000000-0000-0000-0000-000000000001"
                        estimate: "Estimate 4"
                    }
                ) {
                    estimate {
                        id
                        estimate
                        cost
                    }
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let id = left["createEstimate"]["estimate"]["id"].as_str().unwrap();
        let id = Uuid::parse_str(id).unwrap();

        let right = serde_json::json!({
            "createEstimate": {
                "estimate": {
                    "id": id,
                    "estimate": "Estimate 4",
                    "cost": 0,
                }
            }
        });

        assert_eq!(left, right);

        let result = Estimate::fetch_one(id, &app.pg_pool).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_delete_estimate() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

        let query = r#"
            mutation deleteEstimate($id: ID!){
                deleteEstimate(
                    input: {
                        id: $id
                    }
                ) {
                    id
                }
            }
        "#;

        let left = client
            .query_with_vars::<Value, Vars>(query, Vars { id: id.into() })
            .await
            .unwrap();

        let right = serde_json::json!({
            "deleteEstimate": {
                "id": id
            }
        });

        assert_eq!(left, right);

        let result = Estimate::fetch_one(id, &app.pg_pool).await;

        assert!(matches!(result.err().unwrap(), AppError::RecordNotFound))
    }

    #[tokio::test]
    async fn test_estimate_has_assemblies() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            query {
                estimate(id: "00000000-0000-0000-0000-000000000001") {
                    assemblies {
                        id
                        assembly
                        quantity
                    }               
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
            "estimate": {
                "assemblies": [
                    {
                        "id": "00000000-0000-0000-0000-000000000001",
                        "assembly": "Assembly 1",
                        "quantity": 10
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000002",
                        "assembly": "Assembly 2",
                        "quantity": 20
                    }
                ]
            }
        });

        assert_eq!(left, right)
    }

    #[tokio::test]
    async fn test_add_assembly_to_estimate() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            mutation {
                addAssemblyToEstimate(
                    input: {
                        estimateId: "00000000-0000-0000-0000-000000000003"
                        assemblyId: "00000000-0000-0000-0000-000000000003"
                    }
                ) {
                    estimate {
                        id
                        assemblies {
                            id
                            assembly
                            quantity
                        }
                    }
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
            "addAssemblyToEstimate": {
                "estimate": {
                    "id": "00000000-0000-0000-0000-000000000003",
                    "assemblies": [
                        {
                            "id": "00000000-0000-0000-0000-000000000003",
                            "assembly": "Assembly 3",
                            "quantity": 0
                        }
                    ]
                }
            }
        });

        assert_eq!(left, right);

        // let result = Estimate::fetch_one("00000000-0000-0000-0000-000000000003", &app.pg_pool).await;
        //
        // assert!(result.is_ok())
    }
}
