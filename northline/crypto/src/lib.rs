pub mod errors;

// Re-export error type.
pub use errors::CryptoError;

// TODO (Phase 2 in TODO.md):
// - Define EncryptedConfig.
// - Implement:
//   - encrypt_config(plaintext: &[u8]) -> Result<EncryptedConfig, CryptoError>
//   - decrypt_config(cipher: &EncryptedConfig) -> Result<Vec<u8>, CryptoError>
//   - load_encrypted_config() -> Result<Option<EncryptedConfig>, CryptoError>
//   - save_encrypted_config(cfg: &EncryptedConfig) -> Result<(), CryptoError>
