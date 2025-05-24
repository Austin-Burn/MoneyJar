use super::whoInWhat_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use diesel::sqlite::SqliteConnection;

use crate::{establish_connection};
use crate::WhoInWhat::dsl::WhoInWhat;
use crate::WhoInWhat::*;

pub fn create_who_in_what(conn: &mut SqliteConnection, wiw_user_id: String, wiw_event_id: String) -> Result<(), Error> {
    let who_in_what = NewWhoInWhat::new(wiw_user_id, wiw_event_id);
    insert_into(WhoInWhat)
        .values(who_in_what)
        .execute(conn)
        .map_err(|_| Error::NotFound)?;
    Ok(())
}

pub fn wiw_get_users(conn: &mut SqliteConnection, wiw_event_id: String) -> Result<Vec<String>, Error> {
    let users: Vec<String> = WhoInWhat.filter(event_id.eq(wiw_event_id)).select(user_id).load::<String>(conn)
    .map_err(|_| Error::NotFound)?;
    Ok(users)
}

pub fn wiw_get_events(conn: &mut SqliteConnection, wiw_user_id: String) -> Result<Vec<String>, Error> {
    let events: Vec<String> = WhoInWhat.filter(user_id.eq(wiw_user_id)).select(event_id).load::<String>(conn)
    .map_err(|_| Error::NotFound)?;
    Ok(events)
}

pub fn delete_who_in_what(conn: &mut SqliteConnection, get_user_id: String, get_events_id: String) -> Result<(), Error> {
    delete(WhoInWhat.filter(user_id.eq(get_user_id).and(event_id.eq(get_events_id)))).execute(conn).map_err(|_| Error::NotFound)?;
    Ok(())
}






