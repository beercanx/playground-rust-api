
use axum::{response::Html, routing::get, Json, Router};
use serde::Serialize;
use tokio::signal;

#[tokio::main]
async fn main() {

    let application = router();

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("listening on http://{}", tcp_listener.local_addr().unwrap());

    axum::serve(tcp_listener, application)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/html", get(hello_world_html))
        .route("/json", get(hello_world_json))
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn hello_world_html() -> Html<String> {
    Html(format!("<h1>{}</h1>", hello_world().await))
}

async fn hello_world_json() -> Json<Message> {
    Json(Message{message: hello_world().await})
}

#[derive(Serialize)]
struct Message {
    message: &'static str,
}

async fn shutdown_signal() {

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[cfg(test)]
mod tests {

    // See: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

    use super::*;
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

        let router = router();

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

        let router = router();

        let response = router
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

        let router = router();

        let response = router
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

        let router = router();

        let response = router
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
