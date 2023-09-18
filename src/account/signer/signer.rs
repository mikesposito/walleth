use bip32::XPrv;
use secp256k1::{ecdsa::Signature, Message, Secp256k1, SecretKey};

use crate::Signable;

/// A `Signer` is a safe wrapper around a Hierarchical Deterministic (HD) wallet
/// secret key. It can sign messages.
///
/// # Example
///
/// ```
/// use walleth::Signer;
///
/// let signer = Signer::new(private_key);
/// let signature = signer.sign(&[0; 32]);
/// ```
pub struct Signer {
	/// The secret key, derived from a private key
	secret_key: SecretKey,
}

impl Signer {
	/// Create a new signer from a private key
	///
	/// # Example
	///
	/// ```
	/// use walleth::Signer;
	///
	/// let signer = Signer::new(private_key);
	/// ```
	pub fn new(private_key: XPrv) -> Result<Self, String> {
		let secret_key = get_secret_key_from_private_key(&private_key)?;

		Ok(Self { secret_key })
	}

	/// Sign a message digest
	/// Returns a Signature
	///
	/// # Example
	///
	/// ```
	/// use walleth::Signer;
	///
	/// let signer = Signer::new(private_key);
	/// let signature = signer.sign(&[0; 32]);
	/// ```
	pub fn sign(&self, signable: &Signable) -> Result<Signature, String> {
		Ok(Secp256k1::new().sign_ecdsa(&signable.to_signable_message(), &self.secret_key))
	}
}

/// Parse a message digest
/// Returns a Message
///
/// # Example
///
/// ```
/// use walleth::signer::parse_message_digest;
///
/// let message = parse_message_digest(&[0; 32]);
/// ```
pub fn parse_message_digest(message: &[u8]) -> Result<Message, String> {
	match Message::from_slice(message) {
		Ok(message) => Ok(message),
		Err(e) => Err(e.to_string()),
	}
}

/// Get a secret key from a private key
/// Returns a SecretKey
///
/// # Example
///
/// ```
/// use walleth::get_secret_key_from_private_key;
///
/// let secret_key = get_secret_key_from_private_key(&private_key);
/// ```
pub fn get_secret_key_from_private_key(private_key: &XPrv) -> Result<SecretKey, String> {
	match SecretKey::from_slice(&private_key.to_bytes()) {
		Ok(keypair) => Ok(keypair),
		Err(e) => Err(e.to_string()),
	}
}
