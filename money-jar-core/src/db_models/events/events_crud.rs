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

pub fn create_event(conn: &mut SqliteConnection, event_owner_id: String, event_name: String, event_reoccuring: bool) -> Result<String, Error> {
    let event_id = Uuid::new_v4().to_string();
    let event = NewEvent::new(event_id.clone(), event_owner_id.clone(), event_name, event_reoccuring);
    
    // Create the event
    let result = insert_into(Events)
        .values(event)
        .execute(conn);
    
    match result {
        Ok(_) => println!("Successfully inserted event with ID {}", event_id),
        Err(e) => {
            println!("Failed to insert event: {:?}", e);
            return Err(Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, Box::new("Failed to create event".to_string())));
        }
    }

    // Add the owner to the who_in_what table
    let who_in_what = NewWhoInWhat::new(event_owner_id, event_id.clone());
    let result = insert_into(WhoInWhat)
        .values(who_in_what)
        .execute(conn);
    
    match result {
        Ok(_) => println!("Successfully added owner to who_in_what table"),
        Err(e) => {
            println!("Failed to add owner to who_in_what table: {:?}", e);
            return Err(Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, Box::new("Failed to add owner to event".to_string())));
        }
    }

    Ok(event_id)
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
    println!("Searching for events with owner_id: {}", user_owner_id);
    let events: Vec<GetEvent> = Events.filter(owner_id.eq(user_owner_id.clone()))
        .load::<GetEvent>(conn)
        .map_err(|e| {
            println!("Database error getting events: {:?}", e);
            Error::NotFound
        })?;
    println!("Found {} events for owner {}", events.len(), user_owner_id);
    if events.is_empty() {
        println!("No events found for owner {}", user_owner_id);
    } else {
        for event in &events {
            println!("Found event: id={}, name={}, owner_id={}", event.id, event.name, event.owner_id);
        }
    }
    Ok(events)
}

pub fn delete_event(conn: &mut SqliteConnection, event_id: String) -> Result<(), Error> {
    delete(Events.filter(id.eq(event_id))).execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}
