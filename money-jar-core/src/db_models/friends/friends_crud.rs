use super::friends_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use diesel::sqlite::SqliteConnection;
use crate::{establish_connection};
use crate::Friends::dsl::Friends;
use crate::Friends::*;
use crate::db_models::errors::ModelError;

pub fn create_friend(conn: &mut SqliteConnection, friend_one: String, friend_two: String) -> Result<(), ModelError> {
    // Check if friendship already exists
    let friend_exists = select(exists(Friends.filter(user_id.eq(friend_one.clone()).and(friend_id.eq(friend_two.clone())))))
        .get_result::<bool>(conn)?;
        
    if friend_exists {
        return Err(ModelError::Database(diesel::result::Error::DatabaseError(
            DatabaseErrorKind::UniqueViolation,
            Box::new("Friendship already exists".to_string())
        )));
    }
    
    // Create bidirectional friendship
    let friend = NewFriend::new(friend_one.clone(), friend_two.clone());
    insert_into(Friends)
        .values(friend)
        .execute(conn)?;
        
    let other_friend = NewFriend::new(friend_two, friend_one);
    insert_into(Friends)
        .values(other_friend)
        .execute(conn)?;
        
    Ok(())
}

pub fn get_friends(conn: &mut SqliteConnection, search_id: String) -> Result<Vec<String>, ModelError> {
    let friends: Vec<String> = Friends
        .filter(user_id.eq(search_id))
        .select(friend_id)
        .load::<String>(conn)?;
    Ok(friends)
}

pub fn delete_friend(conn: &mut SqliteConnection, search_id: &String, id_two: &String) -> Result<(), ModelError> {    
    delete(Friends.filter(user_id.eq(search_id).and(friend_id.eq(id_two))))
        .execute(conn)?;
        
    delete(Friends.filter(user_id.eq(&id_two).and(friend_id.eq(&search_id))))
        .execute(conn)?;
        
    Ok(())
}




