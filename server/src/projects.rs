use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::Json;
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
struct Project {
    id: i32,
    project: String,
}

pub async fn list(Extension(pg_pool): Extension<PgPool>) -> impl IntoResponse {
    // TODO: remove unwrap()
    let projects = sqlx::query_as!(Project, "SELECT id, project from project")
        .fetch_all(&pg_pool)
        .await
        .unwrap();

    Json(projects)
}

pub async fn get(Path(id): Path<u64>) {
    debug!("{}", id)
}

pub async fn create() {}

pub async fn delete() {}

// async fn update() {}
