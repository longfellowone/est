mod common;

#[cfg(test)]
mod tests {
    use crate::common::{TestApp, Vars};
    use reqwest_graphql::Client;
    use serde_json::Value;
    use server::error::AppError;
    use server::estimating::Estimate;
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

        // Todo: Make this check better
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
}
