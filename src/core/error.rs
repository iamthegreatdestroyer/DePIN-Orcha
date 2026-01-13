// Error types for DePIN Orcha

#[derive(thiserror::Error, Debug)]
pub enum OrchError {
    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("ML service error: {0}")]
    MLServiceError(String),

    #[error("Optimization failed: {0}")]
    OptimizationError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type OrchResult<T> = Result<T, OrchError>;
