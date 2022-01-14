use crate::StatusCode;
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::Json;
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Project {
    id: i32,
    project: String,
}

impl Project {}

pub async fn list(Extension(pg_pool): Extension<PgPool>) -> impl IntoResponse {
    // TODO: remove unwrap()
    let projects = sqlx::query_as!(
        Project,
        r#"
        SELECT id, project
        From project
        "#
    )
    .fetch_all(&pg_pool)
    .await
    .unwrap();

    (StatusCode::OK, Json(projects))
}

pub async fn get(Path(id): Path<i32>, Extension(pg_pool): Extension<PgPool>) -> impl IntoResponse {
    debug!("{}", id);

    let project = sqlx::query_as!(
        Project,
        r#"
        SELECT id, project
        FROM project
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&pg_pool)
    .await
    .unwrap();

    (StatusCode::OK, Json(project))
}

pub async fn create() -> impl IntoResponse {
    (StatusCode::CREATED, Json("created"))
}

pub async fn delete() {}

// pub async fn update() -> Result<impl IntoResponse, StatusCode> {
// // }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{App, Configuration};
    use axum::body::Body;
    use axum::http;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn projects_list() {
        let app = App::new(Configuration::test()).await;

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/projects")
            .body(Body::empty())
            .unwrap();

        let response = app.router.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_json = serde_json::from_slice::<Vec<Project>>(&body).unwrap();

        let project = Project {
            id: 1,
            project: "Project 1".to_string(),
        };

        assert_eq!(body_json.len(), 3);
        assert_eq!(body_json[0], project);
    }

    #[tokio::test]
    async fn projects_get() {
        let app = App::new(Configuration::test()).await;

        let request = Request::builder()
            .method(http::Method::GET)
            .uri("/projects/1")
            .body(Body::empty())
            .unwrap();

        let response = app.router.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_json = serde_json::from_slice::<Project>(&body).unwrap();

        let project = Project {
            id: 1,
            project: "Project 1".to_string(),
        };

        assert_eq!(body_json, project);
    }
}
