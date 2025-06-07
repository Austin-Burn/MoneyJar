pub mod users;
pub mod friends;
pub mod events;
pub mod who_in_what;
pub mod transactions;
pub mod items;
pub mod pay_batches;

use axum::Router;
use crate::state::AppState;

pub fn all_routes() -> Router<AppState> {
    Router::new()
        .merge(users::user_routes())
        .merge(friends::friend_routes())
        .merge(events::event_routes())
        .merge(who_in_what::who_in_what_routes())
        .merge(transactions::transaction_routes())
        .merge(items::item_routes())
        .merge(pay_batches::pay_batch_routes())
} 