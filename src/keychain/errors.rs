use crate::VaultError;

#[derive(Debug)]
pub enum KeychainError {
  VaultError(VaultError),
  KeyNotFoundForAddress(String),
}

impl From<VaultError> for KeychainError {
  fn from(error: VaultError) -> Self {
    Self::VaultError(error)
  }
}

#[derive(Debug)]
pub enum AccountError {
  InvalidPublicKey,
}
