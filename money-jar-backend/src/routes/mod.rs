pub mod users;
pub mod friends;
pub mod events;
pub mod who_in_what;

use axum::Router;

pub fn all_routes() -> Router {
    Router::new()
        .merge(users::user_routes())
        .merge(friends::friend_routes())
        .merge(events::event_routes())
        .merge(who_in_what::who_in_what_routes())
} 