use std::fmt::Display;

use identity::{AccountError, IdentityError, SignerError};

#[derive(Debug)]
pub enum HDKeyError {
  GenericError,
  WrongDerivationPath,
  InvalidMnemonic,
  InvalidSignature,
  InvalidPrivateKey,
}

impl Display for HDKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::WrongDerivationPath => write!(f, "Wrong derivation path"),
      Self::InvalidSignature => write!(f, "Invalid signature"),
      Self::InvalidPrivateKey => write!(f, "Invalid private key"),
      Self::InvalidMnemonic => write!(f, "Invalid mnemonic"),
      Self::GenericError => write!(f, "Generic error"),
    }
  }
}

impl std::error::Error for HDKeyError {}

impl From<AccountError> for HDKeyError {
  fn from(_: AccountError) -> Self {
    Self::WrongDerivationPath
  }
}

impl From<SignerError> for HDKeyError {
  fn from(error: SignerError) -> Self {
    match error {
      SignerError::InvalidPrivateKey => Self::InvalidPrivateKey,
      SignerError::InvalidSignature => Self::InvalidSignature,
      _ => Self::GenericError,
    }
  }
}

impl Into<Box<dyn IdentityError>> for HDKeyError {
  fn into(self) -> Box<dyn IdentityError> {
    Box::new(self)
  }
}

impl IdentityError for HDKeyError {}
