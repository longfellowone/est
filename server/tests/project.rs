mod common;

#[cfg(test)]
mod tests {
    use crate::common::TestApp;
    use reqwest_graphql::Client;
    use serde_json::Value;

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
                    "id":"00000000-0000-0000-0000-000000000001",
                    "project":"Project 1"
                },
                {
                    "id":"00000000-0000-0000-0000-000000000002",
                    "project":"Project 2"
                },
                {
                    "id":"00000000-0000-0000-0000-000000000003",
                    "project":"Project 3"
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
}

// #[tokio::test]
// async fn projects_get_returns_a_project() {
//     let app = TestApp::new().await;
//     let client = reqwest::Client::new();
//
//     let response = client
//         .get(format!("{}/projects/1", app.addr))
//         .send()
//         .await
//         .expect("get request failed to projects/1");
//
//     assert_eq!(response.status(), StatusCode::OK);
//
//     let response_json = response
//         .json::<Project>()
//         .await
//         .expect("failed to deserialize project");
//
//     let project = Project {
//         id: 1,
//         project: "Project 1".to_string(),
//     };
//
//     assert_eq!(response_json, project);
// }
//
// #[tokio::test]
// async fn projects_create_returns_project_and_saves_to_database() {
//     let app = TestApp::new().await;
//     let client = reqwest::Client::new();
//
//     let project = Project {
//         id: 4,
//         project: "Project 4".to_string(),
//     };
//
//     let response = client
//         .post(format!("{}/projects", app.addr))
//         .json(&project)
//         .send()
//         .await
//         .expect("post request failed to /projects");
//
//     assert_eq!(response.status(), StatusCode::CREATED);
//
//     let response_json = response
//         .json::<Project>()
//         .await
//         .expect("failed to deserialize project");
//
//     assert_eq!(response_json, project);
//
//     let response = client
//         .get(format!("{}/projects/{}", app.addr, project.id))
//         .send()
//         .await
//         .expect(format!("get request failed to /projects/{}", project.id).as_str());
//
//     let response_json = response
//         .json::<Project>()
//         .await
//         .expect("failed to deserialize project");
//
//     assert_eq!(response_json, project);
// }
//
// #[tokio::test]
// async fn projects_delete_removes_project_from_database() {
//     let app = TestApp::new().await;
//     let client = reqwest::Client::new();
//
//     let project_id = "1";
//
//     let response = client
//         .delete(format!("{}/projects/{}", app.addr, project_id))
//         .send()
//         .await
//         .expect(format!("delete request failed to /projects/{}", project_id).as_str());
//
//     assert_eq!(response.status(), StatusCode::NO_CONTENT);
//
//     let response = client
//         .get(format!("{}/projects/{}", app.addr, project_id))
//         .send()
//         .await
//         .expect(format!("get request failed to /projects/{}", project_id).as_str());
//
//     assert_eq!(response.status(), StatusCode::NOT_FOUND);
//
//     // TODO: Add test for code 400 when row was not deleted
// }
