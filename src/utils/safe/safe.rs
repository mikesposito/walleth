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
///
/// # Example
///
/// ```
/// use walleth::Safe;
///
/// let safe = Safe::from_plain_bytes("metadata", &[0; 32], &[0, 1, 2, 3, 4]).unwrap();
///
/// assert_eq!(safe.metadata, "metadata");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Safe<T> {
  pub metadata: T,
  encrypted_bytes: Box<[u8]>,
  nonce: [u8; 24],
}

impl<T> Safe<T> {
  /// Create a new safe from unencrypted data
  /// Returns a Safe
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::{Safe, ChaCha20Poly1305Cipher};
  ///
  /// let key = ChaCha20Poly1305Cipher::new_key();
  /// let safe = Safe::from_plain_bytes("metadata", &[0_u8; 32], &[0, 1, 2, 3, 4]).unwrap();
  ///
  /// assert_eq!(safe.metadata, "metadata");
  /// ```
  pub fn from_plain_bytes(
    metadata: T,
    key: &CipherKey,
    plain_bytes: &[u8],
  ) -> Result<Self, String> {
    let (encrypted_bytes, nonce) = ChaCha20Poly1305Cipher::encrypt(&key, &plain_bytes)?;

    Ok(Safe {
      metadata,
      encrypted_bytes: encrypted_bytes.into_boxed_slice(),
      nonce,
    })
  }

  /// Decrypt the safe with a key. Returns the decrypted bytes.
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::Safe;
  ///
  /// let safe = Safe::from_plain_bytes("metadata", &[0; 32], &[0, 1, 2, 3, 4]).unwrap();
  ///
  /// let decrypted_bytes = safe.decrypt(&[0_u8; 32]);
  ///
  /// assert_eq!(decrypted_bytes.unwrap(), &[0, 1, 2, 3, 4]);
  /// ```
  pub fn decrypt(&self, key: &CipherKey) -> Result<Vec<u8>, String> {
    Ok(ChaCha20Poly1305Cipher::decrypt(
      &key,
      &self.nonce,
      &self.encrypted_bytes,
    )?)
  }
}

impl<T> Safe<T>
where
  T: Serialize,
{
  /// Serialize `Safe` to bytes
  pub fn to_bytes(&self) -> Result<Vec<u8>, SafeError> {
    Ok(bincode::serialize(&self).or(Err(SafeError::Serialization))?)
  }
}

impl<T> TryFrom<Vec<u8>> for Safe<T>
where
  T: DeserializeOwned,
{
  type Error = SafeError;

  /// Deserialize `Safe` from bytes
  fn try_from(bytes: Vec<u8>) -> Result<Self, SafeError> {
    Ok(bincode::deserialize(&bytes).or(Err(SafeError::Deserialization))?)
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
