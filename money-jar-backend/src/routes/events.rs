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
pub fn event_routes() -> Router<AppState> {
    Router::new()
        .route("/api/CreateEvent", post(post_create_event))
        .route("/api/GetEvent", post(post_get_event))
        .route("/api/GetAllEvents", post(post_get_all_events))
        .route("/api/UpdateEventOwnerId", post(post_update_owner_id))
        .route("/api/UpdateEventName", post(post_event_update_name))
        .route("/api/UpdateEventDescription", post(post_update_description))
        .route("/api/UpdateEventDate", post(post_update_date))
        .route("/api/UpdateEventReoccuring", post(post_update_reoccuring))
        .route("/api/UpdateEventReoccuringInterval", post(post_update_reoccuring_interval))
        .route("/api/UpdateEventFinalDate", post(post_update_final_date))
        .route("/api/DeleteEvent", post(post_delete_event))
}

// Request/Response structs
#[derive(Deserialize)]
struct CreateEventRequest {
    owner_id: String,
    name: String,
    reoccuring: bool,
}

#[derive(Deserialize)]
struct UpdateOwnerIdRequest {
    id: String,
    owner_id: String,
}

#[derive(Deserialize)]
struct EventUpdateNameRequest {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct UpdateDescriptionRequest {
    id: String,
    description: String,
}

#[derive(Deserialize)]
struct UpdateDateRequest {
    id: String,
    date: String,
}

#[derive(Deserialize)]
struct UpdateReoccuringRequest {
    id: String,
    reoccuring: bool,
}

#[derive(Deserialize)]
struct UpdateReoccuringIntervalRequest {
    id: String,
    reoccuring_interval: String,
}

#[derive(Deserialize)]
struct UpdateFinalDateRequest {
    id: String,
    final_date: String,
}

#[derive(Deserialize)]
struct DeleteEventRequest {
    id: String,
}

#[derive(Deserialize)]
struct GetEventRequest {
    id: String,
}

#[derive(Serialize)]
struct GetEventResponse {
    event: GetEvent,
}

#[derive(Deserialize)]
struct GetAllEventsRequest {
    id: String,
}

#[derive(Serialize)]
struct GetAllEventsResponse {
    events: Vec<GetEvent>,
}

#[derive(Serialize)]
struct CreateEventResponse {
    id: String,
}

// Route handlers
async fn post_create_event(
    State(state): State<AppState>,
    Json(payload): Json<CreateEventRequest>
) -> (StatusCode, Json<CreateEventResponse>) {
    let mut conn = state.pool.get().unwrap();
    let response = create_event(&mut conn, payload.owner_id, payload.name, payload.reoccuring);
    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(CreateEventResponse { id: String::new() })),
        Ok(id) => (StatusCode::OK, Json(CreateEventResponse { id }))
    }
}

async fn post_update_owner_id(
    State(state): State<AppState>,
    Json(payload): Json<UpdateOwnerIdRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = event_update_owner_id(&mut conn, payload.id, payload.owner_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_event_update_name(
    State(state): State<AppState>,
    Json(payload): Json<EventUpdateNameRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = event_update_name(&mut conn, payload.id, payload.name);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_description(
    State(state): State<AppState>,
    Json(payload): Json<UpdateDescriptionRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = update_description(&mut conn, payload.id, payload.description);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_date(
    State(state): State<AppState>,
    Json(payload): Json<UpdateDateRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = update_date(&mut conn, payload.id, payload.date);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_reoccuring(
    State(state): State<AppState>,
    Json(payload): Json<UpdateReoccuringRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = update_reoccuring(&mut conn, payload.id, payload.reoccuring);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_reoccuring_interval(
    State(state): State<AppState>,
    Json(payload): Json<UpdateReoccuringIntervalRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = update_reoccuring_interval(&mut conn, payload.id, payload.reoccuring_interval);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_final_date(
    State(state): State<AppState>,
    Json(payload): Json<UpdateFinalDateRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = update_final_date(&mut conn, payload.id, payload.final_date);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_delete_event(
    State(state): State<AppState>,
    Json(payload): Json<DeleteEventRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = delete_event(&mut conn, payload.id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_get_event(
    State(state): State<AppState>,
    Json(payload): Json<GetEventRequest>
) -> (StatusCode, Json<GetEventResponse>) {
    let mut conn = state.pool.get().unwrap();
    match get_event(&mut conn, payload.id) {
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(GetEventResponse { 
                event: GetEvent { 
                    id: String::new(), 
                    owner_id: String::new(), 
                    name: String::new(), 
                    description: None, 
                    event_date: None, 
                    reoccuring: false, 
                    reoccuring_interval: None, 
                    final_date: None 
                } 
            })
        ),
        Ok(event) => (
            StatusCode::OK,
            Json(GetEventResponse { event })
        ),
    }
}

async fn post_get_all_events(
    State(state): State<AppState>,
    Json(payload): Json<GetAllEventsRequest>
) -> (StatusCode, Json<GetAllEventsResponse>) {
    let mut conn = state.pool.get().unwrap();
    let response = get_all_events(&mut conn, payload.id);
    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(GetAllEventsResponse { events: vec![] })),
        Ok(events) => (StatusCode::OK, Json(GetAllEventsResponse { events }))
    }
} 