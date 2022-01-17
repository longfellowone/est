use crate::common::TestApp;
use axum::http::StatusCode;

mod common;

#[tokio::test]
async fn health_check_returns_200_with_empty_body() {
    let app = TestApp::new().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.content_length(), Some(0));
}
