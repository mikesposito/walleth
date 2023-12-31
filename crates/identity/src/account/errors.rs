use utils::hex::HexError;

#[derive(Debug)]
pub enum AccountError {
  InvalidHexAddress,
  InvalidKeyLength,
  InvalidPrivateKey,
}

impl std::fmt::Display for AccountError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidHexAddress => write!(f, "Invalid hex address"),
      Self::InvalidKeyLength => write!(f, "Invalid key length"),
      Self::InvalidPrivateKey => write!(f, "Invalid private key"),
    }
  }
}

impl From<HexError> for AccountError {
  fn from(error: HexError) -> Self {
    match error {
      HexError::InvalidHex => Self::InvalidHexAddress,
      HexError::InvalidHexLength => Self::InvalidHexAddress,
      HexError::InvalidHexAddress => Self::InvalidHexAddress,
    }
  }
}

impl std::error::Error for AccountError {}
