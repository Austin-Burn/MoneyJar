use super::events_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use uuid::Uuid;
use crate::{establish_connection};
use crate::Events::dsl::Events;
use crate::Events::*;


pub fn create_event(event_owner_id: String, event_name: String, event_reoccuring: bool) -> Result<(), Error> {
    let conn = &mut establish_connection();
    let event_id = Uuid::new_v4().to_string();
    let event = NewEvent::new(event_id, event_owner_id, event_name, event_reoccuring);
    insert_into(Events)
        .values(event)
        .execute(conn)
        .map_err(|_| Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, Box::new("Failed to create event".to_string())))?;
    Ok(())
}

pub fn event_update_owner_id(event_id: String, user_owner_id: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Events)
        .filter(id.eq(event_id))
        .set(owner_id.eq(user_owner_id))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}



pub fn get_event(event_id: String) -> Result<GetEvent, Error> {
    let conn = &mut establish_connection();
    let event = Events.filter(owner_id.eq(event_id))
        .first::<GetEvent>(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(event)
}

pub fn update_description(event_id: String, user_description: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Events)
        .filter(id.eq(event_id))
        .set(description.eq(user_description))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn update_date(event_id: String, user_date: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Events)
        .filter(id.eq(event_id))
        .set(event_date.eq(user_date))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())

}

pub fn update_reoccuring(event_id: String, user_reoccuring: bool) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Events)
        .filter(id.eq(event_id))
        .set(reoccuring.eq(user_reoccuring))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn update_reoccuring_interval(event_id: String, user_reoccuring_interval: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Events)
        .filter(id.eq(event_id))
        .set(reoccuring_interval.eq(user_reoccuring_interval))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn update_final_date(event_id: String, user_final_date: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Events)
        .filter(id.eq(event_id))
        .set(final_date.eq(user_final_date))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn event_update_name(event_id: String, user_name: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Events)
        .filter(id.eq(event_id))
        .set(name.eq(user_name))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn get_all_events(user_owner_id: String) -> Result<Vec<GetEvent>, Error> {
    let conn = &mut establish_connection();
    let events: Vec<GetEvent> = Events.filter(owner_id.eq(user_owner_id)).load::<GetEvent>(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(events)
}

pub fn delete_event(event_id: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    delete(Events.filter(id.eq(event_id))).execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}
