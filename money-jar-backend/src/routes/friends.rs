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
pub fn friend_routes() -> Router<AppState> {
    Router::new()
        .route("/api/CreateFriend", post(post_create_friend))
        .route("/api/GetFriends", post(post_get_friends))
        .route("/api/DeleteFriend", post(post_delete_friend))
}

// Request/Response structs
#[derive(Deserialize)]
struct CreateFriendRequest {
    id: String,
    friend_id: String,
}

#[derive(Deserialize)]
struct GetFriendsRequest {
    id: String,
}

#[derive(Serialize)]
struct GetFriendsResponse {
    friends: Vec<String>,
}

#[derive(Deserialize)]
struct DeleteFriendRequest {
    id: String,
    friend_id: String,
}

// Route handlers
async fn post_create_friend(
    State(state): State<AppState>,
    Json(payload): Json<CreateFriendRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = create_friend(&mut conn, payload.id, payload.friend_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_get_friends(
    State(state): State<AppState>,
    Json(payload): Json<GetFriendsRequest>
) -> Result<(StatusCode, Json<GetFriendsResponse>), (StatusCode, Json<String>)> {
    let mut conn = state.pool.get().unwrap();
    let response = get_friends(&mut conn, payload.id);
    match response {
        Err(e) => Err((StatusCode::NOT_FOUND, Json(e.to_string()))),
        Ok(friends) => Ok((StatusCode::OK, Json(GetFriendsResponse { friends })))
    }
}

async fn post_delete_friend(
    State(state): State<AppState>,
    Json(payload): Json<DeleteFriendRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = delete_friend(&mut conn, &payload.id, &payload.friend_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
} 