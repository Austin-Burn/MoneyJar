use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = who_in_what)]
pub struct DB_WhoInWhat_Insert {
    pub user_id: String,
    pub event_id: String,
}

impl DB_WhoInWhat_Insert {
    pub fn new(user_id: String, event_id: String) -> Self {
        Self {
            user_id,
            event_id,
        }
    }
}

#[derive(Queryable)]
#[diesel(table_name = who_in_what)]
pub struct DB_WhoInWhat_Select {
    pub user_id: String,
    pub event_id: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = who_in_what)]
pub struct DB_WhoInWhat_Update {
    pub user_id: String,
    pub event_id: String,
} 