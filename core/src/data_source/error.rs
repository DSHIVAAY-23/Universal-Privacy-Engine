//! Data ingestion error types

use thiserror::Error;

/// Errors that can occur during data fetching and processing
#[derive(Debug, Error)]
pub enum DataError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(String),
    
    /// JSON parsing failed
    #[error("JSON parsing failed: {0}")]
    JsonError(String),
    
    /// Requested field not found in JSON
    #[error("Field not found: {0}")]
    FieldNotFound(String),
    
    /// TLS verification failed
    #[error("TLS verification failed: {0}")]
    TlsVerificationFailed(String),
    
    /// Invalid query syntax
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    
    /// Network error
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
