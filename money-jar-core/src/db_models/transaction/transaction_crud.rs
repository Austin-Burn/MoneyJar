use diesel::prelude::*;
use super::transaction_model::*;

pub fn insert_transaction(id: String, event_id: String, amount: f64, description: String, 
                         date: String, payment_method: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let transaction = DB_Transactions_Insert_ATD::new(id, event_id, amount, description, date, payment_method);
    diesel::insert_into(transactions::table)
        .values(transaction)
        .execute(conn)?;
    Ok(())
}

pub fn select_transaction(transaction_id: String) -> Result<DB_Transactions_Select, diesel::result::Error> {
    let conn = &mut establish_connection();
    transactions::table
        .find(transaction_id)
        .first::<DB_Transactions_Select>(conn)
}

pub fn update_transaction(transaction_id: String, update: DB_Transactions_Update) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    diesel::update(transactions::table.find(transaction_id))
        .set(update)
        .execute(conn)?;
    Ok(())
}

pub fn update_transaction_payment_method(transaction_id: String, payment_method: String) -> Result<(), diesel::result::Error> {
    let conn = &mut establish_connection();
    let update = DB_Transactions_Update_F::new(payment_method);
    diesel::update(transactions::table.find(transaction_id))
        .set(update)
        .execute(conn)?;
    Ok(())
}

pub fn select_transaction_payment_method(transaction_id: String) -> Result<String, diesel::result::Error> {
    let conn = &mut establish_connection();
    let payment_method = transactions::table
        .find(transaction_id)
        .select(DB_Transactions_Select_F::as_select())
        .first::<DB_Transactions_Select_F>(conn)?;
    Ok(payment_method.payment_method)
} 