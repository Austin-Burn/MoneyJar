use super::payBatches_models::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::*;
use diesel::sqlite::SqliteConnection;
use crate::{establish_connection};
use crate::PayBatches::dsl::PayBatches;
use crate::PayBatches::*;
use crate::db_models::errors::ModelError;
use uuid::Uuid;

pub fn create_paybatches(conn: &mut SqliteConnection, passed_transaction_ids: Vec<String>, passed_date: String) -> Result<Vec<GetPayBatch>, ModelError> {
    conn.transaction(|conn| {
        let mut created_batches = Vec::new();
        let batch_id = Uuid::new_v4().to_string();
        
        for for_id in passed_transaction_ids {
            let paybatch = NewPayBatch::new(
                batch_id.clone(),
                for_id,
                passed_date.clone()
            );
            
            let created = insert_into(PayBatches)
                .values(paybatch)
                .get_result::<GetPayBatch>(conn)?;
                
            created_batches.push(created);
        }
        
        Ok(created_batches)
    })
}

pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<GetPayBatch>, ModelError> {
    let paybatches = PayBatches.load::<GetPayBatch>(conn)?;
    Ok(paybatches)
}

pub fn get_by_id(conn: &mut SqliteConnection, paybatch_id: &str) -> Result<Vec<GetPayBatch>, ModelError> {
    let paybatches = PayBatches
        .filter(id.eq(paybatch_id))
        .load::<GetPayBatch>(conn)?;
    Ok(paybatches)
}

pub fn get_by_transaction(conn: &mut SqliteConnection, foreign_transaction_id: &str) -> Result<Vec<GetPayBatch>, ModelError> {
    let paybatches = PayBatches
        .filter(transaction_id.eq(foreign_transaction_id))
        .load::<GetPayBatch>(conn)?;
    Ok(paybatches)
}

pub fn delete_paybatch(conn: &mut SqliteConnection, paybatch_id: &str) -> Result<(), ModelError> {
    delete(PayBatches.filter(id.eq(paybatch_id)))
        .execute(conn)?;
    Ok(())
} 