use bip32::XPub;
use hex::decode;
use sha3::{Digest, Keccak256};

#[derive(Clone, Debug)]
pub struct Account {
  pub address: String,
  pub public_key: XPub,
}

impl Account {
  /// Create a new `Account` from an extended public key
  pub fn from_extended_public_key(extended_public_key: &XPub) -> Result<Self, String> {
    let address = extended_public_key_to_address(extended_public_key)?;

    assert_is_valid_hex_address(address.as_str())?;

    Ok(Account {
      address: add0x(address),
      public_key: extended_public_key.to_owned(),
    })
  }
}

/// Assert that a string is a valid hex address
///
/// # Example
///
/// ```
/// use walleth::account::assert_is_valid_hex_address;
///
/// println!("{}", assert_is_valid_hex_address("0x00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD").is_ok());
///
/// assert_eq!(assert_is_valid_hex_address("0x00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD").is_ok(), true);
/// ```
pub fn assert_is_valid_hex_address(value: &str) -> Result<(), String> {
  let unprefixed = remove0x(value);

  assert_is_hex(&unprefixed)?;

  if unprefixed.len() != 40 {
    return Err(format!(
      "String passed into assert_is_valid_hex_address is {} hex chars long instead of 40.",
      value.len()
    ));
  }

  Ok(())
}

/// Assert that a string is a valid hex
///
/// # Example
///
/// ```
/// use walleth::account::assert_is_hex;
///
/// let assertion = assert_is_hex("00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD");
///
/// assert!(assertion.is_ok());
/// ```
pub fn assert_is_hex(value: &str) -> Result<(), String> {
  match decode(value) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}

/// Remove the 0x prefix from a string
///
/// # Example
///
/// ```
/// use walleth::remove0x;
///
/// let unprefixed = remove0x("0x00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD");
/// assert_eq!(
///   unprefixed,
///   "00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD",
/// );
/// ```
pub fn remove0x(value: &str) -> &str {
  match value.starts_with("0x") {
    true => &value[2..],
    _ => value,
  }
}

/// Add the 0x prefix to a string
///
/// # Example
///
/// ```
/// use walleth::add0x;
///
/// assert_eq!(
///   add0x("00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD".to_string()),
///   "0x00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD".to_string(),
/// );
/// ```
pub fn add0x(value: String) -> String {
  match value.starts_with("0x") {
    true => value,
    _ => format!("0x{}", value),
  }
}

/// Convert an extended public key to an ethereum address
pub fn extended_public_key_to_address(extended_public_key: &XPub) -> Result<String, String> {
  let address = keccak_hash(&extended_public_key.to_bytes());
  Ok(address[(address.len() - 40)..].to_string())
}

/// Hash data using the keccak256 algorithm
fn keccak_hash<T>(data: &T) -> String
where
  T: ?Sized + AsRef<[u8]>,
{
  let mut hasher = Keccak256::new();
  hasher.update(data);
  let result = hasher.finalize();
  hex::encode(result)
}
