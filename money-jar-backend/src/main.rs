use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use money_jar_core::hello;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/hello", post(hello_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hello_handler(Json(payload): Json<HelloRequest>) -> (StatusCode, Json<HelloResponse>) {
    let response = hello(payload.name);
    (StatusCode::OK, Json(HelloResponse { message: response }))
}

#[derive(Deserialize)]
struct HelloRequest {
    name: String,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}
