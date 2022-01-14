use crate::StatusCode;
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    id: i32,
    project: String,
}

impl Project {
    async fn fetch_all(pg_pool: PgPool) -> Vec<Project> {
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

        projects
    }

    async fn fetch_one(id: i32, pg_pool: PgPool) -> Project {
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

        project
    }

    async fn create(new_project: Project, pg_pool: PgPool) -> Project {
        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO project (id, project)
            VALUES ($1, $2)
            RETURNING *
            "#,
            new_project.id,
            new_project.project
        )
        .fetch_one(&pg_pool)
        .await
        .unwrap();

        project
    }
}

pub async fn list(Extension(pg_pool): Extension<PgPool>) -> impl IntoResponse {
    let projects = Project::fetch_all(pg_pool).await;

    (StatusCode::OK, Json(projects))
}

pub async fn get(Path(id): Path<i32>, Extension(pg_pool): Extension<PgPool>) -> impl IntoResponse {
    let project = Project::fetch_one(id, pg_pool).await;

    (StatusCode::OK, Json(project))
}

pub async fn create(
    Json(project): Json<Project>,
    Extension(pg_pool): Extension<PgPool>,
) -> impl IntoResponse {
    let project = Project::create(project, pg_pool).await;

    (StatusCode::CREATED, Json(project))
}

pub async fn delete() {
    unimplemented!()
}

pub async fn update() {
    unimplemented!()
}

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

    #[tokio::test]
    async fn projects_create() {
        let app = App::new(Configuration::test()).await;

        let project = Project {
            id: 4,
            project: "Project 4".to_string(),
        };

        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/projects")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_vec(&project).unwrap()))
            .unwrap();

        let response = app.router.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_json = serde_json::from_slice::<Project>(&body).unwrap();

        assert_eq!(body_json, project)
    }
}
