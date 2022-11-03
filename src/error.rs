use thiserror::Error as ThisError;

pub type Result<R, E = Error> = std::result::Result<R, E>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Storage error. {0}")]
    ConnectionError(String),
    #[error("Database error. {0}")]
    DatabaseError(String),
    #[error("Serialization error. {0}")]
    SerializationError(String),
}
