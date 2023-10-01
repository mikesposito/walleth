use hex;

pub enum HexError {
  InvalidHex,
  InvalidHexLength,
  InvalidHexAddress,
}

/// Encode a byte array into a hex string
pub fn encode(data: &[u8]) -> String {
  hex::encode(data)
}

/// Decode a hex `&str` into a byte array
pub fn decode(data: &str) -> Result<Vec<u8>, HexError> {
  hex::decode(data).or(Err(HexError::InvalidHex))
}

/// Assert that a `&String` is a valid hex address
pub fn assert_is_valid_hex_address(value: &String) -> Result<(), HexError> {
  let unprefixed = remove0x(value);

  assert_is_hex(&unprefixed)?;

  if unprefixed.len() != 40 {
    return Err(HexError::InvalidHexLength);
  }

  Ok(())
}

/// Assert that a `&str` is a valid hex
pub fn assert_is_hex(value: &str) -> Result<(), HexError> {
  match decode(value) {
    Ok(_) => Ok(()),
    Err(_) => Err(HexError::InvalidHex),
  }
}

/// Remove the 0x prefix from a string
pub fn remove0x(value: &String) -> String {
  match value.starts_with("0x") {
    true => String::from(&value[2..]),
    _ => value.to_string(),
  }
}

/// Add the 0x prefix to a string
pub fn add0x(value: &String) -> String {
  match value.starts_with("0x") {
    true => value.to_string(),
    _ => format!("0x{}", value),
  }
}
