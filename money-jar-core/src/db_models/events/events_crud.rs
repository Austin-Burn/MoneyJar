use super::events_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use diesel::sqlite::SqliteConnection;
use uuid::Uuid;
use crate::Events::dsl::Events;
use crate::Events::*;
use crate::WhoInWhat::dsl::WhoInWhat;
use crate::whoInWhat::whoInWhat_models::NewWhoInWhat;
use crate::db_models::errors::ModelError;

pub fn create_event(conn: &mut SqliteConnection, event_owner_id: String, event_name: String, event_reoccuring: bool) -> Result<String, ModelError> {
    let results = conn.transaction::<String, ModelError, _>(|conn| {
        let event_id = Uuid::new_v4().to_string();
        let event = NewEvent::new(event_id.clone(), event_owner_id.clone(), event_name, event_reoccuring);
    
        // Create the event
        insert_into(Events)
            .values(event)
            .execute(conn)?;

        // Add the owner to the who_in_what table
        let who_in_what = NewWhoInWhat::new(event_owner_id, event_id.clone());
        insert_into(WhoInWhat)
            .values(who_in_what)
            .execute(conn)?;

        Ok(event_id)
    })?;

    Ok(results)
}

pub fn event_update_owner_id(conn: &mut SqliteConnection, event_id: String, user_owner_id: String) -> Result<(), ModelError> {
    update(Events)
        .filter(id.eq(event_id))
        .set(owner_id.eq(user_owner_id))
        .execute(conn)?;
    Ok(())
}

pub fn get_event(conn: &mut SqliteConnection, event_id: String) -> Result<GetEvent, ModelError> {
    let event = Events.filter(owner_id.eq(event_id))
        .first::<GetEvent>(conn)?;
    Ok(event)
}

pub fn update_description(conn: &mut SqliteConnection, event_id: String, user_description: String) -> Result<(), ModelError> {
    update(Events)
        .filter(id.eq(event_id))
        .set(description.eq(user_description))
        .execute(conn)?;
    Ok(())
}

pub fn update_date(conn: &mut SqliteConnection, event_id: String, user_date: String) -> Result<(), ModelError> {
    update(Events)
        .filter(id.eq(event_id))
        .set(event_date.eq(user_date))
        .execute(conn)?;
    Ok(())
}

pub fn update_reoccuring(conn: &mut SqliteConnection, event_id: String, user_reoccuring: bool) -> Result<(), ModelError> {
    update(Events)
        .filter(id.eq(event_id))
        .set(reoccuring.eq(user_reoccuring))
        .execute(conn)?;
    Ok(())
}

pub fn update_reoccuring_interval(conn: &mut SqliteConnection, event_id: String, user_reoccuring_interval: String) -> Result<(), ModelError> {
    update(Events)
        .filter(id.eq(event_id))
        .set(reoccuring_interval.eq(user_reoccuring_interval))
        .execute(conn)?;
    Ok(())
}

pub fn update_final_date(conn: &mut SqliteConnection, event_id: String, user_final_date: String) -> Result<(), ModelError> {
    update(Events)
        .filter(id.eq(event_id))
        .set(final_date.eq(user_final_date))
        .execute(conn)?;
    Ok(())
}

pub fn event_update_name(conn: &mut SqliteConnection, event_id: String, user_name: String) -> Result<(), ModelError> {
    update(Events)
        .filter(id.eq(event_id))
        .set(name.eq(user_name))
        .execute(conn)?;
    Ok(())
}

pub fn get_all_events(conn: &mut SqliteConnection, user_owner_id: String) -> Result<Vec<GetEvent>, ModelError> {
    let events: Vec<GetEvent> = Events.filter(owner_id.eq(user_owner_id.clone()))
        .load::<GetEvent>(conn)?;
    Ok(events)
}

pub fn delete_event(conn: &mut SqliteConnection, event_id: String) -> Result<(), ModelError> {
    delete(Events.filter(id.eq(event_id)))
        .execute(conn)?;
    Ok(())
}
