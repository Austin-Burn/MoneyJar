use super::users_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use uuid::Uuid;

use crate::{establish_connection};
use crate::Users::dsl::Users;
use crate::Users::*;

pub fn create_user(new_name: String, new_email: String) -> Result<String, Error> {
    let new_id = Uuid::new_v4().to_string();
    let user = NewUser::new(new_id.clone(), new_name, new_email.clone());
    let conn = &mut establish_connection();

    let email_exists = select(exists(Users.filter(email.eq(new_email))))
        .get_result::<bool>(conn)
        .map_err(|_| Error::NotFound)?;

    if email_exists {
        return Err(Error::NotFound)?;
    }

    insert_into(Users)
        .values(user)
        .execute(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(new_id)
}

pub fn update_phone(get_id: String, update_phone: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Users.filter(id.eq(get_id)))
        .set(phone.eq(update_phone))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(())
}

pub fn update_email(get_id: String, update_email: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Users.filter(id.eq(get_id)))
        .set(email.eq(update_email))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(())
}

pub fn update_name(get_id: String, update_name: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    update(Users)
        .filter(id.eq(get_id))
        .set(name.eq(update_name))
        .execute(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(())
}

pub fn get_email(get_id: String) -> Result<String, Error> {
    let conn = &mut establish_connection();
    let result = Users
        .filter(id.eq(get_id))
        .select(email)
        .first::<String>(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(result)
}

pub fn get_name(get_id: String) -> Result<String, Error> {
    let conn = &mut establish_connection();
    let name_exists = Users
        .filter(id.eq(get_id))
        .select(name)
        .first::<String>(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(name_exists)
}

pub fn get_phone(get_id: String) -> Result<Option<String>, Error> {
    let conn = &mut establish_connection();
    let phone_exists = Users
        .filter(id.eq(get_id))
        .select(phone)
        .first::<Option<String>>(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(phone_exists)
}

pub fn get_id(get_id: String) -> Result<String, Error> {
    let conn = &mut establish_connection();
    let id_exists = Users
        .filter(id.eq(get_id))
        .select(id)
        .first::<String>(conn)
        .map_err(|_| Error::NotFound)?;

    Ok(id_exists)
}
