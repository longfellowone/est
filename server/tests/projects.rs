use crate::common::TestApp;
use axum::http::StatusCode;
use server::projects::Project;

mod common;

#[tokio::test]
async fn projects_list_returns_vec_of_projects() {
    let app = TestApp::new().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/projects", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response_json = response.json::<Vec<Project>>().await.unwrap();
    let project = Project {
        id: 1,
        project: "Project 1".to_string(),
    };

    assert_eq!(response_json.len(), 3);
    assert_eq!(response_json[0], project)
}

#[tokio::test]
async fn projects_get_returns_a_project() {
    let app = TestApp::new().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/projects/1", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response_json = response.json::<Project>().await.unwrap();
    let project = Project {
        id: 1,
        project: "Project 1".to_string(),
    };

    assert_eq!(response_json, project);
}

#[tokio::test]
async fn projects_create_returns_project_and_saves_to_database() {
    let app = TestApp::new().await;
    let client = reqwest::Client::new();

    let project = Project {
        id: 4,
        project: "Project 4".to_string(),
    };

    let response = client
        .post(format!("{}/projects", app.address))
        .json(&project)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let response_json = response.json::<Project>().await.unwrap();

    assert_eq!(response_json, project);

    let response = client
        .get(format!("{}/projects/{}", app.address, project.id))
        .send()
        .await
        .unwrap();

    let response_json = response.json::<Project>().await.unwrap();

    assert_eq!(response_json, project);
}

#[tokio::test]
async fn projects_delete_removes_project_from_database() {
    let app = TestApp::new().await;
    let client = reqwest::Client::new();

    let project_id = "1";

    let response = client
        .delete(format!("{}/projects/{}", app.address, project_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // TODO: Test delete worked
    // let response = client
    //     .get(format!("{}/projects/{}", app.address, project_id))
    //     .send()
    //     .await
    //     .unwrap();
    //
    // assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

//     #[tokio::test]
//     async fn projects_delete() {
//         let app = App::new(Configuration::test()).await;
//
//         let project = Project {
//             id: 4,
//             project: "Project 1".to_string(),
//         };
//
//         let request = Request::builder()
//             .method(http::Method::DELETE)
//             .uri(format!("/projects/{}", project.id))
//             .body(Body::empty())
//             .unwrap();
//
//         let response = app.router.oneshot(request).await.unwrap();
//
//         assert_eq!(response.status(), StatusCode::NO_CONTENT)
//
//         // Then make second request to confirm delete
//     }
// }
