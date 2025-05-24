use axum::{
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use money_jar_core::*;

// Route definitions
pub fn friend_routes() -> Router {
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
async fn post_create_friend(Json(payload): Json<CreateFriendRequest>) -> StatusCode {
    let response = create_friend(payload.id, payload.friend_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_get_friends(Json(payload): Json<GetFriendsRequest>) -> Result<(StatusCode, Json<GetFriendsResponse>), (StatusCode, Json<String>)> {
    let response = get_friends(payload.id);
    match response {
        Err(e) => Err((StatusCode::NOT_FOUND, Json(e.to_string()))),
        Ok(friends) => Ok((StatusCode::OK, Json(GetFriendsResponse { friends })))
    }
}

async fn post_delete_friend(Json(payload): Json<DeleteFriendRequest>) -> StatusCode {
    let response = delete_friend(&payload.id, &payload.friend_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
} 