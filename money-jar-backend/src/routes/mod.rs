pub mod users;
pub mod friends;
pub mod events;
pub mod who_in_what;
pub mod transactions;

use axum::Router;
use crate::state::AppState;

pub fn all_routes() -> Router<AppState> {
    Router::new()
        .merge(users::user_routes())
        .merge(friends::friend_routes())
        .merge(events::event_routes())
        .merge(who_in_what::who_in_what_routes())
        .merge(transactions::transaction_routes())
} 