use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use money_jar_core::db_models::items::{GetItem, NewItem, UpdateItem};
use money_jar_core::db_models::errors::ModelError;

pub fn item_routes() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/items", axum::routing::post(create_item))
        .route("/items", axum::routing::get(get_all_items))
        .route("/items/:id", axum::routing::get(get_item))
        .route("/items/:id", axum::routing::put(update_item))
        .route("/items/:id", axum::routing::delete(delete_item))
}

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub event_id: String,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub event_id: String,
}

impl From<GetItem> for ItemResponse {
    fn from(item: GetItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            price: item.price,
            event_id: item.event_id,
        }
    }
}

async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    let new_item = NewItem {
        name: payload.name,
        description: payload.description,
        price: payload.price,
        event_id: payload.event_id,
    };

    let item = money_jar_core::db_models::items::create_item(&mut conn, new_item)
        .map_err(|e| match e {
            ModelError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            ModelError::BusinessRule(msg) => (StatusCode::BAD_REQUEST, msg),
        })?;

    Ok((StatusCode::CREATED, Json(ItemResponse::from(item))))
}

async fn get_all_items(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    let items = money_jar_core::db_models::items::get_all(&mut conn).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get items: {}", e),
        )
    })?;

    Ok((StatusCode::OK, Json(items.into_iter().map(ItemResponse::from).collect::<Vec<_>>())))
}

async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    let item = money_jar_core::db_models::items::get_by_id(&mut conn, id).map_err(|e| match e {
        ModelError::Database(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Item not found".to_string())
        }
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get item: {}", e),
        ),
    })?;

    Ok((StatusCode::OK, Json(ItemResponse::from(item))))
}

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
}

async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    let update_item = UpdateItem {
        name: payload.name,
        description: payload.description,
        price: payload.price,
    };

    let item = money_jar_core::db_models::items::update_item(&mut conn, id, update_item)
        .map_err(|e| match e {
            ModelError::Database(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, "Item not found".to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to update item: {}", e),
            ),
        })?;

    Ok((StatusCode::OK, Json(ItemResponse::from(item))))
}

async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    money_jar_core::db_models::items::delete_item(&mut conn, id).map_err(|e| match e {
        ModelError::Database(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Item not found".to_string())
        }
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete item: {}", e),
        ),
    })?;

    Ok(StatusCode::NO_CONTENT)
} 