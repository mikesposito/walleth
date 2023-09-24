use std::{error::Error, fmt::Display};

use crate::{AccountError, SignerError};

#[derive(Debug)]
pub enum VaultError {
  ForbiddenWhileLocked,
  AccountCreation,
  InvalidPassword,
  InvalidMnemonic,
  SignerCreation,
  KeyDerivation,
  AlreadyUnlocked,
  SafeCreation,
  SafeDecrypt,
}

impl Display for VaultError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ForbiddenWhileLocked => write!(f, "Forbidden while locked"),
      Self::AccountCreation => write!(f, "Account creation error"),
      Self::InvalidPassword => write!(f, "Invalid password"),
      Self::InvalidMnemonic => write!(f, "Invalid mnemonic"),
      Self::SignerCreation => write!(f, "Signer creation error"),
      Self::KeyDerivation => write!(f, "Key derivation error"),
      Self::AlreadyUnlocked => write!(f, "Already unlocked"),
      Self::SafeCreation => write!(f, "Safe creation error"),
      Self::SafeDecrypt => write!(f, "Safe decryption error"),
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

impl Error for VaultError {}
