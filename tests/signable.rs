use walleth::Signable;

const MESSAGE_DIGEST: &str = "ecd0e108a98e192af1d2c25055f4e3bed784b5c877204e73219a5203251feaab";

mod new {
  use super::*;

  #[test]
  fn it_creates_a_new_signable() {
    let signable = Signable::new(b"Hello world!");
    assert_eq!(
      signable.to_signable_message().to_string(),
      MESSAGE_DIGEST.to_string()
    );
  }
}

mod from_str {
  use super::*;

  #[test]
  fn it_creates_a_new_signable() {
    let signable = Signable::from_str("Hello world!");
    assert_eq!(
      signable.to_signable_message().to_string(),
      MESSAGE_DIGEST.to_string()
    );
  }
}

mod from_bytes {
  use super::*;

  #[test]
  fn it_creates_a_new_signable() {
    let signable = Signable::from_bytes(b"Hello world!");
    assert_eq!(
      signable.to_signable_message().to_string(),
      MESSAGE_DIGEST.to_string()
    );
  }
}
