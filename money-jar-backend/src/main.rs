use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use money_jar_core::hello;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use money_jar_core::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

mod routes;
mod state;
use routes::all_routes;
use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager = ConnectionManager::<SqliteConnection>::new("MoneyJarDB.sqlite");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let state = AppState { pool };

    let app = Router::new()
        .merge(all_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:2000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
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

/*
async fn post_create_user(Json(payload): Json<CreateUserRequest>) -> StatusCode {
    let response = create_user(payload.name, payload.email, payload.password);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

async fn post_update_name(Json(payload): Json<UpdateNameRequest>) -> StatusCode{
    let response = update_name(payload.id, payload.name);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct UpdateNameRequest {
    id: String,
    name: String,
}

async fn post_update_email(Json(payload): Json<UpdateEmailRequest>) -> StatusCode {
    let response = update_email(payload.id, payload.email);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct UpdateEmailRequest {
    id: String,
    email: String,
}

async fn post_update_phone(Json(payload): Json<UpdatePhoneRequest>) -> StatusCode {
    let response = update_phone(payload.id, payload.phone);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct UpdatePhoneRequest {
    id: String,
    phone: String,
}

async fn post_get_name(Json(payload): Json<GetNameRequest>) -> (StatusCode, Json<GetNameResponse>) {
    let response = get_name(payload.id);

    match response {
        Err(e) => (StatusCode::NOT_FOUND, Json(GetNameResponse { name: e.to_string() })),
        Ok(name) => (StatusCode::OK, Json(GetNameResponse { name: name })),
    }
}

#[derive(Deserialize)]
struct GetNameRequest {
    id: String,
}

#[derive(Serialize)]
struct GetNameResponse {
    name: String,
}

async fn post_get_email(Json(payload): Json<GetEmailRequest>) -> (StatusCode, Json<GetEmailResponse>) {
    let response = get_email(payload.id);

    match response {
        Err(e) => (StatusCode::NOT_FOUND, Json(GetEmailResponse { email: e.to_string() })),
        Ok(email) => (StatusCode::OK, Json(GetEmailResponse { email: email })),
    }
}

#[derive(Deserialize)]
struct GetEmailRequest {
    id: String,
}

#[derive(Serialize)]
struct GetEmailResponse {
    email: String,
}

async fn post_get_phone(Json(payload): Json<GetPhoneRequest>) -> (StatusCode, Json<GetPhoneResponse>){
    let response = get_phone(payload.id);

    match response {
        Err(e) => (StatusCode::NOT_FOUND, Json(GetPhoneResponse { phone: e.to_string() })),
        Ok(phone) => (StatusCode::OK, Json(GetPhoneResponse { phone: phone.unwrap() })),
    }
}

#[derive(Deserialize)]
struct GetPhoneRequest {
    id: String
}

#[derive(Serialize)]
struct GetPhoneResponse {
    phone: String
}

async fn post_delete_user(Json(payload): Json<DeleteUserRequest>) -> StatusCode {
    let response = delete_user(payload.id);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct DeleteUserRequest {
    id: String,
}

async fn post_login(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<LoginResponse>) {
    let response = get_id(payload.email, payload.password);

    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(LoginResponse { id: "".to_string() })),
        Ok(id) => (StatusCode::OK, Json(LoginResponse { id: id }))
    }
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

//friend routes

async fn post_create_friend(Json(payload): Json<CreateFriendRequest>) -> StatusCode {
    let response = create_friend(payload.id, payload.friend_id);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct CreateFriendRequest {
    id: String,
    friend_id: String,
}

async fn post_get_friends(Json(payload): Json<GetFriendsRequest>) -> Result<(StatusCode, Json<GetFriendsResponse>), (StatusCode, Json<String>)> {
    let response = get_friends(payload.id);

    match response {
        Err(e) => Err((StatusCode::NOT_FOUND, Json(e.to_string()))),
        Ok(friends) => Ok((StatusCode::OK, Json(GetFriendsResponse { friends: friends }))),
    }
}

#[derive(Deserialize)]
struct GetFriendsRequest {
    id: String,
}

#[derive(Serialize)]
struct GetFriendsResponse {
    friends: Vec<String>,
}

async fn post_delete_friend(Json(payload): Json<DeleteFriendRequest>) -> StatusCode {
    let response = delete_friend(&payload.id, &payload.friend_id);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct DeleteFriendRequest {
    id: String,
    friend_id: String,
}

//event routes

async fn post_create_event(Json(payload): Json<CreateEventRequest>) -> StatusCode {
    let response = create_event(payload.owner_id, payload.name, payload.reoccuring);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct CreateEventRequest {
    owner_id: String,
    name: String,
    reoccuring: bool,
}

async fn post_update_owner_id(Json(payload): Json<UpdateOwnerIdRequest>) -> StatusCode {
    let response = event_update_owner_id(payload.id, payload.owner_id);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct UpdateOwnerIdRequest {
    id: String,
    owner_id: String,
}

async fn post_event_update_name(Json(payload): Json<EventUpdateNameRequest>) -> StatusCode {
    let response = event_update_name(payload.id, payload.name);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct EventUpdateNameRequest {
    id: String,
    name: String,
}

async fn post_update_description(Json(payload): Json<UpdateDescriptionRequest>) -> StatusCode {
    let response = update_description(payload.id, payload.description);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct UpdateDescriptionRequest {
    id: String,
    description: String,
}

async fn post_update_date(Json(payload): Json<UpdateDateRequest>) -> StatusCode {
    let response = update_date(payload.id, payload.date);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct UpdateDateRequest {
    id: String,
    date: String,
}

async fn post_update_reoccuring(Json(payload): Json<UpdateReoccuringRequest>) -> StatusCode {
    let response = update_reoccuring(payload.id, payload.reoccuring);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct UpdateReoccuringRequest {
    id: String,
    reoccuring: bool,
}

async fn post_update_reoccuring_interval(Json(payload): Json<UpdateReoccuringIntervalRequest>) -> StatusCode {
    let response = update_reoccuring_interval(payload.id, payload.reoccuring_interval);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct UpdateReoccuringIntervalRequest {
    id: String,
    reoccuring_interval: String,
}

async fn post_update_final_date(Json(payload): Json<UpdateFinalDateRequest>) -> StatusCode {
    let response = update_final_date(payload.id, payload.final_date);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct UpdateFinalDateRequest {
    id: String,
    final_date: String,
}

async fn post_delete_event(Json(payload): Json<DeleteEventRequest>) -> StatusCode {
    let response = delete_event(payload.id);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct DeleteEventRequest {
    id: String,
}

async fn post_get_event(Json(payload): Json<GetEventRequest>) -> (StatusCode, Json<GetEventResponse>) {
    let response = get_event(payload.id);

    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(GetEventResponse { event: GetEvent { id: "".to_string(), owner_id: "".to_string(), name: "".to_string(), description: Some("".to_string()), event_date: Some("".to_string()), reoccuring: false, reoccuring_interval: Some("".to_string()), final_date: Some("".to_string()) } })),
        Ok(event) => (StatusCode::OK, Json(GetEventResponse { event: event })),
    }
}   

#[derive(Deserialize)]
struct GetEventRequest {
    id: String,
}

#[derive(Serialize)]
struct GetEventResponse {
    event: GetEvent,
}

async fn post_get_all_events(Json(payload): Json<GetAllEventsRequest>) -> (StatusCode, Json<GetAllEventsResponse>) {
    let response = get_all_events(payload.id);

    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(GetAllEventsResponse { events: vec![] })),
        Ok(events) => (StatusCode::OK, Json(GetAllEventsResponse { events: events })),
    }
}

#[derive(Deserialize)]
struct GetAllEventsRequest {
    id: String,
}

#[derive(Serialize)]
struct GetAllEventsResponse {
    events: Vec<GetEvent>,
}

//who in what routes

async fn post_create_who_in_what(Json(payload): Json<CreateWhoInWhatRequest>) -> StatusCode {
    let response = create_who_in_what(payload.user_id, payload.event_id);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct CreateWhoInWhatRequest {
    user_id: String,
    event_id: String,
}

async fn post_get_users_from_event(Json(payload): Json<GetUsersFromEventRequest>) -> (StatusCode, Json<GetUsersFromEventResponse>) {
    let response = wiw_get_users(payload.event_id);

    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(GetUsersFromEventResponse { users: vec![] })),
        Ok(users) => (StatusCode::OK, Json(GetUsersFromEventResponse { users: users }))
    }
}

#[derive(Deserialize)]
struct GetUsersFromEventRequest {
    event_id: String,
}

#[derive(Serialize)]
struct GetUsersFromEventResponse {
    users: Vec<String>,
}

async fn post_get_events_from_user(Json(payload): Json<GetEventsFromUserRequest>) -> (StatusCode, Json<GetEventsFromUserResponse>) {
    let response = wiw_get_events(payload.user_id);

    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(GetEventsFromUserResponse { events: vec![] })),
        Ok(events) => (StatusCode::OK, Json(GetEventsFromUserResponse { events: events }))
    }
}

#[derive(Deserialize)]
struct GetEventsFromUserRequest {
    user_id: String,
}

#[derive(Serialize)]
struct GetEventsFromUserResponse {
    events: Vec<String>,
}

async fn post_delete_who_in_what(Json(payload): Json<DeleteWhoInWhatRequest>) -> StatusCode {
    let response = delete_who_in_what(payload.user_id, payload.event_id);

    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

#[derive(Deserialize)]
struct DeleteWhoInWhatRequest {
    user_id: String,
    event_id: String,
}

*/








