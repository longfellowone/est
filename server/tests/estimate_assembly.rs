mod common;

#[cfg(test)]
mod tests {
    use crate::common::TestApp;
    use async_graphql::ID;
    use reqwest_graphql::Client;
    use serde::Serialize;
    use serde_json::Value;
    use server::estimating::EstimateAssembly;
    use uuid::Uuid;

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
                        cost
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
                        "cost": 100.00,
                        "quantity": 10
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000002",
                        "assembly": "Assembly 2",
                        "cost": 200.00,
                        "quantity": 20,
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000003",
                        "assembly": "Assembly 3",
                        "cost": 300.00, // Check that rounding worked from 299.995
                        "quantity": 30,
                    },
                ]
            }
        });

        assert_eq!(left, right)
    }

    #[tokio::test]
    async fn test_add_assembly_to_estimate() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let estimate_id = Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap();
        let assembly_id = Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap();

        let query = r#"
            mutation AddAssemblyToEstimate($estimateId: ID!, $assemblyId: ID!) {
                addAssemblyToEstimate(
                    input: {
                        estimateId: $estimateId,
                        assemblyId: $assemblyId
                    }
                ) {
                    estimate {
                        id
                        assemblies {
                            id
                            assembly
                            cost
                            quantity
                        }
                    }
                }
            }
        "#;

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Vars {
            estimate_id: ID,
            assembly_id: ID,
        }

        let vars = Vars {
            estimate_id: estimate_id.into(),
            assembly_id: assembly_id.into(),
        };

        let left = client
            .query_with_vars::<Value, Vars>(query, vars)
            .await
            .unwrap();

        let right = serde_json::json!({
            "addAssemblyToEstimate": {
                "estimate": {
                    "id": estimate_id,
                    "assemblies": [
                        {
                            "id": assembly_id,
                            "assembly": "Assembly 3",
                            "cost": 300.00,
                            "quantity": 0,
                        }
                    ]
                }
            }
        });

        assert_eq!(left, right);

        let result = EstimateAssembly::fetch_all(estimate_id, &app.pg_pool).await;

        // Todo: Make this check better
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1)
    }
}
