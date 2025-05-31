use diesel::{Insertable, Queryable, Identifiable, AsChangeset};
use crate::schema::Transactions;


//template for structs

//struct TemplateFriends {
//    id: String,
//    FROM_USER_ID: String,
//    TO_USER_ID: String,
//    EVENT_ID: String,
//    AMOUNT: i32,
//    DATE: String
//}

//DB MODELS


//insert models

#[derive(Insertable)]
#[diesel(table_name = Transactions)]
pub struct NewTransaction {
    pub from_user_id: String,
    pub to_user_id: String,
    pub event_id: String,
    pub amount: i32,
    pub date: String,
    pub payment_method: String,
    pub comment: Option<String>
}

impl NewTransaction {
    pub fn new(from_user_id: String, to_user_id: String, event_id: String, amount: i32, date: String, payment_method: String, comment: Option<String>) -> Self {
        Self { from_user_id, to_user_id, event_id, amount, date, payment_method, comment }
    }
}



//get models

#[derive(Queryable)]
#[diesel(table_name = Transactions)]
pub struct GetTransaction {
    pub id: i32,
    pub from_user_id: String,
    pub to_user_id: String,
    pub event_id: String,
    pub amount: i32,
    pub date: String,
    pub payment_method: String,
    pub comment: Option<String>
}

impl GetTransaction {
    pub fn new(id: i32, from_user_id: String, to_user_id: String, event_id: String, amount: i32, date: String, payment_method: String, comment: Option<String>) -> Self {
        Self { id, from_user_id, to_user_id, event_id, amount, date, payment_method, comment }
    }
}


//update models

#[derive(AsChangeset)]
#[diesel(table_name = Transactions)]
pub struct UpdateTransactionComment {
    pub comment: Option<String>
}

impl UpdateTransactionComment {
    pub fn new(comment: Option<String>) -> Self {
        Self { comment }
    }
}

