use diesel::{Insertable, Queryable, Identifiable};
use crate::schema::Friends;


//template for structs

struct TemplateFriends {
    id: String,
    id_two: String
}

//DB MODELS


//insert models

#[derive(Insertable)]
#[diesel(table_name = Friends)]
pub struct NewFriend {
    user_id: String,
    friend_id: String
}

impl NewFriend {
    pub fn new(user_id: String, friend_id: String) -> Self {
        Self { user_id, friend_id }
    }
}


#[derive(Queryable, Identifiable)]
#[diesel(primary_key(user_id, friend_id))]
#[diesel(table_name = Friends)]
struct GetFriend {
    pub user_id: String,
    pub friend_id: String
}




