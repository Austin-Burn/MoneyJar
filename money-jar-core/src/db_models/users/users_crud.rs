use super::users_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use uuid::Uuid;

use crate::{establish_connection};
use crate::Users::dsl::Users;
use crate::Users::*;

pub fn create_user(new_name: String, new_email: String, new_password: String) -> Result<String, Error> {
    let new_id = Uuid::new_v4().to_string();
    let user = NewUser::new(new_id.clone(), new_name, new_email.clone(), new_password.clone());
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

pub fn update_password(get_id: String, update_password: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    let update_password = UpdatePassword::new(update_password);
    update(Users)
        .filter(id.eq(get_id))
        .set(update_password)
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


//named get id as this doesnt seem to be fully enough to be called login YET
pub fn get_id(get_email: String, get_password: String) -> Result<String, Error> {
    let conn = &mut establish_connection();
    let result = Users
        .filter(email.eq(get_email).and(password.eq(get_password)))
        .select(id)
        .first::<String>(conn)
        .map_err(|_| Error::NotFound)?;

    println!("result: {}", result);
    Ok(result)
}


//delete user
pub fn delete_user(get_id: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    delete(Users.filter(id.eq(get_id))).execute(conn).map_err(|_| Error::NotFound)?;
    Ok(())
}





