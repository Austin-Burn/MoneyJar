use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct DB_Transactions_Insert {
    pub id: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub event_id: String,
    pub amount: i32,
    pub date: String,
    pub description: String,
}

impl DB_Transactions_Insert {
    pub fn new(id: String, from_user_id: String, to_user_id: String, 
               event_id: String, amount: i32, date: String, description: String) -> Self {
        Self {
            id,
            from_user_id,
            to_user_id,
            event_id,
            amount,
            date,
            description,
        }
    }
}

#[derive(Queryable)]
#[diesel(table_name = transactions)]
pub struct DB_Transactions_Select {
    pub id: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub event_id: String,
    pub amount: i32,
    pub date: String,
    pub description: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = transactions)]
pub struct DB_Transactions_Update {
    pub id: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub event_id: String,
    pub amount: i32,
    pub date: String,
    pub description: String,
} 