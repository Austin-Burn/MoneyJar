use crate::state::AppState;
use axum::extract::State;
use axum::{http::StatusCode, routing::post, Json, Router};
use money_jar_core::login;
use serde::{Deserialize, Serialize};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/Login", post(post_login))
        .route("/Refresh", post(refresh))
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Response {
    Login { id: String, token: String },
}

async fn post_login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Response>) {
    let mut conn = state.pool.get().unwrap();

    let (id, token) = login(&mut conn, payload.email, payload.password).unwrap();

    (StatusCode::OK, Json(Response::Login { id, token }))
}

async fn refresh() -> StatusCode {
    StatusCode::OK
}
