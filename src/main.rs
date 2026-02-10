
use axum::{response::Html, routing::get, Json, Router};
use serde::Serialize;
use tokio::signal;

#[tokio::main]
async fn main() {

    let application = Router::new()
        .route("/", get(hello_world))
        .route("/html", get(hello_world_html))
        .route("/json", get(hello_world_json));

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("listening on http://{}", tcp_listener.local_addr().unwrap());

    axum::serve(tcp_listener, application)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
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