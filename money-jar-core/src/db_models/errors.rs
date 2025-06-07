use thiserror::Error;
use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Database error: {0}")]
    Database(#[from] DieselError),

    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    #[error("Transaction error: {0}")]
    Transaction(String),
}

// Helper function to convert Diesel errors to our custom errors
pub fn handle_diesel_error(error: DieselError) -> ModelError {
    ModelError::Database(error)
}