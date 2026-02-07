pub mod errors;

pub use errors::CryptoError;

pub struct EncryptedConfig(pub Vec<u8>);

pub fn encrypt_config(_plaintext: &[u8]) -> Result<EncryptedConfig, CryptoError> {
    Err(CryptoError::EncryptionError)
}

pub fn decrypt_config(_cipher: &EncryptedConfig) -> Result<Vec<u8>, CryptoError> {
    Err(CryptoError::DecryptionError)
}

pub fn load_encrypted_config() -> Result<Option<EncryptedConfig>, CryptoError> {
    Err(CryptoError::ConfigNotFound)
}

pub fn save_encrypted_config(_cfg: &EncryptedConfig) -> Result<(), CryptoError> {
    Err(CryptoError::IoError)
}
