use std::error::Error;

use hdkey::HDKey;
use identity::{IdentityError, Initializable};

use crate::KeychainError;

pub fn hdkey_factory(mnemonic: Option<String>) -> Result<HDKey, Box<dyn IdentityError>> {
  match mnemonic {
    Some(mnemonic) => Ok(HDKey::from_mnemonic_str(&mnemonic)?),
    None => Ok(HDKey::new()),
  }
}
