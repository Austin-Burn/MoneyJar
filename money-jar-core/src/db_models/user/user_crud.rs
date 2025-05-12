use diesel::prelude::*;
use super::user_model::*;

pub fn insert_user(id: String, name: String, email: String, phone: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let user = DB_Users_Insert_ATD::new(id, name, email, phone);
    diesel::insert_into(users::table)
        .values(user)
        .execute(conn)?;
    Ok(())
}

pub fn insert_user_basic(id: String, name: String, email: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let user = DB_Users_Insert_ATC::new(id, name, email);
    diesel::insert_into(users::table)
        .values(user)
        .execute(conn)?;
    Ok(())
}

pub fn update_user_phone(user_id: String, phone: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let update = DB_Users_Update_D::new(phone);
    diesel::update(users::table.find(user_id))
        .set(update)
        .execute(conn)?;
    Ok(())
}

pub fn select_user_phone(user_id: String) -> Result<String, diesel::result::Error> {
    let conn = &mut establish_connection();
    let phone_struct = users::table
        .find(user_id)
        .select(DB_Users_Select_D::as_select())
        .first::<DB_Users_Select_D>(conn)?;
    Ok(phone_struct.phone)
} 