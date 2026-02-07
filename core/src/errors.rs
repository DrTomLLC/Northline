use thiserror::Error;

/// Machine-readable error codes for the Northline kernel.
#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    ConfigMissing,
    ConfigInvalid,
    ProviderUnavailable,
    ProviderError,
    SearchUnavailable,
    SearchError,
    LocalModelLoadFailed,
    Timeout,
    Internal,
}

/// Top-level error type for northline-core.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct CoreError {
    pub code: ErrorCode,
    pub message: String,
}

impl CoreError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
