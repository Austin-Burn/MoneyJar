use super::friends_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;


use crate::{establish_connection};
use crate::Friends::dsl::Friends;
use crate::Friends::*;

pub fn create_friend(friend_one: String, friend_two: String) -> Result<(), Error> {
    let conn = &mut establish_connection();
    let friend_exists = select(exists(Friends.filter(user_id.eq(friend_one.clone()).and(friend_id.eq(friend_two.clone())))))
        .get_result::<bool>(conn)
        .map_err(|_| Error::NotFound)?;
    if friend_exists {
    return Err(Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, Box::new("Already Friends".to_string())));
   }
    
    let friend = NewFriend::new(friend_one.clone(),friend_two.clone());
    insert_into(Friends)
        .values(friend)
        .execute(conn)
        .map_err(|_| Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, Box::new("Failed to create friend (user to friend)".to_string())))?;
    let other_friend = NewFriend::new(friend_two, friend_one);
    insert_into(Friends)
        .values(other_friend)
        .execute(conn)
        .map_err(|_| Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, Box::new("Failed to create friend (friend to user)".to_string())))?;
    Ok(())

}

pub fn get_friends(search_id: String) -> Result<Vec<String>, Error> {
    let connection = &mut establish_connection();
    let friends: Vec<String> = Friends.filter(user_id.eq(search_id)).select(friend_id).load::<String>(connection)?;
    Ok(friends)
}

pub fn delete_friend(search_id: &String, id_two: &String) -> Result<(), Error> {    
    let conn = &mut establish_connection();
    delete(Friends.filter(user_id.eq(search_id).and(friend_id.eq(id_two)))).execute(conn).map_err(|_| Error::NotFound)?;
    delete(Friends.filter(user_id.eq(&id_two).and(friend_id.eq(&search_id)))).execute(conn).map_err(|_| Error::NotFound)?;
    Ok(())
}




