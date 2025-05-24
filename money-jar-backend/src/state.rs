use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

#[derive(Clone)]
pub struct AppState {
    pub pool: r2d2::Pool<ConnectionManager<SqliteConnection>>,
} 