mod common;

#[cfg(test)]
mod tests {
    use crate::common::TestApp;
    use reqwest_graphql::Client;
    use serde_json::Value;

    #[tokio::test]
    async fn test_estimates_query() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            query {
                project(id: "00000000-0000-0000-0000-000000000001") {
                    estimates {
                        id
                        description
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
                        "description": "Estimate 1",
                        "cost": 100,
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000002",
                        "description": "Estimate 2",
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
                    description
                    cost                   
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
            "estimate": {
                "id": "00000000-0000-0000-0000-000000000001",
                "description": "Estimate 1",
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
                        id: "00000000-0000-0000-0000-000000000004"
                        projectId: "00000000-0000-0000-0000-000000000001"
                        description: "Estimate 4"
                    }
                ) {
                    estimate {
                        id
                        description
                        cost
                    }
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
            "createEstimate": {
                "estimate": {
                    "id": "00000000-0000-0000-0000-000000000004",
                    "description": "Estimate 4",
                    "cost": 0,
                }
            }
        });

        assert_eq!(left, right);

        let query = r#"
            query {
                estimate(id: "00000000-0000-0000-0000-000000000004") {
                    id
                    description
                    cost
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
            "estimate": {
                "id": "00000000-0000-0000-0000-000000000004",
                "description": "Estimate 4",
                "cost": 0,
            }
        });

        assert_eq!(left, right)
    }
}
