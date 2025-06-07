use diesel::{Queryable, Identifiable, Insertable};
use crate::schema::PayBatches;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = PayBatches)]
pub struct GetPayBatch {
    pub id: String,
    pub transaction_id: String,
    pub date: String
}

// Helper struct for creating multiple pay batches at once

#[derive(Insertable)]
#[diesel(table_name = PayBatches)]
pub struct NewPayBatch {
    pub id: String,
    pub transaction_id: String,
    pub date: String
}

impl NewPayBatch {
    pub fn new(id: String, transaction_id: String, date: String) -> Self {
        Self { id, transaction_id, date }
    }
} 