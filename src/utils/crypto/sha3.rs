use sha3::{Digest, Keccak256};

/// Computes the Keccak-256 hash of the input data.
///
/// # Example
///
/// ```
/// use walleth::utils::crypto::sha3::keccak256;
///
/// let hash = keccak256(&"Hello, world!".as_bytes());
///
/// assert_eq!(
///  hash,
///  [
///    182, 225, 109, 39, 172,
///    90, 180, 39, 167, 246,
///    137, 0, 172, 85, 89,
///    206, 39, 45, 198, 195,
///    124, 130, 179, 224, 82,
///    36, 108, 130, 36, 76,
///    80, 228
///  ]
/// );
/// ```
///
pub fn keccak256(data: &[u8]) -> [u8; 32] {
  let mut hasher = Keccak256::new();
  hasher.update(data);
  hasher.finalize().into()
}
