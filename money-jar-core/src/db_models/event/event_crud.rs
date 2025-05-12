use diesel::prelude::*;
use super::event_model::*;

pub fn insert_event_full(id: String, name: String, description: String, date: String, 
                        reoccuring: bool, reoccuring_interval: String, final_occurrence: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let event = DB_Events_Insert_ATG::new(id, name, description, date, reoccuring, reoccuring_interval, final_occurrence);
    diesel::insert_into(events::table)
        .values(event)
        .execute(conn)?;
    Ok(())
}

pub fn insert_event_basic(id: String, name: String, description: String, date: String, 
                         reoccuring: bool, reoccuring_interval: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let event = DB_Events_Insert_ATF::new(id, name, description, date, reoccuring, reoccuring_interval);
    diesel::insert_into(events::table)
        .values(event)
        .execute(conn)?;
    Ok(())
}

pub fn update_event_final_occurrence(event_id: String, final_occurrence: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let update = DB_Events_Update_G::new(final_occurrence);
    diesel::update(events::table.find(event_id))
        .set(update)
        .execute(conn)?;
    Ok(())
}

pub fn update_event_description(event_id: String, description: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let update = DB_Events_Update_C::new(description);
    diesel::update(events::table.find(event_id))
        .set(update)
        .execute(conn)?;
    Ok(())
}

pub fn update_event_date(event_id: String, date: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let update = DB_Events_Update_D::new(date);
    diesel::update(events::table.find(event_id))
        .set(update)
        .execute(conn)?;
    Ok(())
}

pub fn update_event_reoccurring(event_id: String, reoccuring: bool, interval: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let update = DB_Events_Update_EF::new(reoccuring, interval);
    diesel::update(events::table.find(event_id))
        .set(update)
        .execute(conn)?;
    Ok(())
}

pub fn select_event_final_occurrence(event_id: String) -> Result<String, diesel::result::Error> {
    let conn = &mut establish_connection();
    let final_occurrence = events::table
        .find(event_id)
        .select(DB_Events_Select_G::as_select())
        .first::<DB_Events_Select_G>(conn)?;
    Ok(final_occurrence.final_occurrence)
} 