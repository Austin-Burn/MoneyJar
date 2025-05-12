use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = user_payment_methods)]
pub struct DB_UserPaymentMethods_Insert {
    pub user_id: String,
    pub payment_method: String,
}

impl DB_UserPaymentMethods_Insert {
    pub fn new(user_id: String, payment_method: String) -> Self {
        Self {
            user_id,
            payment_method,
        }
    }
}

#[derive(Queryable)]
#[diesel(table_name = user_payment_methods)]
pub struct DB_UserPaymentMethods_Select {
    pub user_id: String,
    pub payment_method: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = user_payment_methods)]
pub struct DB_UserPaymentMethods_Update {
    pub user_id: String,
    pub payment_method: String,
}

impl DB_UserPaymentMethods_Update {
    pub fn new(user_id: String, payment_method: String) -> Self {
        Self {
            user_id,
            payment_method,
        }
    }
} 