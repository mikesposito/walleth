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
