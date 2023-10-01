use secp256k1::{ecdsa::Signature, Secp256k1, SecretKey};

use super::{Signable, SignerError};

/// A `Signer` is a safe wrapper around a Secp256k1 secret key. It can sign digested messages.
pub struct Signer {
  /// The secret key, derived from a private key
  secret_key: SecretKey,
}

impl Signer {
  /// Create a new signer from private key bytes
  pub fn new(private_key: [u8; 32]) -> Result<Self, SignerError> {
    let secret_key = SecretKey::from_slice(&private_key)?;

    Ok(Self { secret_key })
  }

  /// Sign a message digest
  pub fn sign(&self, signable: &Signable) -> Signature {
    Secp256k1::new().sign_ecdsa(&signable.to_signable_message(), &self.secret_key)
  }

  /// Verify signature
  pub fn verify(&self, signable: &Signable, signature: &[u8]) -> Result<(), SignerError> {
    let secp = Secp256k1::new();
    let public_key = self.secret_key.public_key(&secp);
    let signature = Signature::from_compact(signature)?;

    Ok(secp.verify_ecdsa(&signable.to_signable_message(), &signature, &public_key)?)
  }
}
