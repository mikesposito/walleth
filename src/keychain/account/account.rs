use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};

use crate::{
  hex::{add0x, assert_is_valid_hex_address, encode},
  utils::crypto::sha3::keccak256,
  AccountError,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Account {
  pub address: String,
  pub public_key: Vec<u8>,
}

impl Account {
  /// Create a new `Account` from an extended public key
  pub fn from_public_key(public_key: &PublicKey) -> Result<Self, AccountError> {
    let extended_address = encode(&keccak256(&public_key.serialize()));
    let address = extended_address[extended_address.len() - 40..].to_string();

    assert_is_valid_hex_address(&address)?;

    Ok(Account {
      address: add0x(&address).to_owned(),
      public_key: public_key.serialize().to_vec(),
    })
  }

  /// Create a new `Account` from a private key
  pub fn from_private_key(private_key: SecretKey) -> Result<Self, AccountError> {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    Self::from_public_key(&public_key)
  }
}
