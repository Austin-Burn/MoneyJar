use super::friends_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;


use crate::{establish_connection};
use crate::Events::dsl::Events;
use crate::Events::*;


pub fn create_event(event: NewEvent) -> Result<(), Error> {
    let conn = &mut establish_connection();
    let event_id = Uuid::new_v4().to_string();
    let event = NewEvent::new(event_id, event.owner_id, event.name, event.reoccuring);
    insert_into(Events)
        .values(event)
        .execute(conn)
        .map_err(|e| Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, Box::new("Failed to create event".to_string())))?;
    Ok(())
}

pub fn get_event(event_id: String) -> Result<Event, Error> {
    let conn = &mut establish_connection();
    let event = Events.filter(id.eq(event_id)).first::<Event>(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(event)
}