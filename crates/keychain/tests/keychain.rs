use utils::Controller;
use walleth_keychain::Keychain;

const MNEMONIC: &str =
	"grocery belt target explain clay essay focus spatial skull brain measure matrix toward visual protect owner stone scale slim ghost panda exact combine game";

mod new {
  use super::*;

  #[test]
  fn it_creates_a_new_keychain() {
    let keychain = Keychain::new();
    assert_eq!(keychain.get_state().accounts.len(), 0);
  }
}

mod add_multi_keypair {
  use walleth_keychain::hdkey_factory;

  use super::*;

  #[test]
  fn it_adds_a_new_keypair_of_type_hd() {
    let mut keychain = Keychain::new();

    let hdkey = keychain.add_multi_keypair(hdkey_factory, None);

    assert!(hdkey.is_ok());
  }

  #[test]
  fn it_adds_a_new_keypair_with_mnemonic_arg() {
    let mut keychain = Keychain::new();

    let hdkey = keychain.add_multi_keypair(hdkey_factory, Some(MNEMONIC.to_string()));

    assert!(hdkey.is_ok());
  }

  #[test]
  fn it_fails_with_wrong_mnemonic() {
    let mut keychain = Keychain::new();

    let hdkey = keychain.add_multi_keypair(hdkey_factory, Some("wrong mnemonic".to_string()));

    assert!(hdkey.is_err());
  }
}

mod recover {
  use walleth_keychain::hdkey_factory;

use super::*;

  #[test]
  fn it_recovers_the_keychain() {
    let mut keychain = Keychain::new();
    println!("keychain: {:?}", keychain);
    keychain.add_multi_keypair(hdkey_factory, None).unwrap();
    keychain.add_multi_keypair(hdkey_factory, None).unwrap();
    keychain.add_multi_keypair(hdkey_factory, None).unwrap();
    let backup = keychain.backup("password").unwrap();

    let recovered = Keychain::restore(backup, "password").unwrap();

    assert_eq!(recovered, keychain);
  }
}

mod get_state {
  use super::*;

  #[test]
  fn it_gets_the_keychain_state() {
    let keychain = Keychain::new();

    let state = keychain.get_state();

    assert_eq!(state.accounts.len(), 0);
  }
}
