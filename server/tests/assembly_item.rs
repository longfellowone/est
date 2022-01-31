mod common;

#[cfg(test)]
mod tests {
    use crate::common::TestApp;
    use gql_client::Client;
    use serde_json::Value;

    #[tokio::test]
    async fn test_assembly_has_items() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            query {
                assembly(id: "00000000-0000-0000-0000-000000000001") {
                    id
                    items {
                        id
                        item
                        cost
                        quantity
                    }
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
           "assembly": {
                "id": "00000000-0000-0000-0000-000000000001",
                "items": [
                    {
                        "id": "00000000-0000-0000-0000-000000000001",
                        "item": "Item 1",
                        "cost": 10.00,
                        "quantity": 100
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000003",
                        "item": "Item 3",
                        "cost": 30.00,
                        "quantity": 300
                    }
                ]
            }
        });

        assert_eq!(left, right)
    }
}
