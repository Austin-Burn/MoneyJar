use super::users_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use diesel::sqlite::SqliteConnection;
use uuid::Uuid;
use crate::Users::dsl::Users;
use crate::Users::*;
use crate::db_models::errors::ModelError;

pub fn create_user(conn: &mut SqliteConnection, new_name: String, new_email: String, new_password: String) -> Result<String, ModelError> {
    let new_id = Uuid::new_v4().to_string();
    let user = NewUser::new(new_id.clone(), new_name, new_email.clone(), new_password.clone());

    let email_exists = select(exists(Users.filter(email.eq(new_email))))
        .get_result::<bool>(conn)?;

    if email_exists {
        return Err(ModelError::Database(diesel::result::Error::DatabaseError(
            DatabaseErrorKind::UniqueViolation,
            Box::new("User already exists".to_string())
        )));
    }

    insert_into(Users)
        .values(user)
        .execute(conn)?;

    Ok(new_id)
}

pub fn update_phone(conn: &mut SqliteConnection, get_id: String, update_phone: String) -> Result<(), ModelError> {
    update(Users.filter(id.eq(get_id)))
        .set(phone.eq(update_phone))
        .execute(conn)?;

    Ok(())
}

pub fn update_email(conn: &mut SqliteConnection, get_id: String, update_email: String) -> Result<(), ModelError> {
    update(Users.filter(id.eq(get_id)))
        .set(email.eq(update_email))
        .execute(conn)?;

    Ok(())
}

pub fn update_name(conn: &mut SqliteConnection, get_id: String, update_name: String) -> Result<(), ModelError> {
    update(Users)
        .filter(id.eq(get_id))
        .set(name.eq(update_name))
        .execute(conn)?;

    Ok(())
}

pub fn update_password(conn: &mut SqliteConnection, get_id: String, update_password: String) -> Result<(), ModelError> {
    let update_password = UpdatePassword::new(update_password);
    update(Users)
        .filter(id.eq(get_id))
        .set(update_password)
        .execute(conn)?;

    Ok(())
}

pub fn update_balance(conn: &mut SqliteConnection, get_id: String, new_balance: i32) -> Result<(), ModelError> {
    let update_balance = UpdateBalance::new(new_balance);
    update(Users)
        .filter(id.eq(get_id))
        .set(update_balance)
        .execute(conn)?;

    Ok(())
}

pub fn get_all(conn: &mut SqliteConnection, user_id: String) -> Result<(String, String, Option<String>, i32), ModelError> {
    let result = Users
        .filter(id.eq(user_id))
        .select((name, email, phone, balance))
        .first::<(String, String, Option<String>, i32)>(conn)?;

    Ok(result)
}

pub fn get_email(conn: &mut SqliteConnection, get_id: String) -> Result<String, ModelError> {
    let result = Users
        .filter(id.eq(get_id))
        .select(email)
        .first::<String>(conn)?;

    Ok(result)
}

pub fn get_name(conn: &mut SqliteConnection, get_id: String) -> Result<String, ModelError> {
    let name_exists = Users
        .filter(id.eq(get_id))
        .select(name)
        .first::<String>(conn)?;

    Ok(name_exists)
}

pub fn get_phone(conn: &mut SqliteConnection, get_id: String) -> Result<Option<String>, ModelError> {
    let phone_exists = Users
        .filter(id.eq(get_id))
        .select(phone)
        .first::<Option<String>>(conn)?;
    Ok(phone_exists)
}

pub fn get_balance(conn: &mut SqliteConnection, get_id: String) -> Result<i32, ModelError> {
    let user_balance = Users
        .filter(id.eq(get_id))
        .select(balance)
        .first::<i32>(conn)?;
    Ok(user_balance)
}

pub fn get_id(conn: &mut SqliteConnection, get_email: String, get_password: String) -> Result<String, ModelError> {
    let result = Users
        .filter(email.eq(get_email).and(password.eq(get_password)))
        .select(id)
        .first::<String>(conn)?;

    println!("result: {}", result);
    Ok(result)
}

pub fn delete_user(conn: &mut SqliteConnection, get_id: String) -> Result<(), ModelError> {
    delete(Users.filter(id.eq(get_id))).execute(conn)?;
    Ok(())
}





