use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};

use super::AccountError;
use utils::{
  crypto::sha3::keccak256,
  hex::{add0x, assert_is_valid_hex_address, encode},
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Account<T> {
  pub address: String,
  pub public_key: Vec<u8>,
  pub path: T,
}

impl<T> Account<T> {
  /// Create a new `Account` from an extended public key
  pub fn from_public_key(public_key: &PublicKey, path: T) -> Result<Self, AccountError> {
    let extended_address = encode(&keccak256(&public_key.serialize()));
    let address = extended_address[extended_address.len() - 40..].to_string();

    assert_is_valid_hex_address(&address)?;

    Ok(Account {
      address: add0x(&address).to_owned(),
      public_key: public_key.serialize().to_vec(),
      path,
    })
  }

  /// Create a new `Account` from a private key
  pub fn from_private_key(private_key: [u8; 32], path: T) -> Result<Self, AccountError> {
    let secp = Secp256k1::new();
    let public_key = SecretKey::from_slice(&private_key)
      .or(Err(AccountError::InvalidPrivateKey))?
      .public_key(&secp);

    Self::from_public_key(&public_key, path)
  }
}
