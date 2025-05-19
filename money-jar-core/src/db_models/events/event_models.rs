use diesel::{Insertable, Queryable, Identifiable};
use crate::schema::Events;






// template for structs

struct TemplateEvents {
    id: String,
    owner_id: String,
    name: String,
    description: String,
    date: String,
    reoccuring: bool,
    reoccuring_interval: String,
    final_date: String
}


// DB MODELS

#[derive(Insertable)]
#[diesel(table_name = Events)]
pub struct NewEvent {
    id: String,
    owner_id: String,
    name: String,
    reoccuring: bool
}

impl NewEvent {
    pub fn new(id: String, owner_id: String, name: String, reoccuring: bool) -> Self {
        Self { id, owner_id, name, reoccuring }
    }
}


#[derive(Insertable)]
#[diesel(table_name = Events)]
pub struct UpdateDescription {
    description: String
}

impl UpdateDescription {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}


#[derive(Insertable)]
#[diesel(table_name = Events)]
pub struct UpdateDate {
    date: String
}

impl UpdateDate {
    pub fn new(date: String) -> Self {
        Self { date }
    }
}


#[derive(Insertable)]
#[diesel(table_name = Events)]
pub struct UpdateReoccuring {
    reoccuring: bool
}

impl UpdateReoccuring {
    pub fn new(reoccuring: bool) -> Self {
        Self { reoccuring }
    }
}


#[derive(Insertable)]
#[diesel(table_name = Events)]
pub struct UpdateReoccuringInterval {
    reoccuring_interval: String
}

impl UpdateReoccuringInterval {
    pub fn new(reoccuring_interval: String) -> Self {
        Self { reoccuring_interval }
    }
}


#[derive(Insertable)]
#[diesel(table_name = Events)]
pub struct UpdateFinalDate {
    final_date: String
}   

impl UpdateFinalDate {
    pub fn new(final_date: String) -> Self {
        Self { final_date }
    }
}


#[derive(Insertable)]
#[diesel(table_name = Events)]
pub struct UpdateName {
    name: String
}

impl UpdateName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}













