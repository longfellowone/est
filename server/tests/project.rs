mod common;

#[cfg(test)]
mod tests {
    use crate::common::{TestApp, Vars};
    use reqwest_graphql::Client;
    use serde_json::Value;
    use server::error::AppError;
    use server::postgres::Project;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_projects_query() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            query {
                projects {
                    id
                    project
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
           "projects": [
                {
                    "id": "00000000-0000-0000-0000-000000000001",
                    "project": "Project 1"
                },
                {
                    "id": "00000000-0000-0000-0000-000000000002",
                    "project": "Project 2"
                }
            ]
        });

        assert_eq!(left, right);
    }

    #[tokio::test]
    async fn test_project_by_id_query() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            query {
                project(id: "00000000-0000-0000-0000-000000000001") {
                    id
                    project
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let right = serde_json::json!({
            "project": {
                "id":"00000000-0000-0000-0000-000000000001",
                "project":"Project 1"
            }
        });

        assert_eq!(left, right)
    }

    #[tokio::test]
    async fn test_create_project() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let query = r#"
            mutation {
                createProject(
                    input: {
                        project: "Project 3"
                    }
                ) {
                    project {
                        id
                        project
                    }
                }
            }
        "#;

        let left = client.query::<Value>(query).await.unwrap();

        let id = left["createProject"]["project"]["id"].as_str().unwrap();

        assert!(Uuid::parse_str(id).is_ok());

        let right = serde_json::json!({
            "createProject": {
                "project": {
                    "id": id,
                    "project": "Project 3",
                }
            }
        });

        assert_eq!(left, right);

        let query = r#"
            query Project($id: ID!){
                project(id: $id) {
                    id
                    project
                }
            }
        "#;

        let left = client
            .query_with_vars::<Value, Vars>(query, Vars { id: id.into() })
            .await
            .unwrap();

        let right = serde_json::json!({
            "project": {
                "id": id,
                "project": "Project 3",
            }
        });

        assert_eq!(left, right)
    }

    #[tokio::test]
    async fn test_delete_project() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        let id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

        let query = r#"
            mutation deleteProject($id: ID!) {
                deleteProject(
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
            "deleteProject": {
                "id": id
            }
        });

        assert_eq!(left, right);

        let result = Project::fetch_one(id, &app.pg_pool).await;

        assert!(matches!(result.err().unwrap(), AppError::RecordNotFound))
    }
}
