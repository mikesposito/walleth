use walleth::{Controller, Keychain, Signable};

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

mod from_mnemonic {
  use super::*;

  #[test]
  fn it_creates_a_new_keychain_from_mnemonic() {
    let keychain = Keychain::from_mnemonic(MNEMONIC.to_string());
    assert!(keychain.is_ok());
  }

  #[test]
  fn it_fails_with_wrong_mnemonic() {
    let keychain = Keychain::from_mnemonic("this is wrong".to_string());
    assert!(!keychain.is_ok());
  }
}

mod add_account {
  use super::*;

  #[test]
  fn it_adds_a_new_account() {
    let mut keychain = Keychain::from_mnemonic(MNEMONIC.to_string()).unwrap();

    let key = keychain.add_account().unwrap();

    assert_eq!(keychain.get_state().accounts.len(), 1);
    assert_eq!(keychain.get_state().accounts[0].address, key.address);
    assert_eq!(
      keychain.get_state().accounts[0].address,
      "0x356281bf5382846adf421cf4d4a9421f5f158592".to_string()
    );
  }

  #[test]
  fn it_fails_when_locked() {
    let mut keychain = Keychain::from_mnemonic(MNEMONIC.to_string()).unwrap();
    keychain.lock("my password").unwrap();

    let account = keychain.add_account();

    assert!(!account.is_ok());
  }
}

mod use_signer {
  use super::*;
  use secp256k1::{PublicKey, Secp256k1};

  #[test]
  fn it_signs_a_message() {
    let mut keychain = Keychain::from_mnemonic(MNEMONIC.to_string()).unwrap();
    let key = keychain.add_account().unwrap();
    let message = Signable::from_bytes(b"Hello world!");

    let signature = keychain
      .use_signer(key.address, |signer| signer.sign(&message))
      .unwrap();

    assert!(Secp256k1::new()
      .verify_ecdsa(
        &message.to_signable_message(),
        &signature,
        &PublicKey::from_slice(&key.public_key.to_bytes()).unwrap()
      )
      .is_ok());
  }
}

mod lock {
  use super::*;

  #[test]
  fn it_locks_the_keychain() {
    let mut keychain = Keychain::from_mnemonic(MNEMONIC.to_string()).unwrap();

    keychain.lock("my password").unwrap();

    assert!(!keychain.add_account().is_ok());
  }
}

mod unlock {
  use super::*;

  #[test]
  fn it_unlocks_the_keychain() {
    let mut keychain = Keychain::new();
    let account = keychain.add_account().unwrap();
    keychain.lock("password").unwrap();

    let recovered_accounts = keychain.unlock("password").unwrap();

    assert_eq!(account.address, recovered_accounts[0].address);
  }
}

mod update {

  use super::*;

  #[test]
  fn it_updates_the_keychain_store() {
    let mut keychain = Keychain::new();
    keychain.add_account().unwrap();

    keychain.update(|state| {
      state.accounts = vec![];
    });

    assert_eq!(keychain.get_state().accounts.len(), 0);
  }
}

mod subscribe {
  // use std::sync::mpsc::channel;
  // use std::thread;

  // TODO: .subscribe listener should implement `Send` trait
  //#[test]
  //fn it_subscribes_to_keychain_updates() {
  //	let mut keychain = Keychain::new();
  //	let (tx, rx) = channel();
  //
  //	let handle = thread::spawn(move || {
  //		keychain.subscribe(move |state| {
  //			tx.send(state.accounts.len()).unwrap();
  //		});
  //	});
  //
  //	keychain.add_account().unwrap();
  //
  //	assert_eq!(rx.recv().unwrap(), 1);
  //
  //	handle.join().unwrap();
  //}
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
