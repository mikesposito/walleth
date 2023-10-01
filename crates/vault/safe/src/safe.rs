use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{ChaCha20Poly1305Cipher, CipherKey, SafeError};

/// A safe is a container for encrypted data.
/// It holds some metadata and encrypted bytes.
///
/// The metadata is not encrypted and can be used to
/// store information about the encrypted data.
///
/// The encrypted bytes are encrypted and can be used
/// to store sensitive information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Safe<T> {
  pub metadata: T,
  encrypted_bytes: Box<[u8]>,
  nonce: [u8; 24],
}

impl<T> Safe<T> {
  /// Create a new safe from unencrypted data
  /// Returns a Safe
  pub fn from_plain_bytes(
    metadata: T,
    key: &CipherKey,
    plain_bytes: Vec<u8>,
  ) -> Result<Self, String> {
    let (encrypted_bytes, nonce) = ChaCha20Poly1305Cipher::encrypt(key, &plain_bytes)?;

    Ok(Safe {
      metadata,
      encrypted_bytes: encrypted_bytes.into_boxed_slice(),
      nonce,
    })
  }

  /// Decrypt the safe with a key. Returns the decrypted bytes.
  pub fn decrypt(&self, key: &CipherKey) -> Result<Vec<u8>, String> {
    ChaCha20Poly1305Cipher::decrypt(key, &self.nonce, &self.encrypted_bytes)
  }
}

impl<T> Safe<T>
where
  T: Serialize,
{
  /// Serialize `Safe` to bytes
  pub fn to_bytes(&self) -> Result<Vec<u8>, SafeError> {
    bincode::serialize(&self).or(Err(SafeError::Serialization))
  }
}

impl<T> TryFrom<Vec<u8>> for Safe<T>
where
  T: DeserializeOwned,
{
  type Error = SafeError;

  /// Deserialize `Safe` from bytes
  fn try_from(bytes: Vec<u8>) -> Result<Self, SafeError> {
    bincode::deserialize(&bytes).or(Err(SafeError::Deserialization))
  }
}

impl<T> PartialEq for Safe<T>
where
  T: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.metadata == other.metadata
      && self.encrypted_bytes == other.encrypted_bytes
      && self.nonce == other.nonce
  }
}
