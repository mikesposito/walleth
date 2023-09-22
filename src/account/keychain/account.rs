use bip32::XPub;

use crate::{
  hex::{add0x, assert_is_valid_hex_address, encode},
  utils::crypto::sha3::keccak256,
};

#[derive(Clone, Debug)]
pub struct Account {
  pub address: String,
  pub public_key: XPub,
}

impl Account {
  /// Create a new `Account` from an extended public key
  pub fn from_extended_public_key(extended_public_key: &XPub) -> Result<Self, String> {
    let extended_address = encode(&keccak256(&extended_public_key.to_bytes()));
    let address = extended_address[extended_address.len() - 40..].to_string();

    assert_is_valid_hex_address(&address)?;

    Ok(Account {
      address: add0x(&address).to_owned(),
      public_key: extended_public_key.to_owned(),
    })
  }
}
