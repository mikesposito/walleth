use hex;

/// Encode a byte array into a hex string
pub fn encode(data: &[u8]) -> String {
  hex::encode(data)
}

/// Decode a hex string into a byte array
pub fn decode(data: &str) -> Result<Vec<u8>, String> {
  match hex::decode(data) {
    Ok(bytes) => Ok(bytes),
    Err(e) => Err(e.to_string()),
  }
}

/// Assert that a string is a valid hex address
///
/// # Example
///
/// ```
/// use walleth::utils::hex::assert_is_valid_hex_address;
///
/// assert!(assert_is_valid_hex_address(&"0x00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD".to_string()).is_ok());
/// ```
pub fn assert_is_valid_hex_address(value: &String) -> Result<(), String> {
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
/// use walleth::utils::hex::assert_is_hex;
///
/// let assertion = assert_is_hex(&"00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD".to_string());
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
/// use walleth::utils::hex::remove0x;
///
/// let unprefixed = remove0x(&"0x00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD".to_string());
/// assert_eq!(
///   unprefixed,
///   "00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD".to_string(),
/// );
/// ```
pub fn remove0x(value: &String) -> String {
  match value.starts_with("0x") {
    true => String::from(&value[2..]),
    _ => value.to_string(),
  }
}

/// Add the 0x prefix to a string
///
/// # Example
///
/// ```
/// use walleth::utils::hex::add0x;
///
/// assert_eq!(
///   add0x(&"00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD".to_string()),
///   "0x00C08c440DbDC3A2a9C9D99b30077a53Ba7eDEAD",
/// );
/// ```
pub fn add0x(value: &String) -> String {
  match value.starts_with("0x") {
    true => value.to_string(),
    _ => format!("0x{}", value),
  }
}
