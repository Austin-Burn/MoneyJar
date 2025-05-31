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
pub fn transaction_routes() -> Router<AppState> {
    Router::new()
        .route("/api/CreateTransaction", post(post_create_transaction))
        .route("/api/GetTransactionsBySent", post(post_get_transactions_by_sent))
        .route("/api/GetTransactionsByReceived", post(post_get_transactions_by_received))
        .route("/api/GetTransactionsByEvent", post(post_get_transactions_by_event))
        .route("/api/UpdateTransactionComment", post(post_update_transaction_comment))
        .route("/api/DeleteTransaction", post(post_delete_transaction))
}

// Request/Response structs

#[derive(Deserialize)]
struct CreateTransactionRequest {
    from_user_id: String,
    to_user_id: String,
    event_id: String,
    amount: i32,
    date: String,
    payment_method: String,
    comment: Option<String>,
}

#[derive(Deserialize)]
struct GetTransactionsBySentRequest {
    from_user_id: String,
}

#[derive(Deserialize)]
struct GetTransactionsByReceivedRequest {
    to_user_id: String,
}

#[derive(Deserialize)]
struct GetTransactionsByEventRequest {
    event_id: String,
}

#[derive(Deserialize)]
struct UpdateTransactionCommentRequest {
    transaction_id: i32,
    comment: String,
}

#[derive(Deserialize)]
struct DeleteTransactionRequest {
    transaction_id: i32,
}

#[derive(Serialize)]
struct TransactionResponse {
    id: i32,
    from_user_id: String,
    to_user_id: String,
    event_id: String,
    amount: i32,
    date: String,
    payment_method: String,
    comment: Option<String>,
}

// Route handlers
async fn post_create_transaction(
    State(state): State<AppState>,
    Json(payload): Json<CreateTransactionRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = create_transaction(
        &mut conn,
        payload.from_user_id,
        payload.to_user_id,
        payload.event_id,
        payload.amount,
        payload.date,
        payload.payment_method,
        payload.comment,
    );
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_get_transactions_by_sent(
    State(state): State<AppState>,
    Json(payload): Json<GetTransactionsBySentRequest>
) -> (StatusCode, Json<Vec<TransactionResponse>>) {
    let mut conn = state.pool.get().unwrap();
    let response = get_transaction_by_sent(&mut conn, payload.from_user_id);
    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(Vec::new())),
        Ok(transactions) => {
            let response_transactions: Vec<TransactionResponse> = transactions
                .into_iter()
                .map(|t| TransactionResponse {
                    id: t.id,
                    from_user_id: t.from_user_id,
                    to_user_id: t.to_user_id,
                    event_id: t.event_id,
                    amount: t.amount,
                    date: t.date,
                    payment_method: t.payment_method,
                    comment: t.comment,
                })
                .collect();
            (StatusCode::OK, Json(response_transactions))
        }
    }
}

async fn post_get_transactions_by_received(
    State(state): State<AppState>,
    Json(payload): Json<GetTransactionsByReceivedRequest>
) -> (StatusCode, Json<Vec<TransactionResponse>>) {
    let mut conn = state.pool.get().unwrap();
    let response = get_transaction_by_received(&mut conn, payload.to_user_id);
    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(Vec::new())),
        Ok(transactions) => {
            let response_transactions: Vec<TransactionResponse> = transactions
                .into_iter()
                .map(|t| TransactionResponse {
                    id: t.id,
                    from_user_id: t.from_user_id,
                    to_user_id: t.to_user_id,
                    event_id: t.event_id,
                    amount: t.amount,
                    date: t.date,
                    payment_method: t.payment_method,
                    comment: t.comment,
                })
                .collect();
            (StatusCode::OK, Json(response_transactions))
        }
    }
}

async fn post_get_transactions_by_event(
    State(state): State<AppState>,
    Json(payload): Json<GetTransactionsByEventRequest>
) -> (StatusCode, Json<Vec<TransactionResponse>>) {
    let mut conn = state.pool.get().unwrap();
    let response = get_transaction_by_event(&mut conn, payload.event_id);
    match response {
        Err(_) => (StatusCode::NOT_FOUND, Json(Vec::new())),
        Ok(transactions) => {
            let response_transactions: Vec<TransactionResponse> = transactions
                .into_iter()
                .map(|t| TransactionResponse {
                    id: t.id,
                    from_user_id: t.from_user_id,
                    to_user_id: t.to_user_id,
                    event_id: t.event_id,
                    amount: t.amount,
                    date: t.date,
                    payment_method: t.payment_method,
                    comment: t.comment,
                })
                .collect();
            (StatusCode::OK, Json(response_transactions))
        }
    }
}

async fn post_update_transaction_comment(
    State(state): State<AppState>,
    Json(payload): Json<UpdateTransactionCommentRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = update_transaction_comment(&mut conn, payload.transaction_id, payload.comment);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
}

async fn post_delete_transaction(
    State(state): State<AppState>,
    Json(payload): Json<DeleteTransactionRequest>
) -> StatusCode {
    let mut conn = state.pool.get().unwrap();
    let response = delete_transaction(&mut conn, payload.transaction_id);
    match response {
        Err(_) => StatusCode::NOT_FOUND,
        Ok(_) => StatusCode::OK
    }
} 