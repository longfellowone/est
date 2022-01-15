mod common;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::body::Body;
//     use axum::http;
//     use axum::http::Request;
//     use tower::ServiceExt;
//
//     #[tokio::test]
//     async fn health_check() {
//         let app = App::new(Configuration::test()).await;
//
//         let request = Request::builder()
//             .method(http::Method::GET)
//             .uri("/health_check")
//             .body(Body::empty())
//             .unwrap();
//
//         let response = app.router.oneshot(request).await.unwrap();
//
//         assert_eq!(response.status(), StatusCode::OK);
//
//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//
//         assert!(body.is_empty())
//     }
// }
