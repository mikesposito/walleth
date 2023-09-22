use sha3::{Digest, Sha3_256};

pub fn hash(data: &[u8]) -> [u8; 32] {
  let mut hasher = Sha3_256::new();
  hasher.update(data);
  hasher.finalize().into()
}
