use thiserror::Error;

/// Machine-readable error codes for the Northline kernel.
/// Extend only by adding new variants; do not change semantics of existing ones.
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
///
/// Carries a machine-readable ErrorCode and a human-readable message.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct CoreError {
    pub code: ErrorCode,
    pub message: String,
}

impl CoreError {
    /// Helper constructor.
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
