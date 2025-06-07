pub mod authentication;
pub mod events;
pub mod friends;
pub mod transactions;
pub mod users;
pub mod who_in_what;
pub mod items;
pub mod pay_batches;

use axum::Router;
use crate::state::AppState;

pub fn all_routes() -> Router<AppState> {
    Router::new()
        .nest("/api", users::user_routes())
        .nest("/api", friends::friend_routes())
        .nest("/api", events::event_routes())
        .nest("/api", who_in_what::who_in_what_routes())
        .nest("/api", transactions::transaction_routes())
        .nest("/api", authentication::auth_routes())
        .merge(items::item_routes())
        .merge(pay_batches::pay_batch_routes())
}