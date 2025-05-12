use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use money_jar_core::hello;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use money_jar_core::db_models::users::users_crud::*;

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
        .route("/api/hello", post(hello_handler))
        //user routes          backend api call uses camel case convention while core calls use snake_case convention
        .route("/api/CreateUser", post(CreateUser)) //takes name, email returns id
        .route("/api/UpdateName", post(UpdateName)) //takes id, name returns message
        .route("/api/UpdateEmail", post(UpdateEmail)) //takes id, email returns message
        .route("/api/UpdatePhone", post(UpdatePhone)) //takes id, phone returns message
        .route("/api/GetName", post(GetName)) //takes id returns name
        .route("/api/GetEmail", post(GetEmail)) //takes id returns email
        .route("/api/GetPhone", post(GetPhone)) //takes id returns phone
        ;

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

//user routes

async fn CreateUser(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<CreateUserResponse>) {
    let response = create_user(payload.name, payload.email);
    (StatusCode::OK, Json(CreateUserResponse { message: response }))
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct CreateUserResponse {
    message: String,
}

async fn UpdateName(Json(payload): Json<UpdateNameRequest>) -> (StatusCode, Json<UpdateNameResponse>) {
    let response = update_name(payload.id, payload.name);
    (StatusCode::OK, Json(UpdateNameResponse { message: response }))
}

#[derive(Deserialize)]
struct UpdateNameRequest {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct UpdateNameResponse {
    message: String,
}

async fn UpdateEmail(Json(payload): Json<UpdateEmailRequest>) -> (StatusCode, Json<UpdateEmailResponse>) {
    let response = update_email(payload.id, payload.email);
    (StatusCode::OK, Json(UpdateEmailResponse { message: response }))
}   

#[derive(Deserialize)]
struct UpdateEmailRequest {
    id: String,
    email: String,
}

#[derive(Serialize)]
struct UpdateEmailResponse {
    message: String,
}

async fn UpdatePhone(Json(payload): Json<UpdatePhoneRequest>) -> (StatusCode, Json<UpdatePhoneResponse>) {
    let response = update_phone(payload.id, payload.phone);
    (StatusCode::OK, Json(UpdatePhoneResponse { message: response }))
}

#[derive(Deserialize)]
struct UpdatePhoneRequest {
    id: String,
    phone: String,
}

#[derive(Serialize)]
struct UpdatePhoneResponse {
    message: String,
}

async fn GetName(Json(payload): Json<GetNameRequest>) -> (StatusCode, Json<GetNameResponse>) {
    let response = get_name(payload.id);
    (StatusCode::OK, Json(GetNameResponse { name: response }))
}

#[derive(Deserialize)]
struct GetNameRequest {
    id: String,
}

#[derive(Serialize)]
struct GetNameResponse {
    name: String,
}

async fn GetEmail(Json(payload): Json<GetEmailRequest>) -> (StatusCode, Json<GetEmailResponse>) {
    let response = get_email(payload.id);
    (StatusCode::OK, Json(GetEmailResponse { email: response }))
}

#[derive(Deserialize)]
struct GetEmailRequest {
    id: String,
}

#[derive(Serialize)]
struct GetEmailResponse {
    email: String,
}

async fn GetPhone(Json(payload): Json<GetPhoneRequest>) -> (StatusCode, Json<GetPhoneResponse>) {
    let response = get_phone(payload.id);
    (StatusCode::OK, Json(GetPhoneResponse { phone: response }))
}

#[derive(Deserialize)]
struct GetPhoneRequest {
    id: String,
}

#[derive(Serialize)]
struct GetPhoneResponse {
    phone: String,
}










































