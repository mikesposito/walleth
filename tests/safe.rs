use walleth::{Safe, ChaCha20Poly1305Cipher};

mod from_plain_bytes {
	use super::*;

	#[test]
	fn it_should_create_safe() {
		let key = ChaCha20Poly1305Cipher::new_key();

		let safe = Safe::from_plain_bytes("metadata", &key, &[0, 1, 2, 3, 4]);

		assert!(safe.is_ok());
		assert_eq!(safe.unwrap().metadata, "metadata");
	}
}

mod decrypt {
	use super::*;

	#[test]
	fn it_should_decrypt_safe() {
		let key = ChaCha20Poly1305Cipher::new_key();
		let safe = Safe::from_plain_bytes("metadata", &key, &[0, 1, 2, 3, 4]).unwrap();

    let decrypted_bytes = safe.decrypt(&key);

		assert!(decrypted_bytes.is_ok());
		assert_eq!(decrypted_bytes.unwrap(), &[0, 1, 2, 3, 4]);
	}

  #[test]
  fn it_should_fail_with_wrong_key() {
    let key = ChaCha20Poly1305Cipher::new_key();
		let safe = Safe::from_plain_bytes("metadata", &key, &[0, 1, 2, 3, 4]).unwrap();

    let decrypted_bytes = safe.decrypt(&[0_u8; 32]);

    assert!(!decrypted_bytes.is_ok());
  }
}


