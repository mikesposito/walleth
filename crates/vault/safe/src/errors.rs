use std::fmt::{Display, Formatter, Result};

pub enum SafeError {
  Serialization(String),
  Deserialization(String),
}

impl Display for SafeError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      SafeError::Serialization(message) => write!(f, "Unable to serialize safe > {}", message),
      SafeError::Deserialization(message) => write!(f, "Unable to deserialize safe > {}", message),
    }
  }
}
