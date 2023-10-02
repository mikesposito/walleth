use super::HDKey;
use identity::{IdentityError, Initializable};

pub fn hdkey_factory(mnemonic: Option<String>) -> Result<HDKey, Box<dyn IdentityError>> {
  match mnemonic {
    Some(mnemonic) => Ok(HDKey::from_mnemonic_str(&mnemonic)?),
    None => Ok(HDKey::new()),
  }
}
