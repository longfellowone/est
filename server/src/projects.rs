use crate::StatusCode;
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub project: String,
}

impl Project {
    async fn fetch_all(pg_pool: PgPool) -> Vec<Self> {
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

    async fn fetch_one(id: i32, pg_pool: PgPool) -> Self {
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

    async fn create(new_project: Project, pg_pool: PgPool) -> Self {
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

    async fn delete(id: i32, pg_pool: PgPool) {
        sqlx::query!("DELETE FROM project WHERE id = $1", id)
            .execute(&pg_pool)
            .await
            .unwrap();
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

pub async fn delete(
    Path(id): Path<i32>,
    Extension(pg_pool): Extension<PgPool>,
) -> impl IntoResponse {
    Project::delete(id, pg_pool).await;

    StatusCode::NO_CONTENT
}

// TODO: Implement update method
pub async fn update() {
    unimplemented!()
}
