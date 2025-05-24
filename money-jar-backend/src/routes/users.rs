use axum::{
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use money_jar_core::*;

// Route definitions
pub fn user_routes() -> Router {
    Router::new()
        .route("/api/CreateUser", post(post_create_user))
        .route("/api/UpdateName", post(post_update_name))
        .route("/api/UpdateEmail", post(post_update_email))
        .route("/api/UpdatePhone", post(post_update_phone))
        .route("/api/GetName", post(post_get_name))
        .route("/api/GetEmail", post(post_get_email))
        .route("/api/GetPhone", post(post_get_phone))
        .route("/api/DeleteUser", post(post_delete_user))
        .route("/api/Login", post(post_login))
}

// Request/Response structs
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
async fn post_create_user(Json(payload): Json<CreateUserRequest>) -> StatusCode {
    let response = create_user(payload.name, payload.email, payload.password);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_name(Json(payload): Json<UpdateNameRequest>) -> StatusCode {
    let response = update_name(payload.id, payload.name);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_email(Json(payload): Json<UpdateEmailRequest>) -> StatusCode {
    let response = update_email(payload.id, payload.email);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_phone(Json(payload): Json<UpdatePhoneRequest>) -> StatusCode {
    let response = update_phone(payload.id, payload.phone);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_get_name(Json(payload): Json<GetNameRequest>) -> (StatusCode, Json<GetNameResponse>) {
    let response = get_name(payload.id);
    match response {
        Err(e) => (StatusCode::NOT_FOUND, Json(GetNameResponse { name: e.to_string() })),
        Ok(name) => (StatusCode::OK, Json(GetNameResponse { name: name })),
    }
}

async fn post_get_email(Json(payload): Json<GetEmailRequest>) -> (StatusCode, Json<GetEmailResponse>) {
    let response = get_email(payload.id);
    match response {
        Err(e) => (StatusCode::NOT_FOUND, Json(GetEmailResponse { email: e.to_string() })),
        Ok(email) => (StatusCode::OK, Json(GetEmailResponse { email: email })),
    }
}

async fn post_get_phone(Json(payload): Json<GetPhoneRequest>) -> (StatusCode, Json<GetPhoneResponse>) {
    match get_phone(payload.id) {
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(GetPhoneResponse { phone: e.to_string() }),
        ),
        Ok(Some(p)) => (
            StatusCode::OK,
            Json(GetPhoneResponse { phone: p }),
        ),
        Ok(None) => (
            StatusCode::OK,
            Json(GetPhoneResponse { phone: String::new() }),
        ),
    }
}

async fn post_delete_user(Json(payload): Json<DeleteUserRequest>) -> StatusCode {
    let response = delete_user(payload.id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_login(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<LoginResponse>) {
    let response = get_id(payload.email, payload.password);
    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(LoginResponse { id: "".to_string() })),
        Ok(id) => (StatusCode::OK, Json(LoginResponse { id: id }))
    }
} 