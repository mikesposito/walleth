use bitcoin_hashes::{sha256, Hash};
use secp256k1::Message;

#[derive(Debug, Clone)]
pub struct Signable {
	message: Message,
}

impl Signable {
	/// Create a new signable message from a
	/// message digest byte array
	pub fn new(message: &[u8]) -> Result<Self, String> {
		Ok(Self {
			message: digest_to_message(message)?,
		})
	}

	/// Parse a string into a byte array of a message digest
	pub fn from_str(str: &str) -> Result<Self, String> {
		Ok(Signable {
			message: digest_str(str)?,
		})
	}

	/// Parse a string into a byte array of a message digest
	pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
		Ok(Signable {
			message: digest_bytes(bytes)?,
		})
	}

	/// Get the message digest to be signed
	pub fn to_signable_message(&self) -> Message {
		self.message
	}
}

/// Digest a message string
pub fn digest_str(message: &str) -> Result<Message, String> {
	let hash = sha256::Hash::hash(message.as_bytes());
	digest_to_message(hash.as_byte_array())
}

/// Digest message bytes
pub fn digest_bytes(message: &[u8]) -> Result<Message, String> {
	let hash = sha256::Hash::hash(message);
	digest_to_message(hash.as_byte_array())
}

pub fn digest_to_message(digest: &[u8]) -> Result<Message, String> {
	match Message::from_slice(digest) {
		Ok(message) => Ok(message),
		Err(e) => Err(e.to_string()),
	}
}
