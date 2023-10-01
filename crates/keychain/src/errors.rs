use std::{error::Error, fmt::Display};

use utils::observable::ObservableError;
use vault::VaultError;

#[derive(Debug)]
pub enum KeychainError {
  VaultError(VaultError),
  KeyNotFoundForAddress(String),
  EventEmitterError(ObservableError),
  KeyNotFoundForIndex(usize),
}

impl Display for KeychainError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      KeychainError::VaultError(error) => write!(f, "Vault error: {}", error),
      KeychainError::KeyNotFoundForAddress(address) => {
        write!(f, "Key not found for address: {}", address)
      }
      KeychainError::EventEmitterError(error) => write!(f, "Event emitter error: {}", error),
      KeychainError::KeyNotFoundForIndex(index) => write!(f, "Key not found for index {}", index),
    }
  }
}

impl From<VaultError> for KeychainError {
  fn from(error: VaultError) -> Self {
    Self::VaultError(error)
  }
}

impl From<ObservableError> for KeychainError {
  fn from(error: ObservableError) -> Self {
    Self::EventEmitterError(error)
  }
}

impl Error for KeychainError {}
