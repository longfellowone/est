use crate::common::TestApp;
use async_graphql::ID;
use gql_client::Client;
use serde::Serialize;
use serde_json::Value;
use server::http::assembly_item::AssemblyItem;
use uuid::Uuid;

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
                    cost
                }
            }
        "#;

    let left = client.query::<Value>(query).await.unwrap();

    let right = serde_json::json!({
        "assembly": {
            "id": "00000000-0000-0000-0000-000000000001",
            "assembly": "Assembly 1",
            "cost": 10000,
        }
    });

    assert_eq!(left, right)
}

#[tokio::test]
async fn test_add_item_to_assembly() {
    let app = TestApp::new().await;
    let client = Client::new(&app.addr);

    #[derive(Serialize)]
    struct Vars {
        id: ID,
        input: AddItemToAssemblyInput,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct AddItemToAssemblyInput {
        item_id: ID,
        quantity: i32,
    }

    let assembly_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let item_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    let quantity = 10;

    let vars = Vars {
        id: ID::from(assembly_id),
        input: AddItemToAssemblyInput {
            item_id: ID::from(item_id),
            quantity,
        },
    };

    let query = r#"
        mutation AddItemToAssembly($id: ID!, $input: AddItemToAssemblyInput) {
            addItemToAssembly(
                id: $id
                input: $input
            ) {
                assembly {
                    id
                    cost
                    items {
                        id
                        quantity
                    }
                }
            }
        }
    "#;

    let left = client
        .query_with_vars::<Value, Vars>(query, vars)
        .await
        .unwrap();

    let right = serde_json::json!({
        "addItemToAssembly": {
            "assembly": {
                "id": assembly_id,
                "cost": 10200,
                "items": [
                    {
                        "id": "00000000-0000-0000-0000-000000000001",
                        "quantity": 100,
                    },
                    {
                        "id": item_id,
                        "quantity": quantity,
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000003",
                        "quantity": 300,
                    }
                ]
            }
        }
    });

    assert_eq!(left, right);

    let result = AssemblyItem::fetch_all(assembly_id, &app.pool).await;

    // TODO: Make this check better
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 3)
}

// NOTE: This could be combined with updateAssemblyItemQuantity - delete when quantity is 0
#[tokio::test]
async fn test_remove_item_from_assembly() {
    let app = TestApp::new().await;
    let client = Client::new(&app.addr);

    #[derive(Serialize)]
    struct Vars {
        id: ID,
        input: RemoveItemFromAssemblyInput,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct RemoveItemFromAssemblyInput {
        item_id: ID,
    }

    let assembly_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let item_id = Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap();

    let vars = Vars {
        id: ID::from(assembly_id),
        input: RemoveItemFromAssemblyInput {
            item_id: ID::from(item_id),
        },
    };

    let query = r#"
        mutation RemoveItemFromAssembly($id: ID!, $input: RemoveItemFromAssemblyInput) {
            removeItemFromAssembly(
                id: $id
                input: $input
            ) {
                assembly {
                    id
                    cost
                    items {
                        id
                        quantity
                    }
                }
            }
        }
    "#;

    let left = client
        .query_with_vars::<Value, Vars>(query, vars)
        .await
        .unwrap();

    let right = serde_json::json!({
        "removeItemFromAssembly": {
            "assembly": {
                "id": assembly_id,
                "cost": 1000, // 100 * 10
                "items": [
                    {
                        "id": "00000000-0000-0000-0000-000000000001",
                        "quantity": 100,
                    }
                ]
            }
        }
    });

    assert_eq!(left, right);

    let result = AssemblyItem::fetch_all(assembly_id, &app.pool).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1)
}

#[tokio::test]
async fn test_update_assembly_item_quantity() {
    let app = TestApp::new().await;
    let client = Client::new(&app.addr);

    #[derive(Serialize)]
    struct Vars {
        id: ID,
        input: UpdateAssemblyItemQuantityInput,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct UpdateAssemblyItemQuantityInput {
        id: ID,
        quantity: i32,
    }

    let assembly_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let item_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let quantity = 10;

    let vars = Vars {
        id: ID::from(assembly_id),
        input: UpdateAssemblyItemQuantityInput {
            id: ID::from(item_id),
            quantity,
        },
    };

    let query = r#"
        mutation UpdateAssemblyItemQuantity($id: ID!, $input: UpdateAssemblyItemQuantityInput) {
            updateAssemblyItemQuantity(
                id: $id
                input: $input
            ) {
                assembly {
                    id
                    cost
                    items {
                        id
                        quantity
                    }
                }
            }
        }
    "#;

    let left = client
        .query_with_vars::<Value, Vars>(query, vars)
        .await
        .unwrap();

    let right = serde_json::json!({
        "updateAssemblyItemQuantity": {
            "assembly": {
                "id": assembly_id,
                "cost": 9100,
                "items": [
                    {
                        "id": "00000000-0000-0000-0000-000000000001",
                        "quantity": 10 // 10 (originally 100) * 10 = 100
                    },
                    {
                        "id": "00000000-0000-0000-0000-000000000003",
                        "quantity": 300 // 300 * 30 = 9000
                    }
                ]
            }
        }
    });

    assert_eq!(left, right);

    let result = AssemblyItem::fetch_all(assembly_id, &app.pool).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0].quantity, 10)
}
#[tokio::test]
async fn update_cost_of_item() {
    todo!()
}
