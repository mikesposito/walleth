use sha3::{Digest, Keccak256};

/// Computes the Keccak-256 hash of the input data.
pub fn keccak256(data: &[u8]) -> [u8; 32] {
  let mut hasher = Keccak256::new();
  hasher.update(data);
  hasher.finalize().into()
}
