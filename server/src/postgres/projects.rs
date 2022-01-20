use crate::error::{sqlx_error, AppError};
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, PartialEq, Serialize, Deserialize, SimpleObject)]
pub struct Project {
    pub id: i32,
    pub project: String,
}

impl Project {
    pub async fn fetch_all(pg_pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Project,
            r#"
            SELECT id, project
            From project
            "#
        )
        .fetch_all(pg_pool)
        .await
        .map_err(sqlx_error)
    }

    async fn fetch_one(id: i32, pg_pool: PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
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
        .map_err(sqlx_error)
    }

    async fn create(new_project: Project, pg_pool: PgPool) -> Result<Self, AppError> {
        sqlx::query_as!(
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
        .map_err(sqlx_error)
    }

    async fn delete(id: i32, pg_pool: PgPool) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM project 
            WHERE id = $1
            "#,
            id
        )
        .execute(&pg_pool)
        .await
        .map_err(sqlx_error);

        // TODO: Improve this?
        if let Ok(query) = result {
            if query.rows_affected() == 0 {
                return Err(AppError::BadRequest);
            }
        }

        Ok(())
    }
}

// pub async fn list(Extension(pg_pool): Extension<PgPool>) -> Result<Json<Vec<Project>>, AppError> {
//     let projects = Project::fetch_all(&pg_pool).await?;
//
//     Ok(projects.into())
// }
//
// pub async fn get(
//     Path(id): Path<i32>,
//     Extension(pg_pool): Extension<PgPool>,
// ) -> Result<Json<Project>, AppError> {
//     let project = Project::fetch_one(id, pg_pool).await?;
//
//     Ok(project.into())
// }
//
// pub async fn create(
//     Json(project): Json<Project>,
//     Extension(pg_pool): Extension<PgPool>,
// ) -> Result<(StatusCode, Json<Project>), AppError> {
//     let project = Project::create(project, pg_pool).await?;
//
//     Ok((StatusCode::CREATED, project.into()))
// }
//
// pub async fn delete(
//     Path(id): Path<i32>,
//     Extension(pg_pool): Extension<PgPool>,
// ) -> Result<StatusCode, AppError> {
//     Project::delete(id, pg_pool).await?;
//
//     Ok(StatusCode::NO_CONTENT)
// }
//
// // TODO: Implement update method
// pub async fn update() {
//     unimplemented!()
// }
