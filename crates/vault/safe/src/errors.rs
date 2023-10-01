use std::fmt::{Display, Formatter, Result};

pub enum SafeError {
  Serialization,
  Deserialization,
}

impl Display for SafeError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      SafeError::Serialization => write!(f, "Unable to serialize safe"),
      SafeError::Deserialization => write!(f, "Unable to deserialize safe"),
    }
  }
}
