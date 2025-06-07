use crate::schema::Auth;
use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = Auth)]
pub struct AuthTable {
    pub user_id: String,
    pub token: String,
    pub expiry: String,
}
