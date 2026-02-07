use thiserror::Error;

/// Errors in northline-crypto (config encryption / storage).
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("config not found")]
    ConfigNotFound,

    #[error("config corrupted")]
    ConfigCorrupted,

    #[error("encryption error")]
    EncryptionError,

    #[error("decryption error")]
    DecryptionError,

    #[error("I/O error")]
    IoError,
}
