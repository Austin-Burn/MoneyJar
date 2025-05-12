use diesel::prelude::*;
use super::user_payment_methods_model::*;

pub fn insert_payment_method(user_id: String, payment_method: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let payment = DB_UserPaymentMethods_Insert::new(user_id, payment_method);
    diesel::insert_into(user_payment_methods::table)
        .values(payment)
        .execute(conn)?;
    Ok(())
}

pub fn select_payment_methods(user_id: String) -> Result<Vec<DB_UserPaymentMethods_Select>, diesel::result::Error> {
    let conn = &mut establish_connection();
    user_payment_methods::table
        .filter(user_payment_methods::user_id.eq(user_id))
        .select(DB_UserPaymentMethods_Select::as_select())
        .load::<DB_UserPaymentMethods_Select>(conn)
}

pub fn update_payment_method(user_id: String, old_method: String, new_method: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let update = DB_UserPaymentMethods_Update::new(new_method);
    diesel::update(user_payment_methods::table
        .filter(user_payment_methods::user_id.eq(user_id))
        .filter(user_payment_methods::payment_method.eq(old_method)))
        .set(update)
        .execute(conn)?;
    Ok(())
} 