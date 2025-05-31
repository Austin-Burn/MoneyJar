use super::transactions_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use diesel::sqlite::SqliteConnection;
use std::option::{Option, Option::Some};
use crate::{establish_connection};
use crate::Transactions::dsl::Transactions;
use crate::Transactions::*;



pub fn create_transaction(conn: &mut SqliteConnection, new_from_user_id: String, new_to_user_id: String, new_event_id: String, new_amount: i32, new_date: String, new_payment_method: String, new_comment: Option<String>) -> Result<(), diesel::result::Error> {
    let transaction = NewTransaction::new(new_from_user_id, new_to_user_id, new_event_id, new_amount, new_date, new_payment_method, new_comment);
    let result = insert_into(Transactions)
        .values(transaction)
        .execute(conn);
    
    match result {
        Ok(_) => println!("Successfully inserted transaction"),
        Err(e) => println!("Failed to insert transaction: {}", e),
    }
    
    
    Ok(())
}


pub fn get_transaction_by_sent(conn: &mut SqliteConnection, search_from_user_id: String) -> Result<Vec<GetTransaction>, diesel::result::Error> {
    let transactions:Vec<GetTransaction> = Transactions
        .filter(from_user_id.eq(search_from_user_id))
        .load::<GetTransaction>(conn)
        .map_err(|e| diesel::result::Error::DatabaseError(DatabaseErrorKind::Unknown, Box::new(format!("Failed to get transaction: {}", e))))?;
    Ok(transactions)
}


pub fn get_transaction_by_received(conn: &mut SqliteConnection, search_to_user_id: String) -> Result<Vec<GetTransaction>, diesel::result::Error> {
    let transactions = Transactions
        .filter(to_user_id.eq(search_to_user_id))
        .load::<GetTransaction>(conn)
        .map_err(|e| diesel::result::Error::DatabaseError(DatabaseErrorKind::Unknown, Box::new(format!("Failed to get transaction: {}", e))))?;
    Ok(transactions)
}


pub fn get_transaction_by_event(conn: &mut SqliteConnection, search_event_id: String) -> Result<Vec<GetTransaction>, diesel::result::Error> {
    let transactions = Transactions
        .filter(event_id.eq(search_event_id))
        .load::<GetTransaction>(conn)
        .map_err(|e| diesel::result::Error::DatabaseError(DatabaseErrorKind::Unknown, Box::new(format!("Failed to get transaction: {}", e))))?;
    Ok(transactions)
}

pub fn update_transaction_comment(conn: &mut SqliteConnection, transaction_id: i32, transaction_comment: String) -> Result<(), diesel::result::Error> {
    let transaction = UpdateTransactionComment::new(Some(transaction_comment));
    update(Transactions)
        .filter(id.eq(transaction_id))
        .set(transaction)
        .execute(conn)
        .map_err(|e| diesel::result::Error::DatabaseError(DatabaseErrorKind::Unknown, Box::new(format!("Failed to update transaction comment: {}", e))))?;

    
    Ok(())
}

pub fn update_comment(conn: &mut SqliteConnection, transaction_id: i32, new_comment: String) -> Result<(), diesel::result::Error> {
    let transaction = UpdateTransactionComment::new(Some(new_comment));
    let result = update(Transactions)
        .filter(id.eq(transaction_id))
        .set(transaction)
        .execute(conn)
        .map_err(|e| diesel::result::Error::DatabaseError(DatabaseErrorKind::Unknown, Box::new(format!("Failed to update transaction comment: {}", e))))?;
    
    Ok(())
}

pub fn delete_transaction(conn: &mut SqliteConnection, transaction_id: i32) -> Result<(), diesel::result::Error> {
    let result = delete(Transactions)
        .filter(id.eq(transaction_id))
        .execute(conn)?;
    
    if result == 0 {
        return Err(diesel::result::Error::NotFound);
    }
    
    Ok(())
}
