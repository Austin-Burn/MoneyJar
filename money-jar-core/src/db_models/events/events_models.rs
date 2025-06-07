use crate::schema::Events;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::Serialize;

// template for structs

//struct TemplateEvents {
//    id: String,
//    owner_id: String,
//    name: String,
//    description: Option<String>,
//    date: Option<String>,
//    reoccuring: bool,
//    reoccuring_interval: Option<String>,
//    final_date: Option<String>
//}

// DB MODELS

#[derive(Insertable)]
#[diesel(table_name = Events)]
pub struct NewEvent {
    id: String,
    owner_id: String,
    name: String,
    reoccuring: bool,
}

impl NewEvent {
    pub fn new(id: String, owner_id: String, name: String, reoccuring: bool) -> Self {
        Self {
            id,
            owner_id,
            name,
            reoccuring,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Events)]
pub struct UpdateOwnerId {
    owner_id: String,
}

impl UpdateOwnerId {
    pub fn new(owner_id: String) -> Self {
        Self { owner_id }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Events)]
pub struct EventUpdateName {
    name: String,
}

impl EventUpdateName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Events)]
pub struct UpdateDescription {
    description: String,
}

impl UpdateDescription {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Events)]
pub struct UpdateEventDate {
    event_date: String,
}

impl UpdateEventDate {
    pub fn new(event_date: String) -> Self {
        Self { event_date }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Events)]
pub struct UpdateReoccuring {
    reoccuring: bool,
}

impl UpdateReoccuring {
    pub fn new(reoccuring: bool) -> Self {
        Self { reoccuring }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Events)]
pub struct UpdateReoccuringInterval {
    reoccuring_interval: String,
}

impl UpdateReoccuringInterval {
    pub fn new(reoccuring_interval: String) -> Self {
        Self {
            reoccuring_interval,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = Events)]
pub struct UpdateFinalDate {
    final_date: String,
}

impl UpdateFinalDate {
    pub fn new(final_date: String) -> Self {
        Self { final_date }
    }
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = Events)]
#[serde(rename_all = "camelCase")]
pub struct GetEvent {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub description: Option<String>,
    pub event_date: Option<String>,
    pub reoccuring: bool,
    pub reoccuring_interval: Option<String>,
    pub final_date: Option<String>,
}
