use super::items_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use diesel::sqlite::SqliteConnection;
use crate::{establish_connection};
use crate::Items::dsl::Items;
use crate::Items::*;
use crate::db_models::errors::ModelError;

pub fn create_item(conn: &mut SqliteConnection, new_item: NewItem) -> Result<GetItem, ModelError> {
    let item = NewItem::new(
        new_item.name,
        new_item.description,
        new_item.cost,
        new_item.payment_progress,
        new_item.total,
        new_item.recurring,
        new_item.iteration_count,
        new_item.event_id
    );
    
    insert_into(Items)
        .values(item)
        .get_result(conn)
        .map_err(|e| ModelError::Database(e))
}

pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<GetItem>, ModelError> {
    let items = Items.load::<GetItem>(conn)?;
    Ok(items)
}

pub fn get_by_id(conn: &mut SqliteConnection, item_id: &str) -> Result<GetItem, ModelError> {
    let item = Items
        .filter(id.eq(item_id))
        .first::<GetItem>(conn)?;
    Ok(item)
}

pub fn get_by_event(conn: &mut SqliteConnection, foreign_event_id: &str) -> Result<Vec<GetItem>, ModelError> {
    let items = Items
        .filter(event_id.eq(foreign_event_id))
        .load::<GetItem>(conn)?;
    Ok(items)
}

pub fn update_item_name(conn: &mut SqliteConnection, item_id: &str, new_name: String) -> Result<GetItem, ModelError> {
    update(Items.filter(id.eq(item_id)))
        .set(name.eq(new_name))
        .get_result(conn)
        .map_err(ModelError::Database)
}

pub fn update_item_description(conn: &mut SqliteConnection, item_id: &str, new_description: Option<String>) -> Result<GetItem, ModelError> {
    update(Items.filter(id.eq(item_id)))
        .set(description.eq(new_description))
        .get_result(conn)
        .map_err(ModelError::Database)
}

pub fn update_item_cost(conn: &mut SqliteConnection, item_id: &str, new_cost: i32) -> Result<GetItem, ModelError> {
    update(Items.filter(id.eq(item_id)))
        .set(cost.eq(new_cost))
        .get_result(conn)
        .map_err(ModelError::Database)
}

pub fn update_item_payment_progress(conn: &mut SqliteConnection, item_id: &str, new_payment_progress: i32) -> Result<GetItem, ModelError> {
    update(Items.filter(id.eq(item_id)))
        .set(payment_progress.eq(new_payment_progress))
        .get_result(conn)
        .map_err(ModelError::Database)
}

pub fn update_item_total(conn: &mut SqliteConnection, item_id: &str, new_total: i32) -> Result<GetItem, ModelError> {
    update(Items.filter(id.eq(item_id)))
        .set(total.eq(new_total))
        .get_result(conn)
        .map_err(ModelError::Database)
}

pub fn update_item_recurring(conn: &mut SqliteConnection, item_id: &str, new_recurring: bool) -> Result<GetItem, ModelError> {
    update(Items.filter(id.eq(item_id)))
        .set(recurring.eq(new_recurring))
        .get_result(conn)
        .map_err(ModelError::Database)
}

pub fn update_item_iteration_count(conn: &mut SqliteConnection, item_id: &str, new_iteration_count: i32) -> Result<GetItem, ModelError> {
    update(Items.filter(id.eq(item_id)))
        .set(iteration_count.eq(new_iteration_count))
        .get_result(conn)
        .map_err(ModelError::Database)
}

pub fn delete_item(conn: &mut SqliteConnection, item_id: &str) -> Result<(), ModelError> {
    delete(Items.filter(id.eq(item_id)))
        .execute(conn)?;
    Ok(())
} 