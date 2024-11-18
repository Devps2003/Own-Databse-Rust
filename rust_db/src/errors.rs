use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Parsing error: {0}")]
    ParseError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("VM execution error: {0}")]
    VMError(String),
    
    #[error("Invalid query")]
    InvalidQuery,
}