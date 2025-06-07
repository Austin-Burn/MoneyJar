use diesel::{Insertable, Queryable, Identifiable, AsChangeset};
use crate::schema::Items;

#[derive(Insertable)]
#[diesel(table_name = Items)]
pub struct NewItem {
    pub name: String,
    pub description: Option<String>,
    pub cost: i32,
    pub payment_progress: i32,
    pub total: i32,
    pub recurring: bool,
    pub iteration_count: i32,
    pub event_id: String
}

impl NewItem {
    pub fn new(name: String, description: Option<String>, cost: i32, payment_progress: i32, total: i32, recurring: bool, iteration_count: i32, event_id: String) -> Self {
        Self { name, description, cost, payment_progress, total, recurring, iteration_count, event_id }
    }
}

#[derive(Queryable, Identifiable)]
#[diesel(table_name = Items)]
pub struct GetItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cost: i32,
    pub payment_progress: i32,
    pub total: i32,
    pub recurring: bool,
    pub iteration_count: i32,
    pub event_id: String
}

#[derive(AsChangeset)]
#[diesel(table_name = Items)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cost: Option<i32>,
    pub payment_progress: Option<i32>,
    pub total: Option<i32>,
    pub recurring: Option<bool>,
    pub iteration_count: Option<i32>
}

impl UpdateItem {
    pub fn new(name: Option<String>, description: Option<String>, cost: Option<i32>, payment_progress: Option<i32>, total: Option<i32>, recurring: Option<bool>, iteration_count: Option<i32>) -> Self {
        Self { name, description, cost, payment_progress, total, recurring, iteration_count }
    }
} 