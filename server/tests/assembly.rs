use crate::common::TestApp;
use reqwest_graphql::Client;
use serde_json::Value;

mod common;

#[tokio::test]
async fn test_assembly_by_id_query() {
    let app = TestApp::new().await;
    let client = Client::new(&app.addr);

    let query = r#"
            query {
                assembly(id: "00000000-0000-0000-0000-000000000001") {
                    id
                    assembly
                }
            }
        "#;

    let left = client.query::<Value>(query).await.unwrap();

    let right = serde_json::json!({
        "assembly": {
            "id": "00000000-0000-0000-0000-000000000001",
            "assembly": "Assembly 1",
        }
    });

    assert_eq!(left, right)
}
