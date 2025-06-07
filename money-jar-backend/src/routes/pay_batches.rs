use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use money_jar_core::db_models::payBatches::GetPayBatch;
use money_jar_core::db_models::errors::ModelError;

pub fn pay_batch_routes() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/pay-batches", axum::routing::post(create_pay_batch_group))
        .route("/pay-batches/:id", axum::routing::get(get_pay_batches_by_id))
        .route("/pay-batches/transaction/:transaction_id", axum::routing::get(get_pay_batches_by_transaction))
        .route("/pay-batches/:id", axum::routing::delete(delete_pay_batch_group))
        .route("/pay-batches/:id/transaction/:transaction_id", axum::routing::delete(delete_pay_batch))
}

#[derive(Deserialize)]
pub struct CreatePayBatchRequest {
    pub transaction_ids: Vec<String>,
    pub date: String,
}

#[derive(Serialize)]
pub struct PayBatchResponse {
    pub id: String,
    pub transaction_id: String,
    pub date: String,
}

impl From<GetPayBatch> for PayBatchResponse {
    fn from(batch: GetPayBatch) -> Self {
        Self {
            id: batch.id,
            transaction_id: batch.transaction_id,
            date: batch.date,
        }
    }
}

async fn create_pay_batch_group(
    State(state): State<AppState>,
    Json(payload): Json<CreatePayBatchRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    money_jar_core::db_models::payBatches::create_pay_batch_group(
        &mut conn,
        payload.transaction_ids,
        payload.date,
    )
    .map_err(|e| match e {
        ModelError::Database(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        ),
        ModelError::BusinessRule(msg) => (StatusCode::BAD_REQUEST, msg),
    })?;

    Ok(StatusCode::CREATED)
}

async fn get_pay_batches_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    let batches = money_jar_core::db_models::payBatches::get_pay_batches_by_id(&mut conn, id)
        .map_err(|e| match e {
            ModelError::Database(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, "Pay batch not found".to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get pay batches: {}", e),
            ),
        })?;

    Ok((StatusCode::OK, Json(batches.into_iter().map(PayBatchResponse::from).collect::<Vec<_>>())))
}

async fn get_pay_batches_by_transaction(
    State(state): State<AppState>,
    Path(transaction_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    let batches = money_jar_core::db_models::payBatches::get_pay_batches_by_transaction(&mut conn, transaction_id)
        .map_err(|e| match e {
            ModelError::Database(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, "Pay batch not found".to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get pay batches: {}", e),
            ),
        })?;

    Ok((StatusCode::OK, Json(batches.into_iter().map(PayBatchResponse::from).collect::<Vec<_>>())))
}

async fn delete_pay_batch_group(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    money_jar_core::db_models::payBatches::delete_pay_batch_group(&mut conn, id)
        .map_err(|e| match e {
            ModelError::Database(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, "Pay batch not found".to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete pay batch: {}", e),
            ),
        })?;

    Ok(StatusCode::NO_CONTENT)
}

async fn delete_pay_batch(
    State(state): State<AppState>,
    Path((id, transaction_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    money_jar_core::db_models::payBatches::delete_pay_batch(&mut conn, id, transaction_id)
        .map_err(|e| match e {
            ModelError::Database(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, "Pay batch not found".to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete pay batch: {}", e),
            ),
        })?;

    Ok(StatusCode::NO_CONTENT)
} 