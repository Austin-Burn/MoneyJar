use crate::state::AppState;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use money_jar_core::*;
use serde::{Deserialize, Serialize};

// Route definitions
pub fn friend_routes() -> Router<AppState> {
    Router::new()
        .route("/CreateFriend", post(post_create_friend))
        .route("/GetFriends", post(post_get_friends))
        .route("/DeleteFriend", post(post_delete_friend))
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
    Json(payload): Json<CreateFriendRequest>,
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = create_friend(&mut conn, payload.id, payload.friend_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK,
    }
}

async fn post_get_friends(
    State(state): State<AppState>,
    Json(payload): Json<GetFriendsRequest>,
) -> Result<(StatusCode, Json<GetFriendsResponse>), (StatusCode, Json<String>)> {
    let mut conn = state.pool.get().unwrap();
    let response = get_friends(&mut conn, payload.id);
    match response {
        Err(e) => Err((StatusCode::NOT_FOUND, Json(e.to_string()))),
        Ok(friends) => Ok((StatusCode::OK, Json(GetFriendsResponse { friends }))),
    }
}

async fn post_delete_friend(
    State(state): State<AppState>,
    Json(payload): Json<DeleteFriendRequest>,
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = delete_friend(&mut conn, &payload.id, &payload.friend_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK,
    }
}
