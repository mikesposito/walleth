use secp256k1::{PublicKey, Secp256k1};

use walleth::{Keychain, Signable, Controller};

const MNEMONIC: &str =
	"grocery belt target explain clay essay focus spatial skull brain measure matrix toward visual protect owner stone scale slim ghost panda exact combine game";

#[test]
fn it_creates_new_vault() {
	let keychain = Keychain::new();
	assert_eq!(keychain.get_state().accounts.len(), 0);
}

#[test]
fn it_adds_a_new_account() {
	let mut keychain = Keychain::new();

	keychain.add_account().unwrap();

	assert_eq!(keychain.get_state().accounts.len(), 1);
}

#[test]
fn it_creates_new_vault_from_existing_mnemonic() {
	Keychain::from_mnemonic(MNEMONIC.to_string()).unwrap();
}

#[test]
fn it_adds_the_same_key_given_the_same_mnemonic() {
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
fn it_signs_a_message_with_use_signer_hook() {
	let mut keychain = Keychain::from_mnemonic(MNEMONIC.to_string()).unwrap();
	let key = keychain.add_account().unwrap();
	let message = Signable::from_bytes(b"Hello world!").unwrap();
	let secp256k1 = Secp256k1::new();

	let signature = keychain
		.use_signer(key.address, |signer| signer.sign(&message).unwrap())
		.unwrap();

	assert!(secp256k1
		.verify_ecdsa(
			&message.to_signable_message(),
			&signature,
			&PublicKey::from_slice(&key.public_key.to_bytes()).unwrap()
		)
		.is_ok());
}
