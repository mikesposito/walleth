#[derive(Debug)]
pub enum SignerError {
  InvalidPrivateKey,
}

impl std::fmt::Display for SignerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidPrivateKey => write!(f, "Invalid private key"),
    }
  }
}

impl std::error::Error for SignerError {}
