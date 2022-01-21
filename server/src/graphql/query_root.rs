use crate::postgres::projects::Project;
use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let pg_pool = ctx.data_unchecked::<PgPool>();
        let projects = Project::fetch_all(pg_pool).await.unwrap();

        Ok(projects)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::config::{Http, Postgres};
//     use crate::graphql;
//     use crate::Configuration;
//
//     #[tokio::test]
//     async fn test_projects_query() {
//         let config = Configuration {
//             http: Http {
//                 host: "127.0.0.1".to_string(),
//                 port: 0,
//             },
//             postgres: Postgres {
//                 host: "127.0.0.1".to_string(),
//                 port: 5432,
//                 user: "postgres".to_string(),
//                 password: "postgres".to_string(),
//                 database: "postgres".to_string(),
//                 sslmode: false,
//             },
//         };
//
//         let schema = graphql::schema(&config).await;
//
//         let response = schema.execute("query { projects { id project } }").await;
//         let json_value = response.data.into_json().unwrap();
//
//         let
//
//         assert_eq!(
//             json_value,
//             serde_json::json!({
//                 "projects": [
//                     {
//                         "id" : 1,
//                         "project" : "Project 1"
//                     },
//                     {
//                         "id" : 2,
//                         "project" : "Project 2"
//                     },
//                     {
//                         "id" : 3,
//                         "project" : "Project 3"
//                     }
//                 ]
//             })
//         );
//     }
// }
