use super::events_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use diesel::sqlite::SqliteConnection;
use uuid::Uuid;
use crate::Events::dsl::Events;
use crate::Events::*;

pub fn create_event(conn: &mut SqliteConnection, event_owner_id: String, event_name: String, event_reoccuring: bool) -> Result<(), Error> {
    let event_id = Uuid::new_v4().to_string();
    let event = NewEvent::new(event_id, event_owner_id, event_name, event_reoccuring);
    insert_into(Events)
        .values(event)
        .execute(conn)
        .map_err(|_| Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, Box::new("Failed to create event".to_string())))?;
    Ok(())
}

pub fn event_update_owner_id(conn: &mut SqliteConnection, event_id: String, user_owner_id: String) -> Result<(), Error> {
    update(Events)
        .filter(id.eq(event_id))
        .set(owner_id.eq(user_owner_id))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn get_event(conn: &mut SqliteConnection, event_id: String) -> Result<GetEvent, Error> {
    let event = Events.filter(owner_id.eq(event_id))
        .first::<GetEvent>(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(event)
}

pub fn update_description(conn: &mut SqliteConnection, event_id: String, user_description: String) -> Result<(), Error> {
    update(Events)
        .filter(id.eq(event_id))
        .set(description.eq(user_description))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn update_date(conn: &mut SqliteConnection, event_id: String, user_date: String) -> Result<(), Error> {
    update(Events)
        .filter(id.eq(event_id))
        .set(event_date.eq(user_date))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn update_reoccuring(conn: &mut SqliteConnection, event_id: String, user_reoccuring: bool) -> Result<(), Error> {
    update(Events)
        .filter(id.eq(event_id))
        .set(reoccuring.eq(user_reoccuring))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn update_reoccuring_interval(conn: &mut SqliteConnection, event_id: String, user_reoccuring_interval: String) -> Result<(), Error> {
    update(Events)
        .filter(id.eq(event_id))
        .set(reoccuring_interval.eq(user_reoccuring_interval))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn update_final_date(conn: &mut SqliteConnection, event_id: String, user_final_date: String) -> Result<(), Error> {
    update(Events)
        .filter(id.eq(event_id))
        .set(final_date.eq(user_final_date))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn event_update_name(conn: &mut SqliteConnection, event_id: String, user_name: String) -> Result<(), Error> {
    update(Events)
        .filter(id.eq(event_id))
        .set(name.eq(user_name))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn get_all_events(conn: &mut SqliteConnection, user_owner_id: String) -> Result<Vec<GetEvent>, Error> {
    let events: Vec<GetEvent> = Events.filter(owner_id.eq(user_owner_id)).load::<GetEvent>(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(events)
}

pub fn delete_event(conn: &mut SqliteConnection, event_id: String) -> Result<(), Error> {
    delete(Events.filter(id.eq(event_id))).execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}
