use diesel::prelude::*;
use super::who_in_what_model::*;

pub fn insert_who_in_what(user_id: String, event_id: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let who_in_what = DB_WhoInWhat_Insert::new(user_id, event_id);
    diesel::insert_into(who_in_what::table)
        .values(who_in_what)
        .execute(conn)?;
    Ok(())
}

pub fn select_who_in_what(user_id: String) -> Result<Vec<DB_WhoInWhat_Select>, diesel::result::Error> {
    let conn = &mut establish_connection();
    who_in_what::table
        .filter(who_in_what::user_id.eq(user_id))
        .select(DB_WhoInWhat_Select::as_select())
        .load::<DB_WhoInWhat_Select>(conn)
}

pub fn update_who_in_what(user_id: String, old_event_id: String, new_event_id: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let update = DB_WhoInWhat_Update::new(new_event_id);
    diesel::update(who_in_what::table
        .filter(who_in_what::user_id.eq(user_id))
        .filter(who_in_what::event_id.eq(old_event_id)))
        .set(update)
        .execute(conn)?;
    Ok(())
} 