use walleth_vault_safe::{ChaCha20Poly1305Cipher, Safe};

mod from_plain_bytes {
  use super::*;

  #[test]
  fn it_should_create_safe() {
    let key = ChaCha20Poly1305Cipher::new_key();
    let bytes = [0u8, 1u8, 2u8, 3u8, 4u8].to_vec();

    let safe = Safe::from_plain_bytes("metadata", &key, bytes);

    assert!(safe.is_ok());
    assert_eq!(safe.unwrap().metadata, "metadata");
  }
}

mod decrypt {
  use super::*;

  #[test]
  fn it_should_decrypt_safe() {
    let key = ChaCha20Poly1305Cipher::new_key();
    let bytes = [0u8, 1u8, 2u8, 3u8, 4u8].to_vec();
    let safe = Safe::from_plain_bytes("metadata", &key, bytes.clone()).unwrap();

    let decrypted_bytes = safe.decrypt(&key);

    assert!(decrypted_bytes.is_ok());
    assert_eq!(decrypted_bytes.unwrap(), bytes);
  }

  #[test]
  fn it_should_fail_with_wrong_key() {
    let key = ChaCha20Poly1305Cipher::new_key();
    let bytes = [0u8, 1u8, 2u8, 3u8, 4u8].to_vec();
    let safe = Safe::from_plain_bytes("metadata", &key, bytes).unwrap();

    let decrypted_bytes = safe.decrypt(&[0_u8; 32]);

    assert!(decrypted_bytes.is_err());
  }
}
