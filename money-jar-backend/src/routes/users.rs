use crate::state::AppState;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use money_jar_core::*;
use serde::{Deserialize, Serialize};

// Route definitions
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/CreateUser", post(post_create_user))
        .route("/UpdateName", post(post_update_name))
        .route("/UpdateEmail", post(post_update_email))
        .route("/UpdatePhone", post(post_update_phone))
        .route("/GetName", post(post_get_name))
        .route("/GetEmail", post(post_get_email))
        .route("/GetPhone", post(post_get_phone))
        .route("/DeleteUser", post(post_delete_user))
        .route("/UserGetAll", post(post_get_all))
}

// Request/Response structs

#[derive(Deserialize)]
struct UserGetAllRequest {
    id: String,
}

#[derive(Serialize)]
struct UserGetAllResponse {
    id: String,
    name: String,
    email: String,
    phone: Option<String>,
}
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]

struct UpdateNameRequest {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct UpdateEmailRequest {
    id: String,
    email: String,
}

#[derive(Deserialize)]
struct UpdatePhoneRequest {
    id: String,
    phone: String,
}

#[derive(Deserialize)]
struct GetNameRequest {
    id: String,
}

#[derive(Serialize)]
struct GetNameResponse {
    name: String,
}

#[derive(Deserialize)]
struct GetEmailRequest {
    id: String,
}

#[derive(Serialize)]
struct GetEmailResponse {
    email: String,
}

#[derive(Deserialize)]
struct GetPhoneRequest {
    id: String,
}

#[derive(Serialize)]
struct GetPhoneResponse {
    phone: String,
}

#[derive(Deserialize)]
struct DeleteUserRequest {
    id: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    id: String,
}

// Route handlers
async fn post_create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = create_user(&mut conn, payload.name, payload.email, payload.password);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK,
    }
}
async fn post_get_all(
    State(state): State<AppState>,
    Json(payload): Json<UserGetAllRequest>,
) -> (StatusCode, Json<UserGetAllResponse>) {
    let mut conn = state.pool.get().unwrap();
    let response = get_all(&mut conn, payload.id);
    match response {
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(UserGetAllResponse {
                id: "".to_string(),
                name: "".to_string(),
                email: "".to_string(),
                phone: None,
            }),
        ),
        Ok(user) => (
            StatusCode::OK,
            Json(UserGetAllResponse {
                id: user.0,
                name: user.1,
                email: user.2,
                phone: user.3,
            }),
        ),
    }
}
async fn post_update_name(
    State(state): State<AppState>,
    Json(payload): Json<UpdateNameRequest>,
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = update_name(&mut conn, payload.id, payload.name);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK,
    }
}

async fn post_update_email(
    State(state): State<AppState>,
    Json(payload): Json<UpdateEmailRequest>,
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = update_email(&mut conn, payload.id, payload.email);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK,
    }
}

async fn post_update_phone(
    State(state): State<AppState>,
    Json(payload): Json<UpdatePhoneRequest>,
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = update_phone(&mut conn, payload.id, payload.phone);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK,
    }
}

async fn post_get_name(
    State(state): State<AppState>,
    Json(payload): Json<GetNameRequest>,
) -> (StatusCode, Json<GetNameResponse>) {
    let mut conn = state.pool.get().unwrap();
    let response = get_name(&mut conn, payload.id);
    match response {
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(GetNameResponse {
                name: e.to_string(),
            }),
        ),
        Ok(name) => (StatusCode::OK, Json(GetNameResponse { name: name })),
    }
}

async fn post_get_email(
    State(state): State<AppState>,
    Json(payload): Json<GetEmailRequest>,
) -> (StatusCode, Json<GetEmailResponse>) {
    let mut conn = state.pool.get().unwrap();
    let response = get_email(&mut conn, payload.id);
    match response {
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(GetEmailResponse {
                email: e.to_string(),
            }),
        ),
        Ok(email) => (StatusCode::OK, Json(GetEmailResponse { email: email })),
    }
}

async fn post_get_phone(
    State(state): State<AppState>,
    Json(payload): Json<GetPhoneRequest>,
) -> (StatusCode, Json<GetPhoneResponse>) {
    let mut conn = state.pool.get().unwrap();
    match get_phone(&mut conn, payload.id) {
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(GetPhoneResponse {
                phone: e.to_string(),
            }),
        ),
        Ok(Some(p)) => (StatusCode::OK, Json(GetPhoneResponse { phone: p })),
        Ok(None) => (
            StatusCode::OK,
            Json(GetPhoneResponse {
                phone: String::new(),
            }),
        ),
    }
}

async fn post_delete_user(
    State(state): State<AppState>,
    Json(payload): Json<DeleteUserRequest>,
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = delete_user(&mut conn, payload.id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK,
    }
}
