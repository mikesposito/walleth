use std::{error::Error, fmt::Display};

use crate::{AccountError, SafeError, SignerError};

#[derive(Debug)]
pub enum VaultError {
  ForbiddenWhileLocked,
  ForbiddenWhileUnlocked,
  AccountCreation,
  InvalidPassword,
  InvalidMnemonic,
  SignerCreation,
  KeyDerivation,
  AlreadyUnlocked,
  VaultRestoreFromBytes(String),
  SafeCreation,
  SafeDecrypt,
  SafeExport,
  SafeRestore,
}

impl Display for VaultError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ForbiddenWhileLocked => write!(f, "Forbidden while locked"),
      Self::ForbiddenWhileUnlocked => write!(f, "Forbidden while unlocked"),
      Self::AccountCreation => write!(f, "Account creation error"),
      Self::InvalidPassword => write!(f, "Invalid password"),
      Self::InvalidMnemonic => write!(f, "Invalid mnemonic"),
      Self::SignerCreation => write!(f, "Signer creation error"),
      Self::KeyDerivation => write!(f, "Key derivation error"),
      Self::AlreadyUnlocked => write!(f, "Already unlocked"),
      Self::VaultRestoreFromBytes(message) => {
        write!(f, "Vault restore from bytes error: {}", message)
      }
      Self::SafeCreation => write!(f, "Safe creation error"),
      Self::SafeDecrypt => write!(f, "Safe decryption error"),
      Self::SafeExport => write!(f, "Safe export error"),
      Self::SafeRestore => write!(f, "Safe restore error"),
    }
  }
}

impl From<AccountError> for VaultError {
  fn from(_: AccountError) -> Self {
    Self::AccountCreation
  }
}

impl From<SignerError> for VaultError {
  fn from(_: SignerError) -> Self {
    Self::SignerCreation
  }
}

impl From<SafeError> for VaultError {
  fn from(error: SafeError) -> Self {
    match error {
      SafeError::Serialization => Self::SafeExport,
      SafeError::Deserialization => Self::SafeRestore,
    }
  }
}

impl Error for VaultError {}
