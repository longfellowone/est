mod common;

#[cfg(test)]
mod tests {
    use crate::common::{TestApp, Vars};
    use async_graphql::ID;
    use gql_client::Client;
    use serde_json::Value;
    use server::error::AppError;
    use server::http::project::Project;
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
        let id = Uuid::parse_str(id).unwrap();

        let right = serde_json::json!({
            "createProject": {
                "project": {
                    "id": id,
                    "project": "Project 3",
                }
            }
        });

        assert_eq!(left, right);

        let result = Project::fetch_one(id, &app.pool).await;

        // Todo: Make this check better
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_delete_project() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

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

        let id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

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

        let result = Project::fetch_one(id, &app.pool).await;

        assert!(matches!(result.err().unwrap(), AppError::RecordNotFound))
    }

    #[tokio::test]
    async fn test_update_project() {
        let app = TestApp::new().await;
        let client = Client::new(&app.addr);

        #[derive(serde::Serialize)]
        struct Vars {
            input: UpdateProjectInput,
        }

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct UpdateProjectInput {
            id: ID,
            project: String,
        }

        let id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let project = "Project 5";

        let vars = Vars {
            input: UpdateProjectInput {
                id: ID::from(id),
                project: project.to_string(),
            },
        };

        let query = r#"
            mutation updateProject($input: UpdateProjectInput!) {
                updateProject(
                    input: $input
                ) {
                    project {
                        id
                        project
                    }
                }
            }
        "#;

        let left = client
            .query_with_vars::<Value, Vars>(query, vars)
            .await
            .unwrap();

        let right = serde_json::json!({
            "updateProject": {
                "project": {
                    "id": id,
                    "project": project,
                }
            }
        });

        assert_eq!(left, right);

        let result = Project::fetch_one(id, &app.pool).await.unwrap();

        assert_eq!(result.project, project)
    }
}
