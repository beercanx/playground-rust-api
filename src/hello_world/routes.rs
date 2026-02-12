use axum::Router;
use axum::routing::get;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(super::text))
        .route("/html", get(super::html))
        .route("/json", get(super::json))
}

#[cfg(test)]
mod integration_tests {

    // See: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use axum::http::Method;
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::{ServiceExt};

    #[tokio::test]
    async fn hello_world() {

        let router = super::routes();

        let response = router
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, World!");
    }

    #[tokio::test]
    async fn json() {

        let response = super::routes()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/json")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "message": "Hello, World!" }));
    }

    #[tokio::test]
    async fn html() {

        let response = super::routes()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/html")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"<h1>Hello, World!</h1>");
    }

    #[tokio::test]
    async fn not_found() {

        let response = super::routes()
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty());
    }
}
