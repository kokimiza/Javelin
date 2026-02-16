// Infrastructure Layer Errors
// エラーコード: I-xxxx

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("[I-1001] Failed to create directory: {path}")]
    DirectoryCreationFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("[I-1002] Failed to initialize event store at: {path}")]
    EventStoreInitFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("[I-1003] Failed to initialize projection database at: {path}")]
    ProjectionDbInitFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("[I-2001] Event append failed")]
    EventAppendFailed,

    #[error("[I-2002] Event stream load failed: {0}")]
    EventStreamLoadFailed(String),

    #[error("[I-3001] Projection update failed: {0}")]
    ProjectionUpdateFailed(String),

    #[error("[I-3002] Projection query failed: {0}")]
    ProjectionQueryFailed(String),

    #[error("[I-4001] LMDB error: {0}")]
    LmdbError(String),

    #[error("[I-5001] Serialization failed: {0}")]
    SerializationFailed(String),

    #[error("[I-5002] Deserialization failed: {0}")]
    DeserializationFailed(String),

    #[error("[I-9999] Unknown infrastructure error: {0}")]
    Unknown(String),
}

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;
