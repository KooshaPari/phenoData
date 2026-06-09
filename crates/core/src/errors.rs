use thiserror::Error;

pub type Result<T> = std::result::Result<T, CoreError>;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("dataset error: {0}")]
    DatasetError(String),
    #[error("schema mismatch error: expected {expected}, found {found}")]
    SchemaMismatchError { expected: String, found: String },
    #[error("writer is closed")]
    WriterClosedError,
}
