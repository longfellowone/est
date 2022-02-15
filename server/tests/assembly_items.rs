mod common;

// TODO: For update assembly item, $id is assembly ID, $input is item
// updateAsssemblyItem($id: ID!, $input: UpdateAsssemblyItemInput!)

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
                    components {
                        id
                        quantity
                        product {
                            id
                            cost
                        }                                               
                    }
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
           "assembly": {
                "id": "00000000-0000-0000-0000-000000000001",
                "components": [
                    {
                        "id": "00000000-0000-0000-0000-000000000001",
                        "quantity": 100,
                        "product": {
                            "id": "00000000-0000-0000-0000-000000000001",
                            "cost": 10
                        }
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000003",
                        "quantity": 30,
                        "product": {
                            "id": "00000000-0000-0000-0000-000000000003",
                            "cost": 300
                        }
                    }
                ]
            }
        });

        assert_eq!(left, right)
    }
}
