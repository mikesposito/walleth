use std::{
  error::Error,
  fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum ObservableError {
  UnableToLockObserver,
}

impl Display for ObservableError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      ObservableError::UnableToLockObserver => write!(f, "Unable to lock observer"),
    }
  }
}

impl Error for ObservableError {}
