use secp256k1::{ecdsa::Signature, Secp256k1, SecretKey};

use crate::{Signable, SignerError};

/// A `Signer` is a safe wrapper around a Hierarchical Deterministic (HD) wallet
/// secret key. It can sign messages.
///
/// # Example
///
/// ```
/// use walleth::{Signer, Signable, HDWallet};
///
/// let private_key = HDWallet::new().private_key_at_path(0, 0, 0).unwrap();
/// let message = Signable::new(&[0; 32]);
///
/// let signer = Signer::new(private_key.to_bytes());
/// assert!(signer.is_ok());
///
/// let signature = signer.unwrap().sign(&message);
/// ```
pub struct Signer {
  /// The secret key, derived from a private key
  secret_key: SecretKey,
}

impl Signer {
  /// Create a new signer from a private key bytes
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::{Signer, Signable, HDWallet};
  ///
  /// let hdwallet = HDWallet::new();
  /// let private_key = hdwallet.private_key_at_path(0, 0, 0).unwrap();
  ///
  /// let signer = Signer::new(private_key.to_bytes());
  ///
  /// assert!(signer.is_ok());
  /// ```
  pub fn new(private_key: [u8; 32]) -> Result<Self, SignerError> {
    let secret_key =
      get_secret_key_from_bytes(private_key).or(Err(SignerError::InvalidPrivateKey))?;

    Ok(Self { secret_key })
  }

  /// Sign a message digest
  /// Returns a Signature
  ///
  /// # Example
  ///
  /// ```
  /// use walleth::{Signer, Signable, HDWallet};
  ///
  /// let hdwallet = HDWallet::new();
  /// let private_key = hdwallet.private_key_at_path(0, 0, 0).unwrap();
  /// let signer = Signer::new(private_key.to_bytes()).unwrap();
  /// let message = Signable::new(&[0; 32]);
  ///
  /// let signature = signer.sign(&message);
  /// ```
  pub fn sign(&self, signable: &Signable) -> Signature {
    Secp256k1::new().sign_ecdsa(&signable.to_signable_message(), &self.secret_key)
  }
}

/// Get a secret key from a private key
/// Returns a SecretKey
///
/// # Example
///
/// ```
/// use walleth::{get_secret_key_from_bytes, HDWallet};
///
/// let hdwallet = HDWallet::new();
/// let private_key = hdwallet.private_key_at_path(0, 0, 0).unwrap();
///
/// let secret_key = get_secret_key_from_bytes(private_key.to_bytes());
///
/// assert!(secret_key.is_ok());
/// ```
pub fn get_secret_key_from_bytes(private_key: [u8; 32]) -> Result<SecretKey, String> {
  match SecretKey::from_slice(&private_key) {
    Ok(keypair) => Ok(keypair),
    Err(_) => Err("Invalid private key".to_string()),
  }
}
