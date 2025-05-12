use crate::schema::Users;
use super::users_models::*;
use diesel::prelude::*;
use diesel::result::*;
use uuid::Uuid;

use crate::establish_connection;
    

pub fn create_user(name: String, email: String) -> Result<String, Error> {
    let id = Uuid::new_v4().to_string();
    let user = NewUser::new(id, name, email);
    let conn = &mut establish_connection();

    let email_exists = diesel::select(exists(users::table.filter(users::email.eq(email))))
    .get_result::<bool>(conn)
    .map_err(|_| Error::NotFound)?;

    if email_exists.is_some() {
        return Err(Error::NotFound)?;
    }

    diesel::insert_into(Users::table)
        .values(user)
        .execute(conn)
        .map_err(|_| Error::NotFound("Email already exists".into()))?;

    Ok(i)
}

pub fn update_phone(id: String, phone: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    let user = diesel::update(Users::table.filter(Users::id.eq(id)))
        .set(Users::phone.eq(phone))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(())
}

pub fn update_email(id: String, email: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    let user = diesel::update(Users::table.filter(Users::id.eq(id)))
        .set(Users::email.eq(email))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(())
}

pub fn update_name(id: String, name: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    let user = Users::table.filter(Users::id.eq(id))
        .update(Users::name.eq(name))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(())
}

pub fn get_email(id: String) -> Result<String, Error> {
    let conn = &mut establish_connection();
    let email = Users::table.filter(Users::id.eq(id))
        .select(Users::email)
        .first::<String>(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(email)
}

pub fn get_name(id: String) -> Result<String, Error> {
    let conn = &mut establish_connection();
    let name_exists = Users::table.filter(Users::id.eq(id))
        .select(Users::name)
        .first::<String>(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(name_exists)
}

pub fn get_phone(id: String) -> Result<String, Error> {
    let conn = &mut establish_connection();
    let phone_exists = Users::table.filter(Users::id.eq(id))
        .select(Users::phone)
        .first::<String>(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(phone_exists);
}

pub fn get_id(id: String) -> Result<String, Error> {
    let conn = &mut establish_connection();
    let id_exists = Users::table.filter(Users::id.eq(id))
        .select(Users::id)
        .first::<String>(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(id_exists)
}



