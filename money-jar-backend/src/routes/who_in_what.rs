use axum::{
    http::StatusCode,
    routing::post,
    Json,
    Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use money_jar_core::*;
use crate::state::AppState;

// Route definitions
pub fn who_in_what_routes() -> Router<AppState> {
    Router::new()
        .route("/api/CreateWhoInWhat", post(post_create_who_in_what))
        .route("/api/GetUsersFromEvent", post(post_get_users_from_event))
        .route("/api/GetEventsFromUser", post(post_get_events_from_user))
        .route("/api/DeleteWhoInWhat", post(post_delete_who_in_what))
}

// Request/Response structs
#[derive(Deserialize)]
struct CreateWhoInWhatRequest {
    user_id: String,
    event_id: String,
}

#[derive(Deserialize)]
struct GetUsersFromEventRequest {
    event_id: String,
}

#[derive(Serialize)]
struct GetUsersFromEventResponse {
    users: Vec<String>,
}

#[derive(Deserialize)]
struct GetEventsFromUserRequest {
    user_id: String,
}

#[derive(Serialize)]
struct GetEventsFromUserResponse {
    events: Vec<String>,
}

#[derive(Deserialize)]
struct DeleteWhoInWhatRequest {
    user_id: String,
    event_id: String,
}

// Route handlers
async fn post_create_who_in_what(
    State(state): State<AppState>,
    Json(payload): Json<CreateWhoInWhatRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = create_who_in_what(&mut conn, payload.user_id, payload.event_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_get_users_from_event(
    State(state): State<AppState>,
    Json(payload): Json<GetUsersFromEventRequest>
) -> (StatusCode, Json<GetUsersFromEventResponse>) {
    let mut conn = state.pool.get().unwrap();
    let response = wiw_get_users(&mut conn, payload.event_id);
    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(GetUsersFromEventResponse { users: vec![] })),
        Ok(users) => (StatusCode::OK, Json(GetUsersFromEventResponse { users }))
    }
}

async fn post_get_events_from_user(
    State(state): State<AppState>,
    Json(payload): Json<GetEventsFromUserRequest>
) -> (StatusCode, Json<GetEventsFromUserResponse>) {
    let mut conn = state.pool.get().unwrap();
    let response = wiw_get_events(&mut conn, payload.user_id);
    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(GetEventsFromUserResponse { events: vec![] })),
        Ok(events) => (StatusCode::OK, Json(GetEventsFromUserResponse { events }))
    }
}

async fn post_delete_who_in_what(
    State(state): State<AppState>,
    Json(payload): Json<DeleteWhoInWhatRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = delete_who_in_what(&mut conn, payload.user_id, payload.event_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
} 