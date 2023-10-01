use crate::{ChaCha20Poly1305Cipher, CipherKey, SafeError};

/// A safe is a container for encrypted data.
/// It holds some metadata and encrypted bytes.
///
/// The metadata is not encrypted and can be used to
/// store information about the encrypted data.
///
/// The encrypted bytes are encrypted and can be used
/// to store sensitive information.
#[derive(Debug, Clone)]
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

impl<T> From<Safe<T>> for Vec<u8>
where
  T: TryFrom<Vec<u8>> + Into<Vec<u8>>,
{
  /// Serialize `Safe` to bytes
  fn from(safe: Safe<T>) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    let metadata_bytes = safe.metadata.into();

    bytes.append(&mut vec![u8::try_from(metadata_bytes.len()).unwrap()]);
    bytes.append(&mut metadata_bytes.into());
    bytes.append(&mut safe.encrypted_bytes.into());
    bytes.append(&mut safe.nonce.to_vec());

    bytes
  }
}

impl<T> TryFrom<Vec<u8>> for Safe<T>
where
  T: TryFrom<Vec<u8>> + Into<Vec<u8>>,
{
  type Error = SafeError;

  /// Deserialize `Safe` from bytes
  fn try_from(bytes: Vec<u8>) -> Result<Self, SafeError> {
    let metadata_len = bytes[0];
    let metadata = T::try_from(bytes[1..metadata_len as usize + 1].to_vec()).or(Err(
      SafeError::Deserialization("error deserializing metadata".to_string()),
    ))?;
    let encrypted_bytes = bytes[metadata_len as usize + 1..bytes.len() - 24].to_vec();
    let nonce = bytes[bytes.len() - 24..bytes.len()].to_vec();

    Ok(Safe {
      metadata,
      encrypted_bytes: encrypted_bytes.into_boxed_slice(),
      nonce: nonce.try_into().or(Err(SafeError::Deserialization(
        "unexpected bytes length".to_string(),
      )))?,
    })
  }
}

impl<T> PartialEq for Safe<T>
where
  T: PartialEq + TryFrom<Vec<u8>> + Into<Vec<u8>>,
{
  fn eq(&self, other: &Self) -> bool {
    self.metadata == other.metadata
      && self.encrypted_bytes == other.encrypted_bytes
      && self.nonce == other.nonce
  }
}
