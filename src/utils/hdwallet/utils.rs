use bip32::{DerivationPath, Language, Mnemonic, Seed};
use rand_core::OsRng;

/// Generate a new mnemonic phrase
/// with 12 words and in English
///
/// # Example
/// ```
/// use walleth::generate_english_mnemonic;
///
/// let mnemonic = generate_english_mnemonic();
/// ```
pub fn generate_english_mnemonic() -> Mnemonic {
  Mnemonic::random(&mut OsRng, Language::English)
}

/// Generate a new seed from a random english mnemonic phrase
/// with an empty password
///
/// # Example
/// ```
/// use walleth::generate_seed;
///
/// let seed = generate_seed();
/// ```
pub fn generate_seed() -> Seed {
  generate_english_mnemonic().to_seed("")
}

/// Generate a new seed from a mnemonic phrase
/// with an empty password
/// and return it as a vector of bytes
///
/// # Example
/// ```
/// use walleth::generate_seed_bytes;
///
/// let seed = generate_seed_bytes();
/// ```
pub fn generate_seed_bytes() -> Vec<u8> {
  generate_english_mnemonic()
    .to_seed("")
    .as_bytes()
    .to_vec()
}

/// Parse a mnemonic phrase
/// and return it as a `Mnemonic`
///
/// # Example
/// ```
/// use walleth::parse_mnemonic;
///
/// let mnemonic = parse_mnemonic("oak ethics setup flat gesture security must leader people boring donkey one".to_string());
/// ```
pub fn parse_mnemonic(phrase: String) -> Result<Mnemonic, String> {
  match Mnemonic::new(phrase, Default::default()) {
    Ok(mnemonic) => Ok(mnemonic),
    Err(e) => Err(e.to_string()),
  }
}

/// Get a derivation path from an account, change and index
/// and return it as a `DerivationPath`
///
/// # Example
/// ```
/// use walleth::get_derivation_path;
///
/// let path = get_derivation_path(0, 0, 0);
/// ```
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
