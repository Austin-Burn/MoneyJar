use axum::{
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use money_jar_core::*;

// Route definitions
pub fn event_routes() -> Router {
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

// Route handlers
async fn post_create_event(Json(payload): Json<CreateEventRequest>) -> StatusCode {
    let response = create_event(payload.owner_id, payload.name, payload.reoccuring);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_owner_id(Json(payload): Json<UpdateOwnerIdRequest>) -> StatusCode {
    let response = event_update_owner_id(payload.id, payload.owner_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_event_update_name(Json(payload): Json<EventUpdateNameRequest>) -> StatusCode {
    let response = event_update_name(payload.id, payload.name);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_description(Json(payload): Json<UpdateDescriptionRequest>) -> StatusCode {
    let response = update_description(payload.id, payload.description);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_date(Json(payload): Json<UpdateDateRequest>) -> StatusCode {
    let response = update_date(payload.id, payload.date);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_reoccuring(Json(payload): Json<UpdateReoccuringRequest>) -> StatusCode {
    let response = update_reoccuring(payload.id, payload.reoccuring);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_reoccuring_interval(Json(payload): Json<UpdateReoccuringIntervalRequest>) -> StatusCode {
    let response = update_reoccuring_interval(payload.id, payload.reoccuring_interval);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_update_final_date(Json(payload): Json<UpdateFinalDateRequest>) -> StatusCode {
    let response = update_final_date(payload.id, payload.final_date);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_delete_event(Json(payload): Json<DeleteEventRequest>) -> StatusCode {
    let response = delete_event(payload.id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_get_event(Json(payload): Json<GetEventRequest>) -> (StatusCode, Json<GetEventResponse>) {
    match get_event(payload.id) {
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

async fn post_get_all_events(Json(payload): Json<GetAllEventsRequest>) -> (StatusCode, Json<GetAllEventsResponse>) {
    let response = get_all_events(payload.id);
    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(GetAllEventsResponse { events: vec![] })),
        Ok(events) => (StatusCode::OK, Json(GetAllEventsResponse { events }))
    }
} 