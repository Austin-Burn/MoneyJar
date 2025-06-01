pub mod events;
pub mod friends;
pub mod transactions;
pub mod users;
pub mod who_in_what;

use crate::state::AppState;
use axum::Router;

pub fn all_routes() -> Router<AppState> {
    Router::new()
        .nest("/api", users::user_routes())
        .nest("/api", friends::friend_routes())
        .nest("/api", events::event_routes())
        .nest("/api", who_in_what::who_in_what_routes())
        .nest("/api", transactions::transaction_routes())
}
