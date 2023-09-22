use secp256k1::Message;

use crate::utils::crypto::sha3;

#[derive(Debug, Clone)]
pub struct Signable {
  message: Message,
}

impl Signable {
  /// Create a new signable message from a
  /// message digest byte array
  pub fn new(message: &[u8]) -> Self {
    Self {
      message: digest_bytes(message),
    }
  }

  /// Parse a string into a byte array of a message digest
  pub fn from_str(str: &str) -> Self {
    Signable {
      message: digest_str(str),
    }
  }

  /// Parse a string into a byte array of a message digest
  pub fn from_bytes(bytes: &[u8]) -> Self {
    Signable {
      message: digest_bytes(bytes),
    }
  }

  /// Get the message digest to be signed
  pub fn to_signable_message(&self) -> Message {
    self.message
  }
}

/// Digest a message string
pub fn digest_str(message: &str) -> Message {
  Message::from_slice(&sha3::hash(message.as_bytes())).unwrap()
}

/// Digest message bytes
pub fn digest_bytes(message: &[u8]) -> Message {
  // Unwrap is safe because the hash is always 32 bytes
  Message::from_slice(&sha3::hash(message)).unwrap()
}
