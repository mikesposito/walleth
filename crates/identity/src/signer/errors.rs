#[derive(Debug)]
pub enum SignerError {
  GenericError,
  InvalidPrivateKey,
  InvalidSignature,
}

impl std::fmt::Display for SignerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidPrivateKey => write!(f, "Invalid private key"),
      Self::InvalidSignature => write!(f, "Invalid signature"),
      Self::GenericError => write!(f, "Secp256k1 error"),
    }
  }
}

impl std::error::Error for SignerError {}

impl From<secp256k1::Error> for SignerError {
  fn from(error: secp256k1::Error) -> Self {
    match error {
      secp256k1::Error::InvalidSecretKey => Self::InvalidPrivateKey,
      secp256k1::Error::InvalidSignature => Self::InvalidSignature,
      _ => Self::GenericError,
    }
  }
}
