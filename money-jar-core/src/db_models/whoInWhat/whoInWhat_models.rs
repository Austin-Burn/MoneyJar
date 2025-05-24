use diesel::{Insertable, Queryable};
use crate::schema::WhoInWhat;





//struct TemplateWhoInWhat {
//    user_id: String,
//    event_id: String
//}


//DB MODELS

#[derive(Insertable)]
#[diesel(table_name = WhoInWhat)]
pub struct NewWhoInWhat {
    user_id: String,
    event_id: String
}

impl NewWhoInWhat {
    pub fn new(user_id: String, event_id: String) -> Self {
        Self { user_id, event_id }
    }
}

#[derive(Queryable)]
#[diesel(table_name = WhoInWhat)]
pub struct WIWGetUsers {
    pub user_id: String
}


impl WIWGetUsers {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

#[derive(Queryable)]
#[diesel(table_name = WhoInWhat)]
pub struct WIWGetEvents {
    pub event_id: String
}

impl WIWGetEvents {
    pub fn new(event_id: String) -> Self {
        Self { event_id }
    }
}













