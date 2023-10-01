use bip32::{DerivationPath, Language, Mnemonic, Seed};
use rand_core::OsRng;

/// Generate a new mnemonic phrase
/// with 12 words and in English
pub fn generate_english_mnemonic() -> Mnemonic {
  Mnemonic::random(&mut OsRng, Language::English)
}

/// Generate a new seed from a random english mnemonic phrase
/// with an empty password
pub fn generate_seed() -> Seed {
  generate_english_mnemonic().to_seed("")
}

/// Generate a new seed from a mnemonic phrase
/// with an empty password
/// and return it as a vector of bytes
pub fn generate_seed_bytes() -> Vec<u8> {
  generate_english_mnemonic().to_seed("").as_bytes().to_vec()
}

/// Parse a mnemonic phrase
/// and return it as a `Mnemonic`
pub fn parse_mnemonic(phrase: String) -> Result<Mnemonic, String> {
  match Mnemonic::new(phrase, Default::default()) {
    Ok(mnemonic) => Ok(mnemonic),
    Err(e) => Err(e.to_string()),
  }
}

/// Get a derivation path from an account, change and index
/// and return it as a `DerivationPath`
pub fn get_derivation_path(
  account: usize,
  change: usize,
  index: usize,
) -> Result<DerivationPath, String> {
  match format!("m/44'/60'/{}'/{}/{}", account, change, index).parse() {
    Ok(path) => Ok(path),
    Err(e) => Err(e.to_string()),
  }
}
